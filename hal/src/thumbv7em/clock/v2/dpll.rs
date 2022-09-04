//! # Digital Phase-Locked Loop
//!
//! ## Overview
//!
//! The `dpll` module provides access to the two digital phase-locked loops
//! (DPLLs) within the `OSCCTRL` peripheral.
//!
//! A DPLL is used to multiply clock frequencies. It takes a lower-frequency
//! input clock and produces a higher-frequency output clock. It works by taking
//! the output clock, dividing it down to the same frequency as the input clock,
//! comparing phase between the two signals, and locking that phase difference
//! to zero. Consequently, the clock divider within the feedback loop sets the
//! frequency multiplication factor.
//!
//! The DPLLs operate over a large range of frequencies, but their operating
//! region is not infinite. Specifically, they can only accept input frequencies
//! between 32 kHz and 3.2 MHz, and they can only output frequencies in the
//! range of 96 MHz to 200 MHz.
//!
//! Creating and configuring a [`Dpll`] proceeds according to the principles
//! outlined in the [`clock` module documentation]. It is best shown with an
//! example.
//!
//! ## Example
//!
//! Suppose we start with the default clock tree after power-on reset.
//!
//! ```text
//! DFLL (48 MHz)
//! └── GCLK0 (48 MHz)
//!     └── Master clock (48 MHz)
//!
//! OSCULP base clock
//! ```
//!
//! We would like to transform it to a clock tree like this:
//!
//! ```text
//! DFLL (48 MHz)
//! └── GCLK1 (2 MHz)
//!     └── DPLL (200 MHz)
//!         └── GCLK0 (200 MHz)
//!             └── Master clock (200 MHz)
//!
//! OSCULP base clock
//! ```
//!
//! Let's start by using [`clock_system_at_reset`] to access the HAL clocking
//! structs.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         dpll::Dpll,
//!         gclk::{Gclk, GclkDiv16},
//!         pclk::Pclk,
//!     },
//!     pac::Peripherals,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! ```
//!
//! First, we would like to divide down the 48 MHz output of the [`Dfll`] to
//! produce a valid input frequency for the [`Dpll`]. We start by feeding the
//! already-[`Enabled`] [`Dfll`] to [`Gclk1`] with a [`GclkDivider`] of 24,
//! producing a 2 MHz output frequency. This has the side effect of
//! [`Increment`]ing the [`Counter`] for [`EnabledDfll`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dpll::Dpll,
//! #         gclk::{Gclk, GclkDiv16},
//! #         pclk::Pclk,
//! #     },
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! let (gclk1, dfll) = Gclk::from_source(tokens.gclks.gclk1, clocks.dfll);
//! let gclk1 = gclk1.div(GclkDiv16::Div(24)).enable();
//! ```
//!
//! Next, we use the output of [`Gclk1`] to enable the peripheral channel clock
//! ([`Pclk`]) for [`Dpll0`]. This [`Increment`]s the [`Counter`] for
//! [`EnabledGclk1`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dpll::Dpll,
//! #         gclk::{Gclk, GclkDiv16},
//! #         pclk::Pclk,
//! #     },
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let (gclk1, dfll) = Gclk::from_source(tokens.gclks.gclk1, clocks.dfll);
//! # let gclk1 = gclk1.div(GclkDiv16::Div(24)).enable();
//! let (pclk_dpll0, gclk1) = Pclk::enable(tokens.pclks.dpll0, gclk1);
//! ```
//!
//! Now we use [`Dpll::from_pclk`], which consumes the [`Pclk`] and returns an
//! instance of [`Dpll0`]. We use builder API functions to set the loop divider
//! to 100 and enable the [`Dpll`]. This will multiply the 2 MHz input clock to
//! produce a 200 MHz output clock.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dpll::Dpll,
//! #         gclk::{Gclk, GclkDiv16},
//! #         pclk::Pclk,
//! #     },
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let (gclk1, dfll) = Gclk::from_source(tokens.gclks.gclk1, clocks.dfll);
//! # let gclk1 = gclk1.div(GclkDiv16::Div(24)).enable();
//! # let (pclk_dpll0, gclk1) = Pclk::enable(tokens.pclks.dpll0, gclk1);
//! let dpll0 = Dpll::from_pclk(tokens.dpll0, pclk_dpll0)
//!     .loop_div(100, 0)
//!     .enable();
//! ```
//!
//! There are two things to note at this point.
//!
//! First, the loop divider has both an integer part and a fractional part.
//! However, users should generally avoid using fractional division, if
//! possible, because it increases the jitter of the output clock. See the
//! [`Dpll::loop_div`] documentation for more details.
//!
//! Second, because the input clock frequency and loop division factors are
//! run-time values, the [`Dpll`] cannot verify at compile time that the input
//! and output frequencies satisfy the requirements specified in the
//! [overview](self#overview). Instead, these values are checked at run-time. If
//! either frequency violates its requirement, the call to [`Dpll::enable`] will
//! panic.
//!
//! Finally, we wait until the [`EnabledDpll0`] output is ready, and then we
//! swap the [`EnabledGclk0`], which feeds the processor master clock, from the
//! 48 MHz [`EnabledDfll`] to the 200 MHz [`EnabledDpll0`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dpll::Dpll,
//! #         gclk::{Gclk, GclkDiv16},
//! #         pclk::Pclk,
//! #     },
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let (gclk1, dfll) = Gclk::from_source(tokens.gclks.gclk1, clocks.dfll);
//! # let gclk1 = gclk1.div(GclkDiv16::Div(24)).enable();
//! # let (pclk_dpll0, gclk1) = Pclk::enable(tokens.pclks.dpll0, gclk1);
//! # let dpll0 = Dpll::from_pclk(tokens.dpll0, pclk_dpll0)
//! #     .loop_div(100, 0)
//! #     .enable();
//! while !dpll0.is_ready() {}
//! let (gclk0, dfll, dpll0) = clocks.gclk0.swap_sources(dfll, dpll0);
//! ```
//!
//! We have now achieved the disired clock tree. The complete example is
//! provided below.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         dpll::Dpll,
//!         gclk::{Gclk, GclkDiv16},
//!         pclk::Pclk,
//!     },
//!     pac::Peripherals,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let (gclk1, dfll) = Gclk::from_source(tokens.gclks.gclk1, clocks.dfll);
//! let gclk1 = gclk1.div(GclkDiv16::Div(24)).enable();
//! let (pclk_dpll0, gclk1) = Pclk::enable(tokens.pclks.dpll0, gclk1);
//! let dpll0 = Dpll::from_pclk(tokens.dpll0, pclk_dpll0)
//!     .loop_div(100, 0)
//!     .enable();
//! while !dpll0.is_ready() {}
//! let (gclk0, dfll, dpll0) = clocks.gclk0.swap_sources(dfll, dpll0);
//! ```
//!
//! [`clock` module documentation]: super
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [`Dfll`]: super::dfll::Dfll
//! [`EnabledDfll`]: super::dfll::EnabledDfll
//! [`EnabledGclk0`]: super::gclk::EnabledGclk0
//! [`Gclk1`]: super::gclk::Gclk1
//! [`EnabledGclk1`]: super::gclk::EnabledGclk1
//! [`GclkDivider`]: super::gclk::GclkDivider
//! [`Pclk`]: super::pclk::Pclk

