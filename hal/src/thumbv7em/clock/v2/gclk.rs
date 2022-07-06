//! # GCLK - Generic Clock Controller
//!
//! ## Overview
//!
//! The generic clock controller is central to the clocking system in ATSAMD
//! chips. It provides 12 generic clock generators to modify and distribute
//! clocks to other peripherals. Within the clock tree, these clock generators
//! act as the branch clocks connecting internal or external root or branch
//! clocks to other branch or leaf clocks.
//!
//! Each clock generator takes an input clock, optionally divides it, and
//! produces an output clock. The input clock can be:
//!
//! - An external crystal oscillator ([`Xosc`])
//! - A direct clock input ([`GclkIn`])
//! - Generic clock generator #1 ([`Gclk1`])
//! - The ultra-low power 32 kHz oscillator ([`OscUlp32k`])
//! - The DFLL ([`Dfll`])
//! - A DPLL ([`Dpll`])
//!
//! The output clock can be:
//! - A direct clock output to a GPIO pin ([`GclkOut`])
//! - A peripheral channel clock ([`Pclk`])
//!
//! ## Example
//!
//! The configuration of a [`Gclk`] is best shown with an example. However, the
//! example assumes you are already familiar with the basics of the `clock`
//! module. See the [`clock` module documentation](super) for an overview.
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
//! └── GCLK0 (48 MHz)
//!     └── Master clock (48 MHz)
//!
//! GCLK_IN1 (24 MHz)
//! └── GCLK1 (12 MHz)
//!     ├── SERCOM0
//!     └── GCLK2 (3 MHz)
//!         ├── SERCOM1
//!         └── GCLK_OUT2
//!
//! OSCULP base clock
//! ```
//!
//! Let's start by using [`clock_system_at_reset`] to access the HAL clocking
//! structs. We will also need access to the [`gpio`] [`Pins`].
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         gclk::{Gclk, GclkDiv, Gclk1Div},
//!         gclkio::{GclkIn, GclkOut},
//!         pclk::Pclk,
//!     },
//!     gpio::Pins,
//!     pac::Peripherals,
//!     time::U32Ext,
//! };
//!
//! let mut pac = Peripherals::take().unwrap();
//! let (mut buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let pins = Pins::new(pac.PORT);
//! ```
//!
//! Next, we create an instance of [`GclkIn`] from the [`GclkInToken`], GPIO
//! [`Pin`] and frequency in [`Hertz`].
//!
//! ```ignore
//! let gclk_in1 = GclkIn::enable(tokens.gclk_io.gclk_in1, pins.pb15, 24.mhz());
//! ```
//!
//! We can then call [`Gclk::new`] with a [`GclkToken`] and our instance of
//! [`GclkIn`] to produce a [`Gclk`], in this case [`Gclk1`]. At this point,
//! notice that [`Gclk<G, I>`] takes two type parameters. `G` is a [`GclkId`]
//! type identifying which of the 12 generators this `Gclk` represents. `I` is
//! an `Id` type indicating the clock [`Source`] driving the `Gclk`, which must
//! be a valid [`GclkSourceId`]. [`Gclk1<I>`], is simply an alias for
//! `Gclk<Gclk1Id, I>`.
//!
//! ```ignore
//! let (gclk1, gclk_in1) = Gclk::new(tokens.gclks.gclk1, gclk_in1);
//! ```
//!
//! The call to `Gclk::new` also increments the dependent clock count for the
//! [`EnabledGclkIn`], which we have seen is critical for clock tree safety.
//!
//! While we have created a [`Gclk`], we have not yet enabled it. But before
//! doing so, we would like to set the divider to reduce the input frequency of
//! 48 MHz to a 24 MHz output. We call `Gclk::div`, which uses a builder API, so
//! that it can be chained with the call to `Gclk::enable`.
//!
//! ```ignore
//! let gclk1 = gclk1.div(Gclk1Div::Div(2)).enable();
//! ```
//!
//! Note that the divider value supplied to `Gclk::div` must be wrapped by the
//! [`Gclk1Div`] enum. This is for a few different reasons. First, [`Gclk1`]
//! accepts a wider range of divider values than the other [`Gclk`]s, which use
//! [`GclkDiv`] instead. Second, the actual divider value is controlled by two
//! register fields, and the set of valid values is best expressed as a Rust
//! enum. These two enums are connected by the [`GclkDivider`] trait.
//!
//! Once [`Gclk1`] is enabled, we can use it to enable the [`Pclk`] for
//! [`Sercom0`]. This follows the usual pattern. We provide a [`PclkToken`] and
//! the [`EnabledGclk1`]. In return, we get an enabled [`Pclk`] and the
//! [`EnabledGclk1`] counter is [`Increment`]ed.
//!
//! ```ignore
//! let (pclk_sercom0, gclk1) = Pclk::enable(tokens.pclks.sercom0, gclk1);
//! ```
//!
//! Next, we create an instance of [`Gclk2`], using [`Gclk1`] as its [`Source`].
//! However, keep in mind that this is only true for [`Gclk1`]. No other
//! [`Gclk`] can act as a [`Source`] for another [`Gclk`].
//!
//! ```ignore
//! let (gclk2, gclk1) = Gclk::new(tokens.gclks.gclk2, gclk1);
//! ```
//!
//! The pattern repeats now. We divide [`Gclk1`] by 4 to produce the [`Gclk2`]
//! output. Then we enable it to produce an [`EnabledGclk2`] and use it to yield
//! another [`Pclk`].
//!
//!
//! ```ignore
//! let gclk2 = gclk2.div(GclkDiv::Div(4)).enable();
//! let (pclk_sercom1, gclk2) = Pclk::enable(tokens.pclks.sercom1, gclk2);
//! ```
//!
//! Finally, we output [`Gclk2`] directly to a GPIO pin. We supply a
//! [`GclkOutToken`], the GPIO [`Pin`] and the [`EnabledGclk2`] to yield a
//! [`GclkOut`].
//!
//! ```ignore
//! let (gclk_out2, gclk2) = GclkOut::enable(tokens.gclk_io.gclk_out2, pins.pa16, gclk2);
//! ```
//!
//! The full example is provided below.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         gclk::{Gclk, GclkDiv, Gclk1Div},
//!         gclkio::{GclkIn, GclkOut},
//!         pclk::Pclk,
//!     },
//!     gpio::Pins,
//!     pac::Peripherals,
//!     time::U32Ext,
//! };
//!
//! let mut pac = Peripherals::take().unwrap();
//! let (mut buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let pins = Pins::new(pac.PORT);
//! let gclk_in1 = GclkIn::enable(tokens.gclk_io.gclk_in1, pins.pb15, 24.mhz());
//! let (gclk1, gclk_in1) = Gclk::new(tokens.gclks.gclk1, gclk_in1);
//! let gclk1 = gclk1.div(Gclk1Div::Div(2)).enable();
//! let (pclk_sercom0, gclk1) = Pclk::enable(tokens.pclks.sercom0, gclk1);
//! let (gclk2, gclk1) = Gclk::new(tokens.gclks.gclk2, gclk1);
//! let gclk2 = gclk2.div(GclkDiv::Div(4)).enable();
//! let (pclk_sercom1, gclk2) = Pclk::enable(tokens.pclks.sercom1, gclk2);
//! let (gclk_out2, gclk2) = GclkOut::enable(tokens.gclk_io.gclk_out2, pins.pa16, gclk2);
//! ```
//!
//! ## `Gclk0`
//!
//! [`Gclk0`] is significant and special relative to the other [`Gclk`]s. It is
//! the clock generator for the processor's master clock, so it can never be
//! disabled. Consequently, it has a special API not available to the other
//! [`Gclk`]s. While normal [`Gclk`]s can only change their clock [`Source`] or
//! divider while disabled, [`Gclk0`] can never be disabled, so we provide this
//! functionality on [`EnabledGclk0`] instead.
//!
//! We model the master clock's dependence on [`Gclk0`] by setting its dependent
//! clock count to [`U1`] in [`clock_system_at_reset`]. Consequently, because
//! there is no way to safely decrement the count, the [`EnabledGclk0`] can
//! never be [`disable`]d.
//!
//! Additionally, we provide functions to change the clock [`Source`], divider,
//! etc. on [`EnabledGclk0`], but we restrict them to the case where `N = U1`.
//! This prevents users from changing its [`Source`] or divider if any *other,
//! additional* clock depends on it (besides the master clock).
//!
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [`Xosc`]: super::xosc::Xosc
//! [`GclkIn`]: super::gclkio::GclkIn
//! [`OscUlp32k`]: super::osculp32k::OscUlp32k
//! [`Dfll`]: super::dfll::Dfll
//! [`Dpll`]: super::dpll::Dpll
//! [`GclkInToken`]: super::gclkio::GclkInToken
//! [`GclkIn`]: super::gclkio::GclkIn
//! [`EnabledGclkIn`]: super::gclkio::EnabledGclkIn
//! [`GclkOutToken`]: super::gclkio::GclkOutToken
//! [`GclkOut`]: super::gclkio::GclkOut
//! [`PclkToken`]: super::pclk::PclkToken
//! [`Pclk`]: super::pclk::Pclk
//! [`gpio`]: crate::gpio
//! [`Pins`]: crate::gpio::Pins
//! [`Pin`]: crate::gpio::Pin
//! [`Sercom0`]: crate::sercom::Sercom0
//! [`disable`]: EnabledGclk::disable

