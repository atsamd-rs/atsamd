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
//! - A direct clock output ([`GclkOut`])
//! - A peripheral channel clock ([`Pclk`])
//!
//! ## Example
//!
//! The configuration of a [`Gclk`] is best shown with an example. However, the
//! example assumes you are already familiar with the basics of the `clock`
//! module. See the [module-level documentation](super) for an overview.
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
//! We can then call `Gclk::new` with a [`GclkToken`] and our `GclkIn` to
//! produce a [`Gclk`], in this case [`Gclk1`]. At this point, notice that
//! [`Gclk<G, I>`] takes two type parameters. `G` is a [`GclkId`] number, which
//! identifies which of the 12 generators this `Gclk` represents. `I` is an `Id`
//! type indicating the clock [`Source`] driving the `Gclk`, which must be a
//! valid [`GclkSourceId`]. [`Gclk1<I>`], then, is simply an alias for
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
//! [`EnabledGclk1`] counter is incremented.
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
//! ```
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
//! never be disabled.
//!
//! Additionally, we provide functions to change the clock [`Source`], divider,
//! etc.  on [`EnabledGclk0`], but we restrict them to the case where `N = U1`.
//! This prevents users from changing its [`Source`] or divider if any *other,
//! additional* clock depends on it, besides the master clock.
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

use core::marker::PhantomData;

use paste::paste;
use seq_macro::seq;
use typenum::{U0, U1};

use crate::pac;
use crate::pac::gclk::genctrl::DIVSEL_A;
use crate::pac::NVMCTRL;

use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::gclk::{RegisterBlock, GENCTRL};
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

/// A [`GclkToken`] equals a hardware register
/// Provide a safe register interface for [`Gclk`]s
///
/// This `struct` takes ownership of a [`GclkNum`] and provides an API to
/// access the corresponding registers
pub struct GclkToken<G: GclkId> {
    gen: PhantomData<G>,
}

impl<G: GclkId> GclkToken<G> {
    /// Create a new instance of [`GclkToken`]
    ///
    /// # Safety
    ///
    /// Users must never create two simulatenous instances of this `struct` with
    /// the same [`GclkNum`]
    #[inline]
    pub(super) unsafe fn new() -> Self {
        GclkToken { gen: PhantomData }
    }

    /// Used to mask out the correct bit based on [`GclkNum`]
    #[inline]
    fn mask(&self) -> u16 {
        1 << G::NUM
    }

    /// Provides the base pointer to the [`Gclk`] registers
    ///
    /// # Safety
    ///
    /// Only one [GclkToken] accessible at any given time
    #[inline]
    fn gclk(&self) -> &RegisterBlock {
        unsafe { &*pac::GCLK::ptr() }
    }

    /// Provides a pointer to the individual Generator Control [`GENCTRL`]
    /// registers
    ///
    /// Each GCLK 0 to 11 has its own Generator Control [`GENCTRL`] register
    /// controlling the settings of that specific generator
    #[inline]
    fn genctrl(&self) -> &GENCTRL {
        &self.gclk().genctrl[G::NUM]
    }

    /// Block until synchronization has completed
    ///
    /// Used for any registers annotated with
    ///
    /// * "Write-Synchronized"
    /// * "Read-Synchronized"
    ///
    /// in the Property field
    #[inline]
    fn wait_syncbusy(&self) {
        while self.gclk().syncbusy.read().genctrl().bits() & self.mask() != 0 {}
    }

    /// Set the clock source for the [`Gclk`] generator
    #[inline]
    fn set_source(&mut self, source: DynGclkSourceId) {
        self.genctrl().modify(|_, w| w.src().variant(source.into()));
        self.wait_syncbusy();
    }

    /// When dividing an input clock with a odd division factor the duty-cycle
    /// is not 50-50, enabling this ensures 50-50 duty-cycle on the
    /// resulting generator clock
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

