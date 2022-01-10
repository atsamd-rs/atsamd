#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use hal::pac;

use hal::clock::GenericClockController;
use hal::sercom::v2::{spi, uart, Sercom0, Sercom4};
use hal::sercom::I2CMaster3;
use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

// The docs could be further improved with details of the specific channels etc
atsamd_hal::bsp_pins!(
    // ---------- Left Hand Side ----------

    PA02 {
        /// A0_PA02_AIN0
        name: a0,
        aliases: {
            AlternateB: AIn0
        }
    }

    PA03 {
        /// AREF
        name: aref,
    }

    PB08 {
        /// A1_PB08_AIN2
        name: a1
        aliases: {
            AlternateB: AIn1
        }
    },

    PB09 {
        /// A2_PB09_AIN2
        name: a2
        aliases: {
            AlternateB: AIn2
        }
    },

    PA04 {
        /// A3_PB04_AIN2
        name: a3
        aliases: {
            AlternateB: AIn3
        }
    },

    PA05 {
        /// A4_PB05_AIN2
        name: a4
        aliases: {
            AlternateB: AIn4
        }
    },

    PA06 {
        /// D8_DGI_GPIO2/VBAT
        name: battery
        aliases: {
            AlternateB: Battery
        }
    },

    PA07 {
        /// D9_DGI_GPIO3
        name: d9
    },

    PA10 {
        /// D1/TX_PA10_TCC0-W2
        name: d1
        aliases: {
            AlternateC: UartTx
        }
    },

    PA11 {
        /// D0/RX_PA11_TCC0-W3
        name: d0
        aliases: {
            AlternateC: UartRx
        }
    },

    PB10 {
        /// D23_PB10_S4_SPI_MOSI
        name: d23
        aliases: {
            AlternateD: Mosi
        }
    },

    PB11 {
        /// D24_PB11_S4_SPI_SCK
        name: d24
        aliases: {
            AlternateD: Sck
        }
    },

    PA12 {
        /// D22_PA12_S4_SPI_MISO
        name: d22
        aliases: {
            AlternateD: Miso
        }
    },

    PA15 {
        /// D5_PA15_TCC0-W5
        name: d5
    },

    // ---------- Right Hand Side ----------

    // This pin is listed on the board schematic but its unclear what its attached to
    // PB03 {
    //     /// PB03_RX_LED
    //     name: rx_led
    // },

    PB02 {
        /// A5_PB02_AIN10
        name: a5
    },

    // This pin is listed on the board schematic but is related to debugging and probably should
    // not be used in software
    // PA31 {
    //     /// PA31_SWDIO
    //     name: swdio
    // },

    // This pin is listed on the board schematic but is related to debugging and probably should
    // not be used in software
    // PA30 {
    //     /// PA30_SWCLK
    //     name: swclk
    // },

    // This pin is listed on the board schematic but its unclear what its attached to
    // PA27 {
    //     /// PA27_TX_LED
    //     name: tx_led
    // },

    PB23 {
        /// UART1/TX_PB12_A4
        ///
        /// Tx for W600
        name: w600_tx
    },

    PB22 {
        /// UART1/RX_PB11_A3
        ///
        /// Rx for W600
        name: w600_rx
    },

    PA25 {
        /// SAMD21_D+
        ///
        /// USB Data Plus
        name: usb_dp
        aliases: {
            AlternateG: UsbDp
        }
    },

    PA24 {
        /// SAMD21_D-
        ///
        /// USB Data Minus
        name: usb_dm
        aliases: {
            AlternateG: UsbDm
        }
    },

    PA23 {
        /// D33/SCL/PA23
        name: d33,
        aliases: {
            AlternateC: Scl
        }
    },

    PA22 {
        /// D32/SDA/PA22
        name: d32,
        aliases: {
            AlternateC: Sda
        }
    },

    PA21 {
        /// RESET_W600
        name: reset_w600
    },

    PA20 {
        /// D6_PA20_TCC0-W6
        name: d6
    },

    PA19 {
        /// D12_MISO
        name: d12
    },

    PA18 {
        /// D10_SS
        name: d10
    },

    PA17 {
        /// D13_SCK
        name: d13
    },

    PA16 {
        /// D11_MOSI
        name: d11
    },
);

/// I2C master for the labelled SDA & SCL pins
pub type I2C = I2CMaster3<Sda, Scl>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom3: pac::SERCOM3,
    pm: &mut pac::PM,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2C {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom3_core(&gclk0).unwrap();
    let baud = baud.into();
    let sda = sda.into();
    let scl = scl.into();
    I2CMaster3::new(clock, baud, sercom3, pm, sda, scl)
}

type SpiPads = spi::Pads<Sercom4, Miso, Mosi, Sck>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

/// Convenience function for setting up the D24/SCK, D23/MOSI, and D22/MISO pins
/// as a SPI Master.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom4: pac::SERCOM4,
    pm: &mut pac::PM,
    sck: impl Into<Sck>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom4_core(&gclk0).unwrap();
    let freq = clock.freq();
    let pads = spi::Pads::default()
        .data_in(miso.into())
        .data_out(mosi.into())
        .sclk(sck.into());
    spi::Config::new(pm, sercom4, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

/// UART pads
pub type UartPads = uart::Pads<Sercom0, UartRx, UartTx>;
/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the D0 and D1 pins to
///
/// operate as UART RX/TX (respectively) running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    rx: impl Into<UartRx>,
    tx: impl Into<UartTx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom0_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(rx.into()).tx(tx.into());

    uart::Config::new(pm, sercom0, pads, clock.freq())
        .baud(baud, uart::BaudMode::Fractional(uart::Oversampling::Bits16))
        .enable()
}

/// Convenience method for getting the USB Bus Allocator.
///
/// Basic usage would look like the below:
/// ```no_run
/// use wio_lite_w600::hal::clock::GenericClockController;
/// use wio_lite_w600::pac::Peripherals;
///
/// let mut peripherals = Peripherals::take().unwrap();
/// let mut clocks = GenericClockController::with_internal_32kosc(
///     peripherals.GCLK,
///     &mut peripherals.PM,
///     &mut peripherals.SYSCTRL,
///     &mut peripherals.NVMCTRL,
/// );
/// let pins = bsp::Pins::new(peripherals.PORT);
///
/// let bus_allocator = bsp::usb_allocator(
///     peripherals.USB,
///     &mut clocks,
///     &mut peripherals.PM,
///     pins.usb_dm,
///     pins.usb_dp,
/// );
/// ```
/// However to take advantage of USB interrupts you will need, to do some unsafe
/// rust. See the USB code examples in the `examples/` directory of the project.
#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut hal::clock::GenericClockController,
    pm: &mut pac::PM,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(clock, pm, dm, dp, usb))
}
