#![no_std]

pub use atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

pub use hal::common::*;

pub use hal::pac;

use gpio::{Floating, Input, Port};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin, SPIMaster0};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use gpio::v2::{AnyPin, PA24, PA25};
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;
#[cfg(feature = "usb")]
use usb_device::bus::UsbBusWrapper;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    pac: pac,

    /// D0/A0/DAC, EXTERNAL_INT_2
    /// Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pin d0 = a2,
    /// D1/A1/AREF, EXTERNAL_INT_3
    pin d1 = a3,
    /// D2/A2, TC4/WO[0], PWM4_CH0, TC4_CH0, EXTERNAL_INT_8
    pin d2 = b8,
    /// D3/A3, TC4/WO[1], PWM4_CH1, TC4_CH1, EXTERNAL_INT_9
    pin d3 = b9,
    /// D4, ACCEL_INT1,  EXTERNAL_INT_5
    pin accel_int1 = a21,
    /// D5, ACCEL_INT2,  EXTERNAL_INT_4
    pin accel_int2 = a20,
    /// D6/A6 PWM1_CH0, TCC1_CH0, EXTERNAL_INT_6
    pin d6 = a6,
    /// D7/A7, PWM1_CH1, TCC1_CH1, EXTERNAL_INT_7, SPI_SCK (if ENABLE_BASE_SPI)
    pin d7 = a7,
    /// D8/A8,  PWM0_CH0, TCC0_CH0, EXTERNAL_INT_NMI, SPI_MISO
    pin d8 = a8,
    /// D9/A9,  PWM0_CH1, TCC0_CH1, EXTERNAL_INT_9, SPI_SS
    pin d9 = a9,
    /// D10/A10, PWM0_CH2, TCC0_CH2, EXTERNAL_INT_10, SPI_MOSI
    pin d10 = a10,
    /// D11/A11, PWM0_CH3, TCC0_CH3, EXTERNAL_INT_11, SPI_SCK (if not ENABLE_BASE_SPI)
    pin d11 = a11,
    /// D12/A12, SERIAL_TX, EXTERNAL_INT_2
    pin d12 = b2,
    /// D13/A13, SERIAL_RX, EXTERNAL_INT_3
    pin d13 = b3,
    /// D14, OUTPUT
    /// red LED, set to low to turn LED on, to high to turn it off.
    /// PWM3_CH1, TC3_CH1
    pin led_red = a15,
    /// D15, OUTPUT
    /// green LED, set to low to turn LED on, to high to turn it off.
    /// PWM5_CH0, TC5_CH0
    pin led_green = b10,
    /// D16, OUTPUT
    /// blue LED, set to low to turn LED on, to high to turn it off.
    /// PWM5_CH1, TC5_CH1,
    pin led_blue = b11,
    /// D17, INPUT, GPS Timepulse,  EXTERNAL_INT_14
    /// this pin will turn to high every second synchronized to GPS time signal.
    pin gps_timepulse = a14,
    /// D18, OUTPUT, GPS Enable
    /// set this pin to high to turn the GPS module on,
    /// set it to low to conserve energy.
    pin gps_enable = a18,
    /// D19, INPUT, User Button,  EXTERNAL_INT_0
    /// this pin is high by default, pulled to low when button is pressed.
    pin button = a16,
    /// D22, OUTPUT, Power Enable,
    /// when running on battery and without USB connected, the regulator that suplies
    /// power has to enabled manually, this can be done to set this pin to high to enabe power.
    pin enable_pin_io = b22,
    /// D23, INPUT, EXTERNAL_INT_1
    pin switch_sense = a17,

    /// Analog, INPUT, battery voltage
    pin bat_volt = a5,

    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,

    /// SERIAL1_RX, EXTERNAL_INT_13
    pin serial1_rx = a13,
    /// SERIAL1_TX, EXTERNAL_INT_12
    pin serial1_tx = a12,

    /// The I2C data line
    pin i2c_sda = a22,
    /// The I2C clock line
    pin i2c_scl = a23,

    /// LoRa RESET
    pin lora_reset = a4,
    /// MAG_INT
    pin mag_int = a19,
);

/// Convenience for setting up pins D8-D11 for SPI.
/// This powers up SERCOM0 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    sck: gpio::Pa11<Input<Floating>>,
    mosi: gpio::Pa10<Input<Floating>>,
    miso: gpio::Pa8<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster0<
    hal::sercom::Sercom0Pad0<gpio::Pa8<gpio::PfC>>,
    hal::sercom::Sercom0Pad2<gpio::Pa10<gpio::PfC>>,
    hal::sercom::Sercom0Pad3<gpio::Pa11<gpio::PfC>>,
> {
    let gclk0 = clocks.gclk0();
    SPIMaster0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom0,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom3: pac::SERCOM3,
    pm: &mut pac::PM,
    sda: gpio::Pa22<Input<Floating>>,
    scl: gpio::Pa23<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster3<
    hal::sercom::Sercom3Pad0<gpio::Pa22<gpio::PfC>>,
    hal::sercom::Sercom3Pad1<gpio::Pa23<gpio::PfC>>,
> {
    let gclk0 = clocks.gclk0();
    I2CMaster3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom3,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}

#[cfg(feature = "usb")]
pub fn usb_bus(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: impl AnyPin<Id = PA24>,
    dp: impl AnyPin<Id = PA25>,
) -> UsbBusWrapper<UsbBus> {
    let gclk0 = clocks.gclk0();
    dbgprint!("making usb clock");
    let usb_clock = &clocks.usb(&gclk0).unwrap();
    dbgprint!("got clock");
    UsbBusWrapper::new(UsbBus::new(usb_clock, pm, dm, dp, usb))
}