use core::marker::PhantomData;

use typenum::U0;

use crate::pac::oscctrl::dpll::{dpllstatus, dpllsyncbusy, DPLLCTRLA, DPLLCTRLB, DPLLRATIO};
use crate::pac::oscctrl::DPLL;

use crate::pac::oscctrl::dpll::dpllctrlb::REFCLK_A;

use crate::time::Hertz;
use crate::typelevel::{Counter, Decrement, Increment, Sealed};

use super::gclk::GclkId;
use super::pclk::{Pclk, PclkId, PclkToken};
use super::xosc::{Xosc0Id, Xosc1Id, XoscId};
use super::xosc32k::Xosc32kId;
use super::{Enabled, Source};

//==============================================================================
// DpllToken
//==============================================================================

/// Singleton token that can be exchanged for a [`Dpll`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// typically represent clocks that are disabled at power-on reset.
///
/// [`DpllToken`]s are no different. Both [`Dpll`]s are disabled at power-on
/// reset. To use a [`Dpll`], you must first exchange the token for an actual
/// clock with [`Dpll::from_pclk`] or [`Dpll::from_source`].
///
/// [`DpllToken`] is generic over the [`DpllId`], where each corresponding token
/// represents one of the two respective [`Dpll`]s.
//
// # Internal notes
//
// `DpllToken` is generic over the `DpllId`, and each corresponding instance is
// a singleton. There should never be more than one instance of `DpllToken` with
// a given `DpllId`, because `DpllToken` relies on this fact for memory safety.
//
// Users see `DpllToken` as merely an opaque token. but internally, `DpllToken`
// is also used as a register interface. The tokens are zero-sized, so they can
// be carried by all clock types without introducing any memory bloat.
//
// As part of that register interface, each `DpllToken` can access its
// corresponding `DPLL*` registers. That each `DpllToken` is a singleton
// guarantees each corresponding set of registers is written from only one
// location. This allows `DpllToken` to be `Sync`, even though the PAC `OSCCTRL`
// struct is not.
pub struct DpllToken<D: DpllId> {
    dpll: PhantomData<D>,
}

