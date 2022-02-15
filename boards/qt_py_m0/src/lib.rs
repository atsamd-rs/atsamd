#![no_std]
#![deny(nonstandard_style)]
#![deny(rust_2018_idioms)]

//! # Adafruit QT Py Board Support Package
//!
//! This crate provides a board support package for the Adafruit QT Py board.
//! This device is a small form-factor breadboard-compatible SAMD21E-based
//! device with an on-board WS2812 LED ("neopixel"), [STEMMA I2C][stemma]
//! ([Qwiic][qwiic]-compatible) connector, and USB-C running USB
//! 2.0 connectivity.
//!
//! # Useful External Links
//! - [Adafruit QT Py Product Page][qtpy]
//! - [QT Py schematics][schematics]
//!
//! [qtpy]: https://learn.adafruit.com/adafruit-qt-py
//! [stemma]: https://www.adafruit.com/category/1005
//! [qwiic]: https://www.sparkfun.com/qwiic
//! [schematics]: https://cdn-learn.adafruit.com/assets/assets/000/095/390/original/adafruit_products_QTPy_sch.png

pub use atsamd_hal as hal;
pub use hal::pac;

use hal::bsp_pins;
use hal::clock::GenericClockController;
use hal::sercom::v2::spi;
use hal::sercom::v2::uart::{self, BaudMode, Oversampling};
use hal::sercom::v2::{Sercom0, Sercom2};
use hal::sercom::I2CMaster1;
use hal::time::Hertz;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

#[cfg(feature = "usb")]
use hal::usb::UsbBus;
#[cfg(feature = "usb")]
use usb_device::bus::UsbBusAllocator;

bsp_pins! {
    // General purpose pins.
    PA02 {
        name: a0
        aliases: {
            Reset: A0Reset
        }
    }
    PA03 {
        name: a1
        aliases: {
            Reset: A1Reset
        }
    }
    PA04 {
        name: a2
        aliases: {
            Reset: A2Reset
        }
    }
    PA05 {
        name: a3
        aliases: {
            Reset: A3Reset
        }
    }

    // UART port pins.
    PA06 {
        name: tx
        aliases: {
            AlternateD: UartTx,
            Reset: UartTxReset
        }
    },
    PA07 {
        name: rx
        aliases: {
            AlternateD: UartRx,
            Reset: UartRxReset
        }
    }

    // SPI port pins.
    PA09 {
        name: miso
        aliases: {
            AlternateD: SpiMiso,
            Reset: MisoReset
        }
    }
    PA10 {
        name: mosi
        aliases: {
            AlternateD: SpiMosi,
            Reset: MosiReset
        }
    }
    PA11 {
        name: sclk
        aliases: {
            AlternateD: SpiSck,
            Reset: SckReset
        }
    }

    // I2C port pins.
    PA16 {
        name: sda
        aliases: {
            AlternateC: I2cSda,
            Reset: I2cSdaReset
        }
    }
    PA17 {
        name: scl
        aliases: {
            AlternateC: I2cScl,
            Reset: I2cSclReset
        }
    }

    // Neopixel power and data pins.
    PA15 {
        name: neopixel_power
        aliases: {
            PushPullOutput: NeopixelPower,
            Reset: NeopixelPowerReset
        }
    }
    PA18 {
        name: neopixel_data
        aliases: {
            PushPullOutput: NeopixelData,
            Reset: NeopixelDataReset
        }
    }

    // USB pins.
    PA24 {
        name: usb_dm,
        aliases: {
            AlternateG: UsbDm,
            Reset: UsbDmReset
        }
    }
    PA25 {
        name: usb_dp,
        aliases: {
            AlternateG: UsbDp,
            Reset: UsbDpReset
        }
    }

    // Factory non-populated flash part on flip side of board.
    PA08 {
        name: flash_cs
        aliases: {
            PushPullOutput: FlashCs,
            Reset: FlashCsReset
        }
    }
    PA19 {
        name: flash_miso
        aliases: {
            AlternateD: FlashMiso,
            Reset: FlashMisoReset
        }
    }
    PA22 {
        name: flash_mosi
        aliases: {
            AlternateC: FlashMosi,
            Reset: FlashMosiReset
        }
    }
    PA23 {
        name: flash_sclk
        aliases: {
            AlternateC: FlashSck,
            Reset: FlashSckReset
        }
    }
}

impl Pins {
    /// Splits this `Pins` into categorized sets of pins.
    pub fn split(self) -> Sets {
        let analog = Analog {
            a0: self.a0,
            a1: self.a1,
            a2: self.a2,
            a3: self.a3,
        };
        let uart = Uart {
            tx: self.tx,
            rx: self.rx,
        };
        let spi = Spi {
            miso: self.miso,
            mosi: self.mosi,
            sclk: self.sclk,
        };
        let i2c = I2c {
            sda: self.sda,
            scl: self.scl,
        };
        let neopixel = Neopixel {
            power: self.neopixel_power,
            data: self.neopixel_data,
        };
        let usb = Usb {
            dm: self.usb_dm,
            dp: self.usb_dp,
        };
        Sets {
            analog,
            uart,
            spi,
            i2c,
            neopixel,
            usb,
        }
    }
}

