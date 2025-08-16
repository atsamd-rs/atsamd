//! # Generic Clock Controllers
//!
//! ## Overview
//!
//! The generic clock controller is central to the clocking system in ATSAMD
//! chips. It provides 12 generic clock generators to modify and distribute
//! clocks to other peripherals. Within the clock tree, these clock generators
//! act as the branch clocks, connecting internal or external root or branch
//! clocks to other branch or leaf clocks.
//!
//! Each clock generator takes an input clock, optionally divides it, and
//! produces an output clock. The input clock can be:
//!
//! - A GPIO input ([`Pin`])
//! - An external crystal oscillator ([`Xosc`])
//! - An external 32 kHz oscillator ([`Xosc32k`])
//! - The ultra-low power 32 kHz oscillator ([`OscUlp32k`])
//! - The 48 MHz DFLL ([`Dfll`])
//! - A DPLL ([`Dpll`])
//! - Generic clock generator #1 ([`Gclk1`])
//!
//! The output clock can be:
//! - A peripheral channel clock ([`Pclk`])
//! - A GPIO pin ([`GclkOut`])
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
//!     └── Main clock (48 MHz)
//! ```
//!
//! We would like to transform it to a clock tree like this:
//!
//! ```text
//! DFLL (48 MHz)
//! └── GCLK0 (48 MHz)
//!     └── Main clock (48 MHz)
//!
//! GCLK_IN1 (PB14, 24 MHz)
//! └── GCLK1 (12 MHz)
//!     ├── SERCOM0
//!     └── GCLK2 (3 MHz)
//!         ├── SERCOM1
//!         └── GCLK_OUT2 (PA16, 3 MHz)
//! ```
//!
//! Let's start by using [`clock_system_at_reset`] to access the HAL clocking
//! structs. We will also need access to the [`gpio`] [`Pins`].
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         gclk::{Gclk, GclkDiv8, GclkDiv16},
//!         pclk::Pclk,
//!     },
//!     gpio::Pins,
//!     pac::Peripherals,
//!     fugit::RateExtU32,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (mut buses, clocks, tokens) = clock_system_at_reset(
//!     pac.oscctrl,
//!     pac.osc32kctrl,
//!     pac.gclk,
//!     pac.mclk,
//!     &mut pac.nvmctrl,
//! );
//! let pins = Pins::new(pac.port);
//! ```
//!
//! Next, we use [`Gclk::from_pin`] to create a [`Gclk`] from a [`GclkToken`],
//! GPIO [`Pin`] and frequency, in [`Hertz`]. In this case, we create an
//! instance of [`Gclk1`].
//!
//! At this point, notice that [`Gclk<G, I>`] takes two type parameters. `G` is
//! a [`GclkId`] identifying which of the 12 generators this `Gclk` represents.
//! [`Gclk1<I>`] is simply an alias for `Gclk<Gclk1Id, I>`. `I` is an [`Id`
//! type](super#id-types) identifying the input clock, which must be a valid
//! [`GclkSourceId`]. In this case, `I` is `PB14`, which is a `GclkSourceId` for
//! `Gclk1` on this target, because it implements [`GclkIo`] with
//! [`GclkIo::GclkId`]` = Gclk1Id`.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         gclk::{Gclk, GclkDiv8, GclkDiv16},
//! #         pclk::Pclk,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     fugit::RateExtU32,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.oscctrl,
//! #     pac.osc32kctrl,
//! #     pac.gclk,
//! #     pac.mclk,
//! #     &mut pac.nvmctrl,
//! # );
//! # let pins = Pins::new(pac.port);
//! let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.MHz());
//! ```
//!
//! While we have created a [`Gclk`], we have not yet enabled it. But before
//! doing so, we would like to set the divider to reduce the input frequency of
//! 24 MHz to a 12 MHz output. We call `Gclk::div`, which uses a builder API, so
//! that it can be chained with the call to `Gclk::enable`.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         gclk::{Gclk, GclkDiv8, GclkDiv16},
//! #         pclk::Pclk,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     fugit::RateExtU32,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.oscctrl,
//! #     pac.osc32kctrl,
//! #     pac.gclk,
//! #     pac.mclk,
//! #     &mut pac.nvmctrl,
//! # );
//! # let pins = Pins::new(pac.port);
//! # let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.MHz());
//! let gclk1 = gclk1.div(GclkDiv16::Div(2)).enable();
//! ```
//!
//! Note that the divider value supplied to `Gclk::div` must be wrapped by the
//! [`GclkDiv16`] enum. This is for a few different reasons. First, [`Gclk1`]
//! accepts a wider range of divider values than the other [`Gclk`]s, which use
//! [`GclkDiv8`] instead. Second, the actual divider value is controlled by two
//! register fields, and the set of valid values is best expressed as a Rust
//! enum. The `GclkDiv8` and `GclkDiv16` enums are connected by the
//! [`GclkDivider`] trait.
//!
//! Once [`Gclk1`] is enabled, we can use it to enable the [`Pclk`] for
//! [`Sercom0`]. This follows the usual pattern. We provide a [`PclkToken`] and
//! the [`EnabledGclk1`]. In return, we get an enabled [`Pclk`] and the
//! [`EnabledGclk1`] counter is [`Increment`]ed.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         gclk::{Gclk, GclkDiv8, GclkDiv16},
//! #         pclk::Pclk,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     fugit::RateExtU32,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.oscctrl,
//! #     pac.osc32kctrl,
//! #     pac.gclk,
//! #     pac.mclk,
//! #     &mut pac.nvmctrl,
//! # );
//! # let pins = Pins::new(pac.port);
//! # let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.MHz());
//! # let gclk1 = gclk1.div(GclkDiv16::Div(2)).enable();
//! let (pclk_sercom0, gclk1) = Pclk::enable(tokens.pclks.sercom0, gclk1);
//! ```
//!
//! Next, we use [`Gclk1`] as a clock [`Source`] to create an instance of
//! [`Gclk2`] with [`Gclk::from_source`]. However, keep in mind that this is
//! only true for [`Gclk1`]. No other [`Gclk`] can act as a [`Source`] for
//! another [`Gclk`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         gclk::{Gclk, GclkDiv8, GclkDiv16},
//! #         pclk::Pclk,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     fugit::RateExtU32,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.oscctrl,
//! #     pac.osc32kctrl,
//! #     pac.gclk,
//! #     pac.mclk,
//! #     &mut pac.nvmctrl,
//! # );
//! # let pins = Pins::new(pac.port);
//! # let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.MHz());
//! # let gclk1 = gclk1.div(GclkDiv16::Div(2)).enable();
//! # let (pclk_sercom0, gclk1) = Pclk::enable(tokens.pclks.sercom0, gclk1);
//! let (gclk2, gclk1) = Gclk::from_source(tokens.gclks.gclk2, gclk1);
//! ```
//!
//! The pattern repeats now. We divide [`Gclk1`] by 4 to produce the [`Gclk2`]
//! output. Then we enable it to produce an [`EnabledGclk2`] and use it to yield
//! another [`Pclk`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         gclk::{Gclk, GclkDiv8, GclkDiv16},
//! #         pclk::Pclk,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     fugit::RateExtU32,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.oscctrl,
//! #     pac.osc32kctrl,
//! #     pac.gclk,
//! #     pac.mclk,
//! #     &mut pac.nvmctrl,
//! # );
//! # let pins = Pins::new(pac.port);
//! # let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.MHz());
//! # let gclk1 = gclk1.div(GclkDiv16::Div(2)).enable();
//! # let (pclk_sercom0, gclk1) = Pclk::enable(tokens.pclks.sercom0, gclk1);
//! # let (gclk2, gclk1) = Gclk::from_source(tokens.gclks.gclk2, gclk1);
//! let gclk2 = gclk2.div(GclkDiv8::Div(4)).enable();
//! let (pclk_sercom1, gclk2) = Pclk::enable(tokens.pclks.sercom1, gclk2);
//! ```
//!
//! Finally, we output [`Gclk2`] directly to a GPIO pin. We supply the GPIO
//! [`Pin`] to the [`EnabledGclk2`] to yield a [`GclkOut`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         gclk::{Gclk, GclkDiv8, GclkDiv16},
//! #         pclk::Pclk,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     fugit::RateExtU32,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.oscctrl,
//! #     pac.osc32kctrl,
//! #     pac.gclk,
//! #     pac.mclk,
//! #     &mut pac.nvmctrl,
//! # );
//! # let pins = Pins::new(pac.port);
//! # let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.MHz());
//! # let gclk1 = gclk1.div(GclkDiv16::Div(2)).enable();
//! # let (pclk_sercom0, gclk1) = Pclk::enable(tokens.pclks.sercom0, gclk1);
//! # let (gclk2, gclk1) = Gclk::from_source(tokens.gclks.gclk2, gclk1);
//! # let gclk2 = gclk2.div(GclkDiv8::Div(4)).enable();
//! # let (pclk_sercom1, gclk2) = Pclk::enable(tokens.pclks.sercom1, gclk2);
//! let (gclk2, gclk2_out) = gclk2.enable_gclk_out(pins.pa16);
//! ```
//!
//! The full example is provided below.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         gclk::{Gclk, GclkDiv8, GclkDiv16},
//!         pclk::Pclk,
//!     },
//!     gpio::Pins,
//!     pac::Peripherals,
//!     fugit::RateExtU32,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (mut buses, clocks, tokens) = clock_system_at_reset(
//!     pac.oscctrl,
//!     pac.osc32kctrl,
//!     pac.gclk,
//!     pac.mclk,
//!     &mut pac.nvmctrl,
//! );
//! let pins = Pins::new(pac.port);
//! let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.MHz());
//! let gclk1 = gclk1.div(GclkDiv16::Div(2)).enable();
//! let (pclk_sercom0, gclk1) = Pclk::enable(tokens.pclks.sercom0, gclk1);
//! let (gclk2, gclk1) = Gclk::from_source(tokens.gclks.gclk2, gclk1);
//! let gclk2 = gclk2.div(GclkDiv8::Div(4)).enable();
//! let (pclk_sercom1, gclk2) = Pclk::enable(tokens.pclks.sercom1, gclk2);
//! let (gclk2, gclk2_out) = gclk2.enable_gclk_out(pins.pa16);
//! ```
//!
//! ## `Gclk0`
//!
//! [`Gclk0`] is significant and special relative to the other [`Gclk`]s. It is
//! the clock generator for the processor's main clock, so it can never be
//! disabled. Consequently, it has a special API not available to the other
//! `Gclk`s. While normal `Gclk`s can only change their clock [`Source`] or
//! divider while disabled, `Gclk0` can never be disabled, so we provide this
//! functionality on [`EnabledGclk0`] instead.
//!
//! We model the main clock's consumption of `Gclk0` by setting its [`Enabled`]
//! counter to [`U1`] in [`clock_system_at_reset`]. This prevents users from
//! ever disabling `EnabledGclk0`, because there is no way to [`Decrement`] its
//! `Counter` to [`U0`].
//!
//! Additionally, we provide functions to change the clock `Source`, divider,
//! etc. on `EnabledGclk0`, but we restrict them to the case where `N = U1`.
//! This prevents users from changing its `Source` or divider if any *other,
//! additional* clock consumes it (besides the main clock).
//!
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [`Xosc`]: super::xosc::Xosc
//! [`Xosc32k`]: super::xosc32k::Xosc32k
//! [`OscUlp32k`]: super::osculp32k::OscUlp32k
//! [`Dfll`]: super::dfll::Dfll
//! [`Dpll`]: super::dpll::Dpll
//! [`PclkToken`]: super::pclk::PclkToken
//! [`Pclk`]: super::pclk::Pclk
//! [`Pins`]: crate::gpio::Pins
//! [`Sercom0`]: crate::sercom::Sercom0