use core::cmp::max;
use core::marker::PhantomData;

use paste::paste;
use seq_macro::seq;
use typenum::{U0, U1};

use crate::pac;
use crate::pac::gclk::genctrl::DIVSEL_A;
use crate::pac::NVMCTRL;

use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::gclk::GENCTRL;
use crate::time::Hertz;
use crate::typelevel::{Counter, Decrement, Increment, PrivateIncrement, Sealed};

use super::dfll::DfllId;
use super::dpll::{Dpll0Id, Dpll1Id};
use super::gclkio::GclkInId;
use super::osculp32k::OscUlp32kId;
use super::xosc::{Xosc0Id, Xosc1Id};
use super::xosc32k::Xosc32kId;
use super::{Enabled, Source};

//==============================================================================
// GclkToken
//==============================================================================

/// Singleton token that can be exchanged for a [`Gclk`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// typically represent clocks that are disabled at power-on reset.
///
/// [`GclkToken`]s are no different. All [`Gclk`]s other than [`Gclk0`] are
/// disabled at power-on reset. To use a [`Gclk`], you must first exchange the
/// token for an actual clock with [`Gclk::new`].
///
/// [`GclkToken`] is generic over the [`GclkId`], where each corresponding token
/// represents one of the 12 respective [`Gclk`]s.
//
// # Internal notes
//
// `GclkToken` is generic over the `GclkId`, and each corresponding instance is
// a singleton. There should never be more than one instance of `GclkToken` with
// a given `GclkId`, because `GclkToken` relies on this fact for memory safety.
//
// Users see `GclkToken` as merely an opaque token. but internally, `GclkToken`
// is also used as a register interface. The tokens are zero-sized, so they can
// be carried by all clock types without introducing any memory bloat.
//
// As part of that register interface, each `GclkToken` can access its
// corresponding `GENCTRL` register. That each `GclkToken` is a singleton
// guarantees each corresponding register is written from only one location.
// This allows `GclkToken` to be `Sync`, even though the PAC `GCLK` struct is
// not.
pub struct GclkToken<G: GclkId> {
    gen: PhantomData<G>,
}

