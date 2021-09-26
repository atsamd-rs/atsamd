//! NeoTrellis M4 Express pins

use super::{hal, pac, pac::MCLK, pac::SERCOM2, pac::SERCOM4};

use hal::clock::*;
use hal::define_pins;
use hal::gpio::{self, *};
use hal::sercom::{I2CMaster2, I2CMaster4, PadPin, Sercom4Pad0, Sercom4Pad1, UART4};
use hal::time::Hertz;
#[cfg(feature = "adxl343")]
use hal::{prelude::*, sercom::I2CError};

#[cfg(feature = "adxl343")]
use adxl343::Adxl343;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    pac: pac,

    /// Analog pin 0
    pin a0 = a2,
    /// Analog pin 1
    pin a1 = a5,
    /// INT pin
    pin a2 = a4,
    /// Microphone out
    pin micout = a6,
    /// Microphone in
    pin micin = a7,

    /// SDA
    pin sda = b8,
    /// SCL
    pin scl = b9,

    /// Accelerometer data signal (SDA)
    pin accel_sda = a12,
    /// Accelerometer clock signal (SCL)
    pin accel_scl = a13,

    /// Keypad Column 0
    pin col0 = a14,
    /// Keypad Column 1
    pin col1 = a15,
    /// Keypad Column 2
    pin col2 = a16,
    /// Keypad Column 3
    pin col3 = a17,
    /// Keypad Column 4
    pin col4 = a20,
    /// Keypad Column 5
    pin col5 = a21,
    /// Keypad Column 6
    pin col6 = a22,
    /// Keypad Column 7
    pin col7 = a23,

    /// Keypad Row 0
    pin row0 = a18,
    /// Keypad Row 1
    pin row1 = a19,
    /// Keypad Row 2
    pin row2 = b22,
    /// Keypad Row 3
    pin row3 = b23,

    /// NeoPixels
    pin neopixel = a27,

    /// APA102 (RGB LED control) SCK
    pin dotstar_ci = b2,
    /// APA102 (RGB LED control) MOSI
    pin dotstar_di = b3,
);

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let accel = Accelerometer {
            sda: self.accel_sda,
            scl: self.accel_scl,
        };

        let analog = Analog {
            a0: self.a0,
            a1: self.a1,
            a2: self.a2,
        };

        let audio = Audio {
            input: self.micin,
            output: self.micout,
        };

        let dotstar = Dotstar {
            ci: self.dotstar_ci,
            di: self.dotstar_di,
        };

        let stemma = STEMMA {
            sda: self.sda,
            scl: self.scl,
        };

        let keypad = Keypad {
            col0: self.col0,
            col1: self.col1,
            col2: self.col2,
            col3: self.col3,
            col4: self.col4,
            col5: self.col5,
            col6: self.col6,
            col7: self.col7,
            row0: self.row0,
            row1: self.row1,
            row2: self.row2,
            row3: self.row3,
        };

        Sets {
            accel,
            analog,
            audio,
            dotstar,
            stemma,
            keypad,
            neopixel: self.neopixel,
            port: self.port,
        }
    }
}

/// Sets of pins split apart by category
pub struct Sets {
    /// Accelerometer pins
    pub accel: Accelerometer,

    /// Analog pins
    pub analog: Analog,

    /// Audio pins
    pub audio: Audio,

    /// Dotstar (RGB LED) pins
    pub dotstar: Dotstar,

    /// STEMMA JST connector, which can be I2C, SPI, or UART
    pub stemma: STEMMA,

    /// Keypad pins
    pub keypad: Keypad,

    /// Neopixel pins
    pub neopixel: NeoPixel,

    /// Port
    pub port: Port,
}

/// Accelerometer pins
pub struct Accelerometer {
    pub sda: Pa12<Input<Floating>>,
    pub scl: Pa13<Input<Floating>>,
}

