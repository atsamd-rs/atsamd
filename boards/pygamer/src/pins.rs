//! PyGamer pins

use super::{hal, pac::MCLK, pac::SERCOM5, target_device};

use crate::hal::gpio::{self, *};
use hal::define_pins;
use hal::sercom::{PadPin, UART5};
use hal::time::Hertz;

use super::pac::gclk::{genctrl::SRC_A, pchctrl::GEN_A};
use hal::clock::GenericClockController;

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

    // I2C (connected to LIS3DH accelerometer)
    /// STEMMA SDA
    pin sda = a12,
    /// STEMMA SCL 
    pin scl = a13,

    /// USB D- pin
    pin usb_dm = a24,
    /// USB D+ pin
    pin usb_dp = a25,

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

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let neopixel = self.neopixel;

        let usb = USB {
            dm: self.usb_dm,
            dp: self.usb_dp,
        };

        let uart = UART {
            rx: self.rx,
            tx: self.tx,
        };

        Sets {
            neopixel,
            usb,
            uart,
            port: self.port,
        }
    }
}

/// Sets of pins split apart by category
pub struct Sets {
    /// Neopixel (RGB LED) pins
    pub neopixel: gpio::Pa15<gpio::Input<gpio::Floating>>,

    /// USB pins
    pub usb: USB,

    /// UART (external pinout) pins
    pub uart: UART,

    /// Port
    pub port: Port,
}

/// USB pins
pub struct USB {
    pub dm: Pa24<Input<Floating>>,
    pub dp: Pa25<Input<Floating>>,
}

impl USB {
    #[cfg(feature = "usb")]
    pub fn usb_allocator(
        self,
        usb: super::pac::USB,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UsbBusAllocator<UsbBus> {
        clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
        let usb_gclk = clocks.get_gclk(GEN_A::GCLK2).unwrap();
        let usb_clock = &clocks.usb(&usb_gclk).unwrap();

        UsbBusAllocator::new(UsbBus::new(
            usb_clock,
            mclk,
            self.dm.into_function(port),
            self.dp.into_function(port),
            usb,
        ))
    }
}

/// UART pins
pub struct UART {
    pub tx: Pb16<Input<Floating>>,
    pub rx: Pb17<Input<Floating>>,
}

impl UART {
    /// Convenience for setting up the labelled TX, RX pins in the
    /// to operate as a UART device at the specified baud rate.
    pub fn uart<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        baud: F,
        sercom5: SERCOM5,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UART5<
        hal::sercom::Sercom5Pad1<gpio::Pb17<gpio::PfC>>,
        hal::sercom::Sercom5Pad0<gpio::Pb16<gpio::PfC>>,
        (),
        (),
    > {
        let gclk0 = clocks.gclk0();

        UART5::new(
            &clocks.sercom5_core(&gclk0).unwrap(),
            baud.into(),
            sercom5,
            mclk,
            (self.rx.into_pad(port), self.tx.into_pad(port)),
        )
    }
}