impl<G: GclkId> GclkToken<G> {
    /// Create a new instance of [`GclkToken`]
    ///
    /// # Safety
    ///
    /// Each `GclkToken`s is a singleton. There must never be two simulatenous
    /// instances with the same [`GclkId`].
    #[inline]
    pub(super) unsafe fn new() -> Self {
        GclkToken { gen: PhantomData }
    }

    /// SYNCBUSY register mask for the corresponding GCLK
    const MASK: u16 = 1 << G::NUM;

    /// Provide a reference to the corresponding [`GENCTRL`] register
    #[inline]
    fn genctrl(&self) -> &GENCTRL {
        // Safety: `GENCTRL` is not `Sync`, because it has interior mutability.
        // However, each `GclkToken` represents only one of the 12 GCLKs, and
        // this function only ever returns a reference to the corresponding
        // `GENCTRL`, so there is no risk of accessing the same register from
        // multiple execution contexts. Division of the PAC `GCLK` struct into
        // individual `GclkTokens`s based on `GclkId` is what lets us make
        // each `GclkToken` `Sync`.
        let gclk = unsafe { &*pac::GCLK::ptr() };
        &gclk.genctrl[G::NUM]
    }

    /// Block until synchronization has completed
    ///
    /// Reads or writes to synchronized fields must be accompanied by a check of
    /// the `SYNCBUSY` register. See the datasheet for more details.
    #[inline]
    fn wait_syncbusy(&self) {
        // Safety: We are only reading from the `SYNCBUSY` register, and we are
        // only observing the bit corresponding to this particular `GclkId`, so
        // there is no risk of memory corruption.
        let gclk = unsafe { &*pac::GCLK::ptr() };
        while gclk.syncbusy.read().genctrl().bits() & Self::MASK != 0 {}
    }

    /// Set the clock source for the [`Gclk`] generator
    #[inline]
    fn set_source(&mut self, source: DynGclkSourceId) {
        self.genctrl().modify(|_, w| w.src().variant(source.into()));
        self.wait_syncbusy();
    }

    /// Set the [`GclkDivider`] value
    ///
    /// Use the internal interface of [`GclkDivider`] to set the `DIV` and
    /// `DIVSEL` fields of the `GENCTRL` register.
    #[inline]
    fn set_div(&mut self, div: G::Divider) {
        let (divsel, div) = div.divsel_div();
        // Safety: The `DIVSEL` and `DIV` values are derived from the
        // `GclkDivider` type, so they are guaranteed to be valid.
        self.genctrl().modify(|_, w| unsafe {
            w.divsel().variant(divsel);
            w.div().bits(div)
        });
        self.wait_syncbusy();
    }

    /// Output a 50-50 duty-cycle clock when using an odd division factor
    #[inline]
    fn improve_duty_cycle(&mut self, flag: bool) {
        self.genctrl().modify(|_, w| w.idc().bit(flag));
        self.wait_syncbusy();
    }

    /// Set the state of [`GclkOut`] pins when the GCLK_IO output is disabled
    #[inline]
    fn output_off_value(&mut self, high: bool) {
        self.genctrl().modify(|_, w| w.oov().bit(high));
        self.wait_syncbusy();
    }

    /// Enable ouput of the [`Gclk`] on a GPIO [`Pin`]
    ///
    /// [`Pin`]: crate::gpio::Pin
    #[inline]
    fn enable_gclk_out(&mut self) {
        self.genctrl().modify(|_, w| w.oe().set_bit());
        self.wait_syncbusy();
    }

