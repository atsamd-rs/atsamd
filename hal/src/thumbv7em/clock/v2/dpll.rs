#![warn(missing_docs)]
#![deny(warnings)]
//! Digital Phase Locked Loop (DPLL)
//!
//! DPLL allows user to multiply an input signal and supports a handful of them.
//! It maintains the output signal frequency by constant phase comparison
//! against the reference, input signal.
//!
//! There are two DPLLs that are available
//!
//! - [`Enabled`]`<`[`Dpll`]`<`[`Pll0`]`, _>>`: [`Dpll0`]
//! - [`Enabled`]`<`[`Dpll`]`<`[`Pll1`]`, _>>`: [`Dpll1`]
//!
//! Each of them can operate in 3 different modes
//!
//! - [`Enabled`]`<`[`Dpll`]`<_, `[`PclkDriven`]`>`: `Gclk` as a reference clock
//! - [`Enabled`]`<`[`Dpll`]`<_, `[`XoscDriven`]`>`: `Xosc{0, 1}` as a reference
//!   clock
//! - [`Enabled`]`<`[`Dpll`]`<_, `[`Xosc32kDriven`]`>`: `Xosc32k` as a
//!   reference_clk
//!
//! These can be created by using an appropriate construction function:
//! [`Dpll::from_pclk`], [`Dpll::from_xosc32k`] or [`Dpll::from_xosc`]
//! and enabled by [`Dpll::enable`] function call

use core::marker::PhantomData;

use typenum::U0;

use crate::pac::oscctrl::dpll::{dpllstatus, dpllsyncbusy, DPLLCTRLA, DPLLCTRLB, DPLLRATIO};
use crate::pac::oscctrl::DPLL;

pub use crate::pac::oscctrl::dpll::dpllctrlb::REFCLK_A as DpllSrc;

use crate::clock::types::{Counter, Decrement, Enabled, Increment};
use crate::clock::v2::{Source, SourceMarker};
use crate::time::Hertz;
use crate::typelevel::Sealed;

use super::gclk::{GclkSource, GclkSourceEnum, GclkSourceMarker, GenNum};
use super::gclkio::NotGclkInput;
use super::pclk::{Pclk, PclkSourceMarker, PclkType};

//==============================================================================
// DpllNum
//==============================================================================

/// Trait ensuring all [`DpllNum`] has a numeric identities
pub trait DpllNum: Sealed {
    /// Associated constant marking an index of a [`Dpll`] type. It is useful in
    /// [`DpllToken`] in order to properly apply the offset to get adequate
    /// [`DPLL`] register
    const NUM: usize;
}

/// Type defining a [`Dpll`] providing a numeric identity
///
/// Goal of this type is to be a `Dpll` variant identifier used as a generic
/// parameter in an [`Dpll`]. Also as a source marker type in [`Dpll`]'s
/// dependees.
pub enum Pll0 {}

impl Sealed for Pll0 {}

impl DpllNum for Pll0 {
    const NUM: usize = 0;
}

/// Type defining a [`Dpll`] providing a numeric identity
///
/// Goal of this type is to be a `Dpll` variant identifier used as a generic
/// parameter in an [`Dpll`]. Also as a source marker type in [`Dpll`]'s
/// dependees.
pub enum Pll1 {}

impl Sealed for Pll1 {}

impl DpllNum for Pll1 {
    const NUM: usize = 1;
}

//==============================================================================
// DpllSource
//==============================================================================

/// Source marker trait for [`Dpll`] sources
///
/// Note: This trait is used inconsistently; [`Pclk`] as a source is used
/// directly in an API and therefore matching over this type is redundant.
pub trait DpllSourceMarker: SourceMarker {
    /// Associated constant provides a variant of a low level enum type from PAC
    /// that is used during a HW register write
    const DPLL_SRC: DpllSrc;
}

/// This trait represents a [`Dpll`] source provider.
///
/// Note: This trait is used inconsistently; [`Pclk`] as a source is used
/// directly in an API and therefore abstracting away through this trait is
/// redundant.
pub trait DpllSource: Source {
    /// Associated type used in order to mark
    /// [`Dpll<_, XoscDriven<_, T>>`]/[`Dpll<_, Xosc32kDriven<_, T>>`] type with
    /// a proper `T`, according to what `source` was passed into the
    /// [`Dpll::from_xosc`]/[`Dpll::from_xosc32k`] and to only allow calls into
    /// [`Dpll::free`] with a matching `source`
    type Type: DpllSourceMarker;
}