    /// Enable output of the generator clock on GCLK_IO pins
    #[inline]
    fn enable_gclk_out(&mut self) {
        self.genctrl().modify(|_, w| w.oe().set_bit());
        self.wait_syncbusy();
    }

    /// Set the divider for the [`Gclk`]
    ///
    /// [`Gclk0`] and [`Gclk2`] to [`Gclk11`] has 8 division factor bits
    /// [`Gclk1`] has 16 division factor bits
    #[inline]
    fn set_div(&mut self, div: G::DividerType) {
        // Get the divsel variant and the divider value
        let (variant, div) = div.get_inner();
        self.genctrl().modify(|_, w| unsafe {
            w.divsel().variant(variant);
            w.div().bits(div)
        });
        self.wait_syncbusy();
    }

    /// Deactivate outputting generator clock over `GCLK_IO` pins
    ///
    /// Pin state depends on the `polarity` value set when the output was
    /// enabled with [`enable_gclk_out`]
    #[inline]
    fn disable_gclk_out(&mut self) {
        self.genctrl().modify(|_, w| w.oe().clear_bit());
        self.wait_syncbusy();
    }

    /// Enable the clock generator
    #[inline]
    fn enable(&mut self) {
        self.genctrl().modify(|_, w| w.genen().set_bit());
        self.wait_syncbusy();
    }

    /// Disable the clock generator
    #[inline]
    fn disable(&mut self) {
        self.genctrl().modify(|_, w| w.genen().clear_bit());
        self.wait_syncbusy();
    }
}

//==============================================================================
// GclkId
//==============================================================================

/// Value-level equivalent of [`GclkId`]
///
/// This is the value-level version of the type-level `enum` [`GclkId`]. It
/// selects among the 12 possible GCLKs.
#[allow(missing_docs)]
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

/// Type-level `enum` for GCLK identifiers
///
/// See the documentation / on [type-level enums] for more details on the
/// pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait GclkId: Sealed {
    /// Corresponding variant of [`DynGclkId`]
    const DYN: DynGclkId;
    /// Corresponding numeric index
    const NUM: usize;
    /// Corresponding divider type
    ///
    /// [`Gclk1`] can have a larger divider than the other [`Gclk`]s. The
    /// [`GclkDivider`] trait abstracts over these details.
    type DividerType: GclkDivider;
}

/// Type-level variant representing the identity of GCLK0
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum Gclk0Id {}
impl Sealed for Gclk0Id {}
impl GclkId for Gclk0Id {
    const DYN: DynGclkId = DynGclkId::Gclk0;
    const NUM: usize = 0;
    type DividerType = GclkDiv;
}

/// Type-level variant representing the identity of GCLK1
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum Gclk1Id {}
impl Sealed for Gclk1Id {}
impl GclkId for Gclk1Id {
    const DYN: DynGclkId = DynGclkId::Gclk1;
    const NUM: usize = 1;
    type DividerType = Gclk1Div;
}

seq!(N in 2..=11 {
    paste! {
        /// Type-level variant representing the identity of the corresponding GCLK
        ///
        /// This type is a member of several [type-level enums]. See the documentation
        /// on [type-level enums] for more details on the pattern.
        ///
        /// [type-level enums]: crate::typelevel#type-level-enum
        pub enum [<Gclk N Id>] {}
        impl Sealed for [<Gclk N Id>] {}
        impl GclkId for [<Gclk N Id>] {
            const DYN: DynGclkId = DynGclkId::Gclk~N;
            const NUM: usize = N;
            type DividerType = GclkDiv;
        }
    }
});

//==============================================================================
// Div
//==============================================================================

/// Trait for [`GclkDiv`] providing the inner components
pub trait GclkDivider: Sealed + Default + Copy {
    /// Returns both the divider mode [`DIVSEL_A`] and the divider value
    fn get_inner(&self) -> (DIVSEL_A, u16);
    /// Returns the divider value
    fn get_div(&self) -> u32;
}