    /// Disable ouput of the [`Gclk`] on a GPIO [`Pin`]
    ///
    /// If a corresponding [`Pin`] is in the [`AlternateM`] mode, it's logic
    /// level will depend on the [`output_off_value`].
    ///
    /// [`Pin`]: crate::gpio::Pin
    /// [`AlternateM`]: crate::gpio::AlternateM
    #[inline]
    fn disable_gclk_out(&mut self) {
        self.genctrl().modify(|_, w| w.oe().clear_bit());
        self.wait_syncbusy();
    }

    /// Enable the [`Gclk`]
    #[inline]
    fn enable(&mut self) {
        self.genctrl().modify(|_, w| w.genen().set_bit());
        self.wait_syncbusy();
    }

    /// Disable the [`Gclk`]
    #[inline]
    fn disable(&mut self) {
        self.genctrl().modify(|_, w| w.genen().clear_bit());
        self.wait_syncbusy();
    }
}

//==============================================================================
// DynGclkId
//==============================================================================

/// Value-level enum identifying one of 12 possible [`Gclk`]s
///
/// The variants of this enum identify one of the 12 possible generic clock
/// generators.
///
/// `DynGclkId` is the value-level equivalent of [`GclkId`].
pub enum DynGclkId {
    Gclk0,
    Gclk1,
    Gclk2,
    Gclk3,
    Gclk4,
    Gclk5,
    Gclk6,
    Gclk7,
    Gclk8,
    Gclk9,
    Gclk10,
    Gclk11,
}

//==============================================================================
// GclkId
//==============================================================================

/// Type-level enum identifying one of 12 possible [`Gclk`]s
///
/// The types implementing this trait, i.e. [`Gclk0Id`] - [`Gclk11Id`], are
/// type-level variants of `GclkId`, and they identify one of the 12 possible
/// generic clock generators.
///
/// `GclkId` is the type-level equivalent of [`DynGclkId`]. See the
/// documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait GclkId: Sealed {
    /// Corresponding variant of [`DynGclkId`]
    const DYN: DynGclkId;
    /// Corresponding numeric index
    const NUM: usize;
    /// Corresponding [`GclkDivider`] type
    ///
    /// [`Gclk1`] uses [`Gclk1Div`], while all other [`Gclk`]s use [`GclkDiv`].
    type Divider: GclkDivider;
}

/// Type-level variant of [`GclkId`] representing the identity of GCLK0
///
/// See the documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum Gclk0Id {}
impl Sealed for Gclk0Id {}
impl GclkId for Gclk0Id {
    const DYN: DynGclkId = DynGclkId::Gclk0;
    const NUM: usize = 0;
    type Divider = GclkDiv;
}

/// Type-level variant of [`GclkId`] representing the identity of GCLK1
///
/// See the documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum Gclk1Id {}
impl Sealed for Gclk1Id {}
impl GclkId for Gclk1Id {
    const DYN: DynGclkId = DynGclkId::Gclk1;
    const NUM: usize = 1;
    type Divider = Gclk1Div;
}

seq!(N in 2..=11 {
    paste! {
        /// Type-level variant of [`GclkId`] representing the identity of
        #[doc = "GCLK" N]
        ///
        /// See the documentation on [type-level programming] and specifically
        /// [type-level enums] for more details.
        ///
        /// [type-level programming]: crate::typelevel
        /// [type-level enums]: crate::typelevel#type-level-enum
        pub enum [<Gclk N Id>] {}
        impl Sealed for [<Gclk N Id>] {}
        impl GclkId for [<Gclk N Id>] {
            const DYN: DynGclkId = DynGclkId::Gclk~N;
            const NUM: usize = N;
            type Divider = GclkDiv;
        }
    }
});

//==============================================================================
// GclkDivider
//==============================================================================

/// Trait unifying the two [`Gclk`] divider types, [`GclkDiv`] and [`Gclk1Div`]
///
/// Choosing a [`Gclk`] division factor can be complicated. [`Gclk1`] can accept
/// a 16-bit divider value, while all other [`Gclk`]s only take an 8-bit value.
/// Moreover, the set of valid clock dividers does not form a contiguous range.
/// For example, the valid set of dividers for most [`Gclk`]s is 1-256 and 512.
///
/// The [`Gclk1Div`] and [`GclkDiv`] enums provide simple and intuitive
/// user-facing interfaces to choose the *actual* clock divider value. This
/// trait, on the other hand, provides an internal-facing interface used by HAL
/// authors to extract the clock divider and convert it to the corresponding
/// `DIVSEL` and `DIV` register fields. Users should have no reason to interact
/// with this trait directly.
pub trait GclkDivider: Sealed + Default + Copy {
    /// Returns the actual clock divider value as a `u32`
    fn divider(&self) -> u32;
    /// Return the corresponding `DIVSEL` and and `DIV` register fields
    fn divsel_div(&self) -> (DIVSEL_A, u16);
}

