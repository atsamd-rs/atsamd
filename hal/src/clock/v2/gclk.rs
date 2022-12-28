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
//!     time::U32Ext,
//! };
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
//! Next, we use [`Gclk::from_pin`] to create a [`Gclk`] from a [`GclkToken`],
//! GPIO [`Pin`] and frequency, in [`Hertz`]. In this case, we create an
//! instance of [`Gclk1`].
//!
//! At this point, notice that [`Gclk<G, I>`] takes two type parameters. `G` is
//! a [`GclkId`] identifying which of the 12 generators this `Gclk` represents.
//! [`Gclk1<I>`] is simply an alias for `Gclk<Gclk1Id, I>`. `I` is an
//! [`Id` type](super#id-types) identifying the input clock, which must be a
//! valid [`GclkSourceId`]. In this case, `I` is [`PB14`](gpio::PB14), which is
//! a `GclkSourceId` for `Gclk1`, because it implements [`GclkIo`] with
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
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.mhz());
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
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! # let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.mhz());
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
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! # let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.mhz());
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
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! # let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.mhz());
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
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! # let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.mhz());
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
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! # let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.mhz());
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
//!     time::U32Ext,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (mut buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let pins = Pins::new(pac.PORT);
//! let gclk1 = Gclk::from_pin(tokens.gclks.gclk1, pins.pb15, 24.mhz());
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

use core::cmp::max;
use core::marker::PhantomData;

use paste::paste;
use seq_macro::seq;
use typenum::{U0, U1};

use crate::pac;
use crate::pac::gclk::genctrl::DIVSEL_A;
use crate::pac::NVMCTRL;

use crate::gpio::{self, AlternateM, AnyPin, Pin, PinId};
use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::gclk::GENCTRL;
use crate::time::Hertz;
use crate::typelevel::{Decrement, Increment, PrivateDecrement, PrivateIncrement, Sealed};