/// Enum expressing all possible division factors for all [`Gclk`]s except
/// [`Gclk1`]
///
/// Represents a generic clock generator divider
///
/// * `Div(u8)` expresses the divider directly
/// * `Div2Pow8` equals a division factor of `2^8 = 256`
/// * `Div2Pow9` equals a division factor of `2^9 = 512`
///
/// ## Background
///
/// Division is interpreted differently depending on state of `DIVSEL` flag
///
/// In `DIVSEL` mode `DIV1` (register value 0) the division factor is directly
/// interpreted from the `DIV` register.  The division factor is a u8 value
///
/// In `DIVSEL` mode `DIV2` (register value 1) the division factor is calculated
/// as
///
/// ```text
/// division_factor = 2.pow(1 + DIV_register)
/// ```
///
/// The maximum division factor is 512 even though the register could be able to
/// express much larger dividers. Hardware ignores any larger value, effectively
/// constrained at max division factor.
///
/// See the datasheet for more details
#[derive(Clone, Copy)]
pub enum GclkDiv {
    /// Express the division factor directly, both 0 and 1 perform no division
    /// of the clock
    Div(u8),
    /// Provides a division factor of `2^8 = 256`
    Div2Pow8,
    /// Provides a division factor of `2^9 = 512`
    Div2Pow9,
}

/// Enum expressing all possible division factors for [`Gclk1`]
///
/// Represents [`Gclk1`] clock generator divider
///
/// Division is interpreted differently depending on state of `DIVSEL` flag
///
/// In `DIVSEL` mode `DIV1` (register value 0) the division factor is directly
/// interpreted from the `DIV` register.  The division factor is a u16 value
///
/// In `DIVSEL` mode `DIV2` (register value 1) the division factor is calculated
/// as
///
/// ```text
/// division_factor = 2.pow(1 + DIV_register)
/// ```
///
/// The maximum division factor is 131072 even though the register could be able
/// to express much larger dividers. Hardware ignores any larger value,
/// effectively constrained at max division factor.
///
/// See the datasheet for more details
#[derive(Clone, Copy)]
pub enum Gclk1Div {
    /// Express the division factor directly, both 0 and 1 perform no division
    /// of the clock
    Div(u16),
    /// Provides a division factor of `2^16 = 65536`
    Div2Pow16,
    /// Provides a division factor of `2^17 = 131072`
    Div2Pow17,
}

impl Sealed for GclkDiv {}
impl Sealed for Gclk1Div {}

impl Default for GclkDiv {
    #[inline]
    fn default() -> Self {
        Self::Div(0)
    }
}

impl Default for Gclk1Div {
    #[inline]
    fn default() -> Self {
        Self::Div(0)
    }
}

impl GclkDivider for Gclk1Div {
    #[inline]
    fn get_div(&self) -> u32 {
        match self {
            // Maximum reach of DIV1 mode is 65535
            Gclk1Div::Div(div) => (*div).into(),
            // Set the divider to be 65536
            // 2^(1 + 15) = 65536
            Gclk1Div::Div2Pow16 => 65536,
            // Set the divider to be 131072
            // 2^(1 + 16) = 131072
            Gclk1Div::Div2Pow17 => 131072,
        }
    }

    #[inline]
    fn get_inner(&self) -> (DIVSEL_A, u16) {
        match self {
            // Maximum reach of DIV1 mode is 65535
            Gclk1Div::Div(div) => (DIVSEL_A::DIV1, *div),
            // Set the divider to be 65536
            // 2^(1 + 15) = 65536
            Gclk1Div::Div2Pow16 => (DIVSEL_A::DIV2, 15),
            // Set the divider to be 131072
            // 2^(1 + 16) = 131072
            Gclk1Div::Div2Pow17 => (DIVSEL_A::DIV2, 16),
        }
    }
}