//==============================================================================
// GclkDiv
//==============================================================================

/// Enum for the clock division factor of all [`Gclk`]s other than [`Gclk1`]
///
/// Choosing a [`Gclk`] division factor can be complicated, because the set of
/// valid values is not contiguous. For clocks other than [`Gclk1`], the
/// division factor can be 1-256 or 512. `GclkDiv` provides an enum interface to
/// enforce validity of the division factor. See the datasheet for more details.
#[derive(Clone, Copy)]
pub enum GclkDiv {
    /// Use a literal division factor
    ///
    /// All values in the range `[1-255]` are valid. Zero is also valid, but it
    /// is interpreted as `1`.
    Div(u8),
    /// Use a division factor of `2^8 = 256`
    Div2Pow8,
    /// Use a division factor of `2^9 = 512`
    Div2Pow9,
}

impl Sealed for GclkDiv {}

impl Default for GclkDiv {
    #[inline]
    fn default() -> Self {
        Self::Div(0)
    }
}

impl GclkDivider for GclkDiv {
    #[inline]
    fn divider(&self) -> u32 {
        match self {
            GclkDiv::Div(div) => (*div).into(),
            GclkDiv::Div2Pow8 => 256,
            GclkDiv::Div2Pow9 => 512,
        }
    }

    #[inline]
    fn divsel_div(&self) -> (DIVSEL_A, u16) {
        match self {
            GclkDiv::Div(div) => (DIVSEL_A::DIV1, (*div).into()),
            GclkDiv::Div2Pow8 => (DIVSEL_A::DIV2, 7),
            GclkDiv::Div2Pow9 => (DIVSEL_A::DIV2, 8),
        }
    }
}

//==============================================================================
// Gclk1Div
//==============================================================================

/// Enum for the clock division factor of [`Gclk1`] only
///
/// Choosing the [`Gclk1`] division factor can be complicated, because the set
/// of valid values is not contiguous. For [`Gclk1`], the division factor can be
/// 1-65536 or 131072. `Gclk1Div` provides an enum interface to enforce validity
/// of the division factor. See the datasheet for more details.
#[derive(Clone, Copy)]
pub enum Gclk1Div {
    /// Use a literal division factor
    ///
    /// All values in the range `[1-65535]` are valid. Zero is also valid, but
    /// it is interpreted as `1`.
    Div(u16),
    /// Use a division factor of `2^16 = 65536`
    Div2Pow16,
    /// Use a division factor of `2^17 = 131072`
    Div2Pow17,
}

impl Sealed for Gclk1Div {}

impl Default for Gclk1Div {
    #[inline]
    fn default() -> Self {
        Self::Div(0)
    }
}

impl GclkDivider for Gclk1Div {
    #[inline]
    fn divider(&self) -> u32 {
        match self {
            Gclk1Div::Div(div) => (*div).into(),
            Gclk1Div::Div2Pow16 => 65536,
            Gclk1Div::Div2Pow17 => 131072,
        }
    }

    #[inline]
    fn divsel_div(&self) -> (DIVSEL_A, u16) {
        match self {
            Gclk1Div::Div(div) => (DIVSEL_A::DIV1, *div),
            Gclk1Div::Div2Pow16 => (DIVSEL_A::DIV2, 15),
            Gclk1Div::Div2Pow17 => (DIVSEL_A::DIV2, 16),
        }
    }
}

//==============================================================================
// DynGclkSourceId
//==============================================================================

/// Value-level enum of possible clock sources for a [`Gclk`]
///
/// The variants of this enum identify one of nine possible clock sources for
/// a given [`Gclk`].
///
/// `DynGclkSourceId` is the value-level equivalent of [`GclkSourceId`].
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DynGclkSourceId {
    Dfll,
    Dpll0,
    Dpll1,
    Gclk1,
    GclkIn,
    OscUlp32k,
    Xosc0,
    Xosc1,
    Xosc32k,
}

impl From<DynGclkSourceId> for SRC_A {
    fn from(source: DynGclkSourceId) -> Self {
        use DynGclkSourceId::*;
        use SRC_A::*;
        match source {
            Dfll => DFLL,
            Dpll0 => DPLL0,
            Dpll1 => DPLL1,
            Gclk1 => GCLKGEN1,
            GclkIn => GCLKIN,
            OscUlp32k => OSCULP32K,
            Xosc0 => XOSC0,
            Xosc1 => XOSC1,
            Xosc32k => XOSC32K,
        }
    }
}

//==============================================================================
// GclkSourceId
//==============================================================================

/// Value-level enum of possible clock [`Source`]s for a [`Gclk`]
///
/// The types implementing this trait are type-level variants of `GclkSourceId`,
/// and they identify one of nine possible clock [`Source`]s for a given
/// [`Gclk`]. All implementers of this trait are `Id` types, which are described
/// in more detail in the [`clock` module documentation](super).
///
/// `GclkSourceId` is the type-level equivalent of [`DynGclkSourceId`]. See the
/// documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait GclkSourceId {
    /// Corresponding variant of [`DynGclkSourceId`]
    const DYN: DynGclkSourceId;
}

