#![no_std]
#![deny(missing_docs)]

//! Board support crate for Adafruit's Grand Central M4 Express,
//! an ATSAMD51-based board in an Arduino Mega form factor.

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
        IoSet2,
    },
    time::Hertz,
};

use pac::MCLK;

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_peripherals!(
    SERCOM0 { UartSercom }
    SERCOM1 { UartSercom2 }
    SERCOM5 { UartSercom3 }
    SERCOM3 { I2cSercom }
    SERCOM6 { I2cSercom2 }
    SERCOM7 { SpiSercom }
);

hal::bsp_pins!(
        PA02 {
            /// Analog pin 0
            /// Can act as a true analog output as it has a DAC (which is not
            /// currently supported by this hal) as well as input.
            name: a0
        }
        PA05 {
            /// Analog pin 1
            /// Can act as a true analog output as it has a DAC (which is not
            /// currently supported by this hal) as well as input.
            name: a1
        }
        PB03 {
            /// Analog pin 2
            name: a2
        }
        PC00 {
            /// Analog pin 3
            name: a3
        }
        PC01 {
            /// Analog pin 4
            name: a4
        }
        PC02 {
            /// Analog pin 5
            name: a5
        }
        PC03 {
            /// Analog pin 6
            name: a6
        }
        PB04 {
            /// Analog pin 7
            name: a7
        }
        PB05 {
            /// Analog pin 8
            name: a8
        }
        PB06 {
            /// Analog pin 9
            name: a9
        }
        PB07 {
            /// Analog pin 10
            name: a10
        }
        PB08 {
            /// Analog pin 11
            name: a11
        }
        PB09 {
            /// Analog pin 12
            name: a12
        }
        PA04 {
            /// Analog pin 13
            name: a13
        }
        PA06 {
            /// Analog pin 14
            name: a14
        }
        PA07 {
            /// Analog pin 15
            name: a15
        }
        PB25 {
            /// Digital pin 0, rx
            name: d0
            aliases: {
                AlternateC: UartRx,
            }
        }
        PB24 {
            /// Digital pin 1, tx
            name: d1
            aliases: {
                AlternateC: UartTx,
            }
        }
        PC18 {
            /// Digital pin 2
            name: d2
        }
        PC19 {
            /// Digital pin 3
            name: d3
        }
        PC20 {
            /// Digital pin 4
            name: d4
        }
        PC21 {
            /// Digital pin 5
            name: d5
        }
        PD20 {
            /// Digital pin 6
            name: d6
        }
        PD21 {
            /// Digital pin 7
            name: d7
        }
        PB18 {
            /// Digital pin 8
            name: d8
        }
        PB02 {
            /// Digital pin 9
            name: d9
        }
        PB22 {
            /// Digital pin 10
            name: d10
        }
        PB23 {
            /// Digital pin 11
            name: d11
        }
        PB00 {
            /// Digital pin 12
            name: d12
        }
        PB01 {
            /// Digital pin 13, red LED
            name: d13
            aliases: {
                PushPullOutput: RedLed,
            }
        }
        PB16 {
            /// Digital pin 14, tx3
            name: d14
            aliases: {
                AlternateC: UartTx3,
            }
        }
        PB17 {
            /// Digital pin 15, rx3
            name: d15
            aliases: {
                AlternateC: UartRx3,
            }
        }
        PC22 {
            /// Digital pin 16, tx2
            name: d16
            aliases: {
                AlternateC: UartTx2,
            }
        }
        PC23 {
            /// Digital pin 17, rx2
            name: d17
            aliases: {
                AlternateC: UartRx2,
            }
        }
        PB12 {
            /// Digital pin 18, tx1
            name: d18
            aliases: {
                AlternateC: UartTx1,
            }
        }
        PB13 {
            /// Digital pin 19, rx1
            name: d19
            aliases: {
                AlternateC: UartRx1,
            }
        }
        PB20 {
            /// Digital pin 20, I2C data line
            name: d20
            aliases: {
                AlternateC: Sda,
            }
        }
        PB21 {
            /// Digital pin 21, I2C clock line
            name: d21
            aliases: {
                AlternateC: Scl,
            }
        }
        PD12 {
            /// Digital pin 22
            name: d22
        }
        PA15 {
            /// Digital pin 23
            name: d23
        }
        PC17 {
            /// Digital pin 24, I2C clock line 1
            name: d24
            aliases: {
                AlternateC: Scl1,
            }
        }
        PC16 {
            /// Digital pin 25, I2C data line 1
            name: d25
            aliases: {
                AlternateC: Sda1,
            }
        }
        PA12 {
            /// Digital pin 26
            name: d26
        }
        PA13 {
            /// Digital pin 27
            name: d27
        }
        PA14 {
            /// Digital pin 28
            name: d28
        }
        PB19 {
            /// Digital pin 29
            name: d29
        }
        PA23 {
            /// Digital pin 30
            name: d30
        }
        PA22 {
            /// Digital pin 31
            name: d31
        }
        PA21 {
            /// Digital pin 32
            name: d32
        }
        PA20 {
            /// Digital pin 33
            name: d33
        }
        PA19 {
            /// Digital pin 34
            name: d34
        }
        PA18 {
            /// Digital pin 35
            name: d35
        }
        PA17 {
            /// Digital pin 36
            name: d36
        }
        PA16 {
            /// Digital pin 37
            name: d37
        }
        PB15 {
            /// Digital pin 38
            name: d38
        }
        PB14 {
            /// Digital pin 39
            name: d39
        }
        PC13 {
            /// Digital pin 40
            name: d40
        }
        PC12 {
            /// Digital pin 41
            name: d41
        }
        PC15 {
            /// Digital pin 42
            name: d42
        }
        PC14 {
            /// Digital pin 43
            name: d43
        }
        PC11 {
            /// Digital pin 44
            name: d44
        }
        PC10 {
            /// Digital pin 45
            name: d45
        }
        PC06 {
            /// Digital pin 46
            name: d46
        }
        PC07 {
            /// Digital pin 47
            name: d47
        }
        PC04 {
            /// Digital pin 48
            name: d48
        }
        PC05 {
            /// Digital pin 49
            name: d49
        }
        PD11 {
            /// Digital pin 50, SPI MISO
            name: miso
            aliases: {
                AlternateC: Miso,
            }
        }
        PD08 {
            /// Digital pin 51, SPI MOSI
            name: mosi
            aliases: {
                AlternateC: Mosi,
            }
        }
        PD09 {
            /// Digital pin 52, SPI SCLK
            name: sclk
            aliases: {
                AlternateC: Sclk,
            }
        }
        PD10 {
            /// Digital pin 53
            name: d53
        }
        PA08 {
            /// Flash data 0
            name: flash_d0
            aliases: {
                AlternateD: FlashD0,
            }
        }
        PA09 {
            /// Flash data 1
            name: flash_d1
            aliases: {
                AlternateD: FlashD1,
            }
        }
        PA10 {
            /// Flash data 2
            name: flash_d2
            aliases: {
                AlternateD: FlashD2,
            }
        }
        PA11 {
            /// Flash data 3
            name: flash_d3
            aliases: {
                AlternateD: FlashD3,
            }
        }
        PB10 {
            /// SPI clock for the external flash
            name: flash_sck
            aliases: {
                AlternateD: FlashSclk,
            }
        }
        PB11 {
            /// SPI chip select for the external flash
            name: flash_cs
            aliases: {
                PushPullOutput: FlashCs,
            }
        }
        PC24 {
            /// Neopixel data
            name: neopixel
        }
        PC30 {
            /// tx LED
            name: led_tx
        }
        PC31 {
            /// rx LED
            name: led_rx
        }
        PB26 {
            /// SD card SPI MOSI
            name: sd_mosi
            aliases: {
                AlternateC: SdMosi,
            }
        }
        PB29 {
            /// SD card SPI MISO
            name: sd_miso
            aliases: {
                AlternateC: SdMiso,
            }
        }
        PB27 {
            /// SD card SPI SCLK
            name: sd_sclk
            aliases: {
                AlternateC: SdSclk,
            }
        }
        PB28 {
            /// SD card SPI chip select
            name: sd_cs
            aliases: {
                PushPullOutput: SdCs,
            }
        }
        PB31 {
            /// SD card detect
            name: sd_card_detect
            aliases: {
                PullUpInput: SdCardDetect,
            }
        }
        PA03 {
            /// AREF pin - has 1uF capacitor to ground
            name: aref
        }
        PA24 {
            /// The USB D- pad
            name: usb_dm
            aliases: {
                AlternateG: UsbDm,
            }
        }
        PA25 {
            /// The USB D+ pad
            name: usb_dp
            aliases: {
                AlternateG: UsbDp,
            }
        }
);

/// SPI pads for the labelled SPI peripheral
///
/// You can use these pads with other, user-defined [`spi::Config`]urations.
pub type SpiPads = spi::Pads<SpiSercom, IoSet2, Miso, Mosi, Sclk>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

/// Convenience for setting up the 2x3 header block for SPI.
/// This powers up SERCOM7 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom7: SpiSercom,
    mclk: &mut pac::MCLK,
    sclk: impl Into<Sclk>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom7_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sclk.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);
    spi::Config::new(mclk, sercom7, pads, freq)
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
pub type I2cPads = i2c::Pads<I2cSercom, IoSet2, Sda, Scl>;

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
    let clock = &clocks.sercom3_core(&gclk0).unwrap();
    let freq = clock.freq();
    let baud = baud.into();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(mclk, sercom, pads, freq)
        .baud(baud)
        .enable()
}

/// UART Pads for the labelled UART peripheral
pub type UartPads = uart::Pads<UartSercom, IoSet2, UartRx, UartTx>;

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
    let clock = &clocks.sercom0_core(&gclk0).unwrap();
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
    let gclk0 = clocks.gclk0();
    let clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(clock, mclk, dm, dp, usb))
}
