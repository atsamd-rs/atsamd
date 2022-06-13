use core::marker::PhantomData;

use embedded_hal::spi;
pub use embedded_hal::spi::{Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3};

use crate::sercom::{Sercom, SomePad, APB_CLK_CTRL};
use crate::time::Hertz;
use crate::typelevel::{NoneT, Sealed};

use super::{
    BitOrder, Capability, CtrlA, DefaultSize, DynCapability, Master, MasterHWSS, OpMode, Registers,
    Size, Slave, Spi, ValidPads,
};

#[cfg(any(feature = "samd11", feature = "samd21"))]
use super::CharSize;
#[cfg(feature = "min-samd51g")]
use super::{DynLength, StaticLength};

//=============================================================================
// Config
//=============================================================================

/// A configurable SPI peripheral in its disabled state
///
/// See the [module-level](super) documentation for more details on declaring
/// and instantiating `Pads` types.
pub struct Config<P, M = Master, Z = DefaultSize>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
    regs: Registers<P::Sercom>,
    pads: P,
    mode: PhantomData<M>,
    size: Z,
    freq: Hertz,
    baud: Hertz,
    ctrla: CtrlA,
}

impl<P: ValidPads> Config<P> {
    /// Create a new [`Config`] in the default configuration
    ///
    /// This function will enable the corresponding APB clock, reset the
    /// [`Sercom`] peripheral, and return a [`Config`] in the default
    /// configuration. The default [`OpMode`] is [`Master`], while the default
    /// [`Size`] is an
    #[cfg_attr(
        any(feature = "samd11", feature = "samd21"),
        doc = "[`EightBit`] [`CharSize`]"
    )]
    #[cfg_attr(feature = "min-samd51g", doc = "`EightBit` `CharSize`")]
    /// for SAMD11 and SAMD21 chips or a
    #[cfg_attr(any(feature = "samd11", feature = "samd21"), doc = "`Length` of `U1`")]
    #[cfg_attr(feature = "min-samd51g", doc = "[`Length`] of `U1`")]
    /// for SAMx5x chips. Note that [`Config`] takes ownership of both the
    /// PAC [`Sercom`] struct as well as the [`Pads`].
    ///
    /// Users must configure GCLK manually. The `freq` parameter represents the
    /// GCLK frequency for this [`Sercom`] instance.
    #[inline]
    pub fn new(
        apb_clk_ctrl: &APB_CLK_CTRL,
        mut sercom: P::Sercom,
        pads: P,
        freq: impl Into<Hertz>,
    ) -> Self {
        sercom.enable_apb_clock(apb_clk_ctrl);
        let freq = freq.into();
        let (dipo, dopo) = P::DIPO_DOPO;
        Self {
            regs: Registers { sercom },
            pads,
            mode: PhantomData,
            size: DefaultSize::default(),
            freq,
            baud: Hertz(freq.0 / 4),
            ctrla: CtrlA::default(dipo, dopo),
        }
    }
}