impl<D: DpllId> DpllToken<D> {
    /// Create a new instance of [`DpllToken`]
    ///
    /// # Safety
    ///
    /// Each `DpllToken`s is a singleton. There must never be two simulatenous
    /// instances with the same [`DpllId`].
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self { dpll: PhantomData }
    }

    /// Access the corresponding PAC `DPLL` struct
    #[inline]
    fn dpll(&self) -> &DPLL {
        // Safety: `OSCCTRL` is not `Sync`, because it has interior mutability.
        // However, each `DpllToken` represents only one of the two DPLLs, and
        // this function only ever returns a reference to the corresponding
        // `DPLL*` registers, so there is no risk of accessing the same register
        // from multiple execution contexts. Division of the PAC `OSCCTRL`
        // struct into individual `Token` types is what lets us make each
        // `DpllToken` `Sync`.
        unsafe { &(*crate::pac::OSCCTRL::ptr()).dpll[D::NUM] }
    }

    /// Access the corresponding DPLLCTRLA register
    #[inline]
    fn ctrla(&self) -> &DPLLCTRLA {
        &self.dpll().dpllctrla
    }

    /// Access the corresponding DPLLCTRLB register
    #[inline]
    fn ctrlb(&self) -> &DPLLCTRLB {
        &self.dpll().dpllctrlb
    }

    /// Access the corresponding DPLLRATIO register
    #[inline]
    fn ratio(&self) -> &DPLLRATIO {
        &self.dpll().dpllratio
    }

    /// Access the corresponding DPLLSYNCBUSY register for reading only
    #[inline]
    fn syncbusy(&self) -> dpllsyncbusy::R {
        self.dpll().dpllsyncbusy.read()
    }

    /// Access the corresponding DPLLSTATUS register for reading only
    #[inline]
    fn status(&self) -> dpllstatus::R {
        self.dpll().dpllstatus.read()
    }

    /// Set the DPLL clock source.
    #[inline]
    fn set_source_clock(&mut self, source: DynDpllSourceId) {
        self.ctrlb()
            .modify(|_, w| w.refclk().variant(source.into()));
    }

    /// Set the pre-division factor used when the [`Dpll`] is driven by an
    /// [`Xosc`]
    ///
    /// This pre-divider is ignored when driven by other clocks.
    ///
    /// This function will convert the actual pre-divider to the `DIV` register
    /// field.
    ///
    /// [`Xosc`]: super::xosc::Xosc
    #[inline]
    fn set_prediv(&mut self, prediv: u16) {
        // Safety: The value is masked to the correct bit width by the PAC.
        // An invalid value could produce an invalid clock frequency, but that
        // does not break memory safety.
        self.ctrlb()
            .modify(|_, w| unsafe { w.div().bits(prediv / 2 - 1) });
    }

    /// Set the loop division factor
    ///
    /// The inputs to this function are the natural integer and fractional
    /// parts of the division factor. This function will convert them to the
    /// `LDR` and `LDRFRAC` register fields.
    #[inline]
    fn set_loop_div(&mut self, int: u16, frac: u8) {
        // Safety: The values are masked to the correct bit width by the PAC.
        // Invalid values here could produce invalid clock frequencies, but that
        // does not break memory safety.
        self.ratio().write(|w| unsafe {
            w.ldr().bits(int - 1);
            w.ldrfrac().bits(frac)
        });
        while self.syncbusy().dpllratio().bit_is_set() {}
    }

    /// Ignore the DPLL lock
    #[inline]
    fn set_lock_bypass(&mut self, bypass: bool) {
        self.ctrlb().modify(|_, w| w.lbypass().bit(bypass));
    }

    /// Wake up fast
    ///
    /// Output the clock immediately, without waiting for lock.
    #[inline]
    fn set_wake_up_fast(&mut self, wuf: bool) {
        self.ctrlb().modify(|_, w| w.wuf().bit(wuf));
    }

    /// Set on-demand operation
    #[inline]
    fn set_on_demand(&mut self, on_demand: bool) {
        self.ctrla().modify(|_, w| w.ondemand().bit(on_demand));
    }

    /// Set run-in-standby mode
    #[inline]
    fn set_run_standby(&mut self, run_standby: bool) {
        self.ctrla().modify(|_, w| w.runstdby().bit(run_standby));
    }

    /// Enable the [`Dpll`]
    #[inline]
    fn enable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().set_bit());
        while self.syncbusy().enable().bit_is_set() {}
    }

    /// Disable the [`Dpll`]
    #[inline]
    fn disable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().clear_bit());
        while self.syncbusy().enable().bit_is_set() {}
    }

    /// Check the STATUS register to see if the clock is locked
    #[inline]
    fn is_locked(&self) -> bool {
        self.status().lock().bit()
    }

    /// Check the STATUS register to see if the clock is ready
    #[inline]
    fn is_ready(&self) -> bool {
        self.status().clkrdy().bit()
    }
}