impl GclkSourceId for DfllId {
    const DYN: DynGclkSourceId = DynGclkSourceId::Dfll;
}
impl GclkSourceId for Dpll0Id {
    const DYN: DynGclkSourceId = DynGclkSourceId::Dpll0;
}
impl GclkSourceId for Dpll1Id {
    const DYN: DynGclkSourceId = DynGclkSourceId::Dpll1;
}
impl GclkSourceId for Gclk1Id {
    const DYN: DynGclkSourceId = DynGclkSourceId::Gclk1;
}
impl GclkSourceId for GclkInId {
    const DYN: DynGclkSourceId = DynGclkSourceId::GclkIn;
}
impl GclkSourceId for OscUlp32kId {
    const DYN: DynGclkSourceId = DynGclkSourceId::OscUlp32k;
}
impl GclkSourceId for Xosc0Id {
    const DYN: DynGclkSourceId = DynGclkSourceId::Xosc0;
}
impl GclkSourceId for Xosc1Id {
    const DYN: DynGclkSourceId = DynGclkSourceId::Xosc1;
}
impl GclkSourceId for Xosc32kId {
    const DYN: DynGclkSourceId = DynGclkSourceId::Xosc32k;
}

//==============================================================================
// Gclk
//==============================================================================

/// Generic clock generator used to distribute clocks to various peripherals
///
/// A generic clock generator acts as a branch in the clock tree. It can connect
/// a root or branch clock to other branch or leaf clocks. In particular, all
/// peripheral [`Pclk`]s must be derived from a `Gclk`.
///
/// The type parameter `G` is a [`GclkId`] that determines which of the 12
/// generators this [`Gclk`] represents ([`Gclk0`] - [`Gclk11`]). The type
/// parameter `I` represents the `Id` type for the clock [`Source`] driving this
/// `Gclk`. It must be one of the valid [`GclkSourceId`]s. See the
/// [`clock` module documentation](super) for more detail on `Id` types.
///
/// On its own, an instance of `Gclk` does not represent an enabled clock
/// generator. Instead, it must first be wrapped with [`Enabled`], which
/// implements compile-time safety of the clock tree.
///
/// Because the terminal call to [`enable`] consumes the `Gclk` and returns an
/// [`EnabledGclk`], the remaining API uses the builder pattern, where each
/// method takes and returns `self` by value, allowing them to be easily
/// chained.
///
/// See the [module-level documentation](self) for an example of creating,
/// configuring and using a `Gclk`.
///
/// [`Pclk`]: super::pclk::Pclk
/// [`enable`]: Gclk::enable
pub struct Gclk<G, I>
where
    G: GclkId,
    I: GclkSourceId,
{
    token: GclkToken<G>,
    src: PhantomData<I>,
    src_freq: Hertz,
    div: G::Divider,
    output_off_value: bool,
    improve_duty_cycle: bool,
}

/// An [`Enabled`] [`Gclk`]
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// downstream clocks dependent on this [`Gclk`] and restricts access to the
/// underlying [`Gclk`] to prevent misuse.
///
/// As with [`Enabled`], the default value for `N` is `U0`. Stated differently,
/// if left unspecified, the number of dependent clocks is assumed to be zero.
pub type EnabledGclk<G, I, N = U0> = Enabled<Gclk<G, I>, N>;

/// Type alias for the corresponding [`Gclk`]
///
/// As mentioned in the [module-level documentation](self), `Gclk0` is special,
/// because it provides the processor master clock. We represent this by
/// permanently [`Increment`]ing the depdent clock count of [`EnabledGclk0`],
/// which prevents it from ever being disabled. Accordingly, we also provide a
/// few special methods on [`EnabledGclk0`] to configure the `Gclk` while it is
/// actively running.
pub type Gclk0<I> = Gclk<Gclk0Id, I>;

/// Type aliase for the corresponding [`EnabledGclk`]
///
/// As mentioned in the [module-level documentation](self), `Gclk0` is special,
/// because it provides the processor master clock. We represent this by
/// permanently [`Increment`]ing the depdent clock count of [`EnabledGclk0`],
/// which prevents it from ever being disabled. Accordingly, we also provide a
/// few special methods on [`EnabledGclk0`] to configure the `Gclk` while it is
/// actively running.
pub type EnabledGclk0<I, N = U0> = EnabledGclk<Gclk0Id, I, N>;

seq!(G in 1..=11 {
    paste! {
        /// Type alias for the corresponding [`Gclk`]
        pub type Gclk~G<I> = Gclk<[<Gclk G Id>], I>;

        /// Type aliase for the corresponding [`EnabledGclk`]
        pub type EnabledGclk~G<I, N = U0> = EnabledGclk<[<Gclk G Id>], I, N>;
    }
});