impl<P, M, Z> Config<P, M, Z>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
    /// Obtain a reference to the PAC `SERCOM` struct
    ///
    /// Directly accessing the `SERCOM` could break the invariants of the
    /// type-level tracking in this module, so it is unsafe.
    #[inline]
    pub unsafe fn sercom(&self) -> &P::Sercom {
        &self.regs.sercom
    }

    /// Consume the [`Config`], reset the peripheral, and return the [`Sercom`]
    /// and [`Pads`]
    #[inline]
    pub fn free(mut self) -> (P::Sercom, P) {
        self.regs.reset();
        (self.regs.sercom, self.pads)
    }

    /// Change the [`OpMode`]
    #[inline]
    pub fn op_mode<M2: OpMode>(self) -> Config<P, M2, Z> {
        Config {
            regs: self.regs,
            pads: self.pads,
            mode: PhantomData,
            size: self.size,
            ctrla: self.ctrla,
            freq: self.freq,
            baud: self.baud,
        }
    }

    /// Change the [`Size`]
    #[inline]
    fn size<Z2: Size>(self, size: Z2) -> Config<P, M, Z2> {
        Config {
            regs: self.regs,
            pads: self.pads,
            mode: self.mode,
            size,
            ctrla: self.ctrla,
            freq: self.freq,
            baud: self.baud,
        }
    }

    /// Change the [`CharSize`]
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    #[inline]
    pub fn char_size<C: CharSize>(self) -> Config<P, M, C> {
        self.size(C::default())
    }

    /// Change the transaction [`Length`]
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn length<L: StaticLength>(self) -> Config<P, M, L> {
        self.size(L::default())
    }

    /// Allow the transaction [`Length`] to be set dynamically
    ///
    /// The `LENGTH` register will be set to `length`. If `length == 0`, it will
    /// be set to 1.
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn dyn_length(self, length: u8) -> Config<P, M, DynLength> {
        let size = DynLength::new(length);
        self.size(size)
    }

    /// Set the clock polarity using the builder pattern
    #[inline]
    pub fn cpol(mut self, cpol: Polarity) -> Self {
        self.ctrla.cpol = cpol;
        self
    }

    /// Set the clock phase using the builder pattern
    #[inline]
    pub fn cpha(mut self, cpha: Phase) -> Self {
        self.ctrla.cpha = cpha;
        self
    }

    /// Set the SPI mode (clock polarity & phase) using the builder pattern
    #[inline]
    pub fn spi_mode(mut self, mode: spi::Mode) -> Self {
        self.ctrla.cpol = mode.polarity;
        self.ctrla.cpha = mode.phase;
        self
    }

    /// Set the bit order of transmission (MSB/LSB first) using the builder
    /// pattern
    ///
    /// This only affects the order of bits within each byte. Bytes are always
    /// transferred in little endian order from the 32-bit DATA register.
    #[inline]
    pub fn bit_order(mut self, bit_order: BitOrder) -> Self {
        self.ctrla.bit_order = bit_order;
        self
    }

    /// Set the baud rate using the builder API
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// half the GCLK frequency. The minimum baud rate is the GCLK frequency /
    /// 512. Values outside this range will saturate at the extremes.
    #[inline]
    pub fn baud(mut self, baud: impl Into<Hertz>) -> Self {
        self.baud = baud.into();
        self
    }

    /// Enable or disable the immediate buffer overflow notification using the
    /// builder API
    ///
    /// If set to true, an [`Error::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    pub fn ibon(mut self, enabled: bool) -> Self {
        self.ctrla.ibon = enabled;
        self
    }

    /// Enable or disable run in standby mode using the builder API
    #[inline]
    pub fn run_in_standby(mut self, enabled: bool) -> Self {
        self.ctrla.run_in_standby = enabled;
        self
    }

    /// Enable the SPI peripheral
    ///
    /// SPI transactions are not possible until the peripheral is enabled.
    /// This function is limited to [`ValidConfig`]s.
    #[inline]
    pub fn enable(mut self) -> Spi<P, M, Z>
    where
        Self: ValidConfig,
    {
        self.regs.reset();
        self.regs.set_op_mode(M::MODE, M::MSSEN);
        self.regs.set_ctrla(self.ctrla);
        #[cfg(any(feature = "samd11", feature = "samd21"))]
        self.regs.set_char_size(Z::BITS);
        #[cfg(feature = "min-samd51g")]
        self.regs.set_length(self.size.length());
        self.regs.set_baud(self.freq, self.baud);
        if P::Capability::DYN != DynCapability::Tx {
            self.regs.rx_enable();
        }
        self.regs.enable();
        Spi {
            regs: self.regs,
            pads: self.pads,
            capability: P::Capability::default(),
            mode: PhantomData,
            size: PhantomData,
            freq: self.freq,
        }
    }
}

//=============================================================================
// ValidConfig
//=============================================================================

/// Marker trait for valid SPI [`Config`]urations
///
/// A functional SPI peripheral must have, at a minimum, an SCLK pad and
/// either a Data In or a Data Out pad. Dependeing on the [`OpMode`], an SS
/// pad may also be required.
///
/// The `ValidConfig` trait is implemented only for valid combinations of
/// [`Pads`] and [`OpMode`]. No [`Config`] is valid if the SCK pad is [`NoneT`]
/// or if both the Data In and Data Out pads are `NoneT`. When in [`Master`]
/// `OpMode`, the `SS` pad must be `NoneT`, while in [`MasterHWSS`] or
/// [`Slave`] [`OpMode`], the SS pad must be [`SomePad`].
pub trait ValidConfig: Sealed {}