//==============================================================================
// DynDpllId
//==============================================================================

/// Value-level enum identifying one of two possible [`Dpll`]s
///
/// The variants of this enum identify one of the two possible digital
/// phase-locked loops.
///
/// `DynDpllId` is the value-level equivalent of [`DpllId`].
pub enum DynDpllId {
    Dpll0,
    Dpll1,
}

//==============================================================================
// DpllId
//==============================================================================

/// Type-level enum identifying one of two possible [`Dpll`]s
///
/// The types implementing this trait, i.e. [`Dpll0Id`] and [`Dpll1Id`], are
/// type-level variants of `DpllId`, and they identify one of the two possible
/// digital phase-locked loops.
///
/// `DpllId` is the type-level equivalent of [`DynDpllId`]. See the
/// documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait DpllId: Sealed {
    /// Corresponding variant of [`DynDpllId`]
    const DYN: DynDpllId;
    /// Corresponding numeric index
    const NUM: usize;
}

/// Type-level variant of [`DpllId`] representing the identity of DPLL0
///
/// See the documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub enum Dpll0Id {}

impl Sealed for Dpll0Id {}

impl DpllId for Dpll0Id {
    const DYN: DynDpllId = DynDpllId::Dpll0;
    const NUM: usize = 0;
}

