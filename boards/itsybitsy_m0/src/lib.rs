#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use hal::ehal;
pub use hal::pac;

use hal::clock::GenericClockController;
use hal::ehal::timer::{CountDown, Periodic};
use hal::sercom::{
    v2::{
        spi,
        uart::{self, BaudMode, Oversampling},
    },
    I2CMaster3,
};
use hal::time::Hertz;
use pac::{SERCOM0, SERCOM3, SERCOM4};

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

// Temporary until we can use the `bsp_pins!` macro
pub type UartSercom = SERCOM0;
pub type I2cSercom = SERCOM3;
pub type SpiSercom = SERCOM4;

hal::bsp_pins!(
    PA03 {
        /// AREF pin - has 1uF capacitor to ground
        name: aref
    }
    PA02 {
        /// Analog pin 0. Also hardware capacitive touch capability
        /// Can act as a true analog output as it has a DAC (which is not
        /// currently supported by this hal) as well as input.
        name: a0,
        aliases: {
            AlternateB: A0
        }
    }
    PB08 {
        /// Analog Pin 1, PWM capable, hardware capacitive touch capability
        name: a1
    }
    PB09 {
        /// Analog Pin 2, PWM capable, hardware capacitive touch capability
        name: a2
    }
    PA04 {
        /// Analog Pin 3, hardware capacitive touch capability
        name: a3
    }
    PA05 {
        /// Analog Pin 4, hardware capacitive touch capability
        name: a4
    }
    PB02 {
        /// Analog Pin 5, hardware capacitive touch capability
        name: a5
    }

    PA11 {
        /// Pin 0, rx. Also analog input or I2S LRCLK
        name: d0
        aliases: {
            AlternateC: UartRx
        }
    }
    PA10 {
        /// Pin 1, tx. Also analog input or I2S bitclock
        name: d1
        aliases: {
            AlternateC: UartTx
        }
    }
    PA14 {
        /// Pin 2
        name: d2
    }
    PA09 {
        /// Pin 3, PWM capable. Also analog input or I2S master clock
        name: d3
    }
    PA08 {
        /// Pin 4, PWM capable. Also analog input or I2S data channel 1. Cannot be interrupt pin
        name: d4
    }
    PA15 {
        /// Pin 5, PWM capable. This is a special OUTPUT-only pin, that is level-shifted up to Vhi voltage
        name: d5
        aliases: {
            AlternateE: D5Pwm
        }
    }
    PA21 {
        /// Pin 7. Also I2S LRCLK
        name: d7
    }
    PA07 {
        /// Pin 9, PWM capable, hardware capacitive touch capability
        /// Also analog input or I2S data channel 0
        name: d9
    }
    PA18 {
        /// Pin 10, PWM capable
        name: d10
    }
    PA16 {
        /// Pin 11, PWM capable
        name: d11
    }
    PA19 {
        /// Pin 12. Also I2S data channel 0
        name: d12
    }
    PA17 {
        /// Pin 13, which is also attached to the red LED. PWM capable.
        name: d13
        aliases: {
            PushPullOutput: RedLed
        }
    }

    PA22 {
        /// The I2C data line
        name: sda
        aliases: {
            AlternateC: Sda
        }
    }
    PA23 {
        /// The I2C clock line
        name: scl
        aliases: {
            AlternateC: Scl
        }
    }

    PB11 {
        /// The SPI SCLK
        name: sclk
        aliases: {
            AlternateD: Sclk
        }
    }
    PB10 {
        /// The SPI MOSI
        name: mosi
        aliases: {
            AlternateD: Mosi
        }
    }
    PA12 {
        /// The SPI MISO
        name: miso
        aliases: {
            AlternateD: Miso
        }
    }

    PA24 {
        /// The USB D- pad
        name: usb_dm
        aliases: {
            AlternateG: UsbDm
        }
    }
    PA25 {
        /// The USB D+ pad
        name: usb_dp
        aliases: {
            AlternateG: UsbDp
        }
    }
    PB23 {
        /// SPI clock for the on-board SPI flash
        name: flash_sclk
    }
    PB22 {
        /// SPI MOSI for the on-board SPI flash
        name: flash_mosi
    }
    PB03 {
        /// SPI MISO for the on-board SPI flash
        name: flash_miso
    }
    PA27 {
        /// SPI chip select for the on-board SPI flash
        name: flash_cs
    }
    PA00 {
        /// SPI SCK line for the Apa102 builtin Dotstar led
        name: dotstar_sck
        aliases: {
            PushPullOutput: DotStarClk
        }
    }
    PA01 {
        /// SPI MOSI line for the Apa102 builtin Dotstar led
        name: dotstar_mosi
        aliases: {
            PushPullOutput: DotStarData
        }
    }
    PA13 {
        /// Not connected, used as the SPI MISO line for the Apa102 builtin Dotstar led
        name: dotstar_miso
        aliases: {
            PullUpInput: DotStarNC
        }
    }
);

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
pub type SpiPads = spi::Pads<SpiSercom, Miso, Mosi, Sclk>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up the SPI SERCOM and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: SpiSercom,
    pm: &mut pac::PM,
    sclk: impl Into<Sclk>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom4_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sclk.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);
    spi::Config::new(pm, sercom, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

/// I2C master for the labelled SDA & SCL pins
pub type I2C = I2CMaster3<Sda, Scl>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: I2cSercom,
    pm: &mut pac::PM,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2C {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom3_core(&gclk0).unwrap();
    let baud = baud.into();
    let sda = sda.into();
    let scl = scl.into();
    I2CMaster3::new(clock, baud, sercom, pm, sda, scl)
}

/// UART pads for the labelled RX & TX pins
pub type UartPads = uart::Pads<UartSercom, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: UartSercom,
    pm: &mut pac::PM,
    uart_rx: impl Into<UartRx>,
    uart_tx: impl Into<UartTx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom0_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(pm, sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

#[cfg(feature = "usb")]
/// Convenience function for setting up USB
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(clock, pm, dm, dp, usb))
}