/// [`DpllSource`] subtrait that is used to distinguish between external 32kHz
/// and non-32kHz oscillators
pub trait DpllSourceXosc32k: DpllSource {}

/// [`DpllSource`] subtrait that is used to distinguish between external 32kHz
/// and non-32kHz oscillators
pub trait DpllSourceXosc: DpllSource {}

//==============================================================================
// DpllToken
//==============================================================================

/// Token type required to construct a [`Dpll`] type instance.
///
/// From an [`atsamd_hal`][`crate`] external user perspective, it does not
/// contain any methods and serves only a token purpose.
///
/// Within an [`atsamd_hal`][`crate`], [`DpllToken`] struct is a low-level
/// access abstraction for HW register calls.
pub struct DpllToken<D: DpllNum> {
    dpll: PhantomData<D>,
}

impl<D: DpllNum> DpllToken<D> {
    /// Constructor
    ///
    /// Unsafe: There should always be only a single instance thereof. It can be
    /// retrieved upon disabling and freeing an [`Enabled`]`<`[`Dpll`]`>`
    /// instance returned from `crate::clock::v2::retrieve_clocks` method
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self { dpll: PhantomData }
    }

    #[inline]
    fn dpll(&self) -> &DPLL {
        unsafe { &(*crate::pac::OSCCTRL::ptr()).dpll[D::NUM] }
    }

    #[inline]
    fn ctrla(&self) -> &DPLLCTRLA {
        &self.dpll().dpllctrla
    }

    #[inline]
    fn ctrlb(&self) -> &DPLLCTRLB {
        &self.dpll().dpllctrlb
    }

    #[inline]
    fn ratio(&self) -> &DPLLRATIO {
        &self.dpll().dpllratio
    }

    #[inline]
    fn syncbusy(&self) -> dpllsyncbusy::R {
        self.dpll().dpllsyncbusy.read()
    }

    #[inline]
    fn status(&self) -> dpllstatus::R {
        self.dpll().dpllstatus.read()
    }

    /// Set the loop division, see page 701 in the Datasheet
    ///
    /// Formula for calculating the frequency:
    /// f_clk_dpll = clk_src * (LDR + 1 + (LDRFRAC / 32))
    ///
    /// `int` is including the `+ 1`,
    /// 'frac` is the same as `LDRFRAC`
    ///
    /// Write to the divider must be write synchronized
    #[inline]
    fn set_loop_div(&mut self, int: u16, frac: u8) {
        self.ratio().write(|w| unsafe {
            w.ldr().bits((int - 1) & 0x1FFF);
            w.ldrfrac().bits(frac & 0x1F)
        });
        while self.syncbusy().dpllratio().bit_is_set() {}
    }

    /// Set the clock source.
    #[inline]
    fn set_source_clock(&mut self, variant: DpllSrc) {
        self.ctrlb().modify(|_, w| w.refclk().variant(variant));
    }

    /// When source is a XOSC this has effect, ignored otherwise.
    #[inline]
    fn set_source_div(&mut self, div: u16) {
        self.ctrlb()
            .modify(|_, w| unsafe { w.div().bits(div & 0x7FF) });
    }

    /// Ignore the lock, CLK_DPLLn is always running.
    #[inline]
    fn set_lock_bypass(&mut self, bypass: bool) {
        self.ctrlb().modify(|_, w| w.lbypass().bit(bypass));
    }

    /// Wake up fast, output the clock directly without waiting for lock.
    #[inline]
    fn set_wake_up_fast(&mut self, wuf: bool) {
        self.ctrlb().modify(|_, w| w.wuf().bit(wuf));
    }

    /// Wait until the DPLL clock is ready.
    #[inline]
    fn wait_until_ready(&self) {
        while self.status().clkrdy().bit_is_clear() {}
    }

    /// Wait until the DPLL clock is locked.
    #[inline]
    fn wait_until_locked(&self) {
        while self.status().lock().bit_is_clear() {}
    }

    /// Wait until register has been synchronized.
    #[inline]
    fn wait_until_enable_synced(&self) {
        while self.syncbusy().enable().bit_is_set() {}
    }

    /// Enable the DPLL, ensure register write is synchronized.
    #[inline]
    fn enable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().set_bit());
        self.wait_until_enable_synced();
    }

    /// Disable the DPLL, ensure register write is synchronized.
    #[inline]
    fn disable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().clear_bit());
        self.wait_until_enable_synced();
    }
}

