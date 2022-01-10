//! Grand Central M4 Express Pins

use super::{
    hal, pac, pac::MCLK, pac::SERCOM0, pac::SERCOM1, pac::SERCOM4, pac::SERCOM5, pac::SERCOM6,
    pac::SERCOM7,
};

use hal::define_pins;
use hal::gpio::{self, *};
use hal::sercom::{
    I2CMaster6, PadPin, SPIMaster7, Sercom0Pad0, Sercom0Pad1, Sercom1Pad0, Sercom1Pad1,
    Sercom4Pad0, Sercom4Pad1, Sercom5Pad0, Sercom5Pad1, Sercom6Pad0, Sercom6Pad1, UART0, UART1,
    UART4, UART5,
};
use hal::time::Hertz;

use hal::ehal::{digital::v1_compat::OldOutputPin, timer::CountDown, timer::Periodic};
use ws2812_timer_delay as ws2812;

use hal::clock::GenericClockController;

#[cfg(feature = "usb")]
use super::pac::gclk::{genctrl::SRC_A, pchctrl::GEN_A};
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    pac: pac,

    /// # LED Pins

    /// ## TX LED
    /// Yellow USB serial data transmitted LED
    pin tx_led = c30,

    /// ## RX LED
    /// Yellow USB serial received LED
    pin rx_led = c31,

    /// # Logic Pins
    ///
    /// All logic pins are 3.3V
    ///
    /// PWM capable pins are:
    /// - A1, A2, A12, A15.
    /// - D2-D9, D11, D13-D45, D48, D50-D53.
    /// - MISO, MOSI, SCK, SCL, SDA.

    /// ## Analog 0
    /// This pin is analog input  A0 but is also an analog output due to
    /// having a DAC (digital-to-analog converter). This is the first
    /// DAC, and is 'independent' of A1. You can set the raw voltage to
    /// anything from 0 to 3.3V, unlike PWM outputs, this is a true analog
    /// output.
    pin a0 = a2,

    /// ## Analog 1
    /// This pin is analog input  A1 but is also an analog output due to
    /// having a DAC (digital-to-analog converter). This is the second
    /// DAC, and is 'independent' of A0. You can set the raw voltage to
    /// anything from 0 to 3.3V, unlike PWM outputs this is a true analog
    /// output.
    pin a1 = a5,

    /// ## Analog 2-15
    /// These are each analog input as well as digital I/O pins.
    pin a2 = b3,
    pin a3 = c0,
    pin a4 = c1,
    pin a5 = c2,
    pin a6 = c3,
    pin a7 = b4,
    pin a8 = b5,
    pin a9 = b6,
    pin a10 = b7,
    pin a11 = b8,
    pin a12 = b9,
    pin a13 = a4,
    pin a14 = a6,
    pin a15 = a7,

    /// # Digital GPIO Pins

    /// ## Digital 0:
    /// RX - GPIO #0, also receive (input) pin for Serial0 (hardware UART)
    /// unlike the original Mega this is not used by the USB-to-Serial
    /// chip so its free to use
    ///
    /// mapped as: uart0_rx
    ///pin d0 = b25,
    pin uart0_rx = b25,

    /// ## Digital 1:
    /// TX - GPIO #1, also transmit (output) pin for Serial0 (hardware UART)
    /// unlike the original Mega this is not used by the USB-to-Serial chip
    /// so its free to use
    ///
    /// mapped as: uar0_tx
    ///pin d1 = b24,
    pin uart0_tx = b24,

    /// ## Digital 2-13
    /// These are general purpose GPIO.
    pin d2 = c18,
    pin d3 = c19,
    pin d4 = c20,
    pin d5 = c21,
    pin d6 = d20,
    pin d7 = d21,
    pin d8 = b18,
    pin d9 = b2,
    pin d10 = b22,
    pin d11 = b23,
    pin d12 = b0,

    /// ## Digital 13
    /// Connected to the red LED marked L (see note below) next to the USB
    /// jack. Also PWM output.
    ///
    /// mapped as: red_led
    ///pin d13 = b1,
    pin red_led = b1,

    /// ## Digital 14
    /// TX3 - GPIO #14, to maintain compatibility with the Mega pinout this can
    /// also be a Serial TX (on SERCOM5)
    ///
    /// mapped as: uart3_tx
    ///pin d14 = b16,
    pin uart3_tx = b16,

    /// ## Digital 15
    /// RX3 - GPIO #15, to maintain compatibility with the Mega pinout this can
    /// also be a Serial RX (on SERCOM5)
    ///
    /// mapped as: uart3_rx
    ///pin d15 = b17,
    pin uart3_rx = b17,

    /// ## Digital 16
    /// TX2 - GPIO #16, to maintain compatibility with the Mega pinout this can
    /// also be a Serial TX (on SERCOM1)
    ///
    /// mapped as: uart2_tx
    ///pin d16 = c22,
    pin uart2_tx = c22,

    /// ## Digital 17
    /// RX2 - GPIO #17, to maintain compatibility with the Mega pinout this can
    /// also be a Serial RX (on SERCOM1)
    ///
    /// mapped as: uart2_rx
    ///pin d17 = c23,
    pin uart2_rx = c23,

    /// ## Digital 18
    /// TX1 - GPIO #18, to maintain compatibility with the Mega pinout this can
    /// also be a Serial TX (on SERCOM4)
    ///
    /// mapped as: uart1_tx
    ///pin d18 = b12,
    pin uart1_tx = b12,

    /// ## Digital 19
    /// RX1 - GPIO #19, to maintain compatibility with the Mega pinout this can
    /// also be a Serial RX (on SERCOM4)
    ///
    /// mapped as: uart1_rx
    ///pin d19 = b13,
    pin uart1_rx = b13,

    /// ## Digital 20
    /// SDA - GPIO #20, and also I2C (Wire) data pin - This is the same as the
    /// SDA above
    ///
    /// mapped as: sda
    ///pin d20 = c16,
    pin sda = c16,

    /// ## Digital 21
    /// SCL - GPIO #21, and also I2C (Wire) clock pin - This is the same as the
    /// SCL above. The original Mega had this as a separate I2C port but we
    /// have lots of SERCOMs and to keep things simple, we tied them together
    ///
    /// mapped as: scl
    ///pin d21 = c17,
    pin scl = c17,

    /// ## Digital 22-23
    /// These are general purpose GPIO.
    pin d22 = d12,
    pin d23 = a15,

    /// ## USB Pins

    /// Digital 24
    /// USB D-
    ///
    /// mapped as: usb_dm
    ///pin d24 = a24,
    pin usb_dm = a24,

    /// Digital 25
    /// USB D+
    ///
    /// mapped as: usb_dp
    ///pin d25 = a25,
    pin usb_dp = a25,

    /// # Parallel Capture Peripheral (PCC) Pins
    /// There's a 'camera' input peripheral you can use with some camera chips
    /// to capture video with 14-bit data width. We thought this was neat so
    /// we made sure all those pins were available. Here are the PCC pins
    /// (left) and the Grand Central M4 pins it's mapped to. Unlike other
    /// peripherals, you cannot mux these signals to other pins!

    /// ## Digital 26
    /// PCC: DEN1
    pin d26 = a12,

    /// ## Digital 27
    /// PCC: DEN2
    pin d27 = a13,

    /// ## Digital 28
    /// PCC: CLK
    pin d28 = a14,

    /// ## Digital 29
    /// PCC: XCLK
    pin d29 = b19,

    /// ## Digital 30
    /// PCC: D7
    pin d30 = a23,

    /// ## Digital 31
    /// PCC: D6
    pin d31 = a22,

    /// ## Digital 32
    /// PCC: D5
    pin d32 = a21,

    /// ## Digital 33
    /// PCC: D4
    pin d33 = a20,

    /// ## Digital 34
    /// PCC: D3
    pin d34 = a19,

    /// ## Digital 35
    /// PCC: D2
    pin d35 = a18,

    /// ## Digital 36
    /// PCC: D1
    pin d36 = a17,

    /// ## Digital 37
    /// PCC: D0
    pin d37 = a16,

    /// ## Digital 38
    /// PCC: D9
    pin d38 = b15,

    /// ## Digital 39
    /// PCC: D8
    pin d39 = b14,

    /// ## Digital 40
    /// PCC: D11
    pin d40 = c13,

    /// ## Digital 41
    /// PCC: D10
    pin d41 = c12,

    /// ## Digital 42
    /// PCC: D13
    pin d42 = c15,

    /// ## Digital 43
    /// PCC: D12
    pin d43 = c14,

    /// Digital 44-49
    /// These are general purpose GPIO.
    pin d44 = c11,
    pin d45 = c10,
    pin d46 = c6,
    pin d47 = c7,
    pin d48 = c4,
    pin d49 = c5,

    /// # Hardware SPI
    /// These are the hardware SPI pins, are are connected to the 2x3
    /// header in the middle of the board. you can use them as everyday GPIO
    /// pins (but recommend keeping them free as they are best used for
    /// hardware SPI connections for high speed.)

    /// Digital 50
    /// MISO - This is the same as the header in the middle of the board when
    /// used for SPI
    ///
    /// mapped as: miso
    ///pin d50 = d11,
    pin miso = d11,

    /// Digital 51
    /// MOSI -This is the same as the header in the middle of the board when
    /// used for SPI
    ///
    /// mapped as: mosi
    ///pin d51 = d8,
    pin mosi = d8,

    /// Digital 52
    /// SCK -This is the same as the header in the middle of the board when
    /// used for SPI
    ///
    /// mapped as: sck
    ///pin d52 = d9,
    pin sck = d9,

    /// ## Digital 52
    /// SS - This is just named SS for back-compatibility with the Mega's
    /// SPI secondary-select pin.
    ///
    /// mapped as: SS
    ///pin d53 = d10,
    pin ss = d10,

    /// # QSPI Flash
    /// The QSPI Flash is connected to 6 pins that are not brought out on
    /// the GPIO pads. This way you don't have to worry about the SPI flash
    /// colliding with other devices on the main SPI connection.
    ///
    /// QSPI is neat because it allows you to have 4 data in/out lines instead
    /// of just SPI's single line in and single line out. This means that QSPI
    /// is at least 4 times faster. But in reality is at least 10x faster
    /// because you can clock the QSPI peripheral much faster than a plain SPI
    /// peripheral.

    /// ## QSPI Flash SCK
    pin flash_sck = b10,

    /// ## QSPI Flash IO0
    pin flash_io0 = a8,

    /// ## QSPI Flash IO1
    pin flash_io1 = a9,

    /// ## QSPI Flash IO2
    pin flash_io2 = a10,

    /// ## QSPI Flash IO3
    pin flash_io3 = a11,

    /// ## QSPI Flash CS
    pin flash_cs = b11,

    /// # NeoPixel
    /// The NeoPixel is connected to pin #88. The NeoPixel is powered by the
    /// 3.3V power supply but that hasn't shown to make a big difference in
    /// brightness or color. The NeoPixel is also used by the bootloader to
    /// let you know if the device has enumerated correctly (green) or USB
    /// failure (red).

    /// ## NeoPixel Pin
    pin neopixel = c24,

    /// # Micro SD Card Pins

    /// ## SD Card MOSI
    pin sd_mosi = b26,

    /// ## SD CARD SCK
    pin sd_sck = b27,

    /// ## SD CARD CS
    pin sd_cs = b28,

    /// ## SD CARD MISO
    pin sd_miso = b29,

    /// ## SD CARD SWO?
    pin swo = b30,

    /// ## SD CARD CD
    pin sd_cd = b31,

    /// Analog Reference Pin
    pin aref = a3,

    // TODO: ADC

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
            a6: self.a6,
            a7: self.a7,
            a8: self.a8,
            a9: self.a9,
            a10: self.a10,
            a11: self.a11,
            a12: self.a12,
            a13: self.a13,
            a14: self.a14,
            a15: self.a15,
        };

        let flash = QSPIFlash {
            sck: self.flash_sck,
            cs: self.flash_cs,
            data0: self.flash_io0,
            data1: self.flash_io1,
            data2: self.flash_io2,
            data3: self.flash_io3,
        };

        let spi = SPI {
            sck: self.sck,
            mosi: self.mosi,
            miso: self.miso,
        };

        let sdcard = SdCard {
            cs: self.sd_cs,
            cd: self.sd_cd,
        };

        let i2c = I2C {
            sda: self.sda,
            scl: self.scl,
        };

        let usb = USB {
            dm: self.usb_dm,
            dp: self.usb_dp,
        };

        let uart0 = UART0_ {
            rx: self.uart0_rx,
            tx: self.uart0_tx,
        };

        let uart1 = UART1_ {
            rx: self.uart1_rx,
            tx: self.uart1_tx,
        };

        let uart2 = UART2_ {
            rx: self.uart2_rx,
            tx: self.uart2_tx,
        };

        let uart3 = UART3_ {
            rx: self.uart3_rx,
            tx: self.uart3_tx,
        };

        let neopixel = Neopixel {
            neopixel: self.neopixel,
        };

        Sets {
            port: self.port,
            analog,
            spi,
            usb,
            flash,
            sdcard,
            i2c,
            uart0,
            uart1,
            uart2,
            uart3,
            tx_led: self.tx_led,
            rx_led: self.rx_led,
            neopixel,
        }
    }
}

