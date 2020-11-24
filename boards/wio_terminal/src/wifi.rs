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
    consts::{U128, U512},
    BBBuffer, Consumer, Producer,
};

use cortex_m::interrupt::CriticalSection;
use cortex_m::peripheral::NVIC;

use nb::block;

mod erpc;
mod sys_rpcs;
use sys_rpcs::*;
mod wifi_rpcs;
use wifi_rpcs::*;

/// Wifi methods
pub mod rpc {
    pub use super::sys_rpcs::*;
    pub use super::wifi_rpcs::*;
}

/// A Wifi stack error.
#[derive(Debug, Clone)]
pub enum Error {
    RxOverrun,
    RxRead,
    NotReady,
}

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

pub struct Wifi {
    _pwr: Pa18<Output<PushPull>>,
    ready: Pc20<Input<Floating>>,
    dir_mosi: Pa19<Input<Floating>>,
    uart: WifiUART,

    rx_buff_isr: Producer<'static, U512>,
    rx_buff_input: Consumer<'static, U512>,
    tx_buff_isr: Consumer<'static, U128>,
    tx_buff_input: Producer<'static, U128>,

    sequence: u32,
    fault: Option<Error>,
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

        let fault = None;
        let sequence = 0;

        Ok(Wifi {
            _pwr: pwr,
            ready: pins.ready,
            dir_mosi: pins.dir,
            uart,
            rx_buff_isr,
            rx_buff_input,
            tx_buff_isr,
            tx_buff_input,
            fault,
            sequence,
        })
    }

    pub fn enable(&mut self, _cs: &CriticalSection, nvic: &mut NVIC) {
        unsafe {
            nvic.set_priority(interrupt::SERCOM0_0, 1);
            NVIC::unmask(interrupt::SERCOM0_0);
            // nvic.set_priority(interrupt::SERCOM0_1, 1);
            // NVIC::unmask(interrupt::SERCOM0_1);
            nvic.set_priority(interrupt::SERCOM0_2, 1);
            NVIC::unmask(interrupt::SERCOM0_2);
            // nvic.set_priority(interrupt::SERCOM0_OTHER, 1);
            // NVIC::unmask(interrupt::SERCOM0_OTHER);
        }

        self.uart.intenset(|w| {
            w.rxc().set_bit();
        });
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
                    self.fault = Some(Error::RxOverrun);
                }
            }
            Err(e) => {
                if e != nb::Error::WouldBlock {
                    self.fault = Some(Error::RxRead);
                }
            }
        };
    }

    /// Called from ISR: Handles the signal that the outgoing UART buffer
    /// has room for the next byte.
    pub fn _handle_data_empty(&mut self) {
        if let Ok(rgr) = self.tx_buff_isr.read() {
            let buf = rgr.buf();
            self.uart.write(buf[0]);
            rgr.release(1);
        } else {
            self.uart.intenclr(|w| {
                w.dre().set_bit();
            });
        }
    }

    pub fn debug_read(&mut self) -> Result<bbqueue::GrantR<'static, U512>, ()> {
        self.rx_buff_input.read().map_err(|_| ())
    }
    pub fn debug_usart(&mut self) -> atsamd_hal::target_device::sercom0::usart_int::status::R {
        self.uart.flags()
    }

    /// Reads the fault status, if any
    pub fn fault(&self) -> Option<Error> {
        self.fault.clone()
    }

    /// Issues an RPC, blocking till a response is recieved.
    pub fn blocking_rpc<'a, RPC: erpc::codec::RPC>(
        &mut self,
        mut rpc: RPC,
    ) -> Result<RPC::ReturnValue, erpc::Err<RPC::Error>> {
        let header = rpc.header(self.next_seq());
        self.write_frame(&header.as_bytes())
            .map_err(|_| erpc::Err::TXErr)?;

        // Read the frame header
        let fh = loop {
            let mut r = match self.rx_buff_input.read() {
                Ok(r) => r,
                Err(_) => {
                    continue;
                }
            };
            let b = r.buf();

            let pr = erpc::codec::FramePreamble::parse::<()>(b);
            if let Err(nom::Err::Incomplete(_)) = pr {
                continue;
            }
            if let Err(e) = pr {
                return Err(erpc::Err::Parsing(e));
            }
            let (_, fh) = pr.unwrap();

            drop(b);
            r.release(4);
            break fh;
        };

        // Read the payload, check CRC, hand off to underlying trait to decode
        let result = loop {
            let mut r = match self.rx_buff_input.read() {
                Ok(r) => r,
                Err(_) => {
                    continue;
                }
            };
            let b = r.buf();
            let l = b.len();

            if l < fh.msg_length as usize {
                continue;
            }

            let expect_crc = erpc::codec::crc16(&b[..l]);
            if expect_crc != fh.crc16 {
                r.release(l);
                return Err(erpc::Err::CRCMismatch);
            }

            let out = rpc.parse(b);
            drop(b);
            r.release(l);
            break out;
        };
        result
    }

    fn write_frame(&mut self, msg: &[u8]) -> Result<(), ()> {
        let header = erpc::codec::FramePreamble::new_from_msg(msg);

        self.tx(header.as_bytes().iter().chain(msg));
        Ok(())
    }

    fn tx<'a, D: Iterator<Item = &'a u8>>(&mut self, data: D) {
        for b in data {
            // block!(self.uart.write(*b))?;
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

#[macro_export]
macro_rules! wifi_singleton {
    ($global_name:ident) => {
        static mut $global_name: Option<Wifi> = None;
        static WIFI_RX: BBBuffer<U512> = BBBuffer(ConstBBBuffer::new());
        static WIFI_TX: BBBuffer<U128> = BBBuffer(ConstBBBuffer::new());

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

        // #[interrupt]
        // fn SERCOM0_1() {
        //     // Transmit Complete interrupt.
        //     $global_name.as_mut().map(|wifi| {
        //         wifi.handle_data_empty();
        //     });
        // }

        #[interrupt]
        fn SERCOM0_2() {
            // Recieve Complete interrupt.
            unsafe {
                $global_name.as_mut().map(|wifi| {
                    wifi._handle_rx();
                });
            }
        }

        // #[interrupt]
        // fn SERCOM0_Other() {
        // }
    };
}