//==============================================================================
// Mode structure for Dpll
//==============================================================================

/// 10 bits available for predivision
///
/// ```text
/// f_Div = f_xosc / (2 * (divider + 1))
/// ```
///
/// * Default division factor: 2 (register all 0)
/// * Maxumum division factor: 2048 (register all 1)
pub type DpllPredivider = u16;

/// This trait introduces a notion of different modes for [`Dpll`]
pub trait SrcMode<D: DpllNum>: Sealed {
    /// Return value of an effective predivider that can be applied on a
    /// source frequency
    fn predivider(&self) -> DpllPredivider;
    /// Perform mode specific HW register writes. By principle it should be
    /// called only in [`Dpll::enable`]
    fn enable(&self, token: &mut DpllToken<D>);
}

/// Struct representing a [`Dpll<PllX, _>`] mode when it is powered by a
/// [`Gclk`][`crate::clock::v2::gclk::Gclk`] of choice via [`Pclk<PllX, _>`]
pub struct PclkDriven<D, T>
where
    D: DpllNum + PclkType,
    T: PclkSourceMarker,
{
    reference_clk: Pclk<D, T>,
}

impl<D: DpllNum + PclkType, T: PclkSourceMarker> SrcMode<D> for PclkDriven<D, T> {
    fn predivider(&self) -> DpllPredivider {
        1_u16
    }

    fn enable(&self, token: &mut DpllToken<D>) {
        // Set the source
        token.set_source_clock(DpllSrc::GCLK);
    }
}

impl<D: DpllNum + PclkType, T: PclkSourceMarker> Sealed for PclkDriven<D, T> {}

/// Struct representing a [`Dpll`] mode when it is powered by a non-32kHz
/// external oscillator
pub struct XoscDriven<D: DpllNum, T: DpllSourceMarker> {
    dpll_num: PhantomData<D>,
    src: PhantomData<T>,
    raw_predivider: DpllPredivider,
}

impl<D: DpllNum, T: DpllSourceMarker> SrcMode<D> for XoscDriven<D, T> {
    fn predivider(&self) -> DpllPredivider {
        2 * (1 + self.raw_predivider)
    }

    fn enable(&self, token: &mut DpllToken<D>) {
        // Set the source
        token.set_source_clock(T::DPLL_SRC);

        // Set the predivider
        token.set_source_div(self.raw_predivider);
    }
}

impl<D: DpllNum, T: DpllSourceMarker> Sealed for XoscDriven<D, T> {}

/// Struct representing a [`Dpll`] mode when it is powered by a 32kHz
/// external oscillator
pub struct Xosc32kDriven<D: DpllNum, T: DpllSourceMarker> {
    dpll_num: PhantomData<D>,
    src: PhantomData<T>,
}

impl<D: DpllNum, T: DpllSourceMarker> SrcMode<D> for Xosc32kDriven<D, T> {
    fn predivider(&self) -> DpllPredivider {
        1_u16
    }

    fn enable(&self, token: &mut DpllToken<D>) {
        // Set the source
        token.set_source_clock(T::DPLL_SRC);
    }
}

impl<D: DpllNum, T: DpllSourceMarker> Sealed for Xosc32kDriven<D, T> {}

//==============================================================================
// Dpll
//==============================================================================

/// Struct representing a [`Dpll`] abstraction
///
/// It is generic over:
/// - a numeric variant ([`DpllNum`]; available variants: [`Pll0`], [`Pll1`])
/// - a mode of operation ([`SrcMode<D>`]; available modes: [`PclkDriven`],
///   [`XoscDriven`], [`Xosc32kDriven`])
pub struct Dpll<D, M>
where
    D: DpllNum,
    M: SrcMode<D>,
{
    token: DpllToken<D>,
    src_freq: Hertz,
    mult: u16,
    frac: u8,
    mode: M,
}

