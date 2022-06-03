//! # GCLK - Generic Clock Controller
//!
//! Functionality:
//!
//! * Provides 12 [Gclks](Gclk) fed by clock sources implementing the
//!   [GclkSource] trait
//! * Each Generic Clock Generator provides clock division
//! * Generic Clock Generator output may be consumed by one or many Peripheral
//!   Channels [super::pclk]
//! * The Peripheral Channels outputs the clock to the peripheral modules
//!
//! Gclks are a core part of the clocking system, typically acting as an
//! intermediate step connecting [`GclkSources`](GclkSource) and
//! [`Pclks`](super::pclk::Pclk). [`Pclks`](super::pclk::Pclk) can share
//! a single [`Gclk`].
//!
//! Additionally, [`Gclk1`] has the ability to serve as a [`GclkSource`] itself
//! and provide a signal for other [`Gclks`](Gclk).
//!
//! [`Gclks`](Gclk) provides clock division capability (see [`GclkDiv`]), and
//! yet again [`Gclk1`] stands out by providing larger division factors than
//! other [`Gclks`](Gclk) (see [`Gclk1Div`]).
//!
//! [`Gclk0`] is unique as it is hardwired to drive the
//! ([`MCLK`](crate::pac::MCLK)) which is responsible for the synchronous
//! clocking domain including the CPU clock. It must be configured (to have CPU
//! activity) and therefore hardware has a default state where
//! [`Dpll`][super::dfll::Dfll] is a clock signal source (48 MHz).
//!
//! [`Gclk0`] is returned by [`retrieve_clocks`][super::retrieve_clocks]
//! function. In order to construct other [`Gclks`](Gclk), [`Gclk::new`]
//! function can be used. When [`Gclk`] is configured, it can be enabled by
//! [`Gclk::enable`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{gclk::Gclk, gclkio::GclkOut, retrieve_clocks, pclk::Pclk},
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! # };
//! let mut pac = Peripherals::take().unwrap();
//! let (gclk0, dfll, _, tokens) = retrieve_clocks(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let pins = Pins::new(pac.PORT);
//! let (gclk1, dfll) = Gclk::new(tokens.gclks.gclk1, dfll);
//! let gclk1 = gclk1.enable();
//! let (gclk2, gclk1) = Gclk::new(tokens.gclks.gclk2, gclk1);
//! let gclk2 = gclk2.enable();
//! let (pclk_sercom0, gclk2) = Pclk::enable(tokens.pclks.sercom0, gclk2);
//! ```

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

    /// Enable output of the generator clock over [`GCLK_IO`][GclkIo] pins
    ///
    /// `polarity` sets the "Output Off Value" (OOV) which sets the state
    /// of the pin when the output is disabled.
    ///
    /// Example: `polarity` = true sets the pin high when the output is
    /// disabled with [`disable_gclk_out`]
    #[inline]
    fn enable_gclk_out(&mut self, polarity: bool) {
        self.genctrl().modify(|_, w| {
            w.oe().set_bit();
            w.oov().bit(polarity)
        });
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
/// ```ignore
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
/// ```ignore
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
        let src_freq = source.freq();
        let improve_duty_cycle = false;
        let config = Gclk {
            token,
            src: PhantomData,
            src_freq,
            div: G::DividerType::default(),
            improve_duty_cycle,
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
            improve_duty_cycle: self.improve_duty_cycle,
        };
        let old = old.dec();
        let new = new.inc();
        (config, old, new)
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
    /// Enable the [`Gclk`] clock output
    ///
    /// `polarity` sets the "Output Off Value" which is
    /// the pin state when disabled
    #[inline]
    pub(super) fn enable_gclk_out(&mut self, polarity: bool) {
        self.0.token.enable_gclk_out(polarity);
    }

    /// Disable the [`Gclk`] clock output
    ///
    /// Pin state assumes the value as specified in
    /// `enable_gclk_out(polarity)`
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
