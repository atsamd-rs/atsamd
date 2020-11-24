use atsamd_hal::clock::GenericClockController;
use atsamd_hal::delay::Delay;
use atsamd_hal::gpio::*;
use atsamd_hal::prelude::*;
use atsamd_hal::target_device::{interrupt, MCLK};

use atsamd_hal::sercom::{Error as SercomError, PadPin, Sercom0Pad0, Sercom0Pad2, UART0};
use atsamd_hal::target_device::SERCOM0;
use atsamd_hal::time::Hertz;

use bbqueue;
use bbqueue::{
    consts::{U128, U512},
    BBBuffer, ConstBBBuffer, Consumer, Producer,
};

use cortex_m::interrupt::CriticalSection;
use cortex_m::peripheral::NVIC;

use nb::block;

mod erpc;

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
    tx_buff: BBBuffer<U128>,

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
        let tx_buff = BBBuffer::new();

        let fault = None;
        let sequence = 0;

        Ok(Wifi {
            _pwr: pwr,
            ready: pins.ready,
            dir_mosi: pins.dir,
            uart,
            rx_buff_isr,
            rx_buff_input,
            tx_buff,
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

    pub fn debug_read(&mut self) -> Result<bbqueue::GrantR<'static, U512>, ()> {
        self.rx_buff_input.read().map_err(|_| ())
    }
    pub fn debug_usart(&mut self) -> atsamd_hal::target_device::sercom0::usart_int::status::R {
        self.uart.flags()
    }

    pub fn fault(&self) -> Option<Error> {
        self.fault.clone()
    }

    pub fn send_get_version_id(&mut self) {
        let msg = erpc::codec::Header {
            sequence: self.next_seq(),
            msg_type: erpc::id::MsgType::Invocation,
            service: erpc::id::Service::System,
            request: erpc::id::SystemRequest::VersionID.into(),
        }
        .as_bytes();

        self.flush_tx(&msg).unwrap();
    }

    fn flush_tx(&mut self, msg: &[u8]) -> Result<(), ()> {
        let crc = erpc::codec::crc16(msg);

        for b in &(msg.len() as u16).to_le_bytes() {
            block!(self.uart.write(*b))?;
        }
        for b in &(crc as u16).to_le_bytes() {
            block!(self.uart.write(*b))?;
        }
        for b in msg {
            block!(self.uart.write(*b))?;
        }
        Ok(())
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
                    pins, sercom0, clocks, mclk, port, delay, &WIFI_RX,
                )?);
            }
            Ok(())
        }

        #[interrupt]
        fn SERCOM0_0() {
            // Data Register Empty interrupt.
            unsafe {
                $global_name.as_mut().map(|wifi| {
                    // wifi.handle_data_empty();
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