/// Sets of pins split apart by category
pub struct Sets {
    pub tx_led: Pc30<Input<Floating>>,
    pub rx_led: Pc31<Input<Floating>>,

    /// Analog pins
    pub analog: Analog,

    /// SPI (external pinout) pins
    pub spi: SPI,

    /// SdCard
    pub sdcard: SdCard,

    /// I2C (external pinout) pins
    pub i2c: I2C,

    /// QSPI Flash pins
    pub flash: QSPIFlash,

    /// Neopixel
    pub neopixel: Neopixel,

    /// USB pins
    pub usb: USB,

    /// UART (external pinout) pins
    pub uart0: UART0_,
    pub uart1: UART1_,
    pub uart2: UART2_,
    pub uart3: UART3_,

    /// Port
    pub port: Port,
}

/// SPI pins
pub struct SPI {
    pub mosi: gpio::Pd8<Input<Floating>>,
    pub miso: gpio::Pd11<Input<Floating>>,
    pub sck: gpio::Pd9<Input<Floating>>,
}

impl SPI {
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom7: SERCOM7,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> SPIMaster7<
        hal::sercom::Sercom7Pad3<gpio::Pd11<gpio::PfC>>,
        hal::sercom::Sercom7Pad0<gpio::Pd8<gpio::PfC>>,
        hal::sercom::Sercom7Pad1<gpio::Pd9<gpio::PfC>>,
    > {
        let gclk0 = clocks.gclk0();
        SPIMaster7::new(
            &clocks.sercom7_core(&gclk0).unwrap(),
            bus_speed.into(),
            hal::hal::spi::Mode {
                phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
                polarity: hal::hal::spi::Polarity::IdleLow,
            },
            sercom7,
            mclk,
            (
                self.miso.into_pad(port),
                self.mosi.into_pad(port),
                self.sck.into_pad(port),
            ),
        )
    }
}