/// Type-level variant of [`DpllId`] representing the identity of DPLL1
///
/// See the documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub enum Dpll1Id {}

impl Sealed for Dpll1Id {}

impl DpllId for Dpll1Id {
    const DYN: DynDpllId = DynDpllId::Dpll1;
    const NUM: usize = 1;
}

//==============================================================================
// DynDpllSourceId
//==============================================================================

/// Value-level enum of possible clock sources for a [`Dpll`]
///
/// The variants of this enum identify one of four possible clock sources for
/// a given [`Dpll`].
///
/// `DynDpllSourceId` is the value-level equivalent of [`DpllSourceId`].
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DynDpllSourceId {
    /// The DPLL is driven by a [`Pclk`]
    Pclk,
    /// The DPLL is driven by [`Xosc0`](super::xosc::Xosc0)
    Xosc0,
    /// The DPLL is driven by [`Xosc1`](super::xosc::Xosc1)
    Xosc1,
    /// The DPLL is driven by [`Xosc32k`](super::xosc32k::Xosc32k)
    Xosc32k,
}

impl From<DynDpllSourceId> for REFCLK_A {
    fn from(source: DynDpllSourceId) -> Self {
        match source {
            DynDpllSourceId::Pclk => REFCLK_A::GCLK,
            DynDpllSourceId::Xosc0 => REFCLK_A::XOSC0,
            DynDpllSourceId::Xosc1 => REFCLK_A::XOSC1,
            DynDpllSourceId::Xosc32k => REFCLK_A::XOSC32,
        }
    }
}

//==============================================================================
// DpllSourceId
//==============================================================================

/// Type-level enum of possible clock [`Source`]s for a [`Dpll`]
///
/// The types implementing this trait are type-level variants of `DpllSourceId`,
/// and they identify one of four possible clock [`Source`]s for a given
/// [`Dpll`]. All implementers of this trait are `Id` types, which are described
/// in more detail in the [`clock` module documentation](super).
///
/// `DpllSourceId` is the type-level equivalent of [`DynDpllSourceId`]. See the
/// documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait DpllSourceId {
    /// Corresponding variant of [`DynDpllSourceId`]
    const DYN: DynDpllSourceId;
}

impl<G: GclkId> DpllSourceId for G {
    const DYN: DynDpllSourceId = DynDpllSourceId::Pclk;
}
impl DpllSourceId for Xosc0Id {
    const DYN: DynDpllSourceId = DynDpllSourceId::Xosc0;
}
impl DpllSourceId for Xosc1Id {
    const DYN: DynDpllSourceId = DynDpllSourceId::Xosc1;
}
impl DpllSourceId for Xosc32kId {
    const DYN: DynDpllSourceId = DynDpllSourceId::Xosc32k;
}

//==============================================================================
// Dpll
//==============================================================================

/// Digital phase-locked loop used to multiply clock frequencies
///
/// A DPLL is used to multiply clock frequencies, taking a lower-frequency input
/// clock and producing a higher-frequency output clock.
///
/// The type parameter `D` is a [`DpllId`] that determines which of the two
/// instances this `Dpll` represents ([`Dpll0`] or [`Dpll1`]). The type
/// parameter `I` represents the `Id` type for the clock [`Source`] driving this
/// `Dpll`. It must be one of the valid [`DpllSourceId`]s. See the
/// [`clock` module documentation](super) for more detail on
/// [`Id` types](super#id-types).
///
/// On its own, an instance of `Dpll` does not represent an enabled DPLL.
/// Instead, it must first be wrapped with [`Enabled`], which implements
/// compile-time safety of the clock tree.
///
/// Because the terminal call to [`enable`] consumes the `Dpll` and returns an
/// [`EnabledDpll`], the remaining API uses the builder pattern, where each
/// method takes and returns `self` by value, allowing them to be easily
/// chained.
///
/// See the [module-level documentation](self) for an example of creating,
/// configuring and using a `Dpll`.
///
/// [`enable`]: Dpll::enable
pub struct Dpll<D, I>
where
    D: DpllId,
    I: DpllSourceId,
{
    token: DpllToken<D>,
    src: PhantomData<I>,
    src_freq: Hertz,
    mult: u16,
    frac: u8,
    lock_bypass: bool,
    wake_up_fast: bool,
    on_demand: bool,
    run_standby: bool,
    prediv: u16,
}