use atsamd_hal_macros::{hal_cfg, hal_macro_helper};
use core::cmp::max;
use core::marker::PhantomData;

use paste::paste;
use typenum::{U0, U1};

use crate::pac;

use crate::gpio::{self, AlternateH, AnyPin, Pin, PinId};
use crate::pac::gclk::Genctrl;
#[hal_cfg(any("clock-d11", "clock-d21"))]
use crate::pac::gclk::Gendiv;
use crate::pac::gclk::genctrl::Srcselect;
use crate::time::Hertz;
use crate::typelevel::{Decrement, Increment, PrivateDecrement, PrivateIncrement, Sealed};

use super::dfll::DfllId;
use super::dpll::Dpll0Id;
#[hal_cfg("oscctrl")]
use super::dpll::Dpll1Id;
#[hal_cfg(any("clock-d11", "clock-d21"))]
use super::osc::OscId;
use super::osculp32k::OscUlp32kId;
use super::xosc::Xosc0Id;
#[hal_cfg("xosc1")]
use super::xosc::Xosc1Id;
#[hal_cfg("xosc32k")]
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
/// [`GclkToken`]s are no different.  [`Gclk`]s other than [`Gclk0`], and
/// [`Gclk2`] on SAMD21/SAMD11, are disabled at power-on reset. To use a
/// [`Gclk`], you must first exchange the token for an actual clock with
/// [`Gclk::from_source`] or [`Gclk::from_pin`].
///
/// [`GclkToken`] is generic over the [`GclkId`], where each corresponding token
/// represents one of the [`Gclk`]s.
pub struct GclkToken<G: GclkId> {
    generator: PhantomData<G>,
}

impl<G: GclkId> GclkToken<G> {
    /// Create a new instance of [`GclkToken`]
    ///
    /// # Safety
    ///
    /// Each `GclkToken`s is a singleton. There must never be two simulatenous
    /// instances with the same [`GclkId`]. See the notes on `Token` types and
    /// memory safety in the root of the `clock` module for more details.
    #[inline]
    pub(super) unsafe fn new() -> Self {
        GclkToken {
            generator: PhantomData,
        }
    }

    /// SYNCBUSY register mask for the corresponding GCLK
    #[hal_cfg("clock-d5x")]
    const MASK: u16 = 1 << G::NUM;

    /// Provide a reference to the corresponding [`Genctrl`] register
    #[inline]
    #[hal_macro_helper]
    fn genctrl(&self) -> &Genctrl {
        // Safety: Each `GclkToken` only has access to a mutually exclusive set
        // of registers for the corresponding `GclkId`, and we use a shared
        // reference to the register block. See the notes on `Token` types and
        // memory safety in the root of the `clock` module for more details.
        #[hal_cfg("clock-d5x")]
        unsafe {
            (*pac::Gclk::PTR).genctrl(G::NUM)
        }
        #[hal_cfg(any("clock-d11", "clock-d21"))]
        unsafe {
            (*pac::Gclk::PTR).genctrl()
        }
    }

    #[hal_cfg(any("clock-d11", "clock-d21"))]
    #[inline]
    fn gendiv(&self) -> &Gendiv {
        unsafe { (*pac::Gclk::PTR).gendiv() }
    }

