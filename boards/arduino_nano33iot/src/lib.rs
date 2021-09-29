#![no_std]

pub use atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

use gpio::{Floating, Input, PfC, PfD, Port};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster4, PadPin, SPIMaster1, UART5};
use hal::time::Hertz;

pub use hal::common::*;

pub use hal::pac;

#[cfg(feature = "usb")]
use gpio::v2::{AnyPin, PA24, PA25};
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

// The docs could be further improved with details of the specific channels etc
// Maps the pins to their arduino names and the numbers printed on the board.
// Information from: <https://github.com/arduino/ArduinoCore-samd/blob/master/variants/nano_33_iot/variant.cpp>
bsp_pins!(
    PB23 {
        /// RX
        name: rx
    }
    PB22 {
        /// TX
        name: tx
    }
    PB10 {
        /// Digital 2: PWM, TC
        name: d2
    }
    PB11 {
        /// Digital 3: PWM, TC
        name: d3
    }
    PA07 {
        /// Digital 4: TCC
        name: d4
    }
    PA05 {
        /// Digital 5: PWM, TCC, ADC
        name: d5
    }
    PA04 {
        /// Digital 6: PWM, TCC, ADC
        name: d6
    }
    PA06 {
        /// Digital 7: ADC
        name: d7
    }
    PA18 {
        /// Digital 8
        name: d8
    }
    PA20 {
        /// Digital 9: PWM, TCC
        name: d9
    }
    PA21 {
        /// Digital 10: PWM, TCC
        name: d10
    }
    PA19 {
        /// Digital 11/SCI MISO: PWM, TCC
        name: miso
    }
    PA16 {
        /// Digital 12/SCI MOSI: PWM, TCC
        name: mosi
    }
    PA17 {
        /// Digital 13/LED/SPI SCK: ON-BOARD-LED
        name: led_sck
    }
    PA02 {
        /// Analog 0: DAC
        name: a0
    }
    PB02 {
        /// Analog 1
        name: a1
    }
    PA11 {
        /// Analog 2: PWM, TCC
        name: a2
    }
    PA10 {
        /// Analog 3: PWM, TCC
        name: a3
    }
    PB08 {
        /// Analog 4/SDA
        name: sda
    }
    PB09 {
        /// Analog 5/SCL: PWM< TCC
        name: scl
    }
    PA09 {
        /// Analog 6
        name: a6
    }
    PB03 {
        /// Analog 7
        name: a7
    }
    PA03 {
        /// AREF
        name: aref
    }
    PA12 {
        /// SPI (Lefacy ICSP) 1 / NINA MOSI
        name: nina_mosi
    }
    PA13 {
        /// SPI (Lefacy ICSP) 2 / NINA MISO
        name: nina_miso
    }
    PA14 {
        /// SPI (Lefacy ICSP) 3 / NINA CS
        name: nina_cs
    }
    PA15 {
        /// SPI (Lefacy ICSP) 4 / NINA SCK
        name: nina_sck
    }
    PA27 {
        /// NINA GPIO0
        name: nina_gpio0
    }
    PA08 {
        /// NINA RESET_N
        name: nina_resetn
    }
    PA28 {
        /// NINA ACK
        name: nina_ack
    }
    PA22 {
        /// SerialNina 29: PWM, TC
        name: serial_nina29
    }
    PA23 {
        /// SerialNina 30: PWM, TC
        name: serial_nina30
    }
    PA24 {
        /// USB/DM
        name: usb_dm
    }
    PA25 {
        /// USB/DP
        name: usb_dp
    }
    PA30 {
        /// SWCLK
        name: p34
    }
    PA31 {
        /// SWDIO
        name: p35
    }
);

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

/// EXPERIMENTAL FEATURE STARTS HERE

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom4: pac::SERCOM4,
    pm: &mut pac::PM,
    sda: gpio::Pb8<Input<Floating>>,
    scl: gpio::Pb9<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster4<hal::sercom::Sercom4Pad0<gpio::Pb8<PfD>>, hal::sercom::Sercom4Pad1<gpio::Pb9<PfD>>>
{
    let gclk0 = clocks.gclk0();
    I2CMaster4::new(
        &clocks.sercom4_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom4,
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
    rx: gpio::Pb23<Input<Floating>>,
    tx: gpio::Pb22<Input<Floating>>,
    port: &mut Port,
) -> UART5<
    hal::sercom::Sercom5Pad3<gpio::Pb23<PfD>>,
    hal::sercom::Sercom5Pad2<gpio::Pb22<PfD>>,
    (),
    (),
> {
    let gclk0 = clocks.gclk0();

    UART5::new(
        &clocks.sercom5_core(&gclk0).unwrap(),
        baud.into(),
        sercom5,
        pm,
        (rx.into_pad(port), tx.into_pad(port)),
    )
}

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM4 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom1: pac::SERCOM1,
    pm: &mut pac::PM,
    sck: gpio::Pa17<Input<Floating>>,
    mosi: gpio::Pa16<Input<Floating>>,
    miso: gpio::Pa19<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster1<
    hal::sercom::Sercom1Pad3<gpio::Pa19<PfC>>,
    hal::sercom::Sercom1Pad0<gpio::Pa16<PfC>>,
    hal::sercom::Sercom1Pad1<gpio::Pa17<PfC>>,
> {
    let gclk0 = clocks.gclk0();

    SPIMaster1::new(
        &clocks.sercom1_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom1,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}