impl Accelerometer {
    /// Open the Accelerometer I2C device
    #[cfg(feature = "adxl343")]
    pub fn open(
        self,
        clocks: &mut GenericClockController,
        sercom: SERCOM2,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> Result<
        Adxl343<
            I2CMaster2<
                hal::sercom::Sercom2Pad0<Pa12<gpio::PfC>>,
                hal::sercom::Sercom2Pad1<gpio::Pa13<gpio::PfC>>,
            >,
        >,
        adxl343::accelerometer::Error<I2CError>,
    > {
        Adxl343::new(self.i2c_master(clocks, 100.khz(), sercom, mclk, port))
    }

    /// Configure accelerometer's SDA and SCL pins as an I2C master"
    pub fn i2c_master<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom: SERCOM2,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> I2CMaster2<
        hal::sercom::Sercom2Pad0<Pa12<gpio::PfC>>,
        hal::sercom::Sercom2Pad1<Pa13<gpio::PfC>>,
    > {
        let gclk0 = clocks.gclk0();
        I2CMaster2::new(
            &clocks.sercom2_core(&gclk0).unwrap(),
            bus_speed.into(),
            sercom,
            mclk,
            self.sda.into_pad(port),
            self.scl.into_pad(port),
        )
    }
}

/// Analog pins
pub struct Analog {
    pub a0: Pa2<Input<Floating>>,
    pub a1: Pa5<Input<Floating>>,
    pub a2: Pa4<Input<Floating>>,
}

/// Audio pins
pub struct Audio {
    pub output: Pa6<Input<Floating>>,
    pub input: Pa7<Input<Floating>>,
}

/// Dotstar pins
pub struct Dotstar {
    pub ci: Pb2<Input<Floating>>,
    pub di: Pb3<Input<Floating>>,
}

/// STEMMA JST pins
pub struct STEMMA {
    pub sda: Pb8<Input<Floating>>,
    pub scl: Pb9<Input<Floating>>,
}

impl STEMMA {
    /// Convenience for setting up the labelled SDA, SCL pins to
    /// operate as an I2C master running at the specified frequency.
    pub fn i2c_master<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom4: SERCOM4,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> I2CMaster4<Sercom4Pad0<Pb8<PfD>>, Sercom4Pad1<Pb9<PfD>>> {
        let gclk0 = clocks.gclk0();
        I2CMaster4::new(
            &clocks.sercom4_core(&gclk0).unwrap(),
            bus_speed.into(),
            sercom4,
            mclk,
            self.sda.into_pad(port),
            self.scl.into_pad(port),
        )
    }

    /// Convenience for setting up the labelled SDA, SCL pins to
    /// operate as a UART device at the specified baud rate.
    ///
    /// Here SCL is the RX pin and SDA is the TX pin.
    pub fn uart<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        baud: F,
        sercom4: SERCOM4,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UART4<Sercom4Pad1<Pb9<PfD>>, Sercom4Pad0<Pb8<PfD>>, (), ()> {
        let gclk0 = clocks.gclk0();

        UART4::new(
            &clocks.sercom4_core(&gclk0).unwrap(),
            baud.into(),
            sercom4,
            mclk,
            (self.scl.into_pad(port), self.sda.into_pad(port)),
        )
    }
}

/// Keypad pins
pub struct Keypad {
    pub col0: Pa14<Input<Floating>>,
    pub col1: Pa15<Input<Floating>>,
    pub col2: Pa16<Input<Floating>>,
    pub col3: Pa17<Input<Floating>>,
    pub col4: Pa20<Input<Floating>>,
    pub col5: Pa21<Input<Floating>>,
    pub col6: Pa22<Input<Floating>>,
    pub col7: Pa23<Input<Floating>>,
    pub row0: Pa18<Input<Floating>>,
    pub row1: Pa19<Input<Floating>>,
    pub row2: Pb22<Input<Floating>>,
    pub row3: Pb23<Input<Floating>>,
}

/// NeoPixel pin
pub type NeoPixel = Pa27<Input<Floating>>;
