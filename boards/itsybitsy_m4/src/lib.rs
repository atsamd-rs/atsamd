#![no_std]
#![recursion_limit = "1024"]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use crate::ehal::timer::CountDown;
pub use atsamd_hal as hal;
pub use hal::ehal;
use hal::ehal::timer::Periodic;

pub use hal::{
    clock::GenericClockController,
    dbgprint, pac,
    qspi::{OneShot, Qspi},
    sercom::v2::{
        spi,
        uart::{self, BaudMode, Oversampling},
        IoSet3, Sercom1, Sercom3, UndocIoSet2,
    },
    sercom::I2CMaster2,
    time::Hertz,
};
use pac::MCLK;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

hal::bsp_pins!(
    PA02 {
        /// Analog pin 0.  Can act as a true analog output
        /// as it has a DAC (which is not currently supported
        /// by this hal) as well as input.
        name: a0
        aliases: {
            AlternateB: Analog0
        }
    }
    PA05 {
        /// Analog pin 1.  Can act as a true analog output
        /// as it has a DAC (which is not currently supported
        /// by this hal) as well as input.
        name: a1
        aliases: {
            AlternateB: Analog1
        }
    }
    PB08 {
        /// Analog pin 2
        name: a2
        aliases: {
            AlternateB: Analog2
        }
    }
    PB09 {
        /// Analog pin 3
        name: a3
        aliases: {
            AlternateB: Analog3
        }
    }
    PA04 {
        /// Analog pin 4, PWM capable
        name: a4
        aliases: {
            AlternateB: Analog4
            AlternateD: IoSet3Sercom0Pad0
        }
    }
    PA06 {
        /// Analog pin 5, PWM capable
        name: a5
        aliases: {
            AlternateB: Analog5
            AlternateD: IoSet3Sercom0Pad2
        }
    }
    PA16 {
        /// Pin 0, labeled as RX.
        /// Can be used as SERCOM3 UART RX.
        /// PWM capable
        name: d0_rx
        aliases: {
            AlternateD: UartRx
        }
    }
    PA17 {
        /// Pin 0, labeled as TX.
        /// Can be used as SERCOM3 UART TX.
        /// PWM capable
        name: d1_tx
        aliases: {
            AlternateD: UartTx
        }
    }
    PA07 {
        /// pin 2, PWM capable
        name: d2
    }
    PB22 {
        /// pin 3 Can act as MISO/CS for Sercom1
        name: d3
        aliases: {
            AlternateC: UndocIoset2Pad2
        }
    }
    PA14 {
        /// pin 4, PWM capable
        name: d4
    }
    PA15 {
        /// pin 5. Output-only with rail-to-rail HI level. PWM capable.
        name: d5
    }
    PA18 {
        /// pin 7, PWM capable
        name: d7
    }
    PA19 {
        /// pin 9, PWM capable
        name: d9
    }
    PA20 {
        /// pin 10, PWM capable
        name: d10
    }
    PA21 {
        /// pin 11, PWM capable
        name: d11
    }
    PA23 {
        /// pin 12, PWM capable
        name: d12
    }
    PA22 {
        /// pin 13, connected to builtin red led, PWM capable
        name: d13
        aliases: {
            PushPullOutput: BuiltinLed
        }
    }
    PA00 {
        /// The SPI MOSI - Sercom1, PWM capable
        name: mosi
        aliases: {
            AlternateD: Mosi
        }
    }
    PA01 {
        /// The SPI SCK - Sercom1, PWM capable
        name: sck
        aliases: {
            AlternateD: Sck
        }
    }
    PB23 {
        /// The SPI MISO - Sercom1, PWM capable
        name: miso
        aliases: {
            AlternateC:  Miso
        }
    }
    PB02 {
        /// SPI SCK line for the Apa102 led
        name: dotstar_sck
        aliases: {
            PushPullOutput: DotStarClk
        }
    }
    PB03 {
        /// SPI MOSI line for the Apa102 led
        name: dotstar_mosi
        aliases: {
            PushPullOutput: DotStarData
        }
    }
    PA27 {
        /// Not connected, used as the SPI MISO line for the Apa102 builtin led
        name: dotstar_miso
        aliases: {
           PullUpInput: DotStarNC
        }
    }
    PA12 {
        /// The I2C SDA pin - Sercom2
        name: sda
        aliases: {
            AlternateC: Sda
        }
    }
    PA13 {
        /// The I2C SCL pin - Sercom2
        name: scl
        aliases: {
            AlternateC: Scl
        }
    }
    PA24 {
        ///USB D- pin
        name: usb_dm
        aliases: {
            AlternateH: UsbDm
        }
    }
    PA25 {
        ///USB D+ pin
        name: usb_dp
        aliases: {
            AlternateH: UsbDp
        }
    }
    PB10 {
        ///QSPI FLASH SCK pin
        name: qspi_sck
        aliases: {
            AlternateH: QspiSck
        }
    }
    PB11 {
        ///QSPI FLASH CS pin
        name: qspi_cs
        aliases: {
            AlternateH: QspiCs
        }
    }
    PA08 {
        ///QSPI FLASH DATA0 pin
        name: qspi_d0
        aliases: {
            AlternateH: QspiD0
        }
    }
    PA09 {
        ///QSPI FLASH DATA1 pin
        name: qspi_d1
        aliases: {
            AlternateH: QspiD1
        }
    }
    PA10 {
        ///QSPI FLASH DATA2 pin
        name: qspi_d2
        aliases: {
            AlternateH: QspiD2
        }
    }
    PA11 {
        ///QSPI FLASH DATA2 pin
        name: qspi_d3
        aliases: {
            AlternateH: QspiD3
        }
    }
);