/// Type alias for the corresponding [`Dpll`]
pub type Dpll0<M> = Dpll<Dpll0Id, M>;

/// Type alias for the corresponding [`Dpll`]
pub type Dpll1<M> = Dpll<Dpll1Id, M>;

/// An [`Enabled`] [`Dpll`]
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// clocks consuming this [`Dpll`] and restricts access to the underlying
/// [`Dpll`] to prevent misuse.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the [`Counter`] is assumed to be zero.
pub type EnabledDpll<D, I, N = U0> = Enabled<Dpll<D, I>, N>;

/// Type alias for the corresponding [`EnabledDpll`]
pub type EnabledDpll0<I, N = U0> = EnabledDpll<Dpll0Id, I, N>;

/// Type alias for the corresponding [`EnabledDpll`]
pub type EnabledDpll1<I, N = U0> = EnabledDpll<Dpll1Id, I, N>;

impl<D, G> Dpll<D, G>
where
    D: DpllId + PclkId,
    G: GclkId,
{
    /// Create a [`Dpll`] from a [`Pclk`]
    ///
    /// Creating a [`Dpll`] does not modify any of the hardware registers. It
    /// only creates a struct to track the DPLL configuration.
    ///
    /// The configuration data is stored until the user calls [`enable`]. At
    /// that point, all of the registers are written according to the
    /// initialization procedures specified in the datasheet, and an
    /// [`EnabledDpll`] is returned. The `Dpll` is not active or useful until
    /// that point.
    ///
    /// [`enable`]: Dpll::enable
    #[inline]
    pub fn from_pclk(token: DpllToken<D>, pclk: Pclk<D, G>) -> Self {
        // Drop the `Pclk` here. We can recreate it later
        Dpll::new(token, pclk.freq(), 1)
    }

    /// Consume the [`Dpll`], release the [`DpllToken`], and return the [`Pclk`]
    #[inline]
    pub fn free_pclk(self) -> (DpllToken<D>, Pclk<D, G>) {
        // Safety: We dropped the `Pclk` when creating this `Dpll`.
        // We can safely recreate it here.
        let token = unsafe { PclkToken::new() };
        let pclk = Pclk::new(token, self.src_freq);
        (self.token, pclk)
    }
}