/// Pins grouped by category.
pub struct Sets {
    /// A0-A3 pins.
    pub analog: Analog,
    /// TX/RX pins.
    pub uart: Uart,
    /// SPI pins.
    pub spi: Spi,
    /// I2C/QWIIC pins.
    pub i2c: I2c,
    /// On-board Neopixel pins.
    pub neopixel: Neopixel,
    /// USB pins.
    pub usb: Usb,
}

/// 'Analog' GPIO pins. Marked A0-A3 on the board. Can also be used as normal
/// digital GPIO.
pub struct Analog {
    /// A0 pin.
    pub a0: A0Reset,
    /// A1 pin.
    pub a1: A1Reset,
    /// A2 pin.
    pub a2: A2Reset,
    /// A3 pin.
    pub a3: A3Reset,
}

/// UART mapped to the TX/RX pins on the board.
pub struct Uart {
    /// TX pin.
    pub tx: UartTxReset,
    /// RX pin.
    pub rx: UartRxReset,
}

/// UART pads for the labelled RX & TX pins
pub type UartPads = uart::Pads<Sercom0, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type UartConfig = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

impl Uart {
    /// Convenience function for creating a UART on the TX/RX pins.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        freq: impl Into<Hertz>,
        sercom0: pac::SERCOM0,
        pm: &mut pac::PM,
    ) -> UartConfig {
        let gclk0 = clocks.gclk0();
        let clock = &clocks.sercom0_core(&gclk0).unwrap();
        let rx: UartRx = self.rx.into();
        let tx: UartTx = self.tx.into();
        let pads = uart::Pads::default().rx(rx).tx(tx);
        uart::Config::new(pm, sercom0, pads, clock.freq())
            .baud(freq.into(), BaudMode::Fractional(Oversampling::Bits16))
            .enable()
    }
}

/// SPI pins.
pub struct Spi {
    /// SPI MISO pin.
    pub miso: MisoReset,
    /// SPI MOSI pin.
    pub mosi: MosiReset,
    /// SPI SCK pin.
    pub sclk: SckReset,
}

type SpiPads = spi::Pads<Sercom2, SpiMiso, SpiMosi, SpiSck>;

/// The SPI type for the labeled SPI bus.
pub type SpiConfig = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

impl Spi {
    /// Convenience function for creating a mode 0 SPI interface on the SPI
    /// pins.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        baud: impl Into<Hertz>,
        sercom2: pac::SERCOM2,
        pm: &mut pac::PM,
    ) -> SpiConfig {
        let gclk0 = clocks.gclk0();
        let clock = clocks.sercom2_core(&gclk0).unwrap();
        let pads = spi::Pads::default()
            .data_in(self.miso)
            .data_out(self.mosi)
            .sclk(self.sclk);
        spi::Config::new(pm, sercom2, pads, clock.freq())
            .spi_mode(spi::MODE_0)
            .baud(baud)
            .enable()
    }
}

/// I2C pins.
pub struct I2c {
    /// I2C SDA pin.
    pub sda: I2cSdaReset,
    /// I2C SCL pin.
    pub scl: I2cSclReset,
}

impl I2c {
    /// Convenience function for creating an I2C host on the I2C pins.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        freq: impl Into<Hertz>,
        sercom1: pac::SERCOM1,
        pm: &mut pac::PM,
    ) -> I2CMaster1<I2cSda, I2cScl> {
        let gclk0 = clocks.gclk0();
        let clock = &clocks.sercom1_core(&gclk0).unwrap();
        I2CMaster1::new(
            clock,
            freq.into(),
            sercom1,
            pm,
            self.sda.into(),
            self.scl.into(),
        )
    }
}

/// Neopixel pins.
pub struct Neopixel {
    /// Neopixel power pin. Must be driven high to provide power to the on-board
    /// neopixel.
    pub power: NeopixelPowerReset,
    /// Neopixel data pin.
    pub data: NeopixelDataReset,
}

/// USB pins. These are connected to the on-board USB-C connector.
pub struct Usb {
    /// USB DM pin.
    pub dm: UsbDmReset,
    /// USB DP pin.
    pub dp: UsbDpReset,
}

impl Usb {
    /// Convenience function for creating a USB device attached to the USB pins.
    #[cfg(feature = "usb")]
    pub fn init(
        self,
        usb: pac::USB,
        clocks: &mut GenericClockController,
        pm: &mut pac::PM,
    ) -> UsbBusAllocator<UsbBus> {
        let gclk0 = clocks.gclk0();
        let usb_clock = &clocks.usb(&gclk0).unwrap();
        let (dm, dp): (UsbDm, UsbDp) = (self.dm.into(), self.dp.into());
        UsbBusAllocator::new(UsbBus::new(usb_clock, pm, dm, dp, usb))
    }
}
