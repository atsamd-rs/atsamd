#![no_std]
#![recursion_limit = "1024"]

pub use atsamd_hal as hal;
pub use hal::common::*;
pub use hal::pac;
pub use hal::same54::*;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

use gpio::{Floating, Input, Port};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster7, PadPin, SPIMaster6};
use hal::time::Hertz;

define_pins!(
    /// Maps the pins to their names
    struct Pins,
    pac: pac,

    /// LED Pin
    pin led0 = b21,

    /// RFM95
    pin rfm95_cap_bank = c13,
    pin rfm95_power_supply = b9,
    pin rfm95_radio_reset = a6,
    pin rfm95_spi_not = a7,
    pin rfm95_dio0 = d1,
    pin rfm95_dio1 = b6,
    pin rfm95_dio2 = b7,
    pin rfm95_dio3 = b8,
    pin rfm95_dio4 = a4,
    pin rfm95_dio5 = a5,

    /// ADXL313
    pin adxl313_power_supply = b10,
    pin adxl313_irq0 = b11,
    pin adxl313_irq1 = b12,
    pin adxl313_spi_not = b13,

    /// D7S
    pin d7s_power_supply = a8,
    pin d7s_irq0 = a9,
    pin d7s_irq1 = a10,

    /// TC72
    pin tc72_power_supply = d10,
    pin tc72_spi_not = d11,

    /// TODO ADC

    /// Button
    pin button = b20,

    /// The I2C data line
    pin sda = c12,
    /// The I2C clock line
    pin scl = d9,

    /// The SPI SCK
    pin sck = c17,
    /// The SPI MOSI
    pin mosi = c16,
    /// The SPI MISO
    pin miso = c19,
);

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM6 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom6: pac::SERCOM6,
    mclk: &mut pac::MCLK,
    sck: gpio::Pc17<Input<Floating>>,
    mosi: gpio::Pc16<Input<Floating>>,
    miso: gpio::Pc19<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster6<
    hal::sercom::Sercom6Pad3<gpio::Pc19<gpio::PfC>>,
    hal::sercom::Sercom6Pad0<gpio::Pc16<gpio::PfC>>,
    hal::sercom::Sercom6Pad1<gpio::Pc17<gpio::PfC>>,
> {
    let gclk0 = clocks.gclk0();
    SPIMaster6::new(
        &clocks.sercom6_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom6,
        mclk,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom7: pac::SERCOM7,
    mclk: &mut pac::MCLK,
    sda: gpio::Pc12<Input<Floating>>,
    scl: gpio::Pd9<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster7<
    hal::sercom::Sercom7Pad0<gpio::Pc12<gpio::PfC>>,
    hal::sercom::Sercom7Pad1<gpio::Pd9<gpio::PfC>>,
> {
    let gclk0 = clocks.gclk0();
    I2CMaster7::new(
        &clocks.sercom7_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom7,
        mclk,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}
