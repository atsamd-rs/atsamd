//! # Digital Phase Locked Loop (DPLL)
//!
//! [`Dpll`] allows user to multiply an input signal and supports a handful of
//! them. It maintains the output signal frequency by constant phase comparison
//! against the reference, input signal.
//!
//! There are two Dplls that are available
//!
//! - [`Enabled`]`<`[`Dpll`]`<`[`marker::Dpll0`]`, _>>`: [`Dpll0`]
//! - [`Enabled`]`<`[`Dpll`]`<`[`marker::Dpll1`]`, _>>`: [`Dpll1`]
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
//! - [`Dpll::from_pclk`]
//! - [`Dpll::from_xosc`]
//! - [`Dpll::from_xosc32k`]
//! and then enabled by [`Dpll::enable`] function call

use core::convert::Infallible;
use core::marker::PhantomData;

use typenum::U0;

use crate::pac::oscctrl::dpll::{dpllstatus, dpllsyncbusy, DPLLCTRLA, DPLLCTRLB, DPLLRATIO};
use crate::pac::oscctrl::DPLL;

pub use crate::pac::oscctrl::dpll::dpllctrlb::REFCLK_A as DynDpllSourceId;

use crate::time::Hertz;
use crate::typelevel::{Counter, Decrement, Increment, Sealed};

use super::gclk::GclkSourceId;
use super::pclk::{Pclk, PclkId, PclkSourceId};
use super::xosc::{self, Xosc, XoscId, XoscId0, XoscId1};
use super::xosc32k::{self, Xosc32k, Xosc32kId};
use super::{Driver, Enabled};

//==============================================================================
// DpllId
//==============================================================================

/// Type-level `enum` for DPLL identifiers
///
/// See the documentation / on [type-level enums] for more details on the
/// pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait DpllId: Sealed {
    /// Corresponding numeric index
    const NUM: usize;
}

/// Type-level variant representing the identity of DPLL0
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum DpllId0 {}

impl Sealed for DpllId0 {}

impl DpllId for DpllId0 {
    const NUM: usize = 0;
}

/// Type-level variant representing the identity of DPLL1
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum DpllId1 {}

impl Sealed for DpllId1 {}

impl DpllId for DpllId1 {
    const NUM: usize = 1;
}

//==============================================================================
// DpllSource
//==============================================================================

/// Type-level `enum` for DPLL sources
///
/// See the documentation / on [type-level enums] for more details on the
/// pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait DpllSourceId {
    /// Corresponding variant of [`DynDpllSourceId`]
    const DYN: DynDpllSourceId;
}

impl DpllSourceId for XoscId0 {
    const DYN: DynDpllSourceId = DynDpllSourceId::XOSC0;
}

impl DpllSourceId for XoscId1 {
    const DYN: DynDpllSourceId = DynDpllSourceId::XOSC1;
}

impl DpllSourceId for Xosc32kId {
    const DYN: DynDpllSourceId = DynDpllSourceId::XOSC32;
}

/// [`DpllSource`] subtrait that is used to distinguish between external 32kHz
/// and non-32kHz oscillators
pub trait DpllSourceXosc32k: Driver {}

impl<M, N> DpllSourceXosc32k for Enabled<Xosc32k<M>, N>
where
    M: xosc32k::Mode,
    N: Counter,
{
}

/// [`DpllSource`] subtrait that is used to distinguish between external 32kHz
/// and non-32kHz oscillators
pub trait DpllSourceXosc: Driver {}

impl<X, M, N> DpllSourceXosc for Enabled<Xosc<X, M>, N>
where
    X: XoscId + DpllSourceId,
    M: xosc::Mode<X>,
    N: Counter,
{
}

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
pub struct DpllToken<D: DpllId> {
    dpll: PhantomData<D>,
}

impl<D: DpllId> DpllToken<D> {
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
    fn set_source_clock(&mut self, variant: DynDpllSourceId) {
        self.ctrlb().modify(|_, w| w.refclk().variant(variant));
    }

