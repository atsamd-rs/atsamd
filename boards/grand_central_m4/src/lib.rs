#![no_std]

pub use atsamd_hal as hal;
pub use hal::ehal;
pub use hal::pac;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::{
    clock::GenericClockController,
    qspi::{OneShot, Qspi},
    sercom::v2::{
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
            name: a0
        }
        PA05 {
            name: a1
        }
        PB03 {
            name: a2
        }
        PC00 {
            name: a3
        }
        PC01 {
            name: a4
        }
        PC02 {
            name: a5
        }
        PC03 {
            name: a6
        }
        PB04 {
            name: a7
        }
        PB05 {
            name: a8
        }
        PB06 {
            name: a9
        }
        PB07 {
            name: a10
        }
        PB08 {
            name: a11
        }
        PB09 {
            name: a12
        }
        PA04 {
            name: a13
        }
        PA06 {
            name: a14
        }
        PA07 {
            name: a15
        }
        PB25 {
            name: d0
            aliases: {
                AlternateC: UartRx,
            }
        }
        PB24 {
            name: d1
            aliases: {
                AlternateC: UartTx,
            }
        }
        PC18 {
            name: d2
        }
        PC19 {
            name: d3
        }
        PC20 {
            name: d4
        }
        PC21 {
            name: d5
        }
        PD20 {
            name: d6
        }
        PD21 {
            name: d7
        }
        PB18 {
            name: d8
        }
        PB02 {
            name: d9
        }
        PB22 {
            name: d10
        }
        PB23 {
            name: d11
        }
        PB00 {
            name: d12
        }
        PB01 {
            name: d13
            aliases: {
                PushPullOutput: RedLed,
            }
        }
        PB16 {
            name: d14
            aliases: {
                AlternateC: UartTx3,
            }
        }
        PB17 {
            name: d15
            aliases: {
                AlternateC: UartRx3,
            }
        }
        PC22 {
            name: d16
            aliases: {
                AlternateC: UartTx2,
            }
        }
        PC23 {
            name: d17
            aliases: {
                AlternateC: UartRx2,
            }
        }
        PB12 {
            name: d18
            aliases: {
                AlternateC: UartTx1,
            }
        }
        PB13 {
            name: d19
            aliases: {
                AlternateC: UartRx1,
            }
        }
        PB20 {
            name: d20
            aliases: {
                AlternateC: Sda,
            }
        }
        PB21 {
            name: d21
            aliases: {
                AlternateC: Scl,
            }
        }
        PD12 {
            name: d22
        }
        PA15 {
            name: d23
        }
        PC17 {
            name: d24
            aliases: {
                AlternateC: Scl2,
            }
        }
        PC16 {
            name: d25
            aliases: {
                AlternateC: Sda2,
            }
        }
        PA12 {
            name: d26
        }
        PA13 {
            name: d27
        }
        PA14 {
            name: d28
        }
        PB19 {
            name: d29
        }
        PA23 {
            name: d30
        }
        PA22 {
            name: d31
        }
        PA21 {
            name: d32
        }
        PA20 {
            name: d33
        }
        PA19 {
            name: d34
        }
        PA18 {
            name: d35
        }
        PA17 {
            name: d36
        }
        PA16 {
            name: d37
        }
        PB15 {
            name: d38
        }
        PB14 {
            name: d39
        }
        PC13 {
            name: d40
        }
        PC12 {
            name: d41
        }
        PC15 {
            name: d42
        }
        PC14 {
            name: d43
        }
        PC11 {
            name: d44
        }
        PC10 {
            name: d45
        }
        PC06 {
            name: d46
        }
        PC07 {
            name: d47
        }
        PC04 {
            name: d48
        }
        PC05 {
            name: d49
        }
        PD11 {
            name: d50
            aliases: {
                AlternateC: Miso,
            }
        }
        PD08 {
            name: d51
            aliases: {
                AlternateC: Mosi,
            }
        }
        PD09 {
            name: d52
            aliases: {
                AlternateC: Sclk,
            }
        }
        PD10 {
            name: d53
            aliases: {
                AlternateC: Ss,
            }
        }
        PA08 {
            name: flash_d0
            aliases: {
                AlternateD: FlashD0
            }
        }
        PA09 {
            name: flash_d1
            aliases: {
                AlternateD: FlashD1
            }
        }
        PA10 {
            name: flash_d2
            aliases: {
                AlternateD: FlashD2
            }
        }
        PA11 {
            name: flash_d3
            aliases: {
                AlternateD: FlashD3
            }
        }
        PB10 {
            name: flash_sck
            aliases: {
                AlternateD: FlashSclk
            }
        }
        PB11 {
            name: flash_cs
            aliases: {
                PushPullOutput: FlashCs
            }
        }
        PC24 {
            name: neopixel
        }
        PC30 {
            name: led_tx
        }
        PC31 {
            name: led_rx
        }
        PB26 {
            name: sd_mosi
            aliases: {
                AlternateC: SdMosi,
            }
        }
        PB29 {
            name: sd_miso
            aliases: {
                AlternateC: SdMiso,
            }
        }
        PB27 {
            name: sd_sclk
            aliases: {
                AlternateC: SdSclk,
            }
        }
        PB28 {
            name: sd_cs
        }
        PB31 {
            name: sd_card_detect,
        }
        PA03 {
            name: aref
        }
        PA24 {
            name: usb_dm
            aliases: {
                AlternateG: UsbDm
            }
        }
        PA25 {
            name: usb_dp
            aliases: {
                AlternateG: UsbDp
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