    /// Block until synchronization has completed
    ///
    /// Reads or writes to synchronized fields must be accompanied by a check of
    /// the `SYNCBUSY` register. See the datasheet for more details.
    #[inline]
    #[hal_macro_helper]
    fn wait_syncbusy(&self) {
        // Safety: We are only reading from the `SYNCBUSY` register, and we are
        // only observing the bit corresponding to this particular `GclkId`, so
        // there is no risk of memory corruption.
        #[hal_cfg("clock-d5x")]
        {
            let syncbusy = unsafe { &(*pac::Gclk::PTR).syncbusy() };
            while syncbusy.read().genctrl().bits() & Self::MASK != 0 {}
        }
        #[hal_cfg(any("clock-d11", "clock-d21"))]
        {
            let status = unsafe { &(*pac::Gclk::PTR).status() };
            while status.read().syncbusy().bit() {}
        }
    }
}

//==============================================================================
// DynGclkId
//==============================================================================

/// Value-level enum identifying one of the possible [`Gclk`]s
///
/// The variants of this enum identify one generic clock generator.
///
/// `DynGclkId` is the value-level equivalent of [`GclkId`].
#[hal_macro_helper]
pub enum DynGclkId {
    Gclk0,
    Gclk1,
    Gclk2,
    Gclk3,
    Gclk4,
    Gclk5,
    #[hal_cfg("gclk6")]
    Gclk6,
    #[hal_cfg("gclk7")]
    Gclk7,
    #[hal_cfg("gclk8")]
    Gclk8,
    #[hal_cfg("gclk9")]
    Gclk9,
    #[hal_cfg("gclk10")]
    Gclk10,
    #[hal_cfg("gclk11")]
    Gclk11,
}

//==============================================================================
// GclkId
//==============================================================================

/// Type-level enum identifying one of possible [`Gclk`]s
///
/// The types implementing this trait, i.e. [`Gclk0Id`] - `Gclk11Id`, are
/// type-level variants of `GclkId`, and they identify one of the possible
/// generic clock generators.
///
/// `GclkId` is the type-level equivalent of [`DynGclkId`]. See the
/// documentation on [type-level programming] and specifically [type-level
/// enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait GclkId: Sealed {
    /// Corresponding variant of [`DynGclkId`]
    const DYN: DynGclkId;
    /// Corresponding numeric index (0..12)
    const NUM: usize;
    /// Corresponding [`GclkDivider`] type
    ///
    /// [`Gclk1`] uses [`GclkDiv16`], while all other [`Gclk`]s use
    /// [`GclkDiv8`].
    type Divider: GclkDivider;
}

/// Type-level variant of [`GclkId`] representing the identity of GCLK0
///
/// See the documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub enum Gclk0Id {}
impl Sealed for Gclk0Id {}
impl GclkId for Gclk0Id {
    const DYN: DynGclkId = DynGclkId::Gclk0;
    const NUM: usize = 0;
    type Divider = GclkDiv8;
}

/// Type-level variant of [`GclkId`] representing the identity of GCLK1
///
/// See the documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub enum Gclk1Id {}
impl Sealed for Gclk1Id {}
impl GclkId for Gclk1Id {
    const DYN: DynGclkId = DynGclkId::Gclk1;
    const NUM: usize = 1;
    type Divider = GclkDiv16;
}

macro_rules! make_gclk_id {
    ($num:literal) => {
        paste! {
            #[doc = GCLK $num]
            ///
            /// See the documentation on [type-level programming] and specifically
            /// [type-level enums] for more details.
            ///
            /// [type-level programming]: crate::typelevel
            /// [type-level enums]: crate::typelevel#type-level-enums
            pub enum [<Gclk $num Id>] {}
            impl Sealed for [<Gclk $num Id>] {}
            impl GclkId for [<Gclk $num Id>] {
                const DYN: DynGclkId = DynGclkId::[<Gclk $num>];
                const NUM: usize = $num;
                type Divider = GclkDiv8;
            }
        }
    };
}

make_gclk_id!(2);
make_gclk_id!(3);
make_gclk_id!(4);
make_gclk_id!(5);
#[hal_cfg("gclk6")]
make_gclk_id!(6);
#[hal_cfg("gclk7")]
make_gclk_id!(7);
#[hal_cfg("gclk8")]
make_gclk_id!(8);
#[hal_cfg("gclk9")]
make_gclk_id!(9);
#[hal_cfg("gclk10")]
make_gclk_id!(10);
#[hal_cfg("gclk11")]
make_gclk_id!(11);

//==============================================================================
// GclkDivider
//==============================================================================

/// Trait unifying the two [`Gclk`] divider types, [`GclkDiv8`] and
/// [`GclkDiv16`]
///
/// Choosing a [`Gclk`] division factor can be complicated. [`Gclk1`] can accept
/// a 16-bit divider value, while all other [`Gclk`]s only take an 8-bit value.
/// Moreover, the set of valid clock dividers does not form a contiguous range.
/// For example, the valid set of dividers for most [`Gclk`]s is 1-256 and 512.
///
/// The [`GclkDiv8`] and [`GclkDiv16`] enums provide simple and intuitive
/// user-facing interfaces to choose the *actual* clock divider value. This
/// trait, on the other hand, provides an internal-facing interface used by HAL
/// authors to extract the clock divider and convert it to the corresponding
/// `DIVSEL` and `DIV` register fields. Users should have no reason to interact
/// with this trait directly.
pub trait GclkDivider: Sealed + Default + Copy {
    /// Returns the actual clock divider value as a `u32`
    fn divider(&self) -> u32;
    /// Return the corresponding `DIVSEL` and and `DIV` register fields
    fn divsel_div(&self) -> (bool, u16);
}

//==============================================================================
// GclkDiv8
//==============================================================================

/// Enum for the clock division factor of all [`Gclk`]s other than [`Gclk1`]
///
/// Choosing a [`Gclk`] division factor can be complicated, because the set of
/// valid values is not contiguous. For clocks other than [`Gclk1`], the
/// division factor can be 1-256 or 512. `GclkDiv8` provides an enum interface
/// to enforce validity of the division factor. See the datasheet for more
/// details.
#[derive(Clone, Copy)]
pub enum GclkDiv8 {
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

impl Sealed for GclkDiv8 {}

impl Default for GclkDiv8 {
    #[inline]
    fn default() -> Self {
        Self::Div(0)
    }
}

impl GclkDivider for GclkDiv8 {
    #[inline]
    fn divider(&self) -> u32 {
        match self {
            GclkDiv8::Div(div) => (*div).into(),
            GclkDiv8::Div2Pow8 => 256,
            GclkDiv8::Div2Pow9 => 512,
        }
    }

    #[inline]
    fn divsel_div(&self) -> (bool, u16) {
        match self {
            GclkDiv8::Div(div) => (false, (*div).into()),
            GclkDiv8::Div2Pow8 => (true, 7),
            GclkDiv8::Div2Pow9 => (true, 8),
        }
    }
}

//==============================================================================
// GclkDiv16
//==============================================================================

/// Enum for the clock division factor of [`Gclk1`] only
///
/// Choosing the [`Gclk1`] division factor can be complicated, because the set
/// of valid values is not contiguous. For [`Gclk1`], the division factor can be
/// 1-65536 or 131072. `GclkDiv16` provides an enum interface to enforce
/// validity of the division factor. See the datasheet for more details.
#[derive(Clone, Copy)]
pub enum GclkDiv16 {
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

impl Sealed for GclkDiv16 {}

impl Default for GclkDiv16 {
    #[inline]
    fn default() -> Self {
        Self::Div(0)
    }
}

impl GclkDivider for GclkDiv16 {
    #[inline]
    fn divider(&self) -> u32 {
        match self {
            GclkDiv16::Div(div) => (*div).into(),
            GclkDiv16::Div2Pow16 => 65536,
            GclkDiv16::Div2Pow17 => 131072,
        }
    }

