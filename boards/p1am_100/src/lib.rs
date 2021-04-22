#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

pub use hal::common::*;

pub use hal::target_device as pac;

use gpio::{Floating, Input, Port};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin, SPIMaster4, UART5};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use gpio::v2::{AnyPin, PA24, PA25};
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

bsp_pins!(
    PA02 {
        name: d15,
        aliases: {
            AlternateB: A0
        }
    }
    PA04 {
        name: d18,
        aliases: {
            AlternateB: A3,
            PushPullOutput: BaseSlaveSelect
        }
    }
    PA05 {
        name: d19,
        aliases: {
            AlternateB: A4,
            PullUpInput: BaseSlaveAck
        }
    }
    PA06 {
        name: d20,
        aliases: {
            AlternateB: A5
        }
    }
    PA07 {
        name: d21,
        aliases: {
            AlternateB: A6,
            AlternateG: I2sSerialData
        }
    }
    PA08 {
        name: d11,
        aliases: {
            AlternateC: SdaPin
        }
    }
    PA09 {
        name: d12,
        aliases: {
            AlternateC: SclPin
        }
    }
    PA10 {
        name: d2,
        aliases: {
            AlternateG: I2sSerialClock
        }
    }
    PA11 { name: d3 }
    PA12 {
        name: pa12,
        aliases: {
            AlternateC: SdSdo
        }
    }
    PA13 {
        name: pa13,
        aliases: {
            AlternateC: SdSck
        }
    }
    PA14 {
        name: pa14,
        aliases: {
            PushPullOutput: SdSlaveSelect
        }
    }
    PA15 {
        name: pa15,
        aliases: {
            AlternateC: SdSdi
        }
    }
    PA16 {
        name: d8,
        aliases: {
            AlternateC: Spi0Sdo
        }
    }
    PA17 {
        name: d9,
        aliases: {
            AlternateC: Spi0Sck
        }
    }
    PA18 {
        name: pa18,
        aliases: {
            /// Host Enable, drive high to switch into USB host mode
            #[cfg(feature = "usb")]
            PushPullOutput: UsbId
        }
    }
    PA19 {
        name: d10,
        aliases: {
            AlternateC: Spi0Sdi
        }
    }
    PA20 { name: d6 }
    PA21 { name: d7 }
    PA22 { name: d0 }
    PA23 { name: d1 }
    PA24 {
        name: usb_dm,
        aliases: {
            #[cfg(feature = "usb")]
            AlternateG: UsbDm
        }
    }
    PA25 {
        name: usb_dp,
        aliases: {
            #[cfg(feature = "usb")]
            AlternateG: UsbDp
        }
    }
    PA27 {
        name: pa27,
        aliases: {
            PullUpInput: SdCardDetect
        }
    }
    PA28 {
        name: switch,
        aliases: {
            PullUpInput: Switch
        }
    }
    PB02 {
        name: d16,
        aliases: {
            AlternateB: A1
        }
    }
    PB03 {
        name: d17,
        aliases: {
            AlternateB: A2
        }
    }
    PB08 {
        name: led,
        aliases: {
            PushPullOutput: Led
        }
    }
    PB09 {
        name: pb09,
        aliases: {
            PushPullOutput: BaseEnable,
            AlternateB: AdcBattery
        }
    }
    PB10 { name: d4 }
    PB11 { name: d5 }
    PB22 {
        name: d14,
        aliases: {
            AlternateD: UartTx
        }
    }
    PB23 {
        name: d13,
        aliases: {
            AlternateD: UartRx
        }
    }
);

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM4 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
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
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom4,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}

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
) -> I2CMaster3<
    hal::sercom::Sercom3Pad0<hal::gpio::Pa22<hal::gpio::PfC>>,
    hal::sercom::Sercom3Pad1<hal::gpio::Pa23<hal::gpio::PfC>>,
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

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom5: pac::SERCOM5,
    pm: &mut pac::PM,
    rx: UartRx,
    tx: UartTx,
) -> UART5<
    hal::sercom::Sercom5Pad3<hal::gpio::v2::PB23>,
    hal::sercom::Sercom5Pad2<hal::gpio::v2::PB22>,
    (),
    (),
> {
    let gclk0 = clocks.gclk0();

    UART5::new(
        &clocks.sercom5_core(&gclk0).unwrap(),
        baud.into(),
        sercom5,
        pm,
        (rx.into(), tx.into()),
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