    /// When source is a XOSC this has effect, ignored otherwise.
    #[inline]
    fn set_source_div(&mut self, div: u16) {
        self.ctrlb()
            .modify(|_, w| unsafe { w.div().bits(div & ((1 << 10) - 1)) });
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

    /// Check if [`Dpll`] clock is ready.
    #[inline]
    fn wait_until_ready(&self) -> nb::Result<(), Infallible> {
        if self.status().clkrdy().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }

    /// Check if [`Dpll`] clock is locked.
    #[inline]
    fn wait_until_locked(&self) -> nb::Result<(), Infallible> {
        if self.status().lock().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }

    /// Wait until register has been synchronized.
    #[inline]
    fn wait_until_enable_synced(&self) {
        while self.syncbusy().enable().bit_is_set() {}
    }

    /// Enable the [`Dpll`], ensure register write is synchronized.
    #[inline]
    fn enable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().set_bit());
        self.wait_until_enable_synced();
    }

    /// Disable the [`Dpll`], ensure register write is synchronized.
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
///
/// This value is relevant only for a [`Dpll`] that is [`XoscDriven`].
pub type DpllPredivider = u16;

/// This trait introduces a notion of different modes for [`Dpll`]
pub trait SrcMode<D: DpllId>: Sealed {
    /// Return value of an effective predivider that can be applied on a
    /// source frequency
    fn predivider(&self) -> DpllPredivider;
    /// Perform mode specific HW register writes. By principle it should be
    /// called only in [`Dpll::enable`]
    fn enable(&self, token: &mut DpllToken<D>);
}

/// Struct representing a [`Dpll<marker::DpllX, _>`] mode when it is powered by
/// a [`Gclk`][`crate::clock::v2::gclk::Gclk`] of choice via
/// [`Pclk<marker::DpllX, _>`]
pub struct PclkDriven<D, T>
where
    D: DpllId + PclkId,
    T: PclkSourceId,
{
    reference_clk: Pclk<D, T>,
}

impl<D: DpllId + PclkId, T: PclkSourceId> SrcMode<D> for PclkDriven<D, T> {
    #[inline]
    fn predivider(&self) -> DpllPredivider {
        1_u16
    }

    #[inline]
    fn enable(&self, token: &mut DpllToken<D>) {
        token.set_source_clock(DynDpllSourceId::GCLK);
    }
}

impl<D: DpllId + PclkId, T: PclkSourceId> Sealed for PclkDriven<D, T> {}

/// Struct representing a [`Dpll`] mode when it is powered by a non-32kHz
/// external oscillator
pub struct XoscDriven<D: DpllId, T: DpllSourceId> {
    dpll_num: PhantomData<D>,
    src: PhantomData<T>,
    raw_predivider: DpllPredivider,
}

impl<D: DpllId, T: DpllSourceId> SrcMode<D> for XoscDriven<D, T> {
    #[inline]
    fn predivider(&self) -> DpllPredivider {
        2 * (1 + self.raw_predivider)
    }

    #[inline]
    fn enable(&self, token: &mut DpllToken<D>) {
        token.set_source_clock(T::DYN);
        token.set_source_div(self.raw_predivider);
    }
}

impl<D: DpllId, T: DpllSourceId> Sealed for XoscDriven<D, T> {}

/// Struct representing a [`Dpll`] mode when it is powered by a 32kHz
/// external oscillator
pub struct Xosc32kDriven<D: DpllId, T: DpllSourceId> {
    dpll_num: PhantomData<D>,
    src: PhantomData<T>,
}

impl<D: DpllId, T: DpllSourceId> SrcMode<D> for Xosc32kDriven<D, T> {
    #[inline]
    fn predivider(&self) -> DpllPredivider {
        1_u16
    }

    #[inline]
    fn enable(&self, token: &mut DpllToken<D>) {
        token.set_source_clock(T::DYN);
    }
}

impl<D: DpllId, T: DpllSourceId> Sealed for Xosc32kDriven<D, T> {}

//==============================================================================
// Dpll
//==============================================================================