    #[inline]
    fn divsel_div(&self) -> (bool, u16) {
        match self {
            GclkDiv16::Div(div) => (false, *div),
            GclkDiv16::Div2Pow16 => (true, 15),
            GclkDiv16::Div2Pow17 => (true, 16),
        }
    }
}

//==============================================================================
// GclkIo
//==============================================================================

/// Trait mapping each [`PinId`] to its corresponding [`GclkId`] when used as a
/// [`Gclk`] input or output
///
/// If a given [`PinId`] can be used as a [`Gclk`] input or output, it can only
/// be used with one specific [`GclkId`]. This trait provides a mapping from
/// such a `PinId` to its corresponding `GclkId`.
pub trait GclkIo: PinId {
    /// Corresponding [`GclkId`] for this [`PinId`]
    type GclkId: GclkId;
}

// These implementations are much easier to read with `#[rustfmt::skip]`
#[rustfmt::skip]
#[hal_cfg("clock-d5x")]
mod gclkio_impl {
    use atsamd_hal_macros::hal_cfg;

    use super::*;

    #[hal_cfg("pa10")]
    impl GclkIo for gpio::PA10 { type GclkId = Gclk4Id; }
    #[hal_cfg("pa11")]
    impl GclkIo for gpio::PA11 { type GclkId = Gclk5Id; }
    #[hal_cfg("pa14")]
    impl GclkIo for gpio::PA14 { type GclkId = Gclk0Id; }
    #[hal_cfg("pa15")]
    impl GclkIo for gpio::PA15 { type GclkId = Gclk1Id; }
    #[hal_cfg("pa16")]
    impl GclkIo for gpio::PA16 { type GclkId = Gclk2Id; }
    #[hal_cfg("pa17")]
    impl GclkIo for gpio::PA17 { type GclkId = Gclk3Id; }
    #[hal_cfg("pa27")]
    impl GclkIo for gpio::PA27 { type GclkId = Gclk1Id; }
    #[hal_cfg("pa30")]
    impl GclkIo for gpio::PA30 { type GclkId = Gclk0Id; }
    #[hal_cfg("pb10")]
    impl GclkIo for gpio::PB10 { type GclkId = Gclk4Id; }
    #[hal_cfg("pb11")]
    impl GclkIo for gpio::PB11 { type GclkId = Gclk5Id; }
    #[hal_cfg("pb12")]
    impl GclkIo for gpio::PB12 { type GclkId = Gclk6Id; }
    #[hal_cfg("pb13")]
    impl GclkIo for gpio::PB13 { type GclkId = Gclk7Id; }
    #[hal_cfg("pb14")]
    impl GclkIo for gpio::PB14 { type GclkId = Gclk0Id; }
    #[hal_cfg("pb15")]
    impl GclkIo for gpio::PB15 { type GclkId = Gclk1Id; }
    #[hal_cfg("pb16")]
    impl GclkIo for gpio::PB16 { type GclkId = Gclk2Id; }
    #[hal_cfg("pb17")]
    impl GclkIo for gpio::PB17 { type GclkId = Gclk3Id; }
    #[hal_cfg("pb18")]
    impl GclkIo for gpio::PB18 { type GclkId = Gclk4Id; }
    #[hal_cfg("pb19")]
    impl GclkIo for gpio::PB19 { type GclkId = Gclk5Id; }
    #[hal_cfg("pb20")]
    impl GclkIo for gpio::PB20 { type GclkId = Gclk6Id; }
    #[hal_cfg("pb21")]
    impl GclkIo for gpio::PB21 { type GclkId = Gclk7Id; }
    #[hal_cfg("pb22")]
    impl GclkIo for gpio::PB22 { type GclkId = Gclk0Id; }
    #[hal_cfg("pb23")]
    impl GclkIo for gpio::PB23 { type GclkId = Gclk1Id; }
}

#[rustfmt::skip]
#[hal_cfg("clock-d21")]
mod gclkio_impl {
    use super::*;

    impl GclkIo for gpio::PA10 { type GclkId = Gclk4Id; }
    impl GclkIo for gpio::PA11 { type GclkId = Gclk5Id; }
    impl GclkIo for gpio::PA14 { type GclkId = Gclk0Id; }
    impl GclkIo for gpio::PA15 { type GclkId = Gclk1Id; }
    impl GclkIo for gpio::PA16 { type GclkId = Gclk2Id; }
    impl GclkIo for gpio::PA17 { type GclkId = Gclk3Id; }
    #[hal_cfg("pa20")]
    impl GclkIo for gpio::PA20 { type GclkId = Gclk4Id; }
    #[hal_cfg("pa21")]
    impl GclkIo for gpio::PA21 { type GclkId = Gclk5Id; }
    impl GclkIo for gpio::PA22 { type GclkId = Gclk6Id; }
    impl GclkIo for gpio::PA23 { type GclkId = Gclk7Id; }
    #[hal_cfg("pa27")]
    impl GclkIo for gpio::PA27 { type GclkId = Gclk0Id; }
    #[hal_cfg("pa28")]
    impl GclkIo for gpio::PA28 { type GclkId = Gclk0Id; }
    impl GclkIo for gpio::PA30 { type GclkId = Gclk0Id; }

    #[hal_cfg("pb10")]
    impl GclkIo for gpio::PB10 { type GclkId = Gclk4Id; }
    #[hal_cfg("pb11")]
    impl GclkIo for gpio::PB11 { type GclkId = Gclk5Id; }
    #[hal_cfg("pb12")]
    impl GclkIo for gpio::PB12 { type GclkId = Gclk6Id; }
    #[hal_cfg("pb13")]
    impl GclkIo for gpio::PB13 { type GclkId = Gclk7Id; }
    #[hal_cfg("pb14")]
    impl GclkIo for gpio::PB14 { type GclkId = Gclk0Id; }
    #[hal_cfg("pb15")]
    impl GclkIo for gpio::PB15 { type GclkId = Gclk1Id; }
    #[hal_cfg("pb16")]
    impl GclkIo for gpio::PB16 { type GclkId = Gclk2Id; }
    #[hal_cfg("pb17")]
    impl GclkIo for gpio::PB17 { type GclkId = Gclk3Id; }
    #[hal_cfg("pb22")]
    impl GclkIo for gpio::PB22 { type GclkId = Gclk0Id; }
    #[hal_cfg("pb23")]
    impl GclkIo for gpio::PB23 { type GclkId = Gclk1Id; }
}

#[rustfmt::skip]
#[hal_cfg("clock-d11")]
mod gclkio_impl {
    use super::*;