impl<D, T> Dpll<D, PclkDriven<D, T>>
where
    D: DpllNum + PclkType,
    T: PclkSourceMarker,
{
    /// Create a [`Dpll`] from Peripheral Channel (Pclk) fed from Gclk
    ///
    /// Input frequency must be between 32 kHz and 3.2 MHz
    ///
    /// Holds the [`Pclk`] until released on deconstruction ([`Dpll<_,
    /// PclkDriven<_, _>>::free`])
    #[inline]
    pub fn from_pclk(token: DpllToken<D>, reference_clk: Pclk<D, T>) -> Self {
        let src_freq = reference_clk.freq();
        assert!(src_freq.0 >= 32_000);
        assert!(src_freq.0 <= 3_200_000);
        let (mult, frac) = (1, 0);
        Self {
            token,
            src_freq,
            mult,
            frac,
            mode: PclkDriven { reference_clk },
        }
    }

    /// Deconstructs the [`Dpll`], returns the held [`Pclk`]
    #[inline]
    pub fn free(self) -> (DpllToken<D>, Pclk<D, T>) {
        (self.token, self.mode.reference_clk)
    }
}

impl<D, T> Dpll<D, Xosc32kDriven<D, T>>
where
    D: DpllNum,
    T: DpllSourceMarker,
{
    /// Create a [`Dpll`] from an external 32k oscillator
    ///
    /// Input frequency must be between 32 kHz and 3.2 MHz
    ///
    /// Increments a counter in `reference_clk`
    #[inline]
    pub fn from_xosc32k<S>(token: DpllToken<D>, reference_clk: S) -> (Self, S::Inc)
    where
        S: DpllSourceXosc32k<Type = T> + Increment,
    {
        let src_freq = reference_clk.freq();
        assert!(src_freq.0 >= 32_000);
        assert!(src_freq.0 <= 3_200_000);
        let (mult, frac) = (1, 0);

        let dpll = Dpll {
            token,
            src_freq,
            mult,
            frac,
            mode: Xosc32kDriven {
                src: PhantomData,
                dpll_num: PhantomData,
            },
        };
        (dpll, reference_clk.inc())
    }

    /// Deconstructs a [`Dpll`] instance, releases a token and decrements a
    /// counter in `reference_clk`
    #[inline]
    pub fn free<S>(self, reference_clk: S) -> (DpllToken<D>, S::Dec)
    where
        S: DpllSourceXosc32k<Type = T> + Decrement,
    {
        (self.token, reference_clk.dec())
    }
}

impl<D, T> Dpll<D, XoscDriven<D, T>>
where
    D: DpllNum,
    T: DpllSourceMarker,
{
    /// Create a [`Dpll`] from an external oscillator
    /// ([Xosc0][super::xosc::Xosc0]/[Xosc1][super::xosc::Xosc1])
    ///
    /// Input frequency must be between 32 kHz and 3.2 MHz
    ///
    /// Provides additional input pre-divider, see [DpllPredivider]
    ///
    /// Increments a counter in `reference_clk`
    #[inline]
    pub fn from_xosc<S>(
        token: DpllToken<D>,
        reference_clk: S,
        predivider: DpllPredivider,
    ) -> (Self, S::Inc)
    where
        S: DpllSourceXosc<Type = T> + Increment,
    {
        let raw_predivider = predivider;
        let src_freq = reference_clk.freq();
        let (mult, frac) = (1, 0);

        // Assert that the raw_predivider is valid!
        // 2 to 2048

        // Calculate the Dpll input frequency taking into consideration the
        // raw_predivider, but store the actual input source frequency
        let mode = XoscDriven {
            dpll_num: PhantomData,
            src: PhantomData,
            raw_predivider,
        };
        let frequency = src_freq.0 / mode.predivider() as u32;
        assert!(frequency >= 32_000);
        assert!(frequency <= 3_200_000);

        let dpll = Dpll {
            token,
            src_freq,
            mult,
            frac,
            mode,
        };
        (dpll, reference_clk.inc())
    }

    /// Set the predivider, see [DpllPredivider]
    #[inline]
    pub fn set_source_div(mut self, predivider: DpllPredivider) -> Self {
        // Assert the source pre-divider does not go outside input frequency
        // specifications
        let raw_predivider = predivider;
        self.mode.raw_predivider = raw_predivider;
        let frequency = self.src_freq.0 / self.mode.predivider() as u32;
        assert!(frequency >= 32_000);
        assert!(frequency <= 3_200_000);
        self
    }

    /// Deconstructs a [`Dpll`] instance, releases a token and decrements a
    /// counter in `reference_clk`
    #[inline]
    pub fn free<S>(self, reference_clk: S) -> (DpllToken<D>, S::Dec)
    where
        S: DpllSourceXosc<Type = D> + Decrement,
    {
        (self.token, reference_clk.dec())
    }
}