/// Convenience for setting up the onboard QSPI flash.
/// Enables the clocks for the QSPI peripheral in single data rate mode
/// assuming 120MHz system clock, for 4MHz QSPI mode 0 operation.
pub fn qspi_master(
    mclk: &mut MCLK,
    qspi: pac::QSPI,
    sck: impl Into<QspiSck>,
    cs: impl Into<QspiCs>,
    data: (
        impl Into<QspiD0>,
        impl Into<QspiD1>,
        impl Into<QspiD2>,
        impl Into<QspiD3>,
    ),
) -> Qspi<OneShot> {
    Qspi::new(
        mclk,
        qspi,
        sck.into(),
        cs.into(),
        data.0.into(),
        data.1.into(),
        data.2.into(),
        data.3.into(),
    )
}

/// I2C master for the labelled SDA & SCL pins
pub type I2C = I2CMaster2<Sda, Scl>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom2: pac::SERCOM2,
    mclk: &mut pac::MCLK,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2C {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom2_core(&gclk0).unwrap();
    let baud = baud.into();
    let sda = sda.into();
    let scl = scl.into();
    I2CMaster2::new(clock, baud, sercom2, mclk, sda, scl)
}

/// UART Pads for the labelled UART peripheral
pub type UartPads = uart::Pads<Sercom3, IoSet3, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom3: pac::SERCOM3,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<UartRx>,
    uart_tx: impl Into<UartTx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom3_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(mclk, sercom3, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// Convenience for setting up the USB Bus allocator
#[cfg(feature = "usb")]
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

/// Convenience for setting up the dotstar LED using bitbang'ed
/// SPI.
pub fn dotstar_bitbang<T: CountDown + Periodic>(
    miso: DotStarNC,
    mosi: DotStarData,
    clk: DotStarClk,
    timer: T,
) -> apa102_spi::Apa102<bitbang_hal::spi::SPI<DotStarNC, DotStarData, DotStarClk, T>> {
    let spi = bitbang_hal::spi::SPI::new(apa102_spi::MODE, miso, mosi, clk, timer);
    apa102_spi::Apa102::new_with_custom_postamble(spi, 4, false)
}

/// SPI pads for the labelled SPI peripheral
///
/// You can use these pads with other, user-defined [`spi::Config`]urations.
pub type SpiPads = spi::Pads<Sercom1, UndocIoSet2, Miso, Mosi, Sck>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

/// Convenience for setting up the default SPI.
/// This powers up SERCOM1 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom1: pac::SERCOM1,

    mclk: &mut pac::MCLK,
    sck: impl Into<Sck>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom1_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sck) = (miso.into(), mosi.into(), sck.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sck);
    spi::Config::new(mclk, sercom1, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}