    impl GclkIo for gpio::PA08 { type GclkId = Gclk0Id; }
    impl GclkIo for gpio::PA09 { type GclkId = Gclk1Id; }
    #[hal_cfg("pa10")]
    impl GclkIo for gpio::PA10 { type GclkId = Gclk4Id; }
    #[hal_cfg("pa11")]
    impl GclkIo for gpio::PA11 { type GclkId = Gclk5Id; }
    impl GclkIo for gpio::PA14 { type GclkId = Gclk4Id; }
    impl GclkIo for gpio::PA15 { type GclkId = Gclk5Id; }
    #[hal_cfg("pa16")]
    impl GclkIo for gpio::PA16 { type GclkId = Gclk2Id; }
    #[hal_cfg("pa17")]
    impl GclkIo for gpio::PA17 { type GclkId = Gclk3Id; }
    #[hal_cfg("pa22")]
    impl GclkIo for gpio::PA22 { type GclkId = Gclk1Id; }
    #[hal_cfg("pa23")]
    impl GclkIo for gpio::PA23 { type GclkId = Gclk2Id; }
    impl GclkIo for gpio::PA24 { type GclkId = Gclk0Id; }
    impl GclkIo for gpio::PA25 { type GclkId = Gclk0Id; }
    #[hal_cfg("pa27")]
    impl GclkIo for gpio::PA27 { type GclkId = Gclk0Id; }
    impl GclkIo for gpio::PA30 { type GclkId = Gclk0Id; }
    impl GclkIo for gpio::PA31 { type GclkId = Gclk0Id; }
}

//==============================================================================
// Gclk0Io
//==============================================================================

/// Set of [`PinId`]s whose implementations of [`GclkIo`] map to [`Gclk0Id`]
///
/// This is effectively a trait alias for [`PinId`]s that implement [`GclkIo`]
/// with a `GclkId` associated type of [`Gclk0Id`], i.e.
/// `GclkIo<GclkId = Gclk0Id>`. The trait is useful to simplify some function
/// signatures and to help type inference in a few cases.
pub trait Gclk0Io
where
    Self: Sized,
    Self: GclkIo<GclkId = Gclk0Id>,
    Self: GclkSourceId<Resource = Pin<Self, AlternateH>>,
{
}

impl<I: GclkIo<GclkId = Gclk0Id>> Gclk0Io for I {}

//==============================================================================
// DynGclkSourceId
//==============================================================================

/// Value-level enum of possible clock sources for a [`Gclk`]
///
/// The variants of this enum identify one of nine possible clock sources for
/// a given [`Gclk`].
///
/// `DynGclkSourceId` is the value-level equivalent of [`GclkSourceId`].
#[hal_macro_helper]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DynGclkSourceId {
    Dfll,
    Dpll0,
    #[hal_cfg("clock-d5x")]
    Dpll1,
    Gclk1,
    GclkIn,
    #[hal_cfg(any("clock-d11", "clock-d21"))]
    Osc8M,
    #[hal_cfg(any("clock-d11", "clock-d21"))]
    Osc32k,
    OscUlp32k,
    Xosc0,
    #[hal_cfg("clock-d5x")]
    Xosc1,
    Xosc32k,
}

impl From<DynGclkSourceId> for Srcselect {
    #[hal_cfg("clock-d5x")]
    fn from(source: DynGclkSourceId) -> Self {
        match source {
            DynGclkSourceId::Dfll => Srcselect::Dfll,
            DynGclkSourceId::Dpll0 => Srcselect::Dpll0,
            DynGclkSourceId::Dpll1 => Srcselect::Dpll1,
            DynGclkSourceId::Gclk1 => Srcselect::Gclkgen1,
            DynGclkSourceId::GclkIn => Srcselect::Gclkin,
            DynGclkSourceId::OscUlp32k => Srcselect::Osculp32k,
            DynGclkSourceId::Xosc0 => Srcselect::Xosc0,
            DynGclkSourceId::Xosc1 => Srcselect::Xosc1,
            DynGclkSourceId::Xosc32k => Srcselect::Xosc32k,
        }
    }

