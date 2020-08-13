#![no_std]
#![recursion_limit = "1024"]

pub extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

pub use hal::target_device as pac;
pub use hal::common::*;
pub use hal::samd51::*;

use gpio::{Floating, Input, Port, PfC};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster5, PadPin, SPIMaster2, UART3};
use hal::time::Hertz;
use pac::MCLK;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    /// Analog pin 0.  Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pin a0 = a2,

    /// Analog Pin 1
    pin a1 = a5,
    /// Analog Pin 2
    pin a2 = a6,
    /// Analog Pin 3
    pin a3 = a4,
    /// Analog Pin 4
    pin a4 = b8,
    /// Analog Pin 5
    pin a5 = b9,

    /// Pin 0, rx.
    pin d0 = a23,
    /// Pin 1, tx.
    pin d1 = a22,
    /// Pin 2
    pin d2 = b17,
    /// Pin 3 
    pin d3 = b16,
    /// Pin 4
    pin d4 = b13,
    /// Pin 5
    pin d5 = b14,
    /// Pin 6
    pin d6 = b15,
    /// Pin 7
    pin d7 = b12,
    /// Pin 8
    pin d8 = a21,
    /// Pin 9
    pin d9 = a20,
    /// Pin 10
    pin d10 = a18,
    /// Pin 11
    pin d11 = a19,
    /// Pin 12
    pin d12 = a17,
    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a16,
    pin sda = b2,
    pin scl = b3,

    /// The data line attached to the neopixel.
    /// Is also attached to SWCLK.
    pin neopixel = b22,

    /// The SPI SCK attached the to 2x3 header
    pin sck = a13,
    /// The SPI MOSI attached the to 2x3 header
    pin mosi = a12,
    /// The SPI MISO attached the to 2x3 header
    pin miso = a14,

    /// The SCK pin attached to the on-board SPI flash
    pin flash_sck = b10,
    /// The MOSI pin attached to the on-board SPI flash
    pin flash_mosi = a8,
    /// The MISO pin attached to the on-board SPI flash
    pin flash_miso = a9,
    /// The CS pin attached to the on-board SPI flash
    pin flash_cs = b11,

    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,
);

/// Convenience for setting up the 2x3 header block for SPI.
/// This powers up SERCOM2 and configures it for use as an
/// SPI Master in SPI Mode 0.
/// Unlike the `flash_spi_master` function, this
/// one does not accept a CS pin; configuring a pin for CS
/// is the responsibility of the caller, because it could be
/// any OutputPin, or even a pulled up line on the slave.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom2: pac::SERCOM2,
    mclk: &mut pac::MCLK,
    sck: gpio::Pa13<Input<Floating>>,
    mosi: gpio::Pa12<Input<Floating>>,
    miso: gpio::Pa14<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster2<
        hal::sercom::Sercom2Pad2<gpio::Pa14<gpio::PfC>>,
        hal::sercom::Sercom2Pad0<gpio::Pa12<gpio::PfC>>,
        hal::sercom::Sercom2Pad1<gpio::Pa13<gpio::PfC>>
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
    sercom5: pac::SERCOM5,
    mclk: &mut pac::MCLK,
    sda: gpio::Pb2<Input<Floating>>,
    scl: gpio::Pb3<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster5<
        hal::sercom::Sercom5Pad0<gpio::Pb2<gpio::PfD>>,
        hal::sercom::Sercom5Pad1<gpio::Pb3<gpio::PfD>>,
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

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom3: pac::SERCOM3,
    mclk: &mut pac::MCLK,
    d0: gpio::Pa23<Input<Floating>>,
    d1: gpio::Pa22<Input<Floating>>,
    port: &mut Port,
) -> UART3<
        hal::sercom::Sercom3Pad1<gpio::Pa23<PfC>>,
        hal::sercom::Sercom3Pad0<gpio::Pa22<PfC>>, (), ()
    > {
    let gclk0 = clocks.gclk0();

    UART3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        baud.into(),
        sercom3,
        mclk,
        (d0.into_pad(port), d1.into_pad(port)),
    )
}

#[cfg(feature = "usb")]
pub fn usb_allocator(
    dm: gpio::Pa24<Input<Floating>>,
    dp: gpio::Pa25<Input<Floating>>,
    usb: pac::USB,
    clocks: &mut GenericClockController,
    mclk: &mut MCLK,
    port: &mut Port,
) -> UsbBusAllocator<UsbBus> {
    use pac::gclk::{genctrl::SRC_A, pchctrl::GEN_A};

    clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
    let usb_gclk = clocks.get_gclk(GEN_A::GCLK2).unwrap();
    let usb_clock = &clocks.usb(&usb_gclk).unwrap();

    UsbBusAllocator::new(UsbBus::new(
        usb_clock,
        mclk,
        dm.into_function_h(port),
        dp.into_function_h(port),
        usb,
    ))
}
