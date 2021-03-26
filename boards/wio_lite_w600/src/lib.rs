#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

#[cfg(feature = "panic_halt")]
pub extern crate panic_halt;

pub use hal::prelude::*;
pub use hal::*;

pub use hal::target_device as pac;

use gpio::{self, *};

use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin, SPIMaster4, UART2};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use gpio::v2::{AnyPin, PA24, PA25};
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

// The docs could be further improved with details of the specific channels etc
atsamd_hal::bsp_pins!(
    // ---------- Left Hand Side ----------

    // A0_PA02_AIN0
    PA02 {
        name: a0,
    }

    // AREF
    PA03 {
        name: aref,
    }

    // A1_PB08_AIN2
    PB08 {
        name: a1
    },

    // A2_PB09_AIN2
    PB09 {
        name: a2
    },

    // A3_PB04_AIN2
    PA04 {
        name: a3
    },

    // A4_PB05_AIN2
    PA05 {
        name: a4
    },

    // D8_DGI_GPIO2/VBAT
    PA06 {
        name: vbat
    },

    // D9_DGI_GPIO3
    PA07 {
        name: d9
    },

    // D1/TX_PA10_TCC0-W2
    PA10 {
        name: d1
    },

    // D0/RX_PA11_TCC0-W3
    PA11 {
        name: d0
    },

    // D23_PB10_S4_SPI_MOSI
    PB10 {
        name: d23
    },

    // D24_PB11_S4_SPI_SCK
    PB11 {
        name: d24
    },

    // D22_PA12_S4_SPI_MISO
    PA12 {
        name: d22
    },

    // D5_PA15_TCC0-W5
    PA15 {
        name: d5
    },

    // ---------- Right Hand Side ----------

    // PB03_RX_LED
    // ToDo: What is this?
    PB03 {
        name: rx_led
    },

    // A5_PB02_AIN10
    PB02 {
        name: a5
    },

    // PA31_SWDIO
    // ToDo: Is this ever useful in software?
    PA31 {
        name: swdio
    },

    // PA30_SWCLK
    // ToDo: Is this ever useful in software?
    PA30 {
        name: swclk
    },

    // PA27_TX_LED
    // ToDo: What is this?
    PA27 {
        name: tx_led
    },

    // UART1/TX_PB12_A4 (Tx for W600)
    PB23 {
        name: w600_tx
    },

    // UART1/RX_PB11_A3 (Rx for W600)
    PB22 {
        name: w600_rx
    },

    // SAMD21_D+ (USB Data Plus)
    PA25 {
        name: usb_dm
    },

    // SAMD21_D- (USB Data Plus)
    PA24 {
        name: usb_dp
    },

    // D33/SCL/PA23
    PA23 {
        name: d33
    },

    // D32/SDA/PA22
    PA22 {
        name: d32
    },

    // RESET_W600
    PA21 {
        name: reset_w600
    },

    // D6_PA20_TCC0-W6
    PA20 {
        name: d6
    },

    // D12_MISO
    PA19 {
        name: d12
    },

    // D10_SS
    PA18 {
        name: d10
    },

    // D13_SCK
    PA17 {
        name: d13
    },

    // D11_MOSI
    PA16 {
        name: d11
    },
);

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom3: pac::SERCOM3,
    pm: &mut pac::PM,
    sda: gpio::Pa22<Input<Floating>>,
    scl: gpio::Pa23<Input<Floating>>,
    port: &mut Port,
) -> hal::sercom::I2CMaster3<
    hal::sercom::Sercom3Pad0<gpio::Pa22<gpio::PfC>>,
    hal::sercom::Sercom3Pad1<gpio::Pa23<gpio::PfC>>,
> {
    let gclk0 = clocks.gclk0();
    I2CMaster3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom3,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}

/// Convenience function for setting up the D24/SCK, D23/MOSI, and D22/MISO pins
/// as a SPI Master.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    speed: F,
    sercom4: pac::SERCOM4,
    pm: &mut pac::PM,
    sck: gpio::Pb11<Input<Floating>>,
    mosi: gpio::Pb10<Input<Floating>>,
    miso: gpio::Pa12<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster4<
    hal::sercom::Sercom4Pad0<gpio::Pa12<gpio::PfD>>,
    hal::sercom::Sercom4Pad2<gpio::Pb10<gpio::PfD>>,
    hal::sercom::Sercom4Pad3<gpio::Pb11<gpio::PfD>>,
> {
    let gclk0 = clocks.gclk0();

    SPIMaster4::new(
        &clocks.sercom4_core(&gclk0).unwrap(),
        speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom4,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}

/// Convenience for setting up the D0 and D1 pins to
/// operate as UART RX/TX (respectively) running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom2: pac::SERCOM2,
    pm: &mut pac::PM,
    d0: gpio::Pa11<Input<Floating>>,
    d1: gpio::Pa10<Input<Floating>>,
    port: &mut Port,
) -> UART2<
    hal::sercom::Sercom2Pad3<gpio::Pa11<PfD>>,
    hal::sercom::Sercom2Pad2<gpio::Pa10<PfD>>,
    (),
    (),
> {
    let gclk0 = clocks.gclk0();

    UART2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        baud.into(),
        sercom2,
        pm,
        (d0.into_pad(port), d1.into_pad(port)),
    )
}

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: impl AnyPin<Id = PA24>,
    dp: impl AnyPin<Id = PA25>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    UsbBusAllocator::new(UsbBus::new(usb_clock, pm, dm, dp, usb))
}