    #[hal_cfg(any("clock-d11", "clock-d21"))]
    fn from(source: DynGclkSourceId) -> Self {
        match source {
            DynGclkSourceId::Dfll => Srcselect::Dfll48m,
            DynGclkSourceId::Dpll0 => Srcselect::Dpll96m,
            DynGclkSourceId::Gclk1 => Srcselect::Gclkgen1,
            DynGclkSourceId::GclkIn => Srcselect::Gclkin,
            DynGclkSourceId::Osc8M => Srcselect::Osc8m,
            DynGclkSourceId::Osc32k => Srcselect::Osc32k,
            DynGclkSourceId::OscUlp32k => Srcselect::Osculp32k,
            DynGclkSourceId::Xosc0 => Srcselect::Xosc,
            DynGclkSourceId::Xosc32k => Srcselect::Xosc32k,
        }
    }
}

//==============================================================================
// GclkSourceId
//==============================================================================

/// Type-level enum of possible clock [`Source`]s for a [`Gclk`]
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
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait GclkSourceId {
    /// Corresponding variant of [`DynGclkSourceId`]
    const DYN: DynGclkSourceId;

    /// Resource to store in the [`Gclk`]
    ///
    /// Maps to the corresponding [`Pin`] for [`GclkIo`] types. In all other
    /// cases, there is nothing to store, so it is `()`.
    #[doc(hidden)]
    type Resource;
}

impl GclkSourceId for DfllId {
    const DYN: DynGclkSourceId = DynGclkSourceId::Dfll;
    type Resource = ();
}
impl GclkSourceId for Dpll0Id {
    const DYN: DynGclkSourceId = DynGclkSourceId::Dpll0;
    type Resource = ();
}
#[hal_cfg("oscctrl")]
impl GclkSourceId for Dpll1Id {
    const DYN: DynGclkSourceId = DynGclkSourceId::Dpll1;
    type Resource = ();
}
impl GclkSourceId for Gclk1Id {
    const DYN: DynGclkSourceId = DynGclkSourceId::Gclk1;
    type Resource = ();
}
impl<I: GclkIo> GclkSourceId for I {
    const DYN: DynGclkSourceId = DynGclkSourceId::GclkIn;
    type Resource = Pin<I, AlternateH>;
}
impl GclkSourceId for OscUlp32kId {
    const DYN: DynGclkSourceId = DynGclkSourceId::OscUlp32k;
    type Resource = ();
}
#[hal_cfg(any("clock-d11", "clock-d21"))]
impl GclkSourceId for OscId {
    const DYN: DynGclkSourceId = DynGclkSourceId::Osc8M;
    type Resource = ();
}
impl GclkSourceId for Xosc0Id {
    const DYN: DynGclkSourceId = DynGclkSourceId::Xosc0;
    type Resource = ();
}
#[hal_cfg("xosc1")]
impl GclkSourceId for Xosc1Id {
    const DYN: DynGclkSourceId = DynGclkSourceId::Xosc1;
    type Resource = ();
}
#[hal_cfg("xosc32k")]
impl GclkSourceId for Xosc32kId {
    const DYN: DynGclkSourceId = DynGclkSourceId::Xosc32k;
    type Resource = ();
}

//==============================================================================
// NotGclkIo
//==============================================================================

/// Type-level enum of [`GclkSourceId`] types that are not a [`GclkIo`]
///
/// The datasheet notes that a [`Gclk`] can use a GPIO [`Pin`] as either input
/// or output, but not both. Stated differently, you cannot create a [`GclkOut`]
/// from a `Gclk` where the [`GclkSourceId`] is a [`PinId`].
///
/// This trait acts as a [type-level enum] narrowing [`GclkSourceId`] to exclude
/// any types which implement [`GclkIo`].
///
/// [type-level enum]: crate::typelevel#type-level-enums
pub trait NotGclkIo: GclkSourceId<Resource = ()> {}

impl<I: GclkSourceId<Resource = ()>> NotGclkIo for I {}

//==============================================================================
// Settings
//==============================================================================

/// Collection of [`Gclk`] settings
///
/// This structure is largely required due to the thumbv6m chips sharing one
/// GENCTRL register among all the clock generators.  On these chips, all fields
/// in the register need to be updated by a 32b atomic write.
struct Settings<G: GclkId> {
    div: G::Divider,
    output_off_value: bool,
    improve_duty_cycle: bool,
    output_enable: bool,
}

impl<G: GclkId> Clone for Settings<G> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<G: GclkId> Copy for Settings<G> {}

impl<G: GclkId> Default for Settings<G> {
    fn default() -> Self {
        Settings {
            div: G::Divider::default(),
            output_off_value: false,
            improve_duty_cycle: false,
            output_enable: false,
        }
    }
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
/// The type parameter `G` is a [`GclkId`] that determines which of the
/// generators this [`Gclk`] represents ([`Gclk0`] - `Gclk11` on the largest
/// targets). The type parameter `I` represents the `Id` type for the clock
/// [`Source`] driving this `Gclk`. It must be one of the valid
/// [`GclkSourceId`]s. Alternatively, if the `Gclk` is driven by a [GPIO](gpio)
/// [`Pin`], then `I` is a [`PinId`] implementing [`GclkIo`]. See the [`clock`
/// module documentation](super) for more detail on `Id` types.
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
    resource: I::Resource,
    src_freq: Hertz,
    settings: Settings<G>,
}

/// An [`Enabled`] [`Gclk`]
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// clocks consuming this [`Gclk`] and restricts access to the underlying
/// [`Gclk`] to prevent misuse.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
pub type EnabledGclk<G, I, N = U0> = Enabled<Gclk<G, I>, N>;

/// Type alias for the corresponding [`Gclk`]
///
/// As mentioned in the [module-level documentation](self), `Gclk0` is special,
/// because it provides the processor main clock. We represent this by
/// permanently [`Increment`]ing the counter for [`EnabledGclk0`], which
/// prevents it from ever being disabled. Accordingly, we also provide a few
/// special methods on [`EnabledGclk0`] to configure the `Gclk` while it is
/// actively running.
pub type Gclk0<I> = Gclk<Gclk0Id, I>;

/// Type alias for the corresponding [`EnabledGclk`]
///
/// As mentioned in the [module-level documentation](self), `Gclk0` is special,
/// because it provides the processor main clock. We represent this by
/// permanently [`Increment`]ing the counter for [`EnabledGclk0`], which
/// prevents it from ever being disabled. Thus, the default value for `N` is
/// [`U1`] instead of [`U0`]. Accordingly, we also provide a few special methods
/// on [`EnabledGclk0`] to configure the `Gclk` while it is actively running.
pub type EnabledGclk0<I, N = U1> = EnabledGclk<Gclk0Id, I, N>;

macro_rules! make_gclk {
    ($num:literal) => {
        paste! {
            /// Type alias for the corresponding [`Gclk`]
            pub type [<Gclk $num>]<I> = Gclk<[<Gclk $num Id>], I>;

            /// Type alias for the corresponding [`EnabledGclk`]
            pub type [<EnabledGclk $num>]<I, N = U0> = EnabledGclk<[<Gclk $num Id>], I, N>;
        }
    };
}

make_gclk!(1);
make_gclk!(2);
make_gclk!(3);
make_gclk!(4);
make_gclk!(5);
#[hal_cfg("gclk6")]
make_gclk!(6);
#[hal_cfg("gclk7")]
make_gclk!(7);
#[hal_cfg("gclk8")]
make_gclk!(8);
#[hal_cfg("gclk9")]
make_gclk!(9);
#[hal_cfg("gclk10")]
make_gclk!(10);
#[hal_cfg("gclk11")]
make_gclk!(11);

impl<G, I> Gclk<G, I>
where
    G: GclkId,
    I: GclkIo<GclkId = G>,
{
    /// Create a new [`Gclk`] from a GPIO [`Pin`]
    ///
    /// Creating a [`Gclk`] does not modify any of the hardware registers. It
    /// only serves to consume the [`Pin`] and create a struct to track the GCLK
    /// configuration.
    ///
    /// The configuration data is stored until the user calls [`enable`]. At
    /// that point, all of the registers are written according to the
    /// initialization procedures specified in the datasheet, and an
    /// [`EnabledGclk`] is returned. The `Gclk` is not active or useful until
    /// that point.
    ///
    /// [`enable`]: Gclk::enable
    pub fn from_pin<P>(token: GclkToken<G>, pin: P, freq: impl Into<Hertz>) -> Self
    where
        P: AnyPin<Id = I>,
    {
        Gclk {
            token,
            resource: pin.into().into_mode(),
            src_freq: freq.into(),
            settings: Settings::default(),
        }
    }

    /// Consume the [`Gclk`] and free its corresponding resources
    ///
    /// Freeing a [`Gclk`] returns the corresponding [`GclkToken`] and GPIO
    /// [`Pin`].
    pub fn free_pin(self) -> (GclkToken<G>, Pin<I, AlternateH>) {
        (self.token, self.resource)
    }
}

impl<G, I> Gclk<G, I>
where
    G: GclkId,
    I: NotGclkIo,
{
    /// Create a new [`Gclk`] from a clock [`Source`]
    ///
    /// Creating a [`Gclk`] does not modify any of the hardware registers. It
    /// only serves to [`Increment`] the [`Source`]'s [`Enabled`] counter
    /// and create a struct to track the GCLK configuration.
    ///
    /// The configuration data is stored until the user calls [`enable`]. At
    /// that point, all of the registers are written according to the
    /// initialization procedures specified in the datasheet, and an
    /// [`EnabledGclk`] is returned. The `Gclk` is not active or useful until
    /// that point.
    ///
    /// [`enable`]: Gclk::enable
    #[inline]
    pub fn from_source<S>(token: GclkToken<G>, source: S) -> (Gclk<G, I>, S::Inc)
    where
        S: Source<Id = I> + Increment,
    {
        let config = Gclk {
            token,
            resource: (),
            src_freq: source.freq(),
            settings: Settings::default(),
        };
        (config, source.inc())
    }

    /// Consume the [`Gclk`] and free its corresponding resources
    ///
    /// Freeing a [`Gclk`] returns the corresponding [`GclkToken`] and
    /// [`Decrement`]s the [`Source`]'s [`Enabled`] counter.
    #[inline]
    pub fn free_source<S>(self, source: S) -> (GclkToken<G>, S::Dec)
    where
        S: Source<Id = I> + Decrement,
    {
        (self.token, source.dec())
    }
}

impl<G, I> Gclk<G, I>
where
    G: GclkId,
    I: GclkSourceId,
{
    /// Updates the GENCTRL register based on self.settings
    ///
    /// The thumbv7em chips use one GENCTRL per clock generator and are capable
    /// of read-modify-write operations, but the thumbv6 chips share one GENCTRL
    /// register among the clock generators so only support writes.  To
    /// accommodate both, this implementation maintains a [`Settings`] struct
    /// containing the GENCTRL settings, and this method is called after
    /// updating it to apply them.
    #[hal_macro_helper]
    fn update_genctrl(&self, genen: bool) {
        let (divsel, div) = self.settings.div.divsel_div();

        #[hal_cfg("clock-d5x")]
        self.token.genctrl().modify(|_, w| {
            w.divsel().bit(divsel);
            // Safety: The `DIVSEL` and `DIV` values are derived from the
            // `GclkDivider` type, so they are guaranteed to be valid.
            unsafe {
                w.div().bits(div);
            }
            w.oe().bit(self.settings.output_enable);
            w.oov().bit(self.settings.output_off_value);
            w.idc().bit(self.settings.improve_duty_cycle);
            w.genen().bit(genen);
            w.src().variant(I::DYN.into())
        });

        #[hal_cfg(any("clock-d11", "clock-d21"))]
        {
            // Suppress warning for thumbv7em builds
            let _ = div;

            self.token.genctrl().write(|w| {
                w.divsel().bit(divsel);
                w.oe().bit(self.settings.output_enable);
                w.oov().bit(self.settings.output_off_value);
                w.idc().bit(self.settings.improve_duty_cycle);
                w.genen().bit(genen);
                w.src().variant(I::DYN.into());
                unsafe { w.id().bits(G::NUM as u8) }
            });
        }

        self.token.wait_syncbusy();
    }

    /// Modify the source of an existing clock
    ///
    /// This is a helper function for swapping Gclk0 to different clock sources.
    fn change_source<N: GclkSourceId>(
        self,
        resource: N::Resource,
        freq: Hertz,
    ) -> (Gclk<G, N>, I::Resource) {
        let gclk = Gclk {
            token: self.token,
            resource,
            src_freq: freq,
            settings: self.settings,
        };

        // Call update_genctrl() on object that has the correct source type
        gclk.update_genctrl(
            // This method is always called on Gclk0, which is always enabled
            true,
        );
        (gclk, self.resource)
    }

    /// Set the [`GclkDivider`] value
    ///
    /// Set the clock division factor from input to output. This takes either a
    /// [`GclkDiv8`] or [`GclkDiv16`] enum, restricting the possible division
    /// factors to only the valid ones for the given [`Gclk`]. See the
    /// [`GclkDivider`] trait for more details.
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn div(mut self, div: G::Divider) -> Self {
        self.settings.div = div;
        self
    }

    /// Output a 50-50 duty cycle clock when using an odd [`GclkDivider`]
    #[inline]
    pub fn improve_duty_cycle(mut self, flag: bool) -> Self {
        self.settings.improve_duty_cycle = flag;
        self
    }

    /// Return the [`Gclk`] ouput frequency
    ///
    /// This is the input frequency divided by the [`GclkDivider`].
    #[inline]
    pub fn freq(&self) -> Hertz {
        let div = max(1, self.settings.div.divider());
        self.src_freq / div
    }

    /// Set the state of [`GclkOut`] pins when [`GclkIo`] output is disabled
    ///
    /// The output off value (OOV) determines the logic level of a [GPIO](gpio)
    /// [`Pin`] (configured as a [`GclkIo`] output) when the [`Gclk`] is
    /// disabled **OR** the [`GclkOut`] is disabled.
    ///
    /// As mentioned in the [`Gclk`] documentation, configuration options are
    /// not usually applied until the call to [`Gclk::enable`]. However, because
    /// the OOV is relevant when the `Gclk` is *disabled*, we make an exception.
    /// When calling this function, the new OOV will take effect immediately.
    ///
    /// However, remember that the `Pin` is not controlled by the `Gclk` unless
    /// the `Pin` is configured in [`AlternateH`] mode. `Pin`s are automatically
    /// set to `AlternateH` mode when calling [`enable_gclk_out`], but by that
    /// point, the OOV is irrelevant. If you need the `Pin` to be set to its
    /// OOV, you must *manually* set it to `AlternateH` mode before constructing
    /// the `GclkOut`.
    ///
    /// [`enable_gclk_out`]: EnabledGclk::enable_gclk_out
    #[inline]
    pub fn output_off_value(mut self, high: bool) -> Self {
        self.settings.output_off_value = high;
        self.update_genctrl(
            // This method is only accessible via disabled GCLKs
            false,
        );

        self
    }

    /// Enable the [`Gclk`], so that it can be used as a clock [`Source`]
    ///
    /// The returned value is an [`EnabledGclk`] that can be used as a clock
    /// [`Source`] for other clocks.
    #[inline]
    #[hal_macro_helper]
    pub fn enable(self) -> EnabledGclk<G, I> {
        #[hal_cfg(any("clock-d11", "clock-d21"))]
        {
            let (_divsel, div) = self.settings.div.divsel_div();
            self.token.gendiv().write(|w| unsafe {
                w.id().bits(G::NUM as u8);
                w.div().bits(div)
            });
            self.token.wait_syncbusy();
        }

        self.update_genctrl(true);

        Enabled::new(self)
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
    /// only be disabled when no other clocks consume this [`Gclk`].
    #[inline]
    pub fn disable(self) -> Gclk<G, I> {
        self.0.update_genctrl(false);
        self.0
    }
}

/// Special methods for an [`Enabled`] [`Gclk0`]
///
/// [`Gclk0`] is special, because it drives the processor's main clock, which
/// can never be disabled. As discussed in the [module-level documentation],
/// this fact is represented by permanently [`Increment`]ing the counter for
/// [`EnabledGclk0`]. Thus, the minimum value for `N` is `U1` and
/// [`EnabledGclk0`] can never be disabled.
///
/// These methods represent actions that can be taken when `N = U1`, i.e. the
/// [`Enabled`] counter is at its minimum value. This is the only time it's
/// safe to change the [`Gclk0`] [`Source`] or change its [`GclkDivider`] value.
///
/// [module-level documentation]: self
impl<I: GclkSourceId> EnabledGclk0<I, U1> {
    /// Swap [`Gclk0`] from one clock [`Source`] to another
    ///
    /// `Gclk0` will remain fully enabled during the swap.
    ///
    /// Note for thumbv6m chips: Before switching the Generic Clock Generator 0
    /// (GCLKGEN0) from a clock source A to another clock source B, enable the
    /// "ONDEMAND" feature of the clock source A to ensure a proper transition
    /// from clock source A to clock source B.
    #[inline]
    pub fn swap_sources<O, N>(self, old: O, new: N) -> (EnabledGclk0<N::Id, U1>, O::Dec, N::Inc)
    where
        O: Source<Id = I> + Decrement,
        N: Source + Increment,
        N::Id: NotGclkIo,
    {
        let (gclk, _) = self.0.change_source((), new.freq());
        let enabled = Enabled::new(gclk);
        (enabled, old.dec(), new.inc())
    }

    /// Swap [`Gclk0`] from one [`GclkIo`] [`Pin`] to another
    ///
    /// `Gclk0` will remain fully enabled during the swap.
    /// TODO there's only one input IO pad per...
    #[inline]
    pub fn swap_pins<P>(
        self,
        pin: P,
        freq: impl Into<Hertz>,
    ) -> (EnabledGclk0<P::Id, U1>, Pin<I, AlternateH>)
    where
        I: Gclk0Io,
        P: AnyPin,
        P::Id: Gclk0Io,
    {
        let pin = pin.into().into_mode();
        let (gclk, pin) = self.0.change_source(pin, freq.into());
        let enabled = Enabled::new(gclk);
        (enabled, pin)
    }

    /// Swap [`Gclk0`] from a clock [`Source`] to a [`GclkIo`] [`Pin`]
    ///
    /// `Gclk0` will remain fully enabled during the swap.
    #[inline]
    pub fn swap_source_for_pin<S, P>(
        self,
        source: S,
        pin: P,
        freq: impl Into<Hertz>,
    ) -> (EnabledGclk0<P::Id, U1>, S::Dec)
    where
        S: Source<Id = I> + Decrement,
        P: AnyPin,
        P::Id: Gclk0Io,
    {
        let pin = pin.into().into_mode();
        let (gclk, _) = self.0.change_source(pin, freq.into());
        let enabled = Enabled::new(gclk);
        (enabled, source.dec())
    }

    /// Swap [`Gclk0`] from a [`GclkIo`] [`Pin`] to a clock [`Source`]
    ///
    /// `Gclk0` will remain fully enabled during the swap.
    #[inline]
    #[allow(clippy::type_complexity)]
    pub fn swap_pin_for_source<S>(
        self,
        source: S,
    ) -> (EnabledGclk0<S::Id, U1>, Pin<I, AlternateH>, S::Inc)
    where
        I: Gclk0Io,
        S: Source + Increment,
        S::Id: NotGclkIo,
    {
        let (gclk, pin) = self.0.change_source((), source.freq());
        let enabled = Enabled::new(gclk);
        (enabled, pin, source.inc())
    }

    /// Set the [`GclkDivider`] value for [`Gclk0`]
    ///
    /// See [`Gclk::div`] documentation for more details.
    #[inline]
    #[hal_macro_helper]
    pub fn div(&mut self, divider: GclkDiv8) {
        self.0.settings.div = divider;

        // D5x div is in the GENCTRL register, smaller chips keep it separate
        #[hal_cfg(any("clock-d11", "clock-d21"))]
        {
            let (_divsel, div) = divider.divsel_div();
            // Safety: The `DIVSEL` and `DIV` values are derived from the
            // `GclkDivider` type, so they are guaranteed to be valid.
            self.0.token.gendiv().write(|w| unsafe {
                w.id().bits(0);
                w.div().bits(div)
            });
            self.0.token.wait_syncbusy();
        }

        self.0.update_genctrl(true);
    }

    /// Output a 50-50 duty cycle clock when using an odd [`GclkDivider`]
    #[inline]
    pub fn improve_duty_cycle(&mut self, flag: bool) {
        self.0.settings.improve_duty_cycle = flag;
        self.0.update_genctrl(true);
    }

    /// Return the [`Gclk0`] frequency
    ///
    /// See [`Gclk::freq`] documentation for more details.
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.0.freq()
    }

    /// Set the state of [`GclkOut`] pins when [`GclkIo`] output is disabled
    ///
    /// See [`Gclk::output_off_value`] documentation for more details.
    #[inline]
    pub fn output_off_value(&mut self, high: bool) {
        self.0.settings.output_off_value = high;
        self.0.update_genctrl(true);
    }
}

//==============================================================================
// Source
//==============================================================================

impl<G, I, N> Source for EnabledGclk<G, I, N>
where
    G: GclkId,
    I: GclkSourceId,
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

/// Set of [`GclkToken`]s representing the disabled [`Gclk`]s at
/// power-on reset
#[hal_macro_helper]
pub struct GclkTokens {
    pub gclk0: GclkToken<Gclk0Id>,
    pub gclk1: GclkToken<Gclk1Id>,
    pub gclk2: GclkToken<Gclk2Id>,
    pub gclk3: GclkToken<Gclk3Id>,
    pub gclk4: GclkToken<Gclk4Id>,
    pub gclk5: GclkToken<Gclk5Id>,
    #[hal_cfg("gclk6")]
    pub gclk6: GclkToken<Gclk6Id>,
    #[hal_cfg("gclk7")]
    pub gclk7: GclkToken<Gclk7Id>,
    #[hal_cfg("gclk8")]
    pub gclk8: GclkToken<Gclk8Id>,
    #[hal_cfg("gclk9")]
    pub gclk9: GclkToken<Gclk9Id>,
    #[hal_cfg("gclk10")]
    pub gclk10: GclkToken<Gclk10Id>,
    #[hal_cfg("gclk11")]
    pub gclk11: GclkToken<Gclk11Id>,
}

#[hal_macro_helper]
impl GclkTokens {
    /// Create the set of [`GclkToken`]s
    ///
    /// # Safety
    ///
    /// All of the invariants required by `GclkToken::new` must be
    /// upheld here as well.
    #[inline]
    pub(super) unsafe fn new() -> Self {
        GclkTokens {
            gclk0: unsafe { GclkToken::new() },
            gclk1: unsafe { GclkToken::new() },
            gclk2: unsafe { GclkToken::new() },
            gclk3: unsafe { GclkToken::new() },
            gclk4: unsafe { GclkToken::new() },
            gclk5: unsafe { GclkToken::new() },
            #[hal_cfg("gclk6")]
            gclk6: unsafe { GclkToken::new() },
            #[hal_cfg("gclk7")]
            gclk7: unsafe { GclkToken::new() },
            #[hal_cfg("gclk8")]
            gclk8: unsafe { GclkToken::new() },
            #[hal_cfg("gclk9")]
            gclk9: unsafe { GclkToken::new() },
            #[hal_cfg("gclk10")]
            gclk10: unsafe { GclkToken::new() },
            #[hal_cfg("gclk11")]
            gclk11: unsafe { GclkToken::new() },
        }
    }
}

//==============================================================================
// GclkOut
//==============================================================================

/// A GPIO [`Pin`] configured as a [`Gclk`] output
///
/// The existence of this struct serves as proof that the corresponding [`Gclk`]
/// is [`Enabled`] and that it has been output to [`PinId`] `I`.
///
/// See the [module-level documentation](self) for an example of creating a
/// [`GclkOut`] from an [`EnabledGclk`].
pub struct GclkOut<I: GclkIo> {
    pin: Pin<I, AlternateH>,
    freq: Hertz,
}

impl<G, I> GclkOut<I>
where
    G: GclkId,
    I: GclkIo<GclkId = G>,
{
    /// Return the frequency of the corresponding [`Gclk`]
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.freq
    }
}

