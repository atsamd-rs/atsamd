#![no_std]
#![recursion_limit = "1024"]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use hal::target_device::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, Port};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster5, PadPin, SPIMaster2};
use hal::time::Hertz;

#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;
#[cfg(feature = "usb")]
use usb_device::bus::UsbBusWrapper;

define_pins!(

    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    /// Analog pin 0.  Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pin speaker = a2,
    /// enable speaker amplifier
    pin speaker_enable = a27,

    /// Light sensor
    pin light = a7,

    // STEMMA I2C connectors
    /// Pin A1
    pin a1 = a4,
    /// Pin A4
    pin a4 = a5,

    /// D13 LED
    pin d13 = b23,
    /// Neopixel status LED
    pin neopixel = b22,

    // TFT(Thin-film-transistor liquid-crystal display) control ports
    /// TFT Reset
    pin tft_reset = a0,
    /// TFT RD
    pin tft_rd = b4,
    /// TFT RS
    pin tft_rs = b5,
    /// TFT CS
    pin tft_cs = b6,
    /// TFT TE
    pin tft_te = b7,
    /// TFT WR
    pin tft_wr = b9,
    /// TFT Backlight
    pin tft_backlight = b31,

    /// LCD Data 0
    pin lcd_data0 = a16,
    /// LCD Data 1
    pin lcd_data1 = a17,
    /// LCD Data 2
    pin lcd_data2 = a18,
    /// LCD Data 3
    pin lcd_data3 = a19,
    /// LCD Data 4
    pin lcd_data4 = a20,
    /// LCD Data 5
    pin lcd_data5 = a21,
    /// LCD Data 6
    pin lcd_data6 = a22,
    /// LCD Data 7
    pin lcd_data7 = a23,

    /// Touchscreen pins
    /// Touch YD
    pin touch_yd = b0,
    /// Touch XL
    pin touch_xl = b1,
    /// Touch YU
    pin touch_yu = a6,
    /// Touch XR
    pin touch_xr = b8,

    // ESP control - ESP32 WiFi
    /// Pin ESP CS
    pin esp_cs = b14,
    /// Pin ESP GPIO0
    pin esp_gpio0 = b15,
    /// Pin ESP Busy
    pin esp_busy = b16,
    /// Pin ESP Reset
    pin esp_reset = b17,
    /// Pin ESP RTS
    pin esp_rts = a15,

    // UART - Universal Asynchronous Receiver/Transmitter
    /// Pin TX
    pin tx = b12,
    /// Pin RX
    pin rx = b13,

    // SPI - Serial Peripheral Interface
    /// Pin MOSI
    pin mosi = a12,
    /// Pin SCK
    pin sck = a13,
    /// Pin MISO
    pin miso = a14,

    // I2C - ADT7410 Analog Devices temperature sensor
    /// Pin SDA
    pin sda = b2,
    /// Pin SCL
    pin scl = b3,

    /// Pin SD CS
    pin sd_cs = b30,
    /// Pin SD card detect
    pin sd_card_detect = a1,
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
    sercom2: SERCOM2,
    mclk: &mut MCLK,
    sck: gpio::Pa13<Input<Floating>>,
    mosi: gpio::Pa12<Input<Floating>>,
    miso: gpio::Pa14<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster2 {
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
        hal::sercom::SPI2Pinout::Dipo2Dopo0 {
            miso: miso.into_pad(port),
            mosi: mosi.into_pad(port),
            sck: sck.into_pad(port),
        },
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
) -> I2CMaster5 {
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
