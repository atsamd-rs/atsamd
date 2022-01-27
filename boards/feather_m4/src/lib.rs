#![no_std]
#![recursion_limit = "1024"]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use hal::ehal;
pub use hal::pac;

use hal::clock::GenericClockController;
use hal::sercom::v2::{
    i2c, spi,
    uart::{self, BaudMode, Oversampling},
    IoSet1, Sercom1, Sercom2, Sercom5, UndocIoSet1,
};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

hal::bsp_pins!(
    PA02 {
        /// Analog pin 0.  Can act as a true analog output
        /// as it has a DAC (which is not currently supported
        /// by this hal) as well as input.
        name: a0,
    }
    PA05 {
        /// Analog Pin 1
        name: a1,
    }
    PB08 {
        /// Analog Pin 2
        name: a2,
    }
    PB09 {
        /// Analog Pin 3
        name: a3,
    }
    PA04 {
        /// Analog Pin 4
        name: a4,
    }
    PA06 {
        /// Analog Pin 5
        name: a5,
    }
    PB01 {
        /// Analog Vdiv (1/2 resistor divider for monitoring the battery)
        name: battery,
    }

    PB17 {
        /// Pin 0, UART rx
        name: d0,
        aliases: {
            AlternateC: UartRx
        }
    }
    PB16 {
        /// Pin 1, UART tx
        name: d1,
        aliases: {
            AlternateC: UartTx
        }
    }
    PA14 {
        /// Pin 4, PWM capable
        name: d4,
    }
    PA16 {
        /// Pin 5, PWM capable
        name: d5,
    }
    PA18 {
        /// Pin 6, PWM capable
        name: d6,
    }
    PB03 {
        /// Neopixel Pin
        name: neopixel,
    }
    PA19 {
        /// Pin 9, PWM capable.  Also analog input (A7)
        name: d9,
    }
    PA20 {
        /// Pin 10, PWM capable
        name: d10,
    }
    PA21 {
        /// Pin 11, PWM capable
        name: d11,
    }
    PA22 {
        /// Pin 12, PWM capable
        name: d12,
    }
    PA23 {
        /// Pin 13, which is also attached to the red LED. PWM capable.
        name: d13,
        aliases: {
            PushPullOutput: RedLed,
            AlternateE: RedLedPwm
        }
    }
    PA12 {
        /// The I2C data line
        name: sda,
        aliases: {
            AlternateC: Sda
        }
    }
    PA13 {
        /// The I2C clock line
        name: scl,
        aliases: {
            AlternateC: Scl
        }
    }
    PA17 {
        /// The SPI SCK
        name: sck,
        aliases: {
            AlternateC: Sclk
        }
    }
    PB23 {
        /// The SPI MOSI
        name: mosi,
        aliases: {
            AlternateC: Mosi
        }
    }
    PB22 {
        /// The SPI MISO
        name: miso,
        aliases: {
            AlternateC: Miso
        }
    }
    PA24 {
        /// The USB D- pad
        name: usb_dm,
        aliases: {
            AlternateG: UsbDm
        }
    }
    PA25 {
        /// The USB D+ pad
        name: usb_dp,
        aliases: {
            AlternateG: UsbDp
        }
    }
);

/// SPI pads for the labelled SPI peripheral
///
/// You can use these pads with other, user-defined [`spi::Config`]urations.
pub type SpiPads = spi::Pads<Sercom1, UndocIoSet1, Miso, Mosi, Sclk>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM1 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom1: pac::SERCOM1,
    mclk: &mut pac::MCLK,
    sclk: impl Into<Sclk>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom1_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sclk.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);
    spi::Config::new(mclk, sercom1, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

/// I2C pads for the labelled I2C peripheral
///
/// You can use these pads with other, user-defined [`i2c::Config`]urations.
pub type I2cPads = i2c::Pads<Sercom2, IoSet1, Sda, Scl>;

/// I2C master for the labelled I2C peripheral
///
/// This type implements [`Read`](ehal::blocking::i2c::Read), [`Write`](ehal::blocking::i2c::Write) and [`WriteRead`](ehal::blocking::i2c::WriteRead).
pub type I2c = i2c::I2c<i2c::Config<I2cPads>>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: Sercom2,
    mclk: &mut pac::MCLK,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom2_core(&gclk0).unwrap();
    let freq = clock.freq();
    let baud = baud.into();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(mclk, sercom, pads, freq)
        .baud(baud)
        .enable()
}

/// UART pads for the labelled RX & TX pins
pub type UartPads = uart::Pads<Sercom5, IoSet1, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom5: Sercom5,
    mclk: &mut pac::MCLK,
    rx: impl Into<UartRx>,
    tx: impl Into<UartTx>,
) -> Uart {
    let gclk0 = clocks.gclk0();

    let clock = &clocks.sercom5_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(rx.into()).tx(tx.into());
    uart::Config::new(mclk, sercom5, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

#[cfg(feature = "usb")]
/// Convenience function for setting up USB
pub fn usb_allocator(
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
    usb: pac::USB,
    clocks: &mut GenericClockController,
    mclk: &mut pac::MCLK,
) -> UsbBusAllocator<UsbBus> {
    use pac::gclk::{genctrl::SRC_A, pchctrl::GEN_A};

    clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
    let usb_gclk = clocks.get_gclk(GEN_A::GCLK2).unwrap();
    let usb_clock = &clocks.usb(&usb_gclk).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(usb_clock, mclk, dm, dp, usb))
}