impl<G, S, N> EnabledGclk<G, S, N>
where
    G: GclkId,
    S: NotGclkIo,
{
    /// Create and enable a [`GclkOut`]
    ///
    /// Enabling [`GclkIo`] output will [`Increment`] the `EnabledGclk`
    /// counter, which will prevent it from being disabled while the
    /// `GclkOut` exists.
    ///
    /// Note that a given [`Gclk`] can only use [`GclkIo`] for input **or**
    /// output, but not both simultaneously. The [`NotGclkIo`] trait exists to
    /// enforce this requirement.
    ///
    /// Finally, when a [`GclkOut`] is disabled, but the [`Pin`] is still in
    /// [`AlternateH`] mode, it takes the "output off value" of the `Gclk`. See
    /// the [`Gclk::output_off_value`] documentation for more details.
    #[inline]
    pub fn enable_gclk_out<P>(mut self, pin: P) -> (EnabledGclk<G, S, N::Inc>, GclkOut<P::Id>)
    where
        N: Increment,
        P: AnyPin,
        P::Id: GclkIo<GclkId = G>,
    {
        let pin = pin.into().into_mode();
        self.0.settings.output_enable = true;
        self.0.update_genctrl(true);

        let gclk_out = GclkOut {
            pin,
            freq: self.freq(),
        };
        (self.inc(), gclk_out)
    }

    /// Disable a [`GclkOut`] and free its [`Pin`]
    ///
    /// Disabling [`GclkIo`] output will [`Decrement`] the [`EnabledGclk`]
    /// counter. When a [`GclkOut`] is disabled, but the [`Pin`] is still in
    /// [`AlternateH`] mode, it takes the "output off value" of the `Gclk`. See
    /// the [`Gclk::output_off_value`] documentation for more details.
    #[inline]
    pub fn disable_gclk_out<I>(
        mut self,
        gclk_out: GclkOut<I>,
    ) -> (EnabledGclk<G, S, N::Dec>, Pin<I, AlternateH>)
    where
        N: Decrement,
        I: GclkIo<GclkId = G>,
    {
        self.0.settings.output_enable = false;
        self.0.update_genctrl(true);

        (self.dec(), gclk_out.pin)
    }
}
