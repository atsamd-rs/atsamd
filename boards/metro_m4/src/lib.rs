#![no_std]
#![deny(missing_docs)]

//! Board support crate for Adafruit's Metro M4 Express,
//! an ATSAMD51-based board in an 'Arduino compatible'
//! shape and pinout

pub use atsamd_hal as hal;
pub use hal::ehal;
pub use hal::pac;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::{
    clock::GenericClockController,
    qspi::{OneShot, Qspi},
    sercom::{
        i2c, spi,
        uart::{self, BaudMode, Oversampling},
        IoSet1, IoSet6,
    },
    time::Hertz,
};

use pac::MCLK;

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_peripherals!(
    SERCOM2 { SpiSercom }
    SERCOM3 { UartSercom }
    SERCOM5 { I2cSercom }
);

hal::bsp_pins!(
    PA02 {
        /// Analog pin 0.  Can act as a true analog output
        /// as it has a DAC (which is not currently supported
        /// by this hal) as well as input.
        name: a0
    }
    PA05 {
        /// Analog Pin 1
        name: a1
    }
    PA06 {
        /// Analog Pin 2
        name: a2
    }
    PA04 {
        /// Analog Pin 3
        name: a3
    }
    PB08 {
        /// Analog Pin 4
        name: a4
    }
        /// Analog Pin 5
    PB09 {
        name: a5
    }

    PA23{
        /// Pin 0, rx.
        name: d0
        aliases: {
            AlternateC: UartRx
        }
    }
    PA22{
        /// Pin 1, tx.
        name: d1
        aliases: {
            AlternateC: UartTx
        }

    }
    PB17 {
        /// Pin 2
        name: d2
    }
    PB16 {
        /// Pin 3
        name: d3
    }
    PB13 {
        /// Pin 4
        name: d4
    }
    PB14 {
        /// Pin 5
        name: d5
    }
    PB15 {
        /// Pin 6
        name: d6
    }
    PB12 {
        /// Pin 7
        name: d7
    }
    PA21 {
        /// Pin 8
        name: d8
    }
    PA20 {
        /// Pin 9
        name: d9
    }
    PA18 {
        /// Pin 10
        name: d10
    }
    PA19 {
        /// Pin 11
        name: d11
    }
    PA17 {
        /// Pin 12
        name: d12
    }
    PA16 {
        /// Digital pin number 13, which is also attached to
        /// the red LED.  PWM capable.
        name: d13
        aliases: {
            PushPullOutput: RedLed
        }
    }
    PB02 {
        /// The I2C data line
        name: sda
        aliases: {
            AlternateD: Sda
            AlternateB: A6
        }
    }
    PB03 {
        /// The I2C clock line
        name: scl
        aliases: {
            AlternateD: Scl
            AlternateB: A7
        }
    }

    PB22 {
        /// The data line attached to the neopixel.
        /// Is also attached to SWCLK.
        name: neopixel
    }

    PA13 {
        /// The SPI SCLK attached the to 2x3 header
        name: sclk
        aliases: {
            AlternateC: Sclk
        }
    }
    PA12 {
        /// The SPI MOSI attached the to 2x3 header
        name: mosi
        aliases: {
            AlternateC: Mosi
        }
    }
    PA14 {
        /// The SPI MISO attached the to 2x3 header
        name: miso
        aliases: {
            AlternateC: Miso
        }
    }

    PB10 {
        /// The SCK pin attached to the on-board SPI flash
        name: flash_sclk
        aliases: {
            AlternateH: FlashSclk
        }
    }
    PB11 {
        /// The CS pin attached to the on-board SPI flash
        name: flash_cs
        aliases: {
            AlternateH: FlashCs
        }
    }
    PA08 {
        /// The D0 pin attached to the on-board SPI flash
        name: flash_d0
        aliases: {
            AlternateH: FlashD0
        }
    }
    PA09 {
        /// The D1 pin attached to the on-board SPI flash
        name: flash_d1
        aliases: {
            AlternateH: FlashD1
        }
    }
    PA10 {
        /// The D1 pin attached to the on-board SPI flash
        name: flash_d2
        aliases: {
            AlternateH: FlashD2
        }
    }
    PA11 {
        /// The D1 pin attached to the on-board SPI flash
        name: flash_d3
        aliases: {
            AlternateH: FlashD3
        }
    }

    PA24 {
        /// The USB D- pad
        name: usb_dm
        aliases: {
            AlternateH: UsbDm
        }

    }
    PA25 {
        /// The USB D+ pad
        name: usb_dp
        aliases: {
            AlternateH: UsbDp
        }

    }
);

/// SPI pads for the labelled SPI peripheral
///
/// You can use these pads with other, user-defined [`spi::Config`]urations.
pub type SpiPads = spi::Pads<SpiSercom, IoSet1, Miso, Mosi, Sclk>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

/// Convenience for setting up the 2x3 header block for SPI.
/// This powers up SERCOM2 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: Hertz,
    sercom: SpiSercom,
    mclk: &mut pac::MCLK,
    sclk: impl Into<Sclk>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom2_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sclk.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);
    spi::Config::new(mclk, sercom, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

/// Convenience for setting up the onboard QSPI flash.
/// Enables the clocks for the QSPI peripheral in single data rate mode
/// assuming 120MHz system clock, for 4MHz QSPI mode 0 operation.
#[allow(clippy::too_many_arguments)]
pub fn qspi_master(
    mclk: &mut MCLK,
    qspi: pac::QSPI,
    sclk: impl Into<FlashSclk>,
    cs: impl Into<FlashCs>,
    data0: impl Into<FlashD0>,
    data1: impl Into<FlashD1>,
    data2: impl Into<FlashD2>,
    data3: impl Into<FlashD3>,
) -> Qspi<OneShot> {
    Qspi::new(
        mclk,
        qspi,
        sclk.into(),
        cs.into(),
        data0.into(),
        data1.into(),
        data2.into(),
        data3.into(),
    )
}

/// I2C pads for the labelled I2C peripheral
///
/// You can use these pads with other, user-defined [`i2c::Config`]urations.
pub type I2cPads = i2c::Pads<I2cSercom, IoSet6, Sda, Scl>;

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
    mclk: &mut pac::MCLK,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom5_core(&gclk0).unwrap();
    let freq = clock.freq();
    let baud = baud.into();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(mclk, sercom, pads, freq)
        .baud(baud)
        .enable()
}

/// UART Pads for the labelled UART peripheral
pub type UartPads = uart::Pads<UartSercom, IoSet1, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: UartSercom,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<UartRx>,
    uart_tx: impl Into<UartTx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom3_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(mclk, sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

#[cfg(feature = "usb")]
/// Convenience function for setting up USB
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    mclk: &mut pac::MCLK,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    use pac::gclk::{genctrl::SRC_A, pchctrl::GEN_A};

    clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
    let usb_gclk = clocks.get_gclk(GEN_A::GCLK2).unwrap();
    let usb_clock = &clocks.usb(&usb_gclk).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(usb_clock, mclk, dm, dp, usb))
}
