//! PyGamer pins

use super::{hal, target_device};

use crate::hal::gpio::{self, *};
use hal::define_pins;

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

    /// Analog pin 1
    pin a1 = a5,
    /// Analog pin 2
    pin a2 = b8,
    /// Analog pin 3
    pin a3 = b9,
    /// Analog pin 4
    pin a4 = a4,
    /// Analog pin 5
    pin a5 = a6,
    /// Analog pin 6
    pin a6 = b1,
    /// Light sensor (also Analog pin 7)
    pin light = b4,
    /// Digital pin 2 (also Analog pin 8) 
    pin d2 = b3,
    /// Digital pin 3 (also Analog pin 9)
    pin d3 = b2,
    /// Digital pin 5
    pin d5 = a16,
    /// Digital pin 6
    pin d6 = a18,
    /// Accelerometer interrupt pin (also d7)
    pin accel_irq = b14,
    /// Neopixel data line (controls all 5 neopixels, also d8)
    pin neopixel = a15,
    /// Digital pin 9
    pin d9 = a19,
    /// Digital pin 10
    pin d10 = a20,
    /// Digital pin 11
    pin d11 = a21,
    /// Digital pin 12
    pin d12 = a22,
    /// D13 LED/JACDAC
    pin d13 = a23,

    // TFT(Thin-film-transistor liquid-crystal display) control pins
    /// TFT MOSI
    pin tft_mosi = b15,
    /// TFT SCK
    pin tft_sck = b13,
    /// TFT Reset
    pin tft_reset = a0,
    /// TFT DC
    pin tft_dc = b5,
    /// TFT CS
    pin tft_cs = b12,
    /// TFT Backlight (also Analog pin 7)
    pin tft_backlight = a1,

    // UART - Universal Asynchronous Receiver/Transmitter
    /// Pin TX (d1)
    pin tx = b16,
    /// Pin RX (d0)
    pin rx = b17,

    // SPI - Serial Peripheral Interface (connected to sd card slot)
    /// Pin MISO
    pin miso = b22,
    /// Pin MOSI
    pin mosi = b23,
    /// Pin SCK
    pin sck = a17,

    // I2C
    /// STEMMA SDA
    pin sda = a12,
    /// STEMMA SCL 
    pin scl = a13,

    // Miscellanea
    /// SD card chip select (also d4)
    pin sd_cs = a14,
    /// Joystick X
    pin joy_x = b6,
    /// Joystick Y
    pin joy_y = b7,
    /// Button Latch
    pin button_latch = b0,
    /// Button Out
    pin button_out = b30,
    /// Button Clock
    pin button_clock = b31,
);
