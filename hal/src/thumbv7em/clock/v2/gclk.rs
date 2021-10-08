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
//! #     gpio::v2::Pins,
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

use seq_macro::seq;
use typenum::{U0, U1};

use crate::pac;
use crate::pac::NVMCTRL;

pub use crate::pac::gclk::genctrl::{DIVSEL_A, SRC_A as GclkSourceEnum};

use super::gclkio::NotGclkInput;
use crate::clock::v2::{
    types::{Counter, Decrement, Enabled, Increment, PrivateIncrement},
    Source, SourceMarker,
};
use crate::pac::gclk::{RegisterBlock, GENCTRL};
use crate::time::Hertz;
use crate::typelevel::Sealed;

//==============================================================================
// GclkToken
//==============================================================================

/// A [`GclkToken`] equals a hardware register
/// Provide a safe register interface for [`Gclk`]s
///
/// This `struct` takes ownership of a [`GclkNum`] and provides an API to
/// access the corresponding registers
pub struct GclkToken<G: GclkNum> {
    gen: PhantomData<G>,
}

impl<G: GclkNum> GclkToken<G> {
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
    fn set_source(&mut self, variant: GclkSourceEnum) {
        self.genctrl().modify(|_, w| w.src().variant(variant));
        self.wait_syncbusy();
    }