impl<G, I> Gclk<G, I>
where
    G: GclkId,
    I: GclkSourceId,
{
    /// Create a new [`Gclk`]
    ///
    /// Creating a [`Gclk`] does not modify any of the hardware registers. It
    /// only serves to [`Increment`] the [`Source`]'s dependent clock count and
    /// create a struct to track the GCLK configuration.
    ///
    /// The configuration data is stored until the user calls [`enable`]. At
    /// that point, all of the registers are written according to the
    /// initialization procedures specified in the datasheet, and an
    /// [`EnabledGclk`] is returned. The `Gclk` is not active or useful until
    /// that point.
    ///
    /// [`enable`]: Gclk::enable
    #[inline]
    pub fn new<S>(token: GclkToken<G>, source: S) -> (Gclk<G, I>, S::Inc)
    where
        S: Source<Id = I> + Increment,
    {
        let config = Gclk {
            token,
            src: PhantomData,
            src_freq: source.freq(),
            div: G::Divider::default(),
            output_off_value: false,
            improve_duty_cycle: false,
        };
        (config, source.inc())
    }

    /// Consume the [`Gclk`] and free its corresponding resources
    ///
    /// Freeing a [`Gclk`] returns the corresponding [`GclkToken`] and
    /// [`Decrement`]s the [`Source`]'s depdendent clock count.
    #[inline]
    pub fn free<S>(self, source: S) -> (GclkToken<G>, S::Dec)
    where
        S: Source<Id = I> + Decrement,
    {
        (self.token, source.dec())
    }

    /// Swap the [`Gclk`]'s [`Source`]
    ///
    /// A clock [`Source`] is required when creating a [`Gclk`] with [`new`],
    /// which [`Increment`]s the `Source`'s depdendent clock count. Changing
    /// the `Source` would normally require [`free`]ing the [`GclkToken`] and
    /// old `Source` before creating a [`new`] `Gclk` with a new `Source`.
    ///
    /// Alternatively, the calls to [`free`] and [`new`] can be combined with
    /// `swap`. The depdendent clock count for the `Old` `Source` will be
    /// [`Decrement`]ed, while the `New` `Source` will be [`Increment`]ed.
    ///
    /// [`new`]: Gclk::new
    /// [`free`]: Gclk::free
    #[inline]
    pub fn swap<Old, New>(self, old: Old, new: New) -> (Gclk<G, New::Id>, Old::Dec, New::Inc)
    where
        Old: Source<Id = I> + Decrement,
        New: Source + Increment,
        New::Id: GclkSourceId,
    {
        let config = Gclk {
            token: self.token,
            src: PhantomData,
            src_freq: new.freq(),
            div: self.div,
            output_off_value: self.output_off_value,
            improve_duty_cycle: self.improve_duty_cycle,
        };
        let old = old.dec();
        let new = new.inc();
        (config, old, new)
    }

    /// Set the state of [`GclkOut`] pins when GCLK_IO output is disabled
    ///
    /// A [`Gclk`] can be output to a [`gpio`] [`Pin`] by creating an instance
    /// of [`GclkOut`]. Each `GclkOut` will consume a corresponding `Pin` and
    /// convert it to the correct peripheral function mode ([`AlternateM`]).
    ///
    /// This function will set the output state of any `Pin` configured in
    /// `AlternateM` mode (either manually or as part of `GclkOut`) when the
    /// `Gclk` is *disabled*. To enable the `Gclk` output, users must create a
    /// [`GclkOut`] using an [`EnabledGclk`].
    ///
    /// See the [`GclkOut`] documentation for more details.
    ///
    /// [`GclkOut`]: super::gclkio::GclkOut
    /// [`gpio`]: crate::gpio
    /// [`Pin`]: crate::gpio::Pin
    /// [`AlternateM`]: crate::gpio::AlternateM
    #[inline]
    pub fn output_off_value(mut self, high: bool) -> Self {
        self.output_off_value = high;
        self
    }

    /// Output a 50-50 duty cycle clock when using an odd [`GclkDivider`]
    #[inline]
    pub fn improve_duty_cycle(mut self, flag: bool) -> Self {
        self.improve_duty_cycle = flag;
        self
    }

    /// Return the [`Gclk`] ouput frequency
    ///
    /// The output frequency is the [`Source`] frequency divided by the
    /// [`GclkDivider`]. The divider supplied to [`Gclk::div`] is either a
    /// [`GclkDiv`] or [`Gclk1Div`] enum representing all of the valid divider
    /// options for the given [`Gclk`].
    #[inline]
    pub fn freq(&self) -> Hertz {
        let div = max(1, self.div.divider());
        Hertz(self.src_freq.0 / div)
    }

    /// Set the [`GclkDivider`] value
    ///
    /// Set the clock division factor from [`Source`] to [`Gclk`] output. This
    /// takes either a [`GclkDiv`] or [`Gclk1Div`] enum, restricting the
    /// possible division factors to only the valid ones for the given [`Gclk`].
    /// See the [`GclkDivider`] trait for more details.
    #[inline]
    pub fn div(mut self, div: G::Divider) -> Self {
        self.div = div;
        self
    }

    /// Enable the [`Gclk`], so that it can be used as a clock [`Source`]
    ///
    /// As mentioned when creating a [`new`] `Gclk`, no hardware registers are
    /// actually modified until this call. Rather, the desired configuration is
    /// stored internally, and the [`Gclk`] is initialized and configured here
    /// according to the datasheet.
    ///
    /// The returned value is an [`EnabledGclk`] that can be used as a clock
    /// [`Source`] for other clocks.
    ///
    /// [`new`]: Gclk::new
    #[inline]
    pub fn enable(mut self) -> EnabledGclk<G, I> {
        self.token.set_source(I::DYN);
        self.token.output_off_value(self.output_off_value);
        self.token.improve_duty_cycle(self.improve_duty_cycle);
        self.token.set_div(self.div);
        self.token.enable();
        Enabled::new(self)
    }
}