/// I2C pins
pub struct I2C {
    pub sda: Pc16<Input<Floating>>,
    pub scl: Pc17<Input<Floating>>,
}

impl I2C {
    /// Convenience for setting up the labelled SDA, SCL pins to
    /// operate as an I2C master running at the specified frequency.
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom3: SERCOM6,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> I2CMaster6<Sercom6Pad0<Pc16<PfC>>, Sercom6Pad1<Pc17<PfC>>> {
        let gclk0 = clocks.gclk0();
        I2CMaster6::new(
            &clocks.sercom6_core(&gclk0).unwrap(),
            bus_speed.into(),
            sercom3,
            mclk,
            self.sda.into_pad(port),
            self.scl.into_pad(port),
        )
    }
}

/// Sd Card pins
pub struct SdCard {
    pub cs: Pb28<Input<Floating>>,
    pub cd: Pb31<Input<Floating>>,
}

/// USB pins
pub struct USB {
    pub dm: Pa24<Input<Floating>>,
    pub dp: Pa25<Input<Floating>>,
}

impl USB {
    #[cfg(feature = "usb")]
    /// Convenience for setting up the onboard usb port to operate
    /// as a USB device.
    pub fn init(
        self,
        usb: super::pac::USB,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
    ) -> UsbBusAllocator<UsbBus> {
        clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
        let usb_gclk = clocks.get_gclk(GEN_A::GCLK2).unwrap();
        let usb_clock = &clocks.usb(&usb_gclk).unwrap();

        UsbBusAllocator::new(UsbBus::new(usb_clock, mclk, self.dm, self.dp, usb))
    }
}

