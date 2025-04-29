use atsamd_hal::clock::GenericClockController;
use atsamd_hal::ehal;
use atsamd_hal::ehal::delay::DelayNs;
use atsamd_hal::ehal_nb::serial::Write;
use atsamd_hal::ehal::digital::OutputPin;
use atsamd_hal::ehal::spi::{Phase, Polarity, SpiBus, SpiDevice};
use atsamd_hal::pac::Mclk;
use atsamd_hal::sercom::spi;
use atsamd_hal::sercom::spi::Spi;
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
pub type LcdSpiWrapper = SpiDeviceWrapper<LcdCs>;

/// Type alias for the ILI9341 LCD display.
//  pub type LCD = Ili9341<SPIInterface<LcdSpi, LcdDc>, LcdReset>;
pub type LCD = Ili9341<SPIInterface<LcdSpiWrapper, LcdDc>, LcdReset>;
//  impl<SPI, DC> WriteOnlyDataCommand for SPIInterface<SPI, DC>
//  where
//      SPI: SpiDevice,
//      DC: OutputPin,

pub use ili9341::Scroller;

pub struct SpiDeviceWrapper<DC>{
    spi: LcdSpi,
    dc: DC,
}

impl<DC> SpiDeviceWrapper<DC> {
    fn new(spi: LcdSpi, dc: DC) -> Self {
        Self { spi, dc }
    }
}
use atsamd_hal::ehal::spi::{Operation, ErrorType as SpiErrorType};
impl<Cs> SpiErrorType for SpiDeviceWrapper<Cs> {
    type Error = spi::Error;
}
impl<Cs> ehal::spi::SpiDevice<u8> for SpiDeviceWrapper<Cs> {
    /// Perform a transaction against the device.
    ///
    /// - Locks the bus
    /// - Asserts the CS (Chip Select) pin.
    /// - Performs all the operations.
    /// - [Flushes](SpiBus::flush) the bus.
    /// - Deasserts the CS pin.
    /// - Unlocks the bus.
    ///
    /// The locking mechanism is implementation-defined. The only requirement is it must prevent two
    /// transactions from executing concurrently against the same bus. Examples of implementations are:
    /// critical sections, blocking mutexes, returning an error or panicking if the bus is already busy.
    ///
    /// On bus errors the implementation should try to deassert CS.
    /// If an error occurs while deasserting CS the bus error should take priority as the return value.
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error>{
        todo!()
    }

    /// Do a read within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(&mut [Operation::Read(buf)])`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::read`]
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
       // self.spi.read(buf)
       todo!() 
    }

    /// Do a write within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(&mut [Operation::Write(buf)])`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::write`]
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
       SpiBus::write(&mut self.spi, buf);
       Ok(())
    }

    /// Do a transfer within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(&mut [Operation::Transfer(read, write)]`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::transfer`]
    #[inline]
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }

    /// Do an in-place transfer within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(&mut [Operation::TransferInPlace(buf)]`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::transfer_in_place`]
    #[inline]
    fn transfer_in_place(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        todo!()
    }
}

impl Display {
    /// Initialize the display and its corresponding SPI bus peripheral. Return
    /// a tuple containing the configured display driver struct and backlight
    /// pin.
    pub fn init<D: DelayNs>(
        self,
        clocks: &mut GenericClockController,
        sercom7: Sercom7,
        mclk: &mut Mclk,
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

        let spi = SpiDeviceWrapper::new(spi, cs);
        // Create a SPIInterface over the peripheral, then create the ILI9341 driver
        // using said interface and set its default orientation.
        let interface = SPIInterface::new(spi, dc);
        //  interface.send_command(0x01, &[]).ok();
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