impl<D, I> Dpll<D, I>
where
    D: DpllId,
    I: DpllSourceId,
{
    fn new(token: DpllToken<D>, src_freq: Hertz, prediv: u16) -> Self {
        Self {
            token,
            src: PhantomData,
            src_freq,
            mult: 1,
            frac: 0,
            lock_bypass: false,
            wake_up_fast: false,
            on_demand: true,
            run_standby: false,
            prediv,
        }
    }

    /// Create a [`Dpll`] from a clock [`Source`]
    ///
    /// Creating a [`Dpll`] does not modify any of the hardware registers. It
    /// only creates a struct to track the DPLL configuration and [`Increment`]s
    /// the [`Source`] [`Enabled`] [`Counter`].
    ///
    /// The configuration data is stored until the user calls [`enable`]. At
    /// that point, all of the registers are written according to the
    /// initialization procedures specified in the datasheet, and an
    /// [`EnabledDpll`] is returned. The `Dpll` is not active or useful until
    /// that point.
    ///
    /// Note that, when the [`Dpll`] is driven by an [`Xosc`], there is an extra
    /// clock divider between the `Xosc` output and the input to the actual
    /// phase-locked loop. This allows the [`Xosc`] frequency to be above the
    /// maximum DPLL input frequency of 3.2 MHz.
    ///
    /// The `Xosc` pre-divider can be set to any *even* value in the range
    /// `[2, 4096]`. It defaults to the minimum value of 2, but it can be
    /// changed with the [`Dpll::prediv`] method.
    ///
    /// [`Xosc`]: super::xosc::Xosc
    /// [`enable`]: Dpll::enable
    #[inline]
    pub fn from_source<S>(token: DpllToken<D>, source: S) -> (Self, S::Inc)
    where
        S: Source<Id = I> + Increment,
    {
        let prediv = match I::DYN {
            DynDpllSourceId::Xosc0 | DynDpllSourceId::Xosc1 => 2,
            _ => 1,
        };
        let dpll = Dpll::new(token, source.freq(), prediv);
        (dpll, source.inc())
    }

    /// Consume the [`Dpll`], release the [`DpllToken`], and [`Decrement`] the
    /// [`Source`] [`Enabled`] [`Counter`]
    #[inline]
    pub fn free_source<S>(self, source: S) -> (DpllToken<D>, S::Dec)
    where
        S: Source<Id = I> + Decrement,
    {
        (self.token, source.dec())
    }
}

impl<D: DpllId, X: XoscId + DpllSourceId> Dpll<D, X> {
    /// Set the [`Xosc`] pre-division factor
    ///
    /// The [`Xosc`] output frequency is divided down before it enters the
    /// actual phase-locked loop. This function will panic if the pre-division
    /// factor is not an *even* number in the range `[2, 4096]`.
    ///
    /// [`Xosc`]: super::xosc::Xosc
    #[inline]
    pub fn prediv(mut self, prediv: u16) -> Self {
        if prediv < 2 || prediv > 4096 || prediv % 2 != 0 {
            panic!("prediv must be an even number in the interval [2, 4096]");
        }
        self.prediv = prediv;
        self
    }
}

