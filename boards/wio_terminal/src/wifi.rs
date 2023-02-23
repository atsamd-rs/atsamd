use atsamd_hal::{
    clock::GenericClockController,
    delay::Delay,
    ehal::blocking::delay::DelayMs,
    ehal::digital::v2::OutputPin,
    ehal::serial::{Read, Write},
    pac::{interrupt, MCLK, SERCOM0},
    prelude::*,
    sercom::{uart, IoSet2, Sercom0},
};
use bbqueue::{self, BBBuffer, Consumer, Producer};
use heapless::consts::*;

use cortex_m::interrupt::CriticalSection;
use cortex_m::peripheral::NVIC;

pub use erpc::rpcs;
use seeed_erpc as erpc;

use super::pins::aliases::*;

use crate::WIFI_UART_BAUD;

/// The set of pins which are connected to the RTL8720 in some way
pub struct WifiPins {
    pub pwr: WifiPwrReset,
    pub rxd: WifiRxdReset,
    pub txd: WifiTxdReset,
    pub mosi: WifiTxReset,
    pub clk: WifiClkReset,
    pub miso: WifiRxReset,
    pub cs: WifiCsReset,
    pub ready: WifiReadyReset,
    pub dir: WifiDirReset,
}

/// eRPC-based protocol to the RTL8720 chip
pub struct Wifi {
    _pwr: WifiPwr,
    uart: WifiUart,

    rx_buff_isr: Producer<'static, 512>,
    rx_buff_input: Consumer<'static, 512>,
    tx_buff_isr: Consumer<'static, 128>,
    tx_buff_input: Producer<'static, 128>,

    sequence: u32,
}

/// UART pads for the labelled RX & TX pins
pub type WifiUartPads = uart::Pads<Sercom0, IoSet2, WifiRx, WifiTx>;

/// UART device for the labelled RX & TX pins
pub type WifiUart = uart::Uart<uart::Config<WifiUartPads>, uart::Duplex>;

impl Wifi {
    pub fn init(
        pins: WifiPins,
        sercom0: SERCOM0,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
        delay: &mut Delay,
        rx_buff: &'static BBBuffer<512>,
        tx_buff: &'static BBBuffer<128>,
    ) -> Wifi {
        let gclk0 = clocks.gclk0();

        let pads = uart::Pads::default().rx(pins.miso).tx(pins.mosi);
        let uart = uart::Config::new(
            mclk,
            sercom0,
            pads,
            clocks.sercom0_core(&gclk0).unwrap().freq(),
        )
        .baud(
            WIFI_UART_BAUD.Hz(),
            uart::BaudMode::Fractional(uart::Oversampling::Bits16),
        );

        delay.delay_ms(10u8);

        // Reset the RTL8720 MCU.
        let mut pwr: WifiPwr = pins.pwr.into();
        pwr.set_low().ok();
        delay.delay_ms(100u8);
        pwr.set_high().ok();
        delay.delay_ms(200u8);

        let (rx_buff_isr, rx_buff_input) = rx_buff.try_split().unwrap();
        let (tx_buff_input, tx_buff_isr) = tx_buff.try_split().unwrap();

        let sequence = 0;

        let uart = uart.enable();
        Wifi {
            _pwr: pwr,
            uart,
            rx_buff_isr,
            rx_buff_input,
            tx_buff_isr,
            tx_buff_input,
            sequence,
        }
    }

    /// Turns on internal interrupts. Call this after you have finished
    /// initializing the rest of your peripherals but before you start
    /// issuing RPCs against the wifi chip.
    pub fn enable(&mut self, _cs: &CriticalSection, nvic: &mut NVIC) {
        unsafe {
            nvic.set_priority(interrupt::SERCOM0_0, 1);
            NVIC::unmask(interrupt::SERCOM0_0);
            nvic.set_priority(interrupt::SERCOM0_2, 1);
            NVIC::unmask(interrupt::SERCOM0_2);
        }

        self.uart.enable_interrupts(uart::Flags::RXC);
    }

    /// Convenience function to connection an access point with the given
    /// network name and security parameters, and request an IP via DHCP.
    pub fn connect_to_ap<S: Into<heapless::String<U64>>, P: Into<heapless::String<U64>>>(
        &mut self,
        delay: &mut Delay,
        ssid: S,
        pw: P,
        security: erpc::Security,
    ) -> Result<erpc::IPInfo, erpc::Err<()>> {
        self.blocking_rpc(rpcs::AdapterInit {})?;
        self.blocking_rpc(rpcs::DHCPClientStop {
            interface: erpc::L3Interface::Station,
        })?;
        self.blocking_rpc(rpcs::WifiOff {})?;

        delay.delay_ms(35u8);

        self.blocking_rpc(rpcs::WifiOn {
            mode: erpc::WifiMode::Station,
        })?;

        self.blocking_rpc(rpcs::WifiConnect {
            ssid: ssid.into(),
            password: pw.into(),
            security: security,
            semaphore: 0,
        })?;

        self.blocking_rpc(rpcs::DHCPClientStart {
            interface: erpc::L3Interface::Station,
        })?;

        delay.delay_ms(25u8);
        self.blocking_rpc(rpcs::GetIPInfo {
            interface: erpc::L3Interface::Station,
        })
        .map_err(|_| erpc::Err::RPCErr(()))
    }