use super::dfll::DfllId;
use super::dpll::{Dpll0Id, Dpll1Id};
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
/// token for an actual clock with [`Gclk::from_source`] or [`Gclk::from_pin`].
///
/// [`GclkToken`] is generic over the [`GclkId`], where each corresponding token
/// represents one of the 12 respective [`Gclk`]s.
pub struct GclkToken<G: GclkId> {
    gen: PhantomData<G>,
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
        GclkToken { gen: PhantomData }
    }

    /// SYNCBUSY register mask for the corresponding GCLK
    const MASK: u16 = 1 << G::NUM;

    /// Provide a reference to the corresponding [`GENCTRL`] register
    #[inline]
    fn genctrl(&self) -> &GENCTRL {
        // Safety: Each `GclkToken` only has access to a mutually exclusive set
        // of registers for the corresponding `GclkId`, and we use a shared
        // reference to the register block. See the notes on `Token` types and
        // memory safety in the root of the `clock` module for more details.
        unsafe { &(*pac::GCLK::PTR).genctrl[G::NUM] }
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
        let syncbusy = unsafe { &(*pac::GCLK::PTR).syncbusy };
        while syncbusy.read().genctrl().bits() & Self::MASK != 0 {}
    }

    /// Set the clock source for this [`Gclk`]
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

    /// Enable [`Gclk`] output to a GPIO [`Pin`]
    #[inline]
    fn enable_gclk_out(&mut self) {
        self.genctrl().modify(|_, w| w.oe().set_bit());
        self.wait_syncbusy();
    }

    /// Disable [`Gclk`] output on a GPIO [`Pin`]
    ///
    /// If a corresponding [`Pin`] is in the [`AlternateM`] mode, it's logic
    /// level will depend on the [`output_off_value`].
    #[inline]
    fn disable_gclk_out(&mut self) {
        self.genctrl().modify(|_, w| w.oe().clear_bit());
        self.wait_syncbusy();
    }

    #[inline]
    fn configure(&mut self, id: DynGclkSourceId, settings: Settings<G>) {
        let (divsel, div) = settings.div.divsel_div();
        self.genctrl().modify(|_, w| {
            // Safety: The `DIVSEL` and `DIV` values are derived from the
            // `GclkDivider` type, so they are guaranteed to be valid.
            unsafe {
                w.divsel().variant(divsel);
                w.div().bits(div);
            };
            w.src().variant(id.into());
            w.idc().bit(settings.improve_duty_cycle);
            w.oov().bit(settings.output_off_value)
        });
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

seq!(N in 2..=11 {
    paste! {
        /// Type-level variant of [`GclkId`] representing the identity of
        #[doc = "GCLK" N]
        ///
        /// See the documentation on [type-level programming] and specifically
        /// [type-level enums] for more details.
        ///
        /// [type-level programming]: crate::typelevel
        /// [type-level enums]: crate::typelevel#type-level-enums
        pub enum [<Gclk N Id>] {}
        impl Sealed for [<Gclk N Id>] {}
        impl GclkId for [<Gclk N Id>] {
            const DYN: DynGclkId = DynGclkId::Gclk~N;
            const NUM: usize = N;
            type Divider = GclkDiv8;
        }
    }
});

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
    fn divsel_div(&self) -> (DIVSEL_A, u16);
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
    fn divsel_div(&self) -> (DIVSEL_A, u16) {
        match self {
            GclkDiv8::Div(div) => (DIVSEL_A::DIV1, (*div).into()),
            GclkDiv8::Div2Pow8 => (DIVSEL_A::DIV2, 7),
            GclkDiv8::Div2Pow9 => (DIVSEL_A::DIV2, 8),
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
    fn divsel_div(&self) -> (DIVSEL_A, u16) {
        match self {
            GclkDiv16::Div(div) => (DIVSEL_A::DIV1, *div),
            GclkDiv16::Div2Pow16 => (DIVSEL_A::DIV2, 15),
            GclkDiv16::Div2Pow17 => (DIVSEL_A::DIV2, 16),
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
mod gclkio_impl {

    use super::*;

    impl GclkIo for gpio::PA10 { type GclkId = Gclk4Id; }
    impl GclkIo for gpio::PA11 { type GclkId = Gclk5Id; }
    impl GclkIo for gpio::PA14 { type GclkId = Gclk0Id; }
    impl GclkIo for gpio::PA15 { type GclkId = Gclk1Id; }
    impl GclkIo for gpio::PA16 { type GclkId = Gclk2Id; }
    impl GclkIo for gpio::PA17 { type GclkId = Gclk3Id; }
    impl GclkIo for gpio::PA27 { type GclkId = Gclk1Id; }
    impl GclkIo for gpio::PA30 { type GclkId = Gclk0Id; }
    impl GclkIo for gpio::PB10 { type GclkId = Gclk4Id; }
    impl GclkIo for gpio::PB11 { type GclkId = Gclk5Id; }
    #[cfg(feature = "pins-64")]
    impl GclkIo for gpio::PB12 { type GclkId = Gclk6Id; }
    #[cfg(feature = "pins-64")]
    impl GclkIo for gpio::PB13 { type GclkId = Gclk7Id; }
    #[cfg(feature = "pins-64")]
    impl GclkIo for gpio::PB14 { type GclkId = Gclk0Id; }
    #[cfg(feature = "pins-64")]
    impl GclkIo for gpio::PB15 { type GclkId = Gclk1Id; }
    #[cfg(feature = "pins-64")]
    impl GclkIo for gpio::PB16 { type GclkId = Gclk2Id; }
    #[cfg(feature = "pins-64")]
    impl GclkIo for gpio::PB17 { type GclkId = Gclk3Id; }
    #[cfg(feature = "pins-100")]
    impl GclkIo for gpio::PB18 { type GclkId = Gclk4Id; }
    #[cfg(feature = "pins-100")]
    impl GclkIo for gpio::PB19 { type GclkId = Gclk5Id; }
    #[cfg(feature = "pins-100")]
    impl GclkIo for gpio::PB20 { type GclkId = Gclk6Id; }
    #[cfg(feature = "pins-100")]
    impl GclkIo for gpio::PB21 { type GclkId = Gclk7Id; }
    #[cfg(feature = "has-pb22")]
    impl GclkIo for gpio::PB22 { type GclkId = Gclk0Id; }
    #[cfg(feature = "has-pb23")]
    impl GclkIo for gpio::PB23 { type GclkId = Gclk1Id; }
}

//==============================================================================
// Gclk0Io
//==============================================================================

/// Set of [`PinId`]s whose implementations of [`GclkIo`] map to [`Gclk0Id`]
///
/// This is effectively a trait alias for [`PinId`]s that implement [`GclkIo`]
/// with a `GclkId` associated type of [`Gclk0Id`], i.e.
/// `GclkIo<GclkId = Gclk0Id>`. The trait is useful to simply some function
/// signatures and to help type inference in a few cases.
pub trait Gclk0Io
where
    Self: Sized,
    Self: GclkIo<GclkId = Gclk0Id>,
    Self: GclkSourceId<Resource = Pin<Self, AlternateM>>,
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
    type Resource = Pin<I, AlternateM>;
}
impl GclkSourceId for OscUlp32kId {
    const DYN: DynGclkSourceId = DynGclkSourceId::OscUlp32k;
    type Resource = ();
}
impl GclkSourceId for Xosc0Id {
    const DYN: DynGclkSourceId = DynGclkSourceId::Xosc0;
    type Resource = ();
}
impl GclkSourceId for Xosc1Id {
    const DYN: DynGclkSourceId = DynGclkSourceId::Xosc1;
    type Resource = ();
}
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

/// Collection of [`Gclk`] settings to configure on enable
struct Settings<G: GclkId> {
    div: G::Divider,
    output_off_value: bool,
    improve_duty_cycle: bool,
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
/// The type parameter `G` is a [`GclkId`] that determines which of the 12
/// generators this [`Gclk`] represents ([`Gclk0`] - [`Gclk11`]). The type
/// parameter `I` represents the `Id` type for the clock [`Source`] driving this
/// `Gclk`. It must be one of the valid [`GclkSourceId`]s. Alternatively, if the
/// `Gclk` is driven by a [GPIO](gpio) [`Pin`], then `I` is a [`PinId`]
/// implementing [`GclkIo`]. See the [`clock` module documentation](super) for
/// more detail on `Id` types.
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

seq!(G in 1..=11 {
    paste! {
        /// Type alias for the corresponding [`Gclk`]
        pub type Gclk~G<I> = Gclk<[<Gclk G Id>], I>;

        /// Type alias for the corresponding [`EnabledGclk`]
        pub type EnabledGclk~G<I, N = U0> = EnabledGclk<[<Gclk G Id>], I, N>;
    }
});

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
    pub fn free_pin(self) -> (GclkToken<G>, Pin<I, AlternateM>) {
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
    /// Modify the source of an existing clock
    ///
    /// This is a helper function for swapping Gclk0 to different clock sources.
    fn change_source<N: GclkSourceId>(
        mut self,
        resource: N::Resource,
        freq: Hertz,
    ) -> (Gclk<G, N>, I::Resource) {
        self.token.set_source(N::DYN);
        let gclk = Gclk {
            token: self.token,
            resource,
            src_freq: freq,
            settings: self.settings,
        };
        (gclk, self.resource)
    }

    /// Set the [`GclkDivider`] value
    ///
    /// Set the clock division factor from input to output. This takes either a
    /// [`GclkDiv8`] or [`GclkDiv16`] enum, restricting the possible division
    /// factors to only the valid ones for the given [`Gclk`]. See the
    /// [`GclkDivider`] trait for more details.
    #[inline]
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
    /// the `Pin` is configured in [`AlternateM`] mode. `Pin`s are automatically
    /// set to `AlternateM` mode when calling [`enable_gclk_out`], but by that
    /// point, the OOV is irrelevant. If you need the `Pin` to be set to its
    /// OOV, you must *manually* set it to `AlternateM` mode before constructing
    /// the `GclkOut`.
    ///
    /// [`enable_gclk_out`]: EnabledGclk::enable_gclk_out
    #[inline]
    pub fn output_off_value(mut self, high: bool) -> Self {
        self.settings.output_off_value = high;
        self.token.output_off_value(high);
        self
    }

    /// Enable the [`Gclk`], so that it can be used as a clock [`Source`]
    ///
    /// As mentioned in the [`Gclk`] documentation, no hardware registers are
    /// actually modified until this call. Rather, the desired configuration is
    /// stored internally, and the [`Gclk`] is initialized and configured here
    /// according to the datasheet.
    ///
    /// The returned value is an [`EnabledGclk`] that can be used as a clock
    /// [`Source`] for other clocks.
    #[inline]
    pub fn enable(mut self) -> EnabledGclk<G, I> {
        self.token.configure(I::DYN, self.settings);
        self.token.enable();
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
    pub fn disable(mut self) -> Gclk<G, I> {
        self.0.token.disable();
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
    #[inline]
    pub fn swap_pins<P>(
        self,
        pin: P,
        freq: impl Into<Hertz>,
    ) -> (EnabledGclk0<P::Id, U1>, Pin<I, AlternateM>)
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
    pub fn swap_pin_for_source<S>(
        self,
        source: S,
    ) -> (EnabledGclk0<S::Id, U1>, Pin<I, AlternateM>, S::Inc)
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
    pub fn div(&mut self, div: GclkDiv8) {
        self.0.settings.div = div;
        self.0.token.set_div(div);
    }

    /// Output a 50-50 duty cycle clock when using an odd [`GclkDivider`]
    #[inline]
    pub fn improve_duty_cycle(&mut self, flag: bool) {
        self.0.settings.improve_duty_cycle = flag;
        self.0.token.improve_duty_cycle(flag);
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
        self.0.token.output_off_value(high);
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

seq!(N in 1..=11 {
    paste! {
        /// Set of [`GclkToken`]s representing the disabled [`Gclk`]s at
        /// power-on reset
        pub struct GclkTokens {
            #(
                /// [`GclkToken`] for
                #[doc = "[`Gclk" N "`]"]
                pub gclk~N: GclkToken<[<Gclk N Id>]>,
            )*
        }

        impl GclkTokens {
            /// Create the set of [`GclkToken`]s
            ///
            /// # Safety
            ///
            /// All of the invariants required by `GclkToken::new` must be
            /// upheld here as well.
            #[inline]
            pub(super) unsafe fn new(nvmctrl: &mut NVMCTRL) -> Self {
                // Use auto wait states
                nvmctrl.ctrla.modify(|_, w| w.autows().set_bit());
                GclkTokens {
                    #( gclk~N: GclkToken::new(), )*
                }
            }
        }
    }
});

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
    pin: Pin<I, AlternateM>,
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
    /// [`AlternateM`] mode, it takes the "output off value" of the `Gclk`. See
    /// the [`Gclk::output_off_value`] documentation for more details.
    #[inline]
    pub fn enable_gclk_out<P>(mut self, pin: P) -> (EnabledGclk<G, S, N::Inc>, GclkOut<P::Id>)
    where
        N: Increment,
        P: AnyPin,
        P::Id: GclkIo<GclkId = G>,
    {
        let pin = pin.into().into_mode();
        let freq = self.freq();
        self.0.token.enable_gclk_out();
        let gclk_out = GclkOut { pin, freq };
        (self.inc(), gclk_out)
    }

    /// Disable a [`GclkOut`] and free its [`Pin`]
    ///
    /// Disabling [`GclkIo`] output will [`Decrement`] the [`EnabledGclk`]
    /// counter. When a [`GclkOut`] is disabled, but the [`Pin`] is still in
    /// [`AlternateM`] mode, it takes the "output off value" of the `Gclk`. See
    /// the [`Gclk::output_off_value`] documentation for more details.
    #[inline]
    pub fn disable_gclk_out<I>(
        mut self,
        gclk_out: GclkOut<I>,
    ) -> (EnabledGclk<G, S, N::Dec>, Pin<I, AlternateM>)
    where
        N: Decrement,
        I: GclkIo<GclkId = G>,
    {
        self.0.token.disable_gclk_out();
        (self.dec(), gclk_out.pin)
    }
}