/// UART0 pins
pub struct UART0_ {
    pub tx: Pb24<Input<Floating>>,
    pub rx: Pb25<Input<Floating>>,
}

impl UART0_ {
    /// Convenience for setting up the labelled TX, RX pins
    /// to operate as a UART device at the specified baud rate.
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        baud: F,
        sercom0: SERCOM0,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UART0<Sercom0Pad1<gpio::Pb25<gpio::PfC>>, Sercom0Pad0<gpio::Pb24<gpio::PfC>>, (), ()> {
        let gclk0 = clocks.gclk0();

        UART0::new(
            &clocks.sercom0_core(&gclk0).unwrap(),
            baud.into(),
            sercom0,
            mclk,
            (self.rx.into_pad(port), self.tx.into_pad(port)),
        )
    }
}

/// UART1 pins
pub struct UART1_ {
    pub tx: Pb12<Input<Floating>>,
    pub rx: Pb13<Input<Floating>>,
}

impl UART1_ {
    /// Convenience for setting up the labelled TX, RX pins
    /// to operate as a UART device at the specified baud rate.
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        baud: F,
        sercom4: SERCOM4,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UART4<Sercom4Pad1<gpio::Pb13<gpio::PfC>>, Sercom4Pad0<gpio::Pb12<gpio::PfC>>, (), ()> {
        let gclk0 = clocks.gclk0();

        UART4::new(
            &clocks.sercom4_core(&gclk0).unwrap(),
            baud.into(),
            sercom4,
            mclk,
            (self.rx.into_pad(port), self.tx.into_pad(port)),
        )
    }
}

