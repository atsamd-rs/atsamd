#![no_std]

pub use atsamd_hal as hal;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
use hal::clock::GenericClockController;
pub use hal::ehal;
pub use hal::pac;
use hal::sercom::{
    v2::{uart, Sercom0, Sercom4},
    I2CMaster0,
};
use hal::time::Hertz;
#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};
use spi::Pads;

use crate::hal::sercom::v2::spi;
use crate::hal::sercom::v2::uart::{BaudMode, Oversampling};
pub use pins::*;

/// Definitions related to pins and pin aliases
pub mod pins {
    use super::hal;

    hal::bsp_pins!(
        PA02 {
            name: a0
        }
        PA04 {
            name: a1
        }
        PA10 {
            name: a2
        }
        PA11 {
            name: a3
        }
        PA08 {
            name: a4
            aliases: {
                AlternateC: Sda
            }
        }
        PA09 {
            name: a5
            aliases: {
                AlternateC: Scl
            }
        }
        PB08 {
            name: a6
            aliases: {
                AlternateD: UartTx
            }
        }
        PB09 {
            name: a7
            aliases: {
                AlternateD: UartRx
            }
        }
        PA07 {
            name: a8
            aliases: {
                AlternateD: Sclk
            }
        }
        PA05 {
            name: a9
            aliases: {
                AlternateD: Miso
            }
        }
        PA06 {
            name: a10
            aliases: {
                AlternateD: Mosi
            }
        }
        PA17 {
            name: led0
            aliases: {
                PushPullOutput: Led0
            }
        }
        PA18 {
            name: led1
            aliases: {
                PushPullOutput: Led1
            }
        }
        PA19 {
            name: led2
            aliases: {
                PushPullOutput: Led2
            }
        }
        PA24 {
            name: usb_dm
            aliases: {
                AlternateG: UsbDm
            }
        }
        PA25 {
            name: usb_dp
            aliases: {
                AlternateG: UsbDp
            }
        }
    );
}

/// UART pads for the labelled RX & TX pins
pub type UartPads = uart::Pads<Sercom4, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom4: pac::SERCOM4,
    pm: &mut pac::PM,
    uart_rx: impl Into<UartRx>,
    uart_tx: impl Into<UartTx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom4_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(pm, sercom4, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// I2C master for the labelled SDA & SCL pins
pub type I2C = I2CMaster0<Sda, Scl>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2C {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom0_core(&gclk0).unwrap();
    let baud = baud.into();
    let sda = sda.into();
    let scl = scl.into();
    I2CMaster0::new(clock, baud, sercom0, pm, sda, scl)
}

/// SPI pads for the labelled SPI peripheral
///
/// You can use these pads with other, user-defined [`spi::Config`]urations.
pub type SpiPads = Pads<Sercom0, Miso, Mosi, Sclk>;

/// SPI master for the labelled SPI peripheral
pub type Spi = spi::Spi<spi::Config<SpiPads>>;

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM0 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    sclk: impl Into<Sclk>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom4_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sclk.into());
    let pads = Pads::default().data_in(miso).data_out(mosi).sclk(sclk);
    spi::Config::new(pm, sercom0, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

#[cfg(feature = "usb")]
/// Convenience function for setting up USB
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(clock, pm, dm, dp, usb))
}