impl<D, M> Dpll<D, M>
where
    D: DpllNum,
    M: SrcMode<D>,
{
    /// Set the DPLL divider
    ///
    /// Calculated as
    ///
    /// ```
    /// f_clk_dpll = clk_src * (int + (frac / 32))
    /// ```
    ///
    /// The `+ 1` in the datasheet is not forgotten, it is handled by the
    /// underlying register write function
    ///
    /// Example 1:
    /// ```
    /// clk_src = 2 MHz
    /// int = 50
    /// frac = 0
    ///
    /// 2 * 50 = 100 MHz
    /// ```
    /// Example 2:
    /// ```
    /// clk_src = 32 kHz
    /// int = 3000
    /// frac = 24
    ///
    /// 0.032 * (3000 +  24/32) = 96.024 MHz
    /// ```
    #[inline]
    pub fn set_loop_div(mut self, int: u16, frac: u8) -> Self {
        self.token.set_loop_div(int, frac);
        self.mult = int;
        self.frac = frac;
        self
    }

    /// Set to ignore the phase-lock, CLK_DPLL is always running regardless of
    /// lock status
    #[inline]
    pub fn set_lock_bypass(mut self, bypass: bool) -> Self {
        self.token.set_lock_bypass(bypass);
        self
    }

    /// Set to skip waiting for DPLL lock before outputting clock
    #[inline]
    pub fn set_wake_up_fast(mut self, wuf: bool) -> Self {
        self.token.set_wake_up_fast(wuf);
        self
    }
    /// Busy-wait until DPLL has achieved lock
    #[inline]
    pub fn wait_until_locked(self) -> Self {
        self.token.wait_until_locked();
        self
    }
    /// Busy-wait until DPLL is ready
    #[inline]
    pub fn wait_until_ready(self) -> Self {
        self.token.wait_until_ready();
        self
    }
    /// Return the frequency of the DPLL
    #[inline]
    pub fn freq(&self) -> Hertz {
        Hertz(
            self.src_freq.0 / self.mode.predivider() as u32
                * (self.mult as u32 + self.frac as u32 / 32),
        )
    }

    /// Enabling a [`Dpll`] modifies hardware to match the configuration stored
    /// within
    #[inline]
    pub fn enable(mut self) -> Enabled<Self, U0> {
        // TODO: This assertion is suspicious.
        assert!(self.freq().0 <= 200_000_000);
        self.mode.enable(&mut self.token);
        // Set the loop divider ratio
        self.token.set_loop_div(self.mult, self.frac);
        // Enable the DPLL
        self.token.enable();
        Enabled::new(self)
    }
}

/// Encapsulation for Dpll0
pub type Dpll0<M> = Dpll<Pll0, M>;

/// Encapsulation for Dpll1
pub type Dpll1<M> = Dpll<Pll1, M>;

impl<D, M> Enabled<Dpll<D, M>, U0>
where
    D: DpllNum,
    M: SrcMode<D>,
{
    /// Disable the [`Dpll`]
    #[inline]
    pub fn disable(mut self) -> Dpll<D, M> {
        self.0.token.disable();
        self.0
    }
}

//==============================================================================
// GclkSource
//==============================================================================

impl GclkSourceMarker for Pll0 {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::DPLL0;
}

impl NotGclkInput for Pll0 {}

impl SourceMarker for Pll0 {}

impl GclkSourceMarker for Pll1 {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::DPLL1;
}

impl NotGclkInput for Pll1 {}

impl SourceMarker for Pll1 {}

impl<G, D, M, N> GclkSource<G> for Enabled<Dpll<D, M>, N>
where
    G: GenNum,
    D: DpllNum + GclkSourceMarker,
    M: SrcMode<D>,
    N: Counter,
{
    type Type = D;
}

impl<D, M, N> Source for Enabled<Dpll<D, M>, N>
where
    D: DpllNum + GclkSourceMarker,
    M: SrcMode<D>,
    N: Counter,
{
    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
