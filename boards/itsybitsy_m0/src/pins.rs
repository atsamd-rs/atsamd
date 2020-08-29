//! ItsyBitsy M0 pins

use super::{hal, pac, target_device};

#[cfg(feature = "unproven")]
use embedded_hal::timer::{CountDown, Periodic};

use hal::clock::GenericClockController;
use hal::define_pins;
use hal::gpio::{self, *};
use hal::sercom::{I2CMaster3, PadPin, SPIMaster4, UART0};
use hal::time::Hertz;

#[cfg(feature = "unproven")]
use apa102_spi::Apa102;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    /// Analog pin 0.  Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pin a0 = a2,
    /// Analog Pin 1
    pin a1 = b8,
    /// Analog Pin 2
    pin a2 = b9,
    /// Analog Pin 3
    pin a3 = a4,
    /// Analog Pin 4
    pin a4 = a5,
    /// Analog Pin 5
    pin a5 = b2,

    /// Pin 0, rx
    pin d0 = a11,
    /// Pin 1, tx
    pin d1 = a10,
    pin d2 = a14,
    pin d3 = a9,
    pin d4 = a8,
    pin d5 = a15,
    pin d7 = a21,
    pin d9 = a7,
    pin d10 = a18,
    pin d11 = a16,
    pin d12 = a19,
    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a17,

    /// The SPI MOSI
    pin mosi = b10,
    /// The SPI MISO
    pin miso = a12,
    /// The SPI SCK
    pin sck = b11,

    /// The MOSI pin attached to the on-board SPI flash
    pin flash_mosi = b22,
    /// The MISO pin attached to the on-board SPI flash
    pin flash_miso = b3,
    /// The SCK pin attached to the on-board SPI flash
    pin flash_sck = b23,
    /// The CS pin attached to the on-board SPI flash
    pin flash_cs = a27,

    /// The I2C clock
    pin scl = a23,
    /// The I2C data line
    pin sda = a22,

    /// The DotStar clock
    pin dotstar_ci = a0,
    /// The DotStar data line
    pin dotstar_di = a1,
    /// Not connected, but usable as the MISO when addressing
    /// the dotstar over SPI.
    pin dotstar_nc = b0,

    pin swdio = a31,
    pin swdclk = a30,

    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,
);

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let analog = Analog {
            a0: self.a0,
            a1: self.a1,
            a2: self.a2,
            a3: self.a3,
            a4: self.a4,
            a5: self.a5,
        };

        let dotstar = Dotstar {
            ci: self.dotstar_ci,
            di: self.dotstar_di,
            nc: self.dotstar_nc,
        };

        let spi = SPI {
            sck: self.sck,
            mosi: self.mosi,
            miso: self.miso,
        };

        let i2c = I2C {
            sda: self.sda,
            scl: self.scl,
        };

        let usb = USB {
            dm: self.usb_dm,
            dp: self.usb_dp,
        };

        let uart = UART {
            rx: self.d0,
            tx: self.d1,
        };

        Sets {
            analog,
            dotstar,
            spi,
            i2c,
            usb,
            uart,
            port: self.port,
        }
    }
}

/// Sets of pins split apart by category
pub struct Sets {
    /// Analog pins
    pub analog: Analog,

    /// Dotstar (RGB LED) pins
    pub dotstar: Dotstar,

    /// SPI (external pinout) pins
    pub spi: SPI,

    /// I2C (external pinout) pins
    pub i2c: I2C,

    /// USB pins
    pub usb: USB,

    /// UART (external pinout) pins
    pub uart: UART,

    /// Port
    pub port: Port,
}

/// Analog pins
pub struct Analog {
    pub a0: Pa2<Input<Floating>>,
    pub a1: Pb8<Input<Floating>>,
    pub a2: Pb9<Input<Floating>>,
    pub a3: Pa4<Input<Floating>>,
    pub a4: Pa5<Input<Floating>>,
    pub a5: Pb2<Input<Floating>>,
}

/// Dotstar pins
pub struct Dotstar {
    pub ci: Pa0<Input<Floating>>,
    pub di: Pa1<Input<Floating>>,
    pub nc: Pb0<Input<Floating>>,
}

impl Dotstar {
    #[cfg(feature = "unproven")]
    pub fn init<T: CountDown + Periodic>(
        self,
        timer: T,
        port: &mut Port,
    ) -> apa102_spi::Apa102<
        bitbang_hal::spi::SPI<Pb0<Input<PullUp>>, Pa1<Output<PushPull>>, Pa0<Output<PushPull>>, T>,
    > {
        let di = self.di.into_push_pull_output(port);
        let ci = self.ci.into_push_pull_output(port);
        let nc = self.nc.into_pull_up_input(port);

        let spi = bitbang_hal::spi::SPI::new(apa102_spi::MODE, nc, di, ci, timer);
        Apa102::new_with_custom_postamble(spi, 4, false)
    }
}

/// SPI pins
pub struct SPI {
    pub sck: gpio::Pb11<Input<Floating>>,
    pub mosi: gpio::Pb10<Input<Floating>>,
    pub miso: gpio::Pa12<Input<Floating>>,
}

impl SPI {
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom4: pac::SERCOM4,
        pm: &mut pac::PM,
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
            (
                self.miso.into_pad(port),
                self.mosi.into_pad(port),
                self.sck.into_pad(port)),
        )
    }
}

/// I2C pins
pub struct I2C {
    pub sda: Pa22<Input<Floating>>,
    pub scl: Pa23<Input<Floating>>,
}

impl I2C {
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom3: pac::SERCOM3,
        pm: &mut pac::PM,
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
            self.sda.into_pad(port),
            self.scl.into_pad(port),
        )
    }
}

/// USB pins
pub struct USB {
    pub dm: Pa24<Input<Floating>>,
    pub dp: Pa25<Input<Floating>>,
}

impl USB {
    #[cfg(feature = "usb")]
    pub fn init(
        self,
        usb: pac::USB,
        clocks: &mut GenericClockController,
        pm: &mut pac::PM,
        port: &mut Port,
    ) -> UsbBusAllocator<UsbBus> {
        use gpio::IntoFunction;

        let gclk0 = clocks.gclk0();
        let usb_clock = &clocks.usb(&gclk0).unwrap();

        UsbBusAllocator::new(UsbBus::new(
            usb_clock,
            pm,
            self.dm.into_function(port),
            self.dp.into_function(port),
            usb,
        ))
    }
}

/// UART pins
pub struct UART {
    pub tx: Pa10<Input<Floating>>,
    pub rx: Pa11<Input<Floating>>,
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
        hal::sercom::Sercom0Pad3<gpio::Pa11<PfC>>,
        hal::sercom::Sercom0Pad2<gpio::Pa10<PfC>>,
        (),
        (),
    > {
        let gclk0 = clocks.gclk0();

        UART0::new(
            &clocks.sercom0_core(&gclk0).unwrap(),
            baud.into(),
            sercom0,
            pm,
            (
                self.rx.into_pad(port),
                self.tx.into_pad(port),
            ),
        )
    }
}