impl GclkDivider for GclkDiv {
    #[inline]
    fn get_div(&self) -> u32 {
        match self {
            // Maximum reach of DIV1 mode is 255
            GclkDiv::Div(div) => (*div).into(),
            // Set the divider to be 256
            // 2^(1 + 7) = 256
            GclkDiv::Div2Pow8 => 256,
            // Set the divider to be 512
            // 2^(1 + 8) = 512
            GclkDiv::Div2Pow9 => 512,
        }
    }

    #[inline]
    fn get_inner(&self) -> (DIVSEL_A, u16) {
        match self {
            // Maximum reach of DIV1 mode is 255
            GclkDiv::Div(div) => (DIVSEL_A::DIV1, (*div).into()),
            // Set the divider to be 256
            // 2^(1 + 7) = 256
            GclkDiv::Div2Pow8 => (DIVSEL_A::DIV2, 7),
            // Set the divider to be 512
            // 2^(1 + 8) = 512
            GclkDiv::Div2Pow9 => (DIVSEL_A::DIV2, 8),
        }
    }
}

//==============================================================================
// GclkSourceId
//==============================================================================

/// Value-level enum of GCLK sources
///
/// This is the value-level equivalent of the [`GclkSourceId`]
/// [type-level enum]. It lists all possible [`Source`]s for each [`Gclk`].
///
/// [type-level enum]: crate::typelevel#type-level-enum
#[allow(missing_docs)]
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

/// Type-level `enum` for GCLK sources
///
/// See the documentation / on [type-level enums] for more details on the
/// pattern.
///
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

/// This struct represents a disabled [`Gclk`] - generic clock generator
///
/// It is generic over:
/// - a numeric variant (available variants: [`GclkId`] implementors - eg.
///   [`GclkId3`])
/// - a current signal source (expressed via source's marker type)
pub struct Gclk<G, I>
where
    G: GclkId,
    I: GclkSourceId,
{
    /// Unique [`GclkToken`]
    token: GclkToken<G>,
    /// Clock source feeding the [`Gclk`]
    src: PhantomData<I>,
    /// Frequency of the clock source [`Gclk.src`] feeding the [`Gclk`]
    src_freq: Hertz,
    /// [`Gclk`] divider, modifying the [`Gclk.src_freq`]uency; affecting the
    /// output frequency
    div: G::DividerType,
    /// State of `GclkOut` pins when GCLK_IO is disabled
    output_off_value: bool,
    /// Improve duty cycle, used to ensure 50-50 duty cycle with odd dividers
    improve_duty_cycle: bool,
}

pub type EnabledGclk<G, I, N = U0> = Enabled<Gclk<G, I>, N>;