impl<P, M, Z> Sealed for Config<P, M, Z>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
}

impl<P, Z> ValidConfig for Config<P, Master, Z>
where
    P: ValidPads<SS = NoneT>,
    Z: Size,
{
}

impl<P, Z> ValidConfig for Config<P, MasterHWSS, Z>
where
    P: ValidPads,
    Z: Size,
    P::SS: SomePad,
{
}

impl<P, Z> ValidConfig for Config<P, Slave, Z>
where
    P: ValidPads,
    Z: Size,
    P::SS: SomePad,
{
}

//=============================================================================
// Reconfig
//=============================================================================

pub struct Reconfig {
    pub(super) ctrla: CtrlA,
    pub(super) baud: Hertz,
}

impl Reconfig {
    /// Get the clock polarity
    #[inline]
    pub fn get_cpol(&mut self) -> Polarity {
        self.ctrla.cpol
    }

    /// Set the clock polarity
    #[inline]
    pub fn set_cpol(&mut self, cpol: Polarity) {
        self.ctrla.cpol = cpol;
    }

    /// Get the clock phase
    #[inline]
    pub fn get_cpha(&self) -> Phase {
        self.ctrla.cpha
    }

    /// Set the clock phase
    #[inline]
    pub fn set_cpha(&mut self, cpha: Phase) {
        self.ctrla.cpha = cpha
    }

    /// Get the SPI mode (clock polarity & phase)
    #[inline]
    pub fn get_spi_mode(&self) -> spi::Mode {
        spi::Mode {
            polarity: self.ctrla.cpol,
            phase: self.ctrla.cpha,
        }
    }

    /// Set the SPI mode (clock polarity & phase)
    #[inline]
    pub fn set_spi_mode(&mut self, mode: spi::Mode) {
        self.ctrla.cpol = mode.polarity;
        self.ctrla.cpha = mode.phase;
    }

    /// Get the bit order of transmission (MSB/LSB first)
    ///
    /// This only affects the order of bits within each byte. Bytes are always
    /// transferred in little endian order from the 32-bit DATA register.
    #[inline]
    pub fn get_bit_order(&self) -> BitOrder {
        self.ctrla.bit_order
    }

    /// Set the bit order of transmission (MSB/LSB first) using the builder
    /// pattern
    ///
    /// This only affects the order of bits within each byte. Bytes are always
    /// transferred in little endian order from the 32-bit DATA register.
    #[inline]
    pub fn set_bit_order(&mut self, order: BitOrder) {
        self.ctrla.bit_order = order;
    }

    /// Get the baud rate
    ///
    /// The returned baud rate may not exactly match what was set.
    #[inline]
    pub fn get_baud(&mut self) -> Hertz {
        self.baud
    }

    /// Set the baud rate
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// half the GCLK frequency. The minimum baud rate is the GCLK frequency /
    /// 512. Values outside this range will saturate at the extremes.
    #[inline]
    pub fn set_baud(&mut self, baud: impl Into<Hertz>) {
        self.baud = baud.into();
    }

    /// Read the enabled state of the immediate buffer overflow notification
    ///
    /// If set to true, an [`Error::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    pub fn get_ibon(&self) -> bool {
        self.ctrla.ibon
    }

    /// Enable or disable the immediate buffer overflow notification
    ///
    /// If set to true, an [`Error::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    pub fn set_ibon(&mut self, enabled: bool) {
        self.ctrla.ibon = enabled;
    }

    /// Read the enable state of run in standby mode
    #[inline]
    pub fn get_run_in_standby(&self) -> bool {
        self.ctrla.run_in_standby
    }

    /// Enable or disable run in standby mode
    #[inline]
    pub fn set_run_in_standby(&mut self, enabled: bool) {
        self.ctrla.run_in_standby = enabled;
    }
}
