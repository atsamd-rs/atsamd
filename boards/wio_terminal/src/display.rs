use atsamd_hal::clock::GenericClockController;
use atsamd_hal::ehal::blocking::delay::DelayMs;
use atsamd_hal::ehal::digital::v2::OutputPin;
use atsamd_hal::ehal::spi::{Phase, Polarity};
use atsamd_hal::pac::{MCLK, SERCOM7};
use atsamd_hal::sercom::spi;
use atsamd_hal::sercom::{IoSet4, Sercom7};
use atsamd_hal::time::Hertz;
use atsamd_hal::typelevel::NoneT;
use display_interface_spi::SPIInterface;
use ili9341::{DisplaySize240x320, Ili9341, Orientation};

use super::pins::aliases::*;

/// ILI9341 LCD display pins (uses `SERCOM7`)
pub struct Display {
    /// LCD MISO pin
    pub miso: LcdMisoReset,

    /// LCD MOSI pin
    pub mosi: LcdMosiReset,

    /// LCD SCK pin
    pub sck: LcdSckReset,

    /// LCD chip select pin
    pub cs: LcdCsReset,

    /// LCD data/command pin
    pub dc: LcdDcReset,

    /// LCD reset pin
    pub reset: LcdResetReset,

    /// LCD backlight pin
    pub backlight: LcdBacklightReset,
}

pub type LcdPads = spi::Pads<Sercom7, IoSet4, NoneT, LcdMosi, LcdSck>;
pub type LcdSpi = spi::Spi<spi::Config<LcdPads>, spi::Tx>;

/// Type alias for the ILI9341 LCD display.
pub type LCD = Ili9341<SPIInterface<LcdSpi, LcdDc, LcdCs>, LcdReset>;

pub use ili9341::Scroller;

impl Display {
    /// Initialize the display and its corresponding SPI bus peripheral. Return
    /// a tuple containing the configured display driver struct and backlight
    /// pin.
    pub fn init<D: DelayMs<u16>>(
        self,
        clocks: &mut GenericClockController,
        sercom7: SERCOM7,
        mclk: &mut MCLK,
        baud: Hertz,
        delay: &mut D,
    ) -> Result<(LCD, LcdBacklight), ()> {
        // Initialize the SPI peripherial on the configured pins, using SERCOM7.
        let gclk0 = clocks.gclk0();
        let clock = &clocks.sercom7_core(&gclk0).ok_or(())?;
        let pads = spi::Pads::default().data_out(self.mosi).sclk(self.sck);
        let spi = spi::Config::new(mclk, sercom7, pads, clock.freq())
            .spi_mode(spi::MODE_0)
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
