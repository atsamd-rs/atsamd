use core::marker::PhantomData;

use atsamd_hal::{
    clock::{GenericClockController, Sercom6CoreClock},
    delay::Delay,
    pac::{Mclk, Qspi},
    prelude::*,
    qspi,
    sercom::{spi, IoSet1, Sercom6},
    time::Hertz,
    typelevel::NoneT,
};
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use embedded_sdmmc::{SdCard, TimeSource, Timestamp, VolumeIdx, VolumeManager};

use super::pins::aliases::*;

/// QSPI Flash pins (uses `SERCOM4`)
pub struct QSPIFlash {
    /// QSPI Flash `sck` pin
    pub sck: QspiSckReset,

    /// QSPI Flash chip select pin
    pub cs: QspiCsReset,

    /// QSPI Flash `d0` pin
    pub d0: QspiD0Reset,

    /// QSPI Flash `d1` pin
    pub d1: QspiD1Reset,

    /// QSPI Flash `d2` pin
    pub d2: QspiD2Reset,

    /// QSPI Flash `d3` pin
    pub d3: QspiD3Reset,
}

impl QSPIFlash {
    pub fn init(self, mclk: &mut Mclk, qspi: Qspi) -> qspi::Qspi<qspi::OneShot> {
        qspi::Qspi::new(
            mclk, qspi, self.sck, self.cs, self.d0, self.d1, self.d2, self.d3,
        )
    }
}

/// SD Card pins (uses `SERCOM6`)
pub struct SDCard {
    /// SD Card chip select pin
    pub cs: SdCsReset,

    /// SD Card `mosi` pin
    pub mosi: SdMosiReset,

    /// SD Card `sck` pin
    pub sck: SdSckReset,

    /// SD Card `miso` pin
    pub miso: SdMisoReset,

    /// SD Card detect pin
    pub det: SdDetReset,
}

pub type SdPads = spi::Pads<Sercom6, IoSet1, SdMiso, SdMosi, SdSck>;
pub type SdSpi = spi::Spi<spi::Config<SdPads>, spi::Duplex>;

type Controller<TS> = embedded_sdmmc::VolumeManager<
    embedded_sdmmc::SdCard<ExclusiveDevice<SdSpi, SdCs, NoDelay>, Delay>,
    TS,
>;

/// An initialized SPI SDMMC controller.
pub struct SDCardController<TS: TimeSource> {
    pub volume: Controller<TS>,
    sercom6_clk: Sercom6CoreClock,
}

impl<TS: TimeSource> core::ops::Deref for SDCardController<TS> {
    type Target = Controller<TS>;

    fn deref(&self) -> &Self::Target {
        &self.volume
    }
}

impl<TS: TimeSource> core::ops::DerefMut for SDCardController<TS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.volume
    }
}

impl SDCard {
    /// Initialize the controller and its corresponding SPI bus peripheral.
    /// It is suggested to set higher buad rates after initialization, but it is
    /// for now left as the user responsibility.
    pub fn init<TS: TimeSource>(
        self,
        clocks: &mut GenericClockController,
        sercom6: Sercom6,
        mclk: &mut Mclk,
        delay: Delay,
        ts: TS,
    ) -> Result<(SDCardController<TS>, SdDet), ()> {
        let gclk0 = clocks.gclk0();
        let sercom6_clk = clocks.sercom6_core(&gclk0).ok_or(())?;
        let pads = spi::Pads::default()
            .data_out(self.mosi)
            .data_in(self.miso)
            .sclk(self.sck);
        let sdmmc_spi_bus = spi::Config::new(mclk, sercom6, pads, sercom6_clk.freq())
            .spi_mode(spi::MODE_0)
            .baud(400.kHz())
            .enable();
        let sdmmc_cs = self.cs.into_push_pull_output();

        let sdmmc_spi = ExclusiveDevice::new_no_delay(sdmmc_spi_bus, sdmmc_cs)
            .expect("Failed to create SpiDevice");

        let card = embedded_sdmmc::SdCard::new(sdmmc_spi, delay);

        let mut volume_mgr = VolumeManager::new(card, ts);

        Ok((
            SDCardController {
                volume: volume_mgr,
                sercom6_clk,
            },
            self.det.into(),
        ))
    }
}
