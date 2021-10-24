#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use hal::ehal;
pub use hal::pac;

use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::sercom::v2::{spi, Sercom3};
use hal::sercom::I2CMaster5;
use hal::time::{Hertz, MegaHertz};

/// Definitions related to pins and pin aliases
pub mod pins {
    use super::hal;

    hal::bsp_pins!(
        PB09 {
            /// Pin 0, rx. Also analog input (A6)
            name: rx,
        },
        PB08 {
            /// Pin 1, tx. Also analog input (A7)
            name: tx,
        },
        PA28 {
            /// Pin 4, button A.
            name: d4,
        },
        PA14 {
            /// Pin 5, button B.
            name: d5,
        },
        PA15 {
            /// Pin 7, slide switch.
            name: d7,
        },
        PA30 {
            /// Pin 11, speaker enable.
            name: d11,
        },
        PA17 {
            /// Digital pin number 13, which is also attached to the red LED. PWM capable.
            name: d13,
            aliases: {
                PushPullOutput: RedLed
            }
        },
        PB02 {
            /// The I2C SDA. Also D2 and A5.
            name: sda,
            aliases: {
                AlternateD: Sda
            }
        },
        PB03 {
            /// The I2C SCL. Also D3 and A4
            name: scl,
            aliases: {
                AlternateD: Scl
            }
        },
        PB23 {
            /// The data line attached to the neopixel. Also D8.
            name: neopixel,
        },
        PA02 {
            /// The line attached to the speaker. Also D12 and A0.
            name: speaker,
        },
        PA05 {
            /// The SPI SCK. Also D6 and A1
            name: sck,
        },
        PA07 {
            /// The SPI MOSI. Also D10 and A3
            name: mosi,
        },
        PA06 {
            /// The SPI MISO. Also D9 and A2
            name: miso,
        },
        PA21 {
            /// The SCK pin attached to the on-board SPI flash
            name: flash_sck,
            aliases: {
                AlternateD: FlashSck
            }
        },
        PA20 {
            /// The MOSI pin attached to the on-board SPI flash
            name: flash_mosi,
            aliases: {
                AlternateD: FlashMosi
            }
        },
        PA16 {
            /// The MISO pin attached to the on-board SPI flash
            name: flash_miso,
            aliases: {
                AlternateD: FlashMiso
            }
        },
        PB22 {
            /// The CS pin attached to the on-board SPI flash
            name: flash_cs,
            aliases: {
                PushPullOutput: FlashCs
            }
        },
        PA00 {
            name: accel_sda,
        },
        PA01 {
            name: accel_scl,
        },
    );
}
pub use pins::*;

/// SPI pads for the flash chip
pub type FlashPads = spi::Pads<Sercom3, FlashMiso, FlashMosi, FlashSck>;

/// SPI master for the onboard SPI flash chip.
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type FlashSpi = spi::Spi<spi::Config<FlashPads>, spi::Duplex>;

/// Convenience for accessing the on-board SPI Flash device.
/// This powers up SERCOM3 and configures it for use as an
/// SPI Master.
pub fn flash_spi_master(
    clocks: &mut GenericClockController,
    sercom3: pac::SERCOM3,
    pm: &mut pac::PM,
    sck: impl Into<FlashSck>,
    mosi: impl Into<FlashMosi>,
    miso: impl Into<FlashMiso>,
    cs: impl Into<FlashCs>,
) -> (FlashSpi, FlashCs) {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom3_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (sck, mosi, miso, mut cs) = (sck.into(), mosi.into(), miso.into(), cs.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sck);
    let spi = spi::Config::new(pm, sercom3, pads, freq)
        .baud(MegaHertz(48))
        .spi_mode(spi::MODE_0)
        .enable();

    cs.set_high().unwrap();

    (spi, cs)
}

/// I2C master for the labelled SDA & SCL pins
pub type I2C = I2CMaster5<Sda, Scl>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom5: pac::SERCOM5,
    pm: &mut pac::PM,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2C {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom5_core(&gclk0).unwrap();
    let baud = baud.into();
    let sda = sda.into();
    let scl = scl.into();
    I2CMaster5::new(clock, baud, sercom5, pm, sda, scl)
}