    /// Called from ISR: Handles the signal that the UART has recieved
    /// a byte that needs to be read.
    pub fn _handle_rx(&mut self) {
        match self.uart.read() {
            Ok(b) => {
                if let Ok(mut wgr) = self.rx_buff_isr.grant_exact(1) {
                    wgr[0] = b;
                    wgr.commit(1);
                } else {
                    panic!("overrun");
                }
            }
            Err(nb::Error::Other(e)) => {
                panic!("unrecoverable read error");
            }
            // Skip WouldBlock
            Err(nb::Error::WouldBlock) => (),
        };
    }

    /// Called from ISR: Handles the signal that the outgoing UART buffer
    /// has room for the next byte.
    pub fn _handle_data_empty(&mut self) {
        if let Ok(rgr) = self.tx_buff_isr.read() {
            let buf = rgr.buf();
            self.uart.write(buf[0]).ok();
            rgr.release(1);
        } else {
            self.uart.disable_interrupts(uart::Flags::DRE);
        }
    }

    /// Issues an RPC, blocking till a response is recieved.
    pub fn blocking_rpc<'a, RPC: erpc::RPC>(
        &mut self,
        mut rpc: RPC,
    ) -> Result<RPC::ReturnValue, erpc::Err<RPC::Error>> {
        // Transmit the request.
        let mut tx_buff = heapless::Vec::new();
        tx_buff
            .extend_from_slice(&rpc.header(self.next_seq()).as_bytes())
            .map_err(|_| erpc::Err::TXErr)?;
        rpc.args(&mut tx_buff);
        self.write_frame(&tx_buff).map_err(|_| erpc::Err::TXErr)?;

        loop {
            let result = self.recieve_rpc_response(&mut rpc);
            if let Err(erpc::Err::NotOurs) = result {
                continue;
            };
            break result;
        }
    }

    fn recieve_rpc_response<'a, RPC: erpc::RPC>(
        &mut self,
        rpc: &mut RPC,
    ) -> Result<RPC::ReturnValue, erpc::Err<RPC::Error>> {
        let fh = self.recieve_frame_header(rpc)?; // Read the frame header

        // Read the payload, check CRC, hand off to underlying trait to decode
        let mut buffer = [0u8; 2048];
        let sz = fh.msg_length as usize;
        self.recieve_bytes(&mut buffer[..sz]);

        fh.check_crc(&buffer[..sz])?;
        rpc.parse(&buffer[..sz])
    }

    fn recieve_frame_header<'a, RPC: erpc::RPC>(
        &mut self,
        _rpc: &mut RPC,
    ) -> Result<erpc::FrameHeader, erpc::Err<RPC::Error>> {
        let mut buffer = [0u8; 4];
        self.recieve_bytes(&mut buffer);

        match erpc::FrameHeader::parse(&buffer[..]) {
            Err(e) => Err(erpc::Err::Parsing(e)),
            Ok(fh) => Ok(fh.1),
        }
    }

    fn recieve_bytes(&mut self, mut buffer: &mut [u8]) {
        while buffer.len() > 0 {
            let r = match self.rx_buff_input.read() {
                Ok(r) => r,
                Err(_) => {
                    continue;
                }
            };
            let b = r.buf();
            let copy_amt = if b.len() < buffer.len() {
                b.len()
            } else {
                buffer.len()
            };

            for (i, b) in b[..copy_amt].iter().enumerate() {
                buffer[i] = *b;
            }
            buffer = &mut buffer[copy_amt..];

            r.release(copy_amt);
        }
    }

    fn write_frame(&mut self, msg: &heapless::Vec<u8, U64>) -> Result<(), ()> {
        let header = erpc::FrameHeader::new_from_msg(msg);
        self.tx(header.as_bytes().iter().chain(msg));
        Ok(())
    }

    fn tx<'a, D: Iterator<Item = &'a u8>>(&mut self, data: D) {
        for b in data {
            if let Ok(mut wgr) = self.tx_buff_input.grant_exact(1) {
                wgr[0] = *b;
                wgr.commit(1);
            }
            self.uart.enable_interrupts(uart::Flags::DRE);
        }
    }

    fn next_seq(&mut self) -> u32 {
        self.sequence += 1;
        self.sequence
    }
}

/// Imports necessary for using `wifi_singleton`.
pub mod wifi_prelude {
    pub use crate::wifi::*;
    pub use atsamd_hal::pac::SERCOM0;
    pub use atsamd_hal::pac::{interrupt, MCLK};
    pub use bbqueue::{BBBuffer, Producer};

    pub use cortex_m::interrupt::CriticalSection;
}

/// Declares static globals for the wifi controller, and wires up interrupts.
#[macro_export]
macro_rules! wifi_singleton {
    ($global_name:ident) => {
        static mut $global_name: Option<Wifi> = None;
        static WIFI_RX: BBBuffer<512> = BBBuffer::new();
        static WIFI_TX: BBBuffer<128> = BBBuffer::new();

        /// Initializes the wifi controller from within an interrupt-free context.
        unsafe fn wifi_init(
            _cs: &CriticalSection,
            pins: WifiPins,
            sercom0: SERCOM0,
            clocks: &mut GenericClockController,
            mclk: &mut MCLK,
            delay: &mut Delay,
        ) {
            unsafe {
                $global_name = Some(Wifi::init(
                    pins, sercom0, clocks, mclk, delay, &WIFI_RX, &WIFI_TX,
                ));
            }
        }

        #[interrupt]
        fn SERCOM0_0() {
            // Data Register Empty interrupt.
            unsafe {
                $global_name.as_mut().map(|wifi| {
                    wifi._handle_data_empty();
                });
            }
        }

        #[interrupt]
        fn SERCOM0_2() {
            // Recieve Complete interrupt.
            unsafe {
                $global_name.as_mut().map(|wifi| {
                    wifi._handle_rx();
                });
            }
        }
    };
}