    /// When dividing an input clock with a odd division factor the duty-cycle
    /// is not 50-50, enabling this ensures 50-50 duty-cycle on the
    /// resulting generator clock
    #[inline]
    fn improve_duty_cycle(&mut self, flag: bool) {
        self.genctrl().modify(|_, w| w.idc().bit(flag));
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
// GclkNum
//==============================================================================

/// Trait ensuring all [`Gclks`](Gclk) have numeric identifiers
pub trait GclkNum: Sealed {
    /// Associated constant marking an index of a [`Gclk`] type. It is useful in
    /// [`GclkToken`] in order to properly apply the offset to get an adequate
    /// [`GENCTRL`] register
    const NUM: usize;
    /// Each [`Gclk`] has a divider; its resolution might vary though. See
    /// [`GclkDivider`] implementors.
    type DividerType: GclkDivider;
}

impl<G: GclkNum> SourceMarker for G {}

/// A module that creates a namespace difference between a [`marker::Gclk0`] ..
/// [`marker::Gclk11`] marker types and a [`Gclk0`] .. [`Gclk11`] builder type
/// aliases
pub mod marker {
    use super::*;

    /// Trait allowing to pick all `GclkX` except [`Gclk0`]
    pub trait NotGclk0: GclkNum {}
    /// Trait allowing to pick all `GclkX` except [`Gclk1`]
    pub trait NotGclk1: GclkNum {}

    /// Type which serves as a source marker for the corresponding
    /// [`super::Gclk`] and provides numerical identity for it
    ///
    /// [`super::Gclk0`] is directly coupled to `MCLK` which provides the
    /// drives a synchronous clocking domain and the main clock
    ///
    /// [`NotGclk0`] can be used to exclude this [`Gclk0`]
    ///
    /// Standard division factor, see [`GclkDiv`]
    pub enum Gclk0 {}
    impl Sealed for Gclk0 {}
    impl NotGclk1 for Gclk0 {}
    impl GclkNum for Gclk0 {
        const NUM: usize = 0;
        type DividerType = GclkDiv;
    }

    /// Type which serves as a source marker for the corresponding
    /// [`super::Gclk`] and provides numerical identity for it
    ///
    /// [`super::Gclk1`] has the ability to be fed into other [`Gclk`]s as a
    /// source
    ///
    /// [`NotGclk1`] can be used to exclude this [`Gclk1`]
    ///
    /// Increased division factor, see [`Gclk1Div`]
    pub enum Gclk1 {}
    impl Sealed for Gclk1 {}
    impl NotGclk0 for Gclk1 {}
    impl GclkNum for Gclk1 {
        const NUM: usize = 1;
        type DividerType = Gclk1Div;
    }

    impl GclkSourceMarker for Gclk1 {
        const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::GCLKGEN1;
    }

    impl NotGclkInput for Gclk1 {}

    macro_rules! impl_gclk1_source {
        ($GclkNum:ident) => {
            impl<T, N> GclkSource<$GclkNum> for Enabled<super::Gclk1<T>, N>
            where
                T: GclkSourceMarker,
                N: Counter,
            {
                type Type = Gclk1;
            }
        };
    }

    impl_gclk1_source!(Gclk0);

    seq!(N in 2..=11 {
        impl_gclk1_source!(Gclk#N);
    });

    seq!(N in 2..=11 {
        /// Type which serves as a source marker for the corresponding
        /// [`super::Gclk`] and provides numerical identity for it
        ///
        /// Standard division factor, see [`GclkDiv`]
        pub enum Gclk#N {}
        impl Sealed for Gclk#N {}
        impl NotGclk0 for Gclk#N {}
        impl NotGclk1 for Gclk#N {}
        impl GclkNum for Gclk#N {
            const NUM: usize = N;
            type DividerType = GclkDiv;
        }
    });
}

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
    fn default() -> Self {
        Self::Div(0)
    }
}

impl Default for Gclk1Div {
    fn default() -> Self {
        Self::Div(0)
    }
}

impl GclkDivider for Gclk1Div {
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
// GclkSource
//==============================================================================

/// Trait generalizing over the markers for [`GclkSources`](GclkSource)
pub trait GclkSourceMarker: SourceMarker {
    /// Associated constant describing a source that is feeding the [`Gclk`]
    ///
    /// Expressed by a low-level enum variant (used for a HW write) provided by
    /// a PAC in [`genctrl::SRC_A`][crate::pac::gclk::genctrl::SRC_A]
    const GCLK_SRC: GclkSourceEnum;
}

/// Trait generalizing over the clock signal source for a [`Gclk`]
pub trait GclkSource<G: GclkNum>: Source {
    /// Associated source marker type
    type Type: GclkSourceMarker;
}

impl<G, T, N> Source for Enabled<Gclk<G, T>, N>
where
    G: GclkNum,
    T: GclkSourceMarker,
    N: Counter,
{
    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}

//==============================================================================
// Gclk
//==============================================================================

/// This struct represents a disabled [`Gclk`] - generic clock generator
///
/// It is generic over:
/// - a numeric variant (available variants: [`GclkNum`] implementors - eg.
///   [`marker::Gclk3`])
/// - a current signal source (expressed via source's marker type)
pub struct Gclk<G, T>
where
    G: GclkNum,
    T: GclkSourceMarker,
{
    /// Unique [`GclkToken`]
    token: GclkToken<G>,
    /// Clock source feeding the [`Gclk`]
    src: PhantomData<T>,
    /// Frequency of the clock source [`Gclk.src`] feeding the [`Gclk`]
    src_freq: Hertz,
    /// [`Gclk`] divider, modifying the [`Gclk.src_freq`]uency; affecting the
    /// output frequency
    div: G::DividerType,
    /// Improve duty cycle, used to ensure 50-50 duty cycle with odd dividers
    improve_duty_cycle: bool,
}

impl<G, T> Gclk<G, T>
where
    G: GclkNum,
    T: GclkSourceMarker,
{
    /// Create a new [`Gclk`]
    ///
    /// Hardware calls are deferred until the call to [`Gclk::enable`]
    #[inline]
    pub fn new<S>(token: GclkToken<G>, source: S) -> (Gclk<G, T>, S::Inc)
    where
        S: GclkSource<G, Type = T> + Increment,
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
        S: GclkSource<G, Type = T> + Decrement,
    {
        (self.token, source.dec())
    }

    /// Swap [`Gclk`] source
    ///
    /// Provided a [`GclkSource`] the [`Gclk`] is updated,
    /// the old clock source token released and returned
    #[inline]
    pub fn swap<Old, New>(self, old: Old, new: New) -> (Gclk<G, New::Type>, Old::Dec, New::Inc)
    where
        Old: GclkSource<G, Type = T> + Decrement,
        New: GclkSource<G> + Increment,
    {
        let (token, old) = self.free(old);
        let (config, new) = Gclk::new(token, new);
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
    pub fn enable(mut self) -> Enabled<Gclk<G, T>, U0> {
        self.token.set_source(T::GCLK_SRC);
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
impl<G, T, N> Enabled<Gclk<G, T>, N>
where
    G: GclkNum,
    T: GclkSourceMarker,
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

impl<G, T> Enabled<Gclk<G, T>, U0>
where
    G: GclkNum,
    T: GclkSourceMarker,
{
    /// Disable the [`Gclk`], possible
    /// when nothing depends on the `Gclk`
    #[inline]
    pub fn disable(mut self) -> Gclk<G, T> {
        self.0.token.disable();
        self.0
    }
}

impl<T: GclkSourceMarker> Enabled<Gclk0<T>, U1> {
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
    ) -> (Enabled<Gclk0<New::Type>, U1>, Old::Dec, New::Inc)
    where
        Old: GclkSource<marker::Gclk0, Type = T> + Decrement,
        New: GclkSource<marker::Gclk0> + Increment,
    {
        let (config, old, new) = self.0.swap(old, new);
        (config.enable().inc(), old, new)
    }

    /// Set the desired [`Gclk`] clock divider
    ///
    /// See [`GclkDiv`] for possible divider factors
    #[inline]
    pub fn div(self, div: GclkDiv) -> Self {
        Enabled::new(self.0.div(div))
    }

    /// When dividing an input clock with a odd division factor the duty-cycle
    /// is not 50-50, enabling this ensures 50-50 duty-cycle on the
    /// resulting generator clock
    #[inline]
    pub fn improve_duty_cycle(self, flag: bool) -> Self {
        Enabled::new(self.0.improve_duty_cycle(flag))
    }
}

//==============================================================================
// Gclk aliases
//==============================================================================

seq!(G in 0..=11 {
    /// Alias of [`Gclk`]`<marker::GclkX, _>`
    pub type Gclk#G<S> = Gclk<marker::Gclk#G, S>;
});

//==============================================================================
// Gclks
//==============================================================================

seq!(N in 1..=11 {
    /// [`Gclk`] tokens ensuring there only exists one instance of each [`Gclk`]
    pub struct Tokens {
        #( /// GclkToken for Gclk#N
           pub gclk#N: GclkToken<marker::Gclk#N>, )*
    }

    impl Tokens {
        pub(super) fn new(nvmctrl: &mut NVMCTRL) -> Self {
            // Use auto wait states
            nvmctrl.ctrla.modify(|_, w| w.autows().set_bit());
            // Return all tokens when initially created
            unsafe {
                Tokens {
                    #( gclk#N: GclkToken::new(), )*
                }
            }
        }
    }
});
