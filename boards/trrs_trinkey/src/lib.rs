#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use hal::ehal;
pub use hal::pac;

use hal::clock::GenericClockController;
use hal::sercom::i2c;
use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_peripherals!(
    /// I2c Put on Sercom2 since we want to leave Sercom0 for user defined operations with the TRRS Jack
    Sercom2 { I2cSercom }
    Sercom0 {TrrsSercom}
);

/// Definitions related to pins and pin aliases
pub mod pins {
    use super::hal;

    hal::bsp_pins!(
        PA02 {
            /// TRRS Tip Pin
            name: tip
        }
        PA03 {
            /// TRRS Tip Switch Pin
            name: tip_switch
            aliases: {
                PullUpInput: TipSwitchSense
            }
        }
        PA06 {
            /// TRRS First Ring Pin
            name: ring_1
        }
        PA07 {
            /// TRRS First Ring Pin Switch
            name: ring_1_switch
            aliases: {
                PullUpInput: Ring1SwitchSense
            }
        }
        PA04 {
            /// TRRS Second Ring Pin
            name: ring_2
        }
        PA05 {
            /// TRRS Sleve Pin
            name: sleeve
        }

        PA08 {
            /// The I2C data line
            name: sda
            aliases: {
                AlternateD: Sda
            }
        }
        PA09 {
            /// The I2C clock line
            name: scl
            aliases: {
                AlternateD: Scl
            }
        }

        PA24 {
            /// The USB D- pad
            name: usb_dm
            aliases: {
                AlternateG: UsbDm
            }
        }
        PA25 {
            /// The USB D+ pad
            name: usb_dp
            aliases: {
                AlternateG: UsbDp
            }
        }

        PA01 {
            /// Neopixel data
            name: neo_pixel
        }


    );
}
pub use pins::*;

/// I2C pads for the labelled I2C peripheral
///
/// You can use these pads with other, user-defined [`i2c::Config`]urations.
pub type I2cPads = i2c::Pads<I2cSercom, Sda, Scl>;

/// I2C master for the labelled I2C peripheral
///
/// This type implements [`Read`](ehal::blocking::i2c::Read),
/// [`Write`](ehal::blocking::i2c::Write) and
/// [`WriteRead`](ehal::blocking::i2c::WriteRead).
pub type I2c = i2c::I2c<i2c::Config<I2cPads>>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: I2cSercom,
    pm: &mut pac::Pm,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom2_core(&gclk0).unwrap();
    let freq = clock.freq();
    let baud = baud.into();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(pm, sercom, pads, freq).baud(baud).enable()
}

#[cfg(feature = "usb")]
/// Convenience function for setting up USB
pub fn usb_allocator(
    usb: pac::Usb,
    clocks: &mut GenericClockController,
    pm: &mut pac::Pm,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(clock, pm, dm, dp, usb))
}