/// Struct representing a [`Dpll`] abstraction
///
/// It is generic over:
/// - a numeric variant (available variants: [`marker::Dpll0`],
///   [`marker::Dpll1`])
/// - a mode of operation (available modes: [`PclkDriven`], [`XoscDriven`],
///   [`Xosc32kDriven`])
pub struct Dpll<D, M>
where
    D: DpllId,
    M: SrcMode<D>,
{
    token: DpllToken<D>,
    src_freq: Hertz,
    mult: u16,
    frac: u8,
    lock_bypass: bool,
    wake_up_fast: bool,
    mode: M,
}

impl<D, T> Dpll<D, PclkDriven<D, T>>
where
    D: DpllId + PclkId,
    T: PclkSourceId,
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
        let (mult, frac) = (1, 0);
        Self {
            token,
            src_freq,
            mult,
            frac,
            lock_bypass: false,
            wake_up_fast: false,
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
    D: DpllId,
    T: DpllSourceId,
{
    /// Create a [`Dpll`] from an external 32k oscillator
    ///
    /// Input frequency must be between 32 kHz and 3.2 MHz
    ///
    /// Increments a counter in `reference_clk`
    #[inline]
    pub fn from_xosc32k<S>(token: DpllToken<D>, reference_clk: S) -> (Self, S::Inc)
    where
        S: DpllSourceXosc32k<Source = T> + Increment,
    {
        let src_freq = reference_clk.freq();
        let (mult, frac) = (1, 0);

        let dpll = Self {
            token,
            src_freq,
            mult,
            frac,
            lock_bypass: false,
            wake_up_fast: false,
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
        S: DpllSourceXosc32k<Source = T> + Decrement,
    {
        (self.token, reference_clk.dec())
    }
}

impl<D, T> Dpll<D, XoscDriven<D, T>>
where
    D: DpllId,
    T: DpllSourceId,
{
    /// Create a [`Dpll`] from an external oscillator
    /// ([Xosc0][super::xosc::Xosc0]/[Xosc1][super::xosc::Xosc1])
    ///
    /// Input frequency must be between 32 kHz and 3.2 MHz
    ///
    /// Provides additional input pre-divider, see [`DpllPredivider`]
    ///
    /// Increments a counter in `reference_clk`
    #[inline]
    pub fn from_xosc<S>(
        token: DpllToken<D>,
        reference_clk: S,
        predivider: DpllPredivider,
    ) -> (Self, S::Inc)
    where
        S: DpllSourceXosc<Source = T> + Increment,
    {
        let raw_predivider = predivider;
        let src_freq = reference_clk.freq();
        let (mult, frac) = (1, 0);

        let mode = XoscDriven {
            dpll_num: PhantomData,
            src: PhantomData,
            raw_predivider,
        };

        let dpll = Self {
            token,
            src_freq,
            mult,
            frac,
            lock_bypass: false,
            wake_up_fast: false,
            mode,
        };
        (dpll, reference_clk.inc())
    }

    /// Set the predivider, see [`DpllPredivider`]
    #[inline]
    pub fn set_source_div(mut self, predivider: DpllPredivider) -> Self {
        self.mode.raw_predivider = predivider;
        self
    }

    /// Deconstructs a [`Dpll`] instance, releases a token and decrements a
    /// counter in `reference_clk`
    #[inline]
    pub fn free<S>(self, reference_clk: S) -> (DpllToken<D>, S::Dec)
    where
        S: DpllSourceXosc<Source = D> + Decrement,
    {
        (self.token, reference_clk.dec())
    }
}

impl<D, M> Dpll<D, M>
where
    D: DpllId,
    M: SrcMode<D>,
{
    /// Set the [`Dpll`] divider
    ///
    /// Calculated as
    ///
    /// ```text
    /// f_clk_dpll = clk_src * (int + (frac / 32))
    /// ```
    ///
    /// The `+ 1` in the datasheet is not forgotten, it is handled by the
    /// underlying register write function
    ///
    /// Example 1:
    /// ```text
    /// clk_src = 2 MHz
    /// int = 50
    /// frac = 0
    ///
    /// 2 * 50 = 100 MHz
    /// ```
    /// Example 2:
    /// ```text
    /// clk_src = 32 kHz
    /// int = 3000
    /// frac = 24
    ///
    /// 0.032 * (3000 +  24/32) = 96.024 MHz
    /// ```
    #[inline]
    pub fn set_loop_div(mut self, int: u16, frac: u8) -> Self {
        self.mult = int;
        self.frac = frac;
        self
    }

    /// Set to ignore the phase-lock, CLK_DPLL is always running regardless of
    /// lock status
    #[inline]
    pub fn set_lock_bypass(mut self, bypass: bool) -> Self {
        self.lock_bypass = bypass;
        self
    }

    /// Set to skip waiting for [`Dpll`] lock before outputting clock
    #[inline]
    pub fn set_wake_up_fast(mut self, wuf: bool) -> Self {
        self.wake_up_fast = wuf;
        self
    }

    /// Return the frequency of the [`Dpll`]
    #[inline]
    pub fn freq(&self) -> Hertz {
        Hertz(
            self.src_freq.0 / self.mode.predivider() as u32
                * (self.mult as u32 + self.frac as u32 / 32),
        )
    }

    /// Enables [`Dpll`] and performs assertions in local configuration
    ///
    /// - Performs HW register writes
    #[inline]
    pub fn enable(self) -> Result<Enabled<Self, U0>, Self> {
        let predivider = self.mode.predivider() as u32;
        let input_frequency = self.src_freq.0 / predivider;
        let output_frequency = self.freq().0;

        // If Xosc mode: Predivider should be within a range <2, 2048>
        // Else: Predivider should be 1
        if (1..=2048).contains(&predivider)
            && (32_000..=3_200_000).contains(&input_frequency)
            && (96_000_000..=200_000_000).contains(&output_frequency)
        {
            unsafe { Ok(self.force_enable()) }
        } else {
            Err(self)
        }
    }

    /// Forcibly enables [`Dpll`] without additional checks in local
    /// configuration
    ///
    /// - Performs HW register writes
    #[inline]
    pub unsafe fn force_enable(mut self) -> Enabled<Self, U0> {
        // Enable the specified mode
        self.mode.enable(&mut self.token);
        // Set the loop divider ratio and other settings
        self.token.set_loop_div(self.mult, self.frac);
        self.token.set_lock_bypass(self.lock_bypass);
        self.token.set_wake_up_fast(self.wake_up_fast);
        // Enable the [`Dpll`]
        self.token.enable();
        Enabled::new(self)
    }
}

/// Alias of [`Dpll`]`<`[`marker::Dpll0`]`, _>`
pub type Dpll0<M> = Dpll<DpllId0, M>;

/// Alias of [`Dpll`]`<`[`marker::Dpll1`]`, _>`
pub type Dpll1<M> = Dpll<DpllId1, M>;

impl<D, M> Enabled<Dpll<D, M>, U0>
where
    D: DpllId,
    M: SrcMode<D>,
{
    /// Disable the [`Dpll`]
    #[inline]
    pub fn disable(mut self) -> Dpll<D, M> {
        self.0.token.disable();
        self.0
    }
}

impl<D, M, N> Enabled<Dpll<D, M>, N>
where
    D: DpllId,
    M: SrcMode<D>,
    N: Counter,
{
    /// Check if [`Dpll`] has achieved lock
    #[inline]
    pub fn wait_until_locked(&self) -> nb::Result<(), Infallible> {
        self.0.token.wait_until_locked()
    }

    /// Check if [`Dpll`] is ready
    #[inline]
    pub fn wait_until_ready(&self) -> nb::Result<(), Infallible> {
        self.0.token.wait_until_ready()
    }
}

//==============================================================================
// GclkSource
//==============================================================================

impl<D, M, N> Driver for Enabled<Dpll<D, M>, N>
where
    D: DpllId + GclkSourceId,
    M: SrcMode<D>,
    N: Counter,
{
    type Source = D;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