/// UART2 pins
pub struct UART2_ {
    pub tx: Pc22<Input<Floating>>,
    pub rx: Pc23<Input<Floating>>,
}

impl UART2_ {
    /// Convenience for setting up the labelled TX, RX pins
    /// to operate as a UART device at the specified baud rate.
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        baud: F,
        sercom1: SERCOM1,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UART1<Sercom1Pad1<gpio::Pc23<gpio::PfC>>, Sercom1Pad0<gpio::Pc22<gpio::PfC>>, (), ()> {
        let gclk0 = clocks.gclk0();

        UART1::new(
            &clocks.sercom1_core(&gclk0).unwrap(),
            baud.into(),
            sercom1,
            mclk,
            (self.rx.into_pad(port), self.tx.into_pad(port)),
        )
    }
}

/// UART3 pins
pub struct UART3_ {
    pub tx: Pb16<Input<Floating>>,
    pub rx: Pb17<Input<Floating>>,
}

impl UART3_ {
    /// Convenience for setting up the labelled TX, RX pins
    /// to operate as a UART device at the specified baud rate.
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        baud: F,
        sercom5: SERCOM5,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UART5<Sercom5Pad1<gpio::Pb17<gpio::PfC>>, Sercom5Pad0<gpio::Pb16<gpio::PfC>>, (), ()> {
        let gclk0 = clocks.gclk0();

        UART5::new(
            &clocks.sercom5_core(&gclk0).unwrap(),
            baud.into(),
            sercom5,
            mclk,
            (self.rx.into_pad(port), self.tx.into_pad(port)),
        )
    }
}

pub struct Analog {
    pub a0: Pa2<Input<Floating>>,
    pub a1: Pa5<Input<Floating>>,
    pub a2: Pb3<Input<Floating>>,
    pub a3: Pc0<Input<Floating>>,
    pub a4: Pc1<Input<Floating>>,
    pub a5: Pc2<Input<Floating>>,
    pub a6: Pc3<Input<Floating>>,
    pub a7: Pb4<Input<Floating>>,
    pub a8: Pb5<Input<Floating>>,
    pub a9: Pb6<Input<Floating>>,
    pub a10: Pb7<Input<Floating>>,
    pub a11: Pb8<Input<Floating>>,
    pub a12: Pb9<Input<Floating>>,
    pub a13: Pa4<Input<Floating>>,
    pub a14: Pa6<Input<Floating>>,
    pub a15: Pa7<Input<Floating>>,
}

/// QSPI flash pins
pub struct QSPIFlash {
    pub sck: Pb10<Input<Floating>>,
    pub cs: Pb11<Input<Floating>>,
    pub data0: Pa8<Input<Floating>>,
    pub data1: Pa9<Input<Floating>>,
    pub data2: Pa10<Input<Floating>>,
    pub data3: Pa11<Input<Floating>>,
}

/// Neopixel pins
pub struct Neopixel {
    pub neopixel: Pc24<Input<Floating>>,
}

impl Neopixel {
    /// Convenience for setting up the onboard neopixels using the provided
    /// Timer preconfigured to 3mhz.
    pub fn init<T: CountDown + Periodic>(
        self,
        timer: T,
        port: &mut Port,
    ) -> ws2812::Ws2812<
        T,
        hal::ehal::digital::v1_compat::OldOutputPin<
            atsamd_hal::common::gpio::Pc24<
                atsamd_hal::common::gpio::Output<atsamd_hal::common::gpio::PushPull>,
            >,
        >,
    > {
        let neopixel_pin: OldOutputPin<_> = self.neopixel.into_push_pull_output(port).into();

        ws2812::Ws2812::new(timer, neopixel_pin)
    }
}
