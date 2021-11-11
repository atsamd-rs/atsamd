use crate::pins::{SdCs, SdMiso, SdMosi, SdSck};
use atsamd_hal::{
    clock::{GenericClockController, Sercom6CoreClock},
    gpio::v2::*,
    pac::{MCLK, QSPI, SERCOM6},
    prelude::*,
    qspi,
    sercom::v2::{
        spi::{self, Phase, Polarity},
        IoSet1, Sercom6,
    },
    time::Hertz,
    typelevel::NoneT,
};
use embedded_sdmmc::{SdMmcSpi, TimeSource};

/// QSPI Flash pins (uses `SERCOM4`)
pub struct QSPIFlash {
    /// QSPI Flash `sck` pin
    pub sck: Pin<PB10, Disabled<Floating>>,

    /// QSPI Flash chip select pin
    pub cs: Pin<PB11, Disabled<Floating>>,

    /// QSPI Flash `d0` pin
    pub d0: Pin<PA08, Disabled<Floating>>,

    /// QSPI Flash `d1` pin
    pub d1: Pin<PA09, Disabled<Floating>>,

    /// QSPI Flash `d2` pin
    pub d2: Pin<PA10, Disabled<Floating>>,

    /// QSPI Flash `d3` pin
    pub d3: Pin<PA11, Disabled<Floating>>,
}

impl QSPIFlash {
    pub fn init(self, mclk: &mut MCLK, qspi: QSPI) -> qspi::Qspi<qspi::OneShot> {
        qspi::Qspi::new(
            mclk, qspi, self.sck, self.cs, self.d0, self.d1, self.d2, self.d3,
        )
    }
}

/// SD Card pins (uses `SERCOM6`)
pub struct SDCard {
    /// SD Card chip select pin
    pub cs: Pin<PC19, Disabled<Floating>>,

    /// SD Card `mosi` pin
    pub mosi: Pin<PC16, Disabled<Floating>>,

    /// SD Card `sck` pin
    pub sck: Pin<PC17, Disabled<Floating>>,

    /// SD Card `miso` pin
    pub miso: Pin<PC18, Disabled<Floating>>,

    /// SD Card detect pin
    pub det: Pin<PD21, Disabled<Floating>>,
}

pub type SdPads = spi::Pads<Sercom6, IoSet1, SdMiso, SdMosi, SdSck>;
pub type SdSpi = spi::Spi<spi::Config<SdPads>, spi::Duplex>;
type Controller<TS> = embedded_sdmmc::Controller<SdMmcSpi<SdSpi, SdCs>, TS>;

/// An initialized SPI SDMMC controller.
pub struct SDCardController<TS: TimeSource> {
    pub cont: Controller<TS>,
    sercom6_clk: Sercom6CoreClock,
}

impl<TS: TimeSource> SDCardController<TS> {
    /// Initializes the MMC card. An error is returned if there is no card
    /// or a communications error occurs.
    pub fn set_baud<B: Into<Hertz>>(&mut self, baud: B) {
        self.cont.device().spi().reconfigure(|c| c.set_baud(baud));
    }
}

impl<TS: TimeSource> core::ops::Deref for SDCardController<TS> {
    type Target = Controller<TS>;

    fn deref(&self) -> &Self::Target {
        &self.cont
    }
}

impl<TS: TimeSource> core::ops::DerefMut for SDCardController<TS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cont
    }
}

impl SDCard {
    /// Initialize the controller and its corresponding SPI bus peripheral.
    pub fn init<TS: TimeSource>(
        self,
        clocks: &mut GenericClockController,
        sercom6: SERCOM6,
        mclk: &mut MCLK,
        ts: TS,
    ) -> Result<(SDCardController<TS>, Pin<PD21, Input<Floating>>), ()> {
        let gclk0 = clocks.gclk0();
        let sercom6_clk = clocks.sercom6_core(&gclk0).ok_or(())?;
        let pads = spi::Pads::default()
            .data_out(self.mosi)
            .data_in(self.miso)
            .sclk(self.sck);
        let spi = spi::Config::new(mclk, sercom6, pads, sercom6_clk.freq())
            .cpol(Polarity::IdleLow)
            .cpha(Phase::CaptureOnFirstTransition)
            .baud(400.khz())
            .enable();

        let cs = self.cs.into_push_pull_output();

        Ok((
            SDCardController {
                cont: embedded_sdmmc::Controller::new(SdMmcSpi::new(spi, cs), ts),
                sercom6_clk,
            },
            self.det.into(),
        ))
    }
}
