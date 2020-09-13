#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

#[cfg(feature = "panic_halt")]
pub extern crate panic_halt;

use hal::prelude::*;
use hal::*;

use gpio::{Floating, Input, Port, PfC, PfD};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster4, PadPin, UART5, SPIMaster1};
use hal::time::Hertz;

pub use hal::common::*;
pub use hal::samd21::*;
pub use hal::target_device as pac;


#[cfg(feature = "usb")]
use hal::clock::GenericClockController;
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

// The docs could be further improved with details of the specific channels etc
define_pins!(
    /// Maps the pins to their arduino names and the numbers printed on the board.
    /// Information from: <https://github.com/arduino/ArduinoCore-samd/blob/master/variants/nano_33_iot/variant.cpp>
    struct Pins,
    target_device: target_device,

    /// RX
    pin rx = b23,

    /// TX
    pin tx = b22,

    /// Digital 2: PWM, TC
    pin d2 = b10,

    /// Digital 3: PWM, TC
    pin d3 = b11,

    /// Digital 4: TCC
    pin d4 = a7,

    /// Digital 5: PWM, TCC, ADC
    pin d5 = a5,

    /// Digital 6: PWM, TCC, ADC
    pin d6 = a4,

    /// Digital 7: ADC
    pin d7 = a6,

    /// Digital 8
    pin d8 = a18,

    /// Digital 9: PWM, TCC
    pin d9 = a20,

    /// Digital 10: PWM, TCC
    pin d10 = a21,

    /// Digital 11/SCI MISO: PWM, TCC
    pin miso = a19,

    /// Digital 12/SCI MOSI: PWM, TCC
    pin mosi = a16,

    /// Digital 13/LED/SPI SCK: ON-BOARD-LED
    pin led_sck = a17,

    /// Analog 0: DAC
    pin a0 = a2,

    /// Analog 1
    pin a1 = b2,

    /// Analog 2: PWM, TCC
    pin a2 = a11,

    /// Analog 3: PWM, TCC
    pin a3 = a10,

    /// Analog 4/SDA
    pin sda = b8,

    /// Analog 5/SCL: PWM< TCC
    pin scl = b9,

    /// Analog 6
    pin a6 = a9,

    /// Analog 7
    pin a7 = b3,

    /// SPI (Lefacy ICSP) 1 / NINA MOSI
    pin nina_mosi = a12,
    /// SPI (Lefacy ICSP) 2 / NINA MISO
    pin nina_miso = a13,
    /// SPI (Lefacy ICSP) 3 / NINA CS
    pin nina_cs = a14,
    /// SPI (Lefacy ICSP) 4 / NINA SCK
    pin nina_sck = a15,
    pin nina_gpio0 = a27,
    pin nina_resetn = a8,
    pin nina_ack = a28,

    /// SerialNina 29: PWM, TC
    pin serial_nina29 = a22,
    /// SerialNina 30: PWM, TC
    pin serial_nina30 = a23,

    pin usb_dm = a24,
    pin usb_dp = a25,
    pin aref = a3,

    pin p34 = a30,
    pin p35 = a31,
);

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: gpio::Pa24<Input<Floating>>,
    dp: gpio::Pa25<Input<Floating>>,
    port: &mut Port,
) -> UsbBusAllocator<UsbBus> {
    use gpio::IntoFunction;

    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    UsbBusAllocator::new(UsbBus::new(
        usb_clock,
        pm,
        dm.into_function(port),
        dp.into_function(port),
        usb,
    ))
}


/// EXPERIMENTAL FEATURE STARTS HERE


/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom4: pac::SERCOM4,
    pm: &mut pac::PM,
    sda: gpio::Pb8<Input<Floating>>,
    scl: gpio::Pb9<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster4<
    hal::sercom::Sercom4Pad0<gpio::Pb8<PfD>>,
    hal::sercom::Sercom4Pad1<gpio::Pb9<PfD>>,
> {
    let gclk0 = clocks.gclk0();
    I2CMaster4::new(
        &clocks.sercom4_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom4,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}


/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom5: pac::SERCOM5,
    pm: &mut pac::PM,
    rx: gpio::Pb23<Input<Floating>>, 
    tx: gpio::Pb22<Input<Floating>>, 
    port: &mut Port,
) -> UART5<
    hal::sercom::Sercom5Pad3<gpio::Pb23<PfD>>,
    hal::sercom::Sercom5Pad2<gpio::Pb22<PfD>>,
    (),
    (),
> {
    let gclk0 = clocks.gclk0();

    UART5::new(
        &clocks.sercom5_core(&gclk0).unwrap(),
        baud.into(),
        sercom5,
        pm,
        (rx.into_pad(port), tx.into_pad(port)),
    )
}


/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM4 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom1: pac::SERCOM1,
    pm: &mut pac::PM,
    sck: gpio::Pa17<Input<Floating>>,
    mosi: gpio::Pa16<Input<Floating>>,
    miso: gpio::Pa19<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster1<
    hal::sercom::Sercom1Pad3<gpio::Pa19<PfC>>,
    hal::sercom::Sercom1Pad0<gpio::Pa16<PfC>>,
    hal::sercom::Sercom1Pad1<gpio::Pa17<PfC>>,
> {
    let gclk0 = clocks.gclk0();

    SPIMaster1::new(
        &clocks.sercom1_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom1,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}
