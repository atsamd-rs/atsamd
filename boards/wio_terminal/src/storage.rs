#[rustfmt::skip]
use atsamd_hal::clock::GenericClockController;
use atsamd_hal::gpio::{
    Floating, Input, Output, Pa10, Pa11, Pa8, Pa9, Pb10, Pb11, Pc16, Pc17, Pc18, Pc19, Pd21, PfC,
    Port, PushPull,
};
use atsamd_hal::hal::spi;
use atsamd_hal::prelude::*;
use atsamd_hal::sercom::{PadPin, SPIMaster6, Sercom6Pad0, Sercom6Pad1, Sercom6Pad2};
use atsamd_hal::target_device::{MCLK, SERCOM6};

/// QSPI Flash pins (uses `SERCOM4`)
pub struct QSPIFlash {
    /// QSPI Flash `sck` pin
    pub sck: Pb10<Input<Floating>>,

    /// QSPI Flash chip select pin
    pub cs: Pb11<Input<Floating>>,

    /// QSPI Flash `d0` pin
    pub d0: Pa8<Input<Floating>>,

    /// QSPI Flash `d1` pin
    pub d1: Pa9<Input<Floating>>,

    /// QSPI Flash `d2` pin
    pub d2: Pa10<Input<Floating>>,

    /// QSPI Flash `d3` pin
    pub d3: Pa11<Input<Floating>>,
}

/// SD Card pins (uses `SERCOM6`)
pub struct SDCard {
    /// SD Card chip select pin
    pub cs: Pc19<Input<Floating>>,

    /// SD Card `mosi` pin
    pub mosi: Pc16<Input<Floating>>,

    /// SD Card `sck` pin
    pub sck: Pc17<Input<Floating>>,

    /// SD Card `miso` pin
    pub miso: Pc18<Input<Floating>>,

    /// SD Card detect pin
    pub det: Pd21<Input<Floating>>,
}

/// Type alias for SERCOM6 initialized for interacting with an SD card.
pub type SDCardSPI =
    SPIMaster6<Sercom6Pad2<Pc18<PfC>>, Sercom6Pad0<Pc16<PfC>>, Sercom6Pad1<Pc17<PfC>>>;

/// Type alias for the SDCard chip select pin.
pub type SDCardCS = Pc19<Output<PushPull>>;

impl SDCard {
    /// Initialize the SDCard and its corresponding SPI bus peripheral.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        sercom6: SERCOM6,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> Result<(SDCardSPI, SDCardCS, Pd21<Input<Floating>>), ()> {
        let gclk0 = clocks.gclk0();
        let spi = SPIMaster6::new(
            &clocks.sercom6_core(&gclk0).ok_or(())?,
            3.mhz(),
            spi::Mode {
                phase: spi::Phase::CaptureOnFirstTransition,
                polarity: spi::Polarity::IdleLow,
            },
            sercom6,
            mclk,
            (
                self.miso.into_pad(port),
                self.mosi.into_pad(port),
                self.sck.into_pad(port),
            ),
        );

        // Configure the chip select pin as a push-pull output.
        let cs = self.cs.into_push_pull_output(port);
        // let dc = self.dc.into_push_pull_output(port);
        // let reset = self.reset.into_push_pull_output(port);
        //
        // // Create a SPIInterface over the peripheral, then create the ILI9341 driver
        // // using said interface and set its default orientation.
        // let interface = SPIInterface::new(spi, dc, cs);
        // let mut ili9341 = Ili9341::new(interface, reset, delay).map_err(|_| ())?;
        // ili9341
        //     .set_orientation(Orientation::LandscapeFlipped)
        //     .map_err(|_| ())?;
        //
        // // Configure the backlight pin as a push-pull output; unfortunately this pin
        // // does not appear to support PWM.
        // //   HIGH - backlight enabled
        // //   LOW  - backlight disabled
        // let mut backlight = self.backlight.into_push_pull_output(port);
        // backlight.set_high()?;

        // Return a result consisting of a Tuple containing the display driver and
        // backlight pin.
        Ok((spi, cs, self.det))
    }
}