impl<G, I, N> EnabledGclk<G, I, N>
where
    G: GclkId,
    I: GclkSourceId,
    N: Counter,
{
    /// Enable the [`Gclk`] output to GPIO pins
    #[inline]
    pub(super) fn enable_gclk_out(&mut self) {
        self.0.token.enable_gclk_out();
    }

    /// Disable the [`Gclk`] output to GPIO pins
    #[inline]
    pub(super) fn disable_gclk_out(&mut self) {
        self.0.token.disable_gclk_out();
    }
}

impl<G, I> EnabledGclk<G, I>
where
    G: GclkId,
    I: GclkSourceId,
{
    /// Disable the [`Gclk`]
    ///
    /// This method is only implemented for `N = U0`, which means the clock can
    /// only be disabled when no other clocks depend on this [`Gclk`].
    #[inline]
    pub fn disable(mut self) -> Gclk<G, I> {
        self.0.token.disable();
        self.0
    }
}

/// Special methods for an [`Enabled`] [`Gclk0`]
///
/// [`Gclk0`] is special, because it drives the processor's master clock, which
/// can never be disabled. As discussed in the [module-level documentation],
/// this fact is represented by permanently [`Increment`]ing the dependent clock
/// count for [`EnabledGclk0`]. Thus, the minimum value for `N` is `U1` and
/// [`Gclk0`] can never be [`disable`]d.
///
/// These methods represent actions that can be taken when `N = U1`, i.e. the
/// depdendent clock count is at its minimum value. This is the only time it's
/// safe to [`swap`] the [`Gclk0`] [`Source`] or change its [`GclkDivider`]
/// value.
///
/// [module-level documentation]: self
/// [`disable`]: EnabledGclk::disable
/// [`swap`]: Gclk::swap
impl<I: GclkSourceId> EnabledGclk0<I, U1> {
    /// Swap the clock [`Source`] for [`Gclk0`]
    ///
    /// See [`Gclk::swap`] documentation for more details.
    #[inline]
    pub fn swap<Old, New>(
        self,
        old: Old,
        new: New,
    ) -> (EnabledGclk0<New::Id, U1>, Old::Dec, New::Inc)
    where
        Old: Source<Id = I> + Decrement,
        New: Source + Increment,
        New::Id: GclkSourceId,
    {
        let (config, old, new) = self.0.swap(old, new);
        (config.enable().inc(), old, new)
    }

    /// Set the [`GclkDivider`] value for [`Gclk0`]
    ///
    /// See [`Gclk::div`] documentation for more details.
    #[inline]
    pub fn div(self, div: GclkDiv) -> Self {
        let mut config = self.0.div(div);
        config.token.set_div(div);
        Enabled::new(config)
    }

    /// Output a 50-50 duty cycle clock when using an odd [`GclkDivider`]
    #[inline]
    pub fn improve_duty_cycle(self, flag: bool) -> Self {
        let mut config = self.0.improve_duty_cycle(flag);
        config.token.improve_duty_cycle(flag);
        Enabled::new(config)
    }
}

//==============================================================================
// Source
//==============================================================================

impl<G, I, N> Source for EnabledGclk<G, I, N>
where
    G: GclkId,
    I: GclkSourceId,
    N: Counter,
{
    type Id = G;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}

//==============================================================================
// Tokens
//==============================================================================

seq!(N in 1..=11 {
    paste! {
        /// Set of [`GclkToken`]s representing the disabled [`Gclk`]s at
        /// power-on reset
        pub struct Tokens {
            #(
                /// [`GclkToken`] for
                #[doc = "[`Gclk" N "`]"]
                pub gclk~N: GclkToken<[<Gclk N Id>]>,
            )*
        }

        impl Tokens {
            /// Create the set of [`GclkToken`]s
            ///
            /// Safety: All of the invariants required by `GclkToken::new` must
            /// be upheld here as well
            #[inline]
            pub(super) unsafe fn new(nvmctrl: &mut NVMCTRL) -> Self {
                // Use auto wait states
                nvmctrl.ctrla.modify(|_, w| w.autows().set_bit());
                Tokens {
                    #( gclk~N: GclkToken::new(), )*
                }
            }
        }
    }
});
