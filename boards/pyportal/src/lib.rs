#![no_std]
#![recursion_limit = "1024"]

pub mod pins;
use atsamd_hal as hal;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use crate::pins::Pins;
pub use hal::target_device::*;
pub use hal::*;

use gpio::{Floating, Input, Port};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster5, PadPin, SPIMaster2};
use hal::time::Hertz;

#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;
#[cfg(feature = "usb")]
use usb_device::bus::UsbBusWrapper;

/// This powers up SERCOM2 and configures it for use as an
/// SPI Master in SPI Mode 0.
/// Unlike the `flash_spi_master` function, this
/// one does not accept a CS pin; configuring a pin for CS
/// is the responsibility of the caller, because it could be
/// any OutputPin, or even a pulled up line on the slave.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom2: SERCOM2,
    mclk: &mut MCLK,
    sck: gpio::Pa13<Input<Floating>>,
    mosi: gpio::Pa12<Input<Floating>>,
    miso: gpio::Pa14<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster2<
        hal::sercom::Sercom2Pad2<gpio::Pa14<gpio::PfC>>,
        hal::sercom::Sercom2Pad0<gpio::Pa12<gpio::PfC>>,
        hal::sercom::Sercom2Pad1<gpio::Pa13<gpio::PfC>>,
    > {
    let gclk0 = clocks.gclk0();
    SPIMaster2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom2,
        mclk,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom5: SERCOM5,
    mclk: &mut MCLK,
    sda: gpio::Pb2<Input<Floating>>,
    scl: gpio::Pb3<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster5<
        hal::sercom::Sercom5Pad0<gpio::Pb2<gpio::PfD>>,
        hal::sercom::Sercom5Pad1<gpio::Pb3<gpio::PfD>>
    > {
    let gclk0 = clocks.gclk0();
    I2CMaster5::new(
        &clocks.sercom5_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom5,
        mclk,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}