impl<D, I> Dpll<D, I>
where
    D: DpllId,
    I: DpllSourceId,
{
    /// Set the [`Dpll`] loop divider, which is also the frequency
    /// multiplication factor
    ///
    /// The inputs to this function are the natural integer and fractional
    /// parts of the division factor, i.e. the division factor is:
    ///
    /// ```text
    /// int + frac / 32
    /// ```
    ///
    /// This function will confirm that the `int` and `frac` values convert to
    /// valid `LDR` and `LDRFRAC` register fields, panicking otherwise.
    #[inline]
    pub fn loop_div(mut self, int: u16, frac: u8) -> Self {
        if int < 1 || int > 0x2000 {
            panic!("Invalid integer part of the DPLL loop divider")
        }
        if int > 31 {
            panic!("Invalid fractional part of the DPLL loop divider")
        }
        self.mult = int;
        self.frac = frac;
        self
    }

    /// Bypass the [`Dpll`] lock
    ///
    /// If `true`, the [`Dpll`] will output its clock regardless of whether it
    /// is locked.
    #[inline]
    pub fn lock_bypass(mut self, bypass: bool) -> Self {
        self.lock_bypass = bypass;
        self
    }

    /// Output the [`Dpll`] clock immediately, without waiting for various
    /// conditions
    ///
    /// See the datasheet for complete details.
    #[inline]
    pub fn wake_up_fast(mut self, wuf: bool) -> Self {
        self.wake_up_fast = wuf;
        self
    }

    /// Set on-demand mode
    ///
    /// See the datasheet for complete details.
    #[inline]
    pub fn on_demand(mut self, on_demand: bool) -> Self {
        self.on_demand = on_demand;
        self
    }

    /// Set run-in-standby mode
    ///
    /// See the datasheet for complete details.
    #[inline]
    pub fn run_standby(mut self, run_standby: bool) -> Self {
        self.run_standby = run_standby;
        self
    }

    #[inline]
    fn input_freq(&self) -> u32 {
        self.src_freq.0 / self.prediv as u32
    }

    #[inline]
    fn output_freq(&self) -> u32 {
        self.input_freq() * (self.mult as u32 + self.frac as u32 / 32)
    }

    /// Return the output frequency of the [`Dpll`]
    #[inline]
    pub fn freq(&self) -> Hertz {
        Hertz(self.output_freq())
    }

    /// Enable the [`Dpll`], so that it can be used as a clock [`Source`]
    ///
    /// As mentioned when creating a new `Dpll`, no hardware registers are
    /// actually modified until this call. Rather, the desired configuration is
    /// stored internally, and the [`Dpll`] is initialized and configured here
    /// according to the datasheet.
    ///
    /// The returned value is an [`EnabledDpll`] that can be used as a clock
    /// [`Source`] for other clocks.
    ///
    /// This function will also check that the input and output clock
    /// frequencies fall within the valid ranges specified in the datasheet.
    /// Specifically, the input frequency must be between 32 kHz and 3.2 MHz,
    /// while the output frequency must be between 96 MHz and 200 MHz. If either
    /// frequency is invalid, this call will panic.
    #[inline]
    pub fn enable(self) -> EnabledDpll<D, I> {
        let input_freq = self.input_freq();
        let output_freq = self.output_freq();
        if input_freq < 32_000 || input_freq > 3_200_000 {
            panic!("Invalid DPLL input frequency");
        }
        if output_freq < 96_000_000 || output_freq > 200_000_000 {
            panic!("Invalid DPLL output frequency");
        }
        // Safety: We just checked that the input and output clock frequencies
        // are within the valid ranges specified in the datasheet
        unsafe { self.enable_unchecked() }
    }

    /// Enable the [`Dpll`] without validating the input & output frequencies
    ///
    /// Safety: This is equivalent to calling [`Dpll::enable`] but without the
    /// checks on input and output frequencies. Using frequencies outside the
    /// ranges specified in the datasheet may not work and could cause clocking
    /// problems.
    #[inline]
    pub unsafe fn enable_unchecked(mut self) -> EnabledDpll<D, I> {
        self.token.set_source_clock(I::DYN);
        if I::DYN == DynDpllSourceId::Xosc0 || I::DYN == DynDpllSourceId::Xosc1 {
            self.token.set_prediv(self.prediv)
        }
        self.token.set_loop_div(self.mult, self.frac);
        self.token.set_lock_bypass(self.lock_bypass);
        self.token.set_wake_up_fast(self.wake_up_fast);
        self.token.set_on_demand(self.on_demand);
        self.token.set_run_standby(self.run_standby);
        self.token.enable();
        Enabled::new(self)
    }
}

impl<D, I> EnabledDpll<D, I>
where
    D: DpllId,
    I: DpllSourceId,
{
    /// Disable the [`Dpll`]
    ///
    /// This method is only implemented for `N = U0`, which means the clock can
    /// only be disabled when no other clocks consume this [`Dpll`].
    #[inline]
    pub fn disable(mut self) -> Dpll<D, I> {
        self.0.token.disable();
        self.0
    }
}

impl<D, I, N> EnabledDpll<D, I, N>
where
    D: DpllId,
    I: DpllSourceId,
    N: Counter,
{
    /// Test whether the [`Dpll`] is locked
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.0.token.is_locked()
    }

    /// Test whether the [`Dpll`] is ready
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.0.token.is_ready()
    }
}

//==============================================================================
// Source
//==============================================================================

impl<D, I, N> Source for EnabledDpll<D, I, N>
where
    D: DpllId,
    I: DpllSourceId,
    N: Counter,
{
    type Id = D;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
