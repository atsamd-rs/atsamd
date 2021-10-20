#![no_std]

pub use atsamd_hal as hal;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

pub use hal::common::*;

pub use hal::pac;

use gpio::{self, *};

use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster2, PadPin, SPIMaster0, UART0};
use hal::time::Hertz;

#[cfg(feature = "unproven")]
use apa102_spi::Apa102;
#[cfg(feature = "unproven")]
use hal::ehal::timer::{CountDown, Periodic};

#[cfg(feature = "usb")]
use gpio::v2::{AnyPin, PA24, PA25};
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    pac: pac,

    /// I2C SDA
    pin d0 = a8,
    pin d1 = a2,
    /// I2C SCL
    pin d2 = a9,
    /// UART RX
    pin d3 = a7,
    /// UART TX
    pin d4 = a6,

    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a10,

    /// The DotStar clock
    pin dotstar_ci = a1,
    /// The DotStar data line
    pin dotstar_di = a0,
    /// Not connected, but usable as the MISO when addressing
    /// the dotstar over SPI.
    pin dotstar_nc = a14,

    pin swdio = a31,
    pin swdclk = a30,

    /// USB host enable pin
    pin usb_host_enable = a28,

    /// The USB SOF 1kHz pad
    pin usb_sof = a23,
    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,
);

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let dotstar = Dotstar {
            ci: self.dotstar_ci,
            di: self.dotstar_di,
            nc: self.dotstar_nc,
        };

        let i2c = I2C {
            sda: self.d0,
            scl: self.d2,
        };

        let usb = USB {
            dm: self.usb_dm,
            dp: self.usb_dp,
        };

        let uart = UART {
            rx: self.d3,
            tx: self.d4,
        };

        Sets {
            dotstar,
            i2c,
            usb,
            uart,
            port: self.port,
        }
    }
}

/// Sets of pins split apart by category
pub struct Sets {
    /// Dotstar (RGB LED) pins
    pub dotstar: Dotstar,

    /// I2C (external pinout) pins
    pub i2c: I2C,

    /// USB pins
    pub usb: USB,

    /// UART (external pinout) pins
    pub uart: UART,

    /// Port
    pub port: Port,
}

/// Dotstar pins
pub struct Dotstar {
    pub ci: gpio::Pa1<Input<Floating>>,
    pub di: gpio::Pa0<Input<Floating>>,
    // Pa14 is NC on the Trinket M0, so its safe to use
    // as the MISO given the HAL requires it.
    pub nc: gpio::Pa14<Input<Floating>>,
}

impl Dotstar {
    #[cfg(feature = "unproven")]
    pub fn init<T: CountDown + Periodic>(
        self,
        timer: T,
        port: &mut Port,
    ) -> apa102_spi::Apa102<
        bitbang_hal::spi::SPI<
            gpio::Pa14<Input<PullUp>>,
            gpio::Pa0<Output<PushPull>>,
            gpio::Pa1<Output<PushPull>>,
            T,
        >,
    > {
        let di = self.di.into_push_pull_output(port);
        let ci = self.ci.into_push_pull_output(port);
        let nc = self.nc.into_pull_up_input(port);

        let spi = bitbang_hal::spi::SPI::new(apa102_spi::MODE, nc, di, ci, timer);
        Apa102::new_with_custom_postamble(spi, 4, false)
    }
}

/// Convenience function for setting up the A7/D3/SCK, A6/D4/MOSI, and
/// A9/D2/MISO pins as a SPI Master.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    speed: F,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    sck: gpio::Pa7<Input<Floating>>,
    mosi: gpio::Pa6<Input<Floating>>,
    miso: gpio::Pa9<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster0<
    hal::sercom::Sercom0Pad1<gpio::Pa9<gpio::PfC>>,
    hal::sercom::Sercom0Pad2<gpio::Pa6<gpio::PfD>>,
    hal::sercom::Sercom0Pad3<gpio::Pa7<gpio::PfD>>,
> {
    let gclk0 = clocks.gclk0();

    SPIMaster0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom0,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}

/// I2C pins
pub struct I2C {
    pub sda: gpio::Pa8<Input<Floating>>,
    pub scl: gpio::Pa9<Input<Floating>>,
}

impl I2C {
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom2: pac::SERCOM2,
        pm: &mut pac::PM,
        port: &mut Port,
    ) -> I2CMaster2<
        hal::sercom::Sercom2Pad0<gpio::Pa8<PfD>>,
        hal::sercom::Sercom2Pad1<gpio::Pa9<PfD>>,
    > {
        let gclk0 = clocks.gclk0();

        I2CMaster2::new(
            &clocks.sercom2_core(&gclk0).unwrap(),
            bus_speed.into(),
            sercom2,
            pm,
            self.sda.into_pad(port),
            self.scl.into_pad(port),
        )
    }
}

/// Convenience for setting up the D0 and D2 pins to operate as I²C
/// SDA/SDL (respectively) running at the specified baud.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom2: pac::SERCOM2,
    pm: &mut pac::PM,
    sda: gpio::Pa8<Input<Floating>>,
    scl: gpio::Pa9<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster2<hal::sercom::Sercom2Pad0<gpio::Pa8<PfD>>, hal::sercom::Sercom2Pad1<gpio::Pa9<PfD>>>
{
    let gclk0 = clocks.gclk0();

    I2CMaster2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom2,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}

/// UART pins
pub struct UART {
    pub rx: gpio::Pa7<Input<Floating>>,
    pub tx: gpio::Pa6<Input<Floating>>,
}

impl UART {
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        baud: F,
        sercom0: pac::SERCOM0,
        pm: &mut pac::PM,
        port: &mut Port,
    ) -> UART0<
        hal::sercom::Sercom0Pad3<gpio::Pa7<PfD>>,
        hal::sercom::Sercom0Pad2<gpio::Pa6<PfD>>,
        (),
        (),
    > {
        let gclk0 = clocks.gclk0();

        UART0::new(
            &clocks.sercom0_core(&gclk0).unwrap(),
            baud.into(),
            sercom0,
            pm,
            (self.rx.into_pad(port), self.tx.into_pad(port)),
        )
    }
}

/// Convenience for setting up the D3 and D4 pins to
/// operate as UART RX/TX (respectively) running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    d3: gpio::Pa7<Input<Floating>>,
    d4: gpio::Pa6<Input<Floating>>,
    port: &mut Port,
) -> UART0<hal::sercom::Sercom0Pad3<gpio::Pa7<PfD>>, hal::sercom::Sercom0Pad2<gpio::Pa6<PfD>>, (), ()>
{
    let gclk0 = clocks.gclk0();

    UART0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        baud.into(),
        sercom0,
        pm,
        (d3.into_pad(port), d4.into_pad(port)),
    )
}

/// USB pins
pub struct USB {
    pub dm: gpio::Pa24<Input<Floating>>,
    pub dp: gpio::Pa25<Input<Floating>>,
}

impl USB {
    #[cfg(feature = "usb")]
    pub fn init(
        self,
        usb: pac::USB,
        clocks: &mut GenericClockController,
        pm: &mut pac::PM,
    ) -> UsbBusAllocator<UsbBus> {
        let gclk0 = clocks.gclk0();
        let usb_clock = &clocks.usb(&gclk0).unwrap();

        UsbBusAllocator::new(UsbBus::new(usb_clock, pm, self.dm, self.dp, usb))
    }
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