seq!(G in 0..=11 {
    paste! {
        /// Type alias for the corresponding [`Gclk`]
        pub type Gclk~G<I> = Gclk<[<Gclk G Id>], I>;

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
    /// Hardware calls are deferred until the call to [`Gclk::enable`]
    #[inline]
    pub fn new<S>(token: GclkToken<G>, source: S) -> (Gclk<G, I>, S::Inc)
    where
        S: Source<Id = I> + Increment,
    {
        let config = Gclk {
            token,
            src: PhantomData,
            src_freq: source.freq(),
            div: G::DividerType::default(),
            output_off_value: false,
            improve_duty_cycle: false,
        };
        (config, source.inc())
    }

    /// Deconstruct the [`Gclk`] and return the inner [`GclkToken`]
    #[inline]
    pub fn free<S>(self, source: S) -> (GclkToken<G>, S::Dec)
    where
        S: Source<Id = I> + Decrement,
    {
        (self.token, source.dec())
    }

    /// Swap [`Gclk`] source
    ///
    /// Provided a [`GclkSource`] the [`Gclk`] is updated,
    /// the old clock source token released and returned
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

    /// Set the state of [`GclkOut`](super::gclkio::GclkOut) pins when GCLK_IO
    /// output is disabled
    #[inline]
    pub fn output_off_value(mut self, high: bool) -> Self {
        self.output_off_value = high;
        self
    }

    /// When dividing an input clock with a odd division factor the duty-cycle
    /// is not 50-50, enabling this ensures 50-50 duty-cycle on the
    /// resulting generator clock
    #[inline]
    pub fn improve_duty_cycle(mut self, flag: bool) -> Self {
        self.improve_duty_cycle = flag;
        self
    }

    /// Returns the actual frequency of the [`Gclk`]
    ///
    /// The division factor set via `.div(GclkDiv)` stores the *actual* desired
    /// division factor, while hardware expresses this a bit differently
    /// under the hood
    ///
    /// Hardware has two modes, see [`GclkDiv`] and [`Gclk1Div`], where the
    /// `DIVSEL` register sets if the division factor is directly stored in
    /// register or if calculated as done in [`Gclk1Div`]
    ///
    /// A division factor of 0 is valid from the hardware point of view,
    /// equal to a division factor of 1, meaning "no division/passthrough"
    #[inline]
    pub fn freq(&self) -> Hertz {
        // Handle the allowed case with DIV-field set to zero
        let div = self.div.get_div();
        match div {
            0 => Hertz(self.src_freq.0),
            _ => Hertz(self.src_freq.0 / div),
        }
    }

    /// Set the desired [`Gclk`] clock divider
    ///
    /// See [`GclkDiv`] for possible divider factors
    #[inline]
    pub fn div(mut self, div: G::DividerType) -> Self {
        self.div = div;
        self
    }

    /// Enabling a [`Gclk`] modifies hardware to match the configuration
    /// stored within.
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

/// The [`Gclk`] generic clock generator
///
/// Provide methods for enable and disable gclk_out to
/// [`GclkOutSource`][super::sources::GclkOutSource]
impl<G, I, N> EnabledGclk<G, I, N>
where
    G: GclkId,
    I: GclkSourceId,
    N: Counter,
{
    /// Enable the [`Gclk`] clock output to GPIO pins
    #[inline]
    pub(super) fn enable_gclk_out(&mut self) {
        self.0.token.enable_gclk_out();
    }

    /// Disable the [`Gclk`] clock output to GPIO pins
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
    /// Disable the [`Gclk`], possible
    /// when nothing depends on the `Gclk`
    #[inline]
    pub fn disable(mut self) -> Gclk<G, I> {
        self.0.token.disable();
        self.0
    }
}

impl<I: GclkSourceId> EnabledGclk0<I, U1> {
    /// Swap source for [`Gclk0`]
    ///
    /// [`Gclk0`] is special since it always has `Mclk`
    /// as a dependency, with only one dependency (`Mclk`)
    /// allow to swap the clock source.
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

    /// Set the desired [`Gclk`] clock divider
    ///
    /// See [`GclkDiv`] for possible divider factors
    #[inline]
    pub fn div(self, div: GclkDiv) -> Self {
        let mut config = self.0.div(div);
        config.token.set_div(div);
        Enabled::new(config)
    }

    /// When dividing an input clock with a odd division factor the duty-cycle
    /// is not 50-50, enabling this ensures 50-50 duty-cycle on the
    /// resulting generator clock
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
// Gclks
//==============================================================================

seq!(N in 1..=11 {
    paste! {
        /// [`Gclk`] tokens ensuring there only exists one instance of each [`Gclk`]
        pub struct Tokens {
            #( /// GclkToken for the corresponding GCLK
               pub gclk~N: GclkToken<[<Gclk N Id>]>, )*
        }

        impl Tokens {
            #[inline]
            pub(super) fn new(nvmctrl: &mut NVMCTRL) -> Self {
                // Use auto wait states
                nvmctrl.ctrla.modify(|_, w| w.autows().set_bit());
                // Return all tokens when initially created
                unsafe {
                    Tokens {
                        #( gclk~N: GclkToken::new(), )*
                    }
                }
            }
        }
    }
});
