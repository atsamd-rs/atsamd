#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

#[cfg(feature = "panic_halt")]
pub extern crate panic_halt;

use hal::prelude::*;
use hal::*;

pub use hal::common::*;
pub use hal::samd21::*;
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
define_pins!(
    /// Maps the pins to their arduino names and the numbers printed on the board.
    /// Information pulled from datasheet and board files for arduino IDE : <https://wiki.seeedstudio.com/Wio-Lite-MG126/#tech-support>
    struct Pins,
    target_device: target_device,

    // ---------- Left Hand Side ----------

    /// A0_PA02_AIN0
    pin a0 = a2,

    /// AREF
    pin aref = a3,

    /// A1_PB08_AIN2
    pin a1 = b8,

    /// A2_PB09_AIN2
    pin a2 = b9,

    /// A3_PB04_AIN2
    pin a3 = a4,

    /// A4_PB05_AIN2
    pin a4 = a5,

    /// D8_DGI_GPIO2/VBAT
    pin vbat = a6,

    /// D9_DGI_GPIO3
    pin d9 = a7,

    /// D1/TX_PA10_TCC0-W2
    pin d1 = a10,

    /// D0/RX_PA11_TCC0-W3
    pin d0 = a11,

    /// D23_PB10_S4_SPI_MOSI
    pin d23 = b10,

    /// D24_PB11_S4_SPI_SCK
    pin d24 = b11,

    /// D22_PA12_S4_SPI_MISO
    pin d22 = a12,

    /// D5_PA15_TCC0-W5
    pin d5 = a15,

    // ---------- Right Hand Side ----------

    /// PB03_RX_LED
    // ToDo: What is this?
    pin rx_led = b3,

    /// A5_PB02_AIN10
    pin a5 = b2,

    /// PA31_SWDIO
    // ToDo: Is this ever useful in software?
    pin swdio = a31,

    /// PA30_SWCLK
    // ToDo: Is this ever useful in software?
    pin swclk = a30,

    /// PA27_TX_LED
    // ToDo: What is this?
    pin tx_led = a27,

    /// UART1/TX_PB12_A4 (Tx for W600)
    pin w600_tx = b23,

    /// UART1/RX_PB11_A3 (Rx for W600)
    pin w600_rx = b22,

    /// SAMD21_D+ (USB Data Plus)
    pin usb_dm = a25,

    /// SAMD21_D- (USB Data Plus)
    pin usb_dp = a24,

    /// D33/SCL/PA23
    pin d33 = a23,

    /// D32/SDA/PA22
    pin d32 = a22,

    /// RESET_W600
    pin reset_w600 = a21,

    /// D6_PA20_TCC0-W6
    pin d6 = a20,

    /// D12_MISO
    pin d12 = a19,

    /// D10_SS
    pin d10 = a18,

    /// D13_SCK
    pin d13 = a17,

    /// D11_MOSI
    pin d11 = a16,
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
