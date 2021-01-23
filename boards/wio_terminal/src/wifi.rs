use atsamd_hal::clock::GenericClockController;
use atsamd_hal::delay::Delay;
use atsamd_hal::gpio::*;
use atsamd_hal::prelude::*;
use atsamd_hal::target_device::{interrupt, MCLK};

use atsamd_hal::sercom::{PadPin, Sercom0Pad0, Sercom0Pad2, UART0};
use atsamd_hal::target_device::SERCOM0;
use atsamd_hal::time::Hertz;

use bbqueue;
use bbqueue::{
    consts::{U128, U512, U64},
    BBBuffer, Consumer, Producer,
};

use cortex_m::interrupt::CriticalSection;
use cortex_m::peripheral::NVIC;

pub use erpc::rpcs;
use seeed_erpc as erpc;

/// The set of pins which are connected to the RTL8720 in some way
pub struct WifiPins {
    pub pwr: Pa18<Input<Floating>>,
    pub rxd: Pc22<Input<Floating>>,
    pub txd: Pc23<Input<Floating>>,
    pub mosi: Pb24<Input<Floating>>,
    pub clk: Pb25<Input<Floating>>,
    pub miso: Pc24<Input<Floating>>,
    pub cs: Pc25<Input<Floating>>,
    pub ready: Pc20<Input<Floating>>,
    pub dir: Pa19<Input<Floating>>,
}

/// eRPC-based protocol to the RTL8720 chip
pub struct Wifi {
    _pwr: Pa18<Output<PushPull>>,
    uart: WifiUART,

    rx_buff_isr: Producer<'static, U512>,
    rx_buff_input: Consumer<'static, U512>,
    tx_buff_isr: Consumer<'static, U128>,
    tx_buff_input: Producer<'static, U128>,

    sequence: u32,
}

type WifiUART = UART0<Sercom0Pad2<Pc24<PfC>>, Sercom0Pad0<Pb24<PfC>>, (), ()>;

impl Wifi {
    pub fn init(
        pins: WifiPins,
        sercom0: SERCOM0,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
        port: &mut Port,
        delay: &mut Delay,
        rx_buff: &'static BBBuffer<U512>,
        tx_buff: &'static BBBuffer<U128>,
    ) -> Result<Wifi, ()> {
        let gclk0 = clocks.gclk0();
        let tx: Sercom0Pad0<_> = pins.mosi.into_pad(port);
        let rx: Sercom0Pad2<_> = pins.miso.into_pad(port);
        let uart = UART0::new(
            &clocks.sercom0_core(&gclk0).ok_or(())?,
            Hertz(1843200),
            sercom0,
            mclk,
            (rx, tx),
        );
        delay.delay_ms(10u8);

        // Reset the RTL8720 MCU.
        let mut pwr = pins.pwr.into_push_pull_output(port);
        pwr.set_low()?;
        delay.delay_ms(100u8);
        pwr.set_high()?;
        delay.delay_ms(200u8);

        let (rx_buff_isr, rx_buff_input) = rx_buff.try_split().unwrap();
        let (tx_buff_input, tx_buff_isr) = tx_buff.try_split().unwrap();

        let sequence = 0;

        Ok(Wifi {
            _pwr: pwr,
            uart,
            rx_buff_isr,
            rx_buff_input,
            tx_buff_isr,
            tx_buff_input,
            sequence,
        })
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

        self.uart.intenset(|w| {
            w.rxc().set_bit();
        });
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
            Err(e) => {
                if e != nb::Error::WouldBlock {
                    panic!("unrecoverable read error");
                }
            }
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
            self.uart.intenclr(|w| {
                w.dre().set_bit();
            });
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

    fn write_frame(&mut self, msg: &heapless::Vec<u8, heapless::consts::U64>) -> Result<(), ()> {
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
            self.uart.intenset(|w| {
                w.dre().set_bit();
            });
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
    pub use atsamd_hal::gpio::Port;
    pub use atsamd_hal::sercom::{Sercom0Pad0, Sercom0Pad2, UART0};
    pub use atsamd_hal::target_device::SERCOM0;
    pub use atsamd_hal::target_device::{interrupt, MCLK};
    pub use bbqueue::{
        consts::{U128, U512},
        BBBuffer, ConstBBBuffer, Producer,
    };

    pub use cortex_m::interrupt::CriticalSection;
}

/// Declares static globals for the wifi controller, and wires up interrupts.
#[macro_export]
macro_rules! wifi_singleton {
    ($global_name:ident) => {
        static mut $global_name: Option<Wifi> = None;
        static WIFI_RX: BBBuffer<U512> = BBBuffer(ConstBBBuffer::new());
        static WIFI_TX: BBBuffer<U128> = BBBuffer(ConstBBBuffer::new());

        /// Initializes the wifi controller from within an interrupt-free context.
        unsafe fn wifi_init(
            _cs: &CriticalSection,
            pins: WifiPins,
            sercom0: SERCOM0,
            clocks: &mut GenericClockController,
            mclk: &mut MCLK,
            port: &mut Port,
            delay: &mut Delay,
        ) -> Result<(), ()> {
            unsafe {
                $global_name = Some(Wifi::init(
                    pins, sercom0, clocks, mclk, port, delay, &WIFI_RX, &WIFI_TX,
                )?);
            }
            Ok(())
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
