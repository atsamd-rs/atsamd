#![no_std]
#![recursion_limit = "1024"]

pub mod pins;

use atsamd_hal as hal;

use hal::*;

pub use hal::common::*;
pub use hal::samd51::*;
pub use hal::target_device as pac;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
pub use pins::Pins;

use embedded_hal::timer::{CountDown, Periodic};
use gpio::{PfC, Port};
use hal::clock::GenericClockController;
use hal::gpio::*;
use hal::sercom::{I2CMaster2, SPIMaster1, UART3};
use hal::time::Hertz;

#[cfg(feature = "use_uart_debug")]
pub use hal::dbgprint;

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM1 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master<F: Into<Hertz>>(
    pins: pins::SPI,
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom1: pac::SERCOM1,
    mclk: &mut pac::MCLK,
    port: &mut Port,
) -> SPIMaster1<
    hal::sercom::Sercom1Pad3<gpio::Pb23<gpio::PfC>>,
    hal::sercom::Sercom1Pad0<gpio::Pa0<gpio::PfD>>,
    hal::sercom::Sercom1Pad1<gpio::Pa1<gpio::PfD>>,
> {
    pins.spi_master(clocks, bus_speed, sercom1, mclk, port)
}

/// Convenience for setting up the dotstar LED using bitbang'ed
/// SPI.
pub fn dotstar_bitbang<T: CountDown + Periodic>(
    pins: pins::Dotstar,
    port: &mut Port,
    timer: T,
) -> apa102_spi::Apa102<
    bitbang_hal::spi::SPI<Pb0<Input<PullUp>>, Pb3<Output<PushPull>>, Pb2<Output<PushPull>>, T>,
> {
    pins.dotstar(port, timer)
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    pins: pins::I2C,
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom4: pac::SERCOM2,
    mclk: &mut pac::MCLK,
    port: &mut Port,
) -> I2CMaster2<hal::sercom::Sercom2Pad0<gpio::Pa12<PfC>>, hal::sercom::Sercom2Pad1<gpio::Pa13<PfC>>>
{
    pins.i2c_master(clocks, bus_speed, sercom4, mclk, port)
}

/// Convenience for setting up the labelled TX, RX pins to
/// operate as a UART running at the specified frequency.
pub fn uart<F: Into<Hertz>>(
    pins: pins::UART,
    clocks: &mut GenericClockController,
    baud: F,
    sercom3: pac::SERCOM3,
    mclk: &mut pac::MCLK,
    port: &mut Port,
) -> UART3<
    hal::sercom::Sercom3Pad1<gpio::Pa16<gpio::PfD>>,
    hal::sercom::Sercom3Pad0<gpio::Pa17<gpio::PfD>>,
    (),
    (),
> {
    pins.uart(clocks, baud, sercom3, mclk, port)
}
