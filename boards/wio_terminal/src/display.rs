use atsamd_hal::clock::GenericClockController;
use atsamd_hal::gpio::v2::*;
use atsamd_hal::hal::blocking::delay::DelayMs;
use atsamd_hal::hal::spi::{Phase, Polarity};
use atsamd_hal::pac::{MCLK, SERCOM7};
use atsamd_hal::prelude::*;
use atsamd_hal::sercom::v2::spi;
use atsamd_hal::sercom::v2::{IoSet4, Sercom7};
use atsamd_hal::time::Hertz;
use atsamd_hal::typelevel::NoneT;
use display_interface_spi::SPIInterface;
use ili9341::{DisplaySize240x320, Ili9341, Orientation};

/// ILI9341 LCD display pins (uses `SERCOM7`)
pub struct Display {
    /// LCD MISO pin
    pub miso: Pin<PB18, Disabled<Floating>>,

    /// LCD MOSI pin
    pub mosi: Pin<PB19, Disabled<Floating>>,

    /// LCD SCK pin
    pub sck: Pin<PB20, Disabled<Floating>>,

    /// LCD chip select pin
    pub cs: Pin<PB21, Disabled<Floating>>,

    /// LCD data/command pin
    pub dc: Pin<PC06, Disabled<Floating>>,

    /// LCD reset pin
    pub reset: Pin<PC07, Disabled<Floating>>,

    /// LCD backlight pin
    pub backlight: Pin<PC05, Disabled<Floating>>,
}

pub type LcdPads = spi::PadsFromIds<Sercom7, IoSet4, NoneT, PB19, PB20>;
pub type LcdSpi = spi::Spi<spi::Config<LcdPads>, spi::Tx>;

/// Type alias for the ILI9341 LCD display.
pub type LCD = Ili9341<
    SPIInterface<LcdSpi, Pin<PC06, Output<PushPull>>, Pin<PB21, Output<PushPull>>>,
    Pin<PC07, Output<PushPull>>,
>;

pub use ili9341::Scroller;

impl Display {
    /// Initialize the display and its corresponding SPI bus peripheral. Return
    /// a tuple containing the configured display driver struct and backlight
    /// pin.
    pub fn init<F: Into<Hertz>, D: DelayMs<u16>>(
        self,
        clocks: &mut GenericClockController,
        sercom7: SERCOM7,
        mclk: &mut MCLK,
        baud: F,
        delay: &mut D,
    ) -> Result<(LCD, Pin<PC05, Output<PushPull>>), ()> {
        // Initialize the SPI peripherial on the configured pins, using SERCOM7.
        let gclk0 = clocks.gclk0();
        let clock = &clocks.sercom7_core(&gclk0).ok_or(())?;
        let pads = spi::Pads::default().data_out(self.mosi).sclk(self.sck);
        let spi = spi::Config::new(mclk, sercom7, pads, clock.freq())
            .cpol(Polarity::IdleLow)
            .cpha(Phase::CaptureOnFirstTransition)
            .baud(baud)
            .enable();

        // Configure the chip select, data/command, and reset pins as push-pull outputs.
        let cs = self.cs.into_push_pull_output();
        let dc = self.dc.into_push_pull_output();
        let reset = self.reset.into_push_pull_output();

        // Create a SPIInterface over the peripheral, then create the ILI9341 driver
        // using said interface and set its default orientation.
        let interface = SPIInterface::new(spi, dc, cs);
        let ili9341 = Ili9341::new(
            interface,
            reset,
            delay,
            Orientation::LandscapeFlipped,
            DisplaySize240x320,
        )
        .map_err(|_| ())?;

        // Configure the backlight pin as a push-pull output; unfortunately this pin
        // does not appear to support PWM.
        //   HIGH - backlight enabled
        //   LOW  - backlight disabled
        let mut backlight = self.backlight.into_push_pull_output();
        backlight.set_high().ok();

        // Return a result consisting of a Tuple containing the display driver and
        // backlight pin.
        Ok((ili9341, backlight))
    }
}
