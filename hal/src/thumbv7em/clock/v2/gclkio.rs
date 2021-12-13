//! # GCLK - Generic Clock Controller Input-Output
//!
//! Used to input and output clock signals over GPIO pins
//!
//! Able to source a clock signal through [`GclkIn`]-capable pins
//! and output clock signals via [`GclkOut`] pins
//!
//! Setting up a [`GclkOut`] pin to output `Gclk0` signal on pin PB14:
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{gclkio::GclkOut, retrieve_clocks},
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
//! let (gclk_out0, gclk0) = GclkOut::enable(tokens.gclk_io.gclk_out0, pins.pb14, gclk0, false);
//! ```
//!
//! Setting up a [`GclkIn`] pin to receive a 48 MHz signal on pin PB17:
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{gclk::Gclk, gclkio::GclkIn},
//! #     time::U32Ext,
//! # };
//! # let mut pac = atsamd_hal::pac::Peripherals::take().unwrap();
//! # let (gclk0, dfll, _, tokens) = atsamd_hal::clock::v2::retrieve_clocks(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = atsamd_hal::gpio::v2::Pins::new(pac.PORT);
//! let gclk_in3 = GclkIn::enable(tokens.gclk_io.gclk_in3, pins.pb17, 48.mhz());
//! let (gclk3, gclk_in3) = Gclk::new(tokens.gclks.gclk3, gclk_in3);
//! let gclk3 = gclk3.enable();
//! ```
//!
//! A [`GclkIn`] is useful for sourcing clocks on other pins than those
//! dedicated to clock input such as the pins used by [`xosc`][super::xosc] or
//! [`xosc32k`][super::xosc32k].
//!
//! It is possible to feed a [`GclkIn`] from a [`GclkOut`] by running a proper
//! wire between GPIO pins. For example, using the code snippets above, by
//! outputting 48 MHz on PB14 and physically connecting it to PB17 yields a 48
//! MHz clock in `gclk_in3`

use core::marker::PhantomData;

use seq_macro::seq;
use typenum::U0;

use crate::gpio::v2::{self as gpio, AlternateM, AnyPin, Pin, PinId};
use crate::time::Hertz;
use crate::typelevel::{Counter, Decrement, Increment, Sealed};

use super::dfll::DfllId;
use super::dpll::{DpllId0, DpllId1};
use super::gclk::*;
use super::osculp32k::OscUlp32kId;
use super::xosc::{XoscId0, XoscId1};
use super::xosc32k::Xosc32kId;
use super::{Enabled, Source};

//==============================================================================
// GclkIo
//==============================================================================

/// Trait for binding [`gpio`] pins to specific [`Gclk`][`super::gclk]
pub trait GclkIo<G: GclkId>: PinId {}

impl GclkIo<GclkId4> for gpio::PA10 {}
impl GclkIo<GclkId5> for gpio::PA11 {}

impl GclkIo<GclkId0> for gpio::PA14 {}
impl GclkIo<GclkId1> for gpio::PA15 {}
impl GclkIo<GclkId2> for gpio::PA16 {}
impl GclkIo<GclkId3> for gpio::PA17 {}

impl GclkIo<GclkId1> for gpio::PA27 {}
impl GclkIo<GclkId0> for gpio::PA30 {}

impl GclkIo<GclkId4> for gpio::PB10 {}
impl GclkIo<GclkId5> for gpio::PB11 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<GclkId6> for gpio::PB12 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<GclkId7> for gpio::PB13 {}

#[cfg(feature = "min-samd51j")]
impl GclkIo<GclkId0> for gpio::PB14 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<GclkId1> for gpio::PB15 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<GclkId2> for gpio::PB16 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<GclkId3> for gpio::PB17 {}
#[cfg(feature = "min-samd51n")]
impl GclkIo<GclkId4> for gpio::PB18 {}
#[cfg(feature = "min-samd51n")]
impl GclkIo<GclkId5> for gpio::PB19 {}
#[cfg(feature = "min-samd51n")]
impl GclkIo<GclkId6> for gpio::PB20 {}
#[cfg(feature = "min-samd51n")]
impl GclkIo<GclkId7> for gpio::PB21 {}

impl GclkIo<GclkId0> for gpio::PB22 {}
impl GclkIo<GclkId1> for gpio::PB23 {}

//==============================================================================
// GclkInToken
//==============================================================================

/// [`GclkInToken`] are singular for each `Gclk`, ensuring that
/// inputs are not multiply constructed
pub struct GclkInToken<G: GclkId> {
    gen: PhantomData<G>,
}

impl<G> GclkInToken<G>
where
    G: GclkId,
{
    /// Create a new [`GclkInToken`] associated to the given
    /// [`Gclk`][`super::gclk]
    #[inline]
    unsafe fn new() -> GclkInToken<G> {
        GclkInToken { gen: PhantomData }
    }
}

//==============================================================================
// GclkIn
//==============================================================================

/// GclkIn requires a [`GclkInToken<G>`] and a compatible [`gpio`] pin
/// and relies on the user specifying the expected input frequency
pub struct GclkIn<G, I>
where
    G: GclkId,
    I: GclkIo<G>,
{
    token: GclkInToken<G>,
    pin: Pin<I, AlternateM>,
    freq: Hertz,
}

impl<G, I> GclkIn<G, I>
where
    G: GclkId,
    I: GclkIo<G>,
{
    /// Consume a [`GclkInToken`], `gpio` pin and a provided frequency to
    /// receive an enabled [`GclkIn`]
    #[inline]
    pub fn enable<F>(token: GclkInToken<G>, pin: impl AnyPin<Id = I>, freq: F) -> Enabled<Self, U0>
    where
        F: Into<Hertz>,
    {
        let pin = pin.into().into_alternate();
        let freq = freq.into();
        Enabled::new(GclkIn { token, pin, freq })
    }

    /// Deconstruct the [`GclkIn`] and return the [`GclkInToken`] and `gpio` pin
    #[inline]
    fn disable(self) -> (GclkInToken<G>, Pin<I, AlternateM>) {
        (self.token, self.pin)
    }
}

impl<G, I> Sealed for GclkIn<G, I>
where
    G: GclkId,
    I: GclkIo<G>,
{
}

impl<G, I> Enabled<GclkIn<G, I>, U0>
where
    G: GclkId,
    I: GclkIo<G>,
{
    /// Disable the [`GclkIn`], deconstruct it and return the [`GclkInToken`]
    /// and `gpio` pin
    #[inline]
    pub fn disable(self) -> (GclkInToken<G>, Pin<I, AlternateM>) {
        self.0.disable()
    }
}

//==============================================================================
// NotGclkInId
//==============================================================================

/// Type-level enum for all [`GclkSourceId`]s *except* [`GclkInId`]
///
/// This trait helps ensure that a [`Gclk`] never acts as both a [`GclkIn`] and
/// [`GclkOut`]. Although the documentation does not mention it, testing shows
/// it is impossible to do so.
///
/// See the documentation on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait NotGclkInId: GclkSourceId {}

impl NotGclkInId for DfllId {}
impl NotGclkInId for DpllId0 {}
impl NotGclkInId for DpllId1 {}
impl NotGclkInId for GclkId1 {}
impl NotGclkInId for OscUlp32kId {}
impl NotGclkInId for XoscId0 {}
impl NotGclkInId for XoscId1 {}
impl NotGclkInId for Xosc32kId {}

//==============================================================================
// GclkInId
//==============================================================================

/// Type-level variant representing the identity of an GCLK input clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum GclkInId {}

impl Sealed for GclkInId {}

//==============================================================================
// Source
//==============================================================================

impl<G, I, N> Source for Enabled<GclkIn<G, I>, N>
where
    G: GclkId,
    I: GclkIo<G>,
    N: Counter,
{
    type Id = GclkInId;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq
    }
}

//==============================================================================
// GclkOutToken
//==============================================================================

/// [`GclkOutToken`] are singular for each `Gclk`, ensuring that
/// outputs are not multiply constructed
pub struct GclkOutToken<G: GclkId> {
    gen: PhantomData<G>,
}

impl<G: GclkId> GclkOutToken<G> {
    /// Create a new [`GclkOutToken`] associated to the given
    /// [`Gclk`][`super::gclk]
    #[inline]
    unsafe fn new() -> GclkOutToken<G> {
        GclkOutToken { gen: PhantomData }
    }
}

//==============================================================================
// GclkOutSource
//==============================================================================

mod private {
    use super::{Counter, Enabled, Gclk, GclkId, GclkSourceId, NotGclkInId, Source};

    type EnabledGclk<T> = Enabled<
        Gclk<<T as Source>::Id, <T as AsEnabledGclk>::GclkSource>,
        <T as AsEnabledGclk>::Count,
    >;

    pub trait AsEnabledGclk: Source
    where
        Self::Id: GclkId,
    {
        type GclkSource: GclkSourceId + NotGclkInId;
        type Count: Counter;
        fn as_enabled_gclk(&mut self) -> &mut EnabledGclk<Self>;
    }

    impl<G, S, N> AsEnabledGclk for Enabled<Gclk<G, S>, N>
    where
        G: GclkId,
        S: GclkSourceId + NotGclkInId,
        N: Counter,
    {
        type GclkSource = S;
        type Count = N;
        fn as_enabled_gclk(&mut self) -> &mut EnabledGclk<Self> {
            self
        }
    }
}

use private::AsEnabledGclk;

//==============================================================================
// GclkOut
//==============================================================================

/// [`GclkOut`] requires a [`GclkOutToken<G>`] and a compatible [`gpio`] pin
/// and will assume the frequency from the source [`Gclk`]
pub struct GclkOut<G, I>
where
    G: GclkId,
    I: GclkIo<G>,
{
    token: GclkOutToken<G>,
    freq: Hertz,
    pin: Pin<I, AlternateM>,
}

impl<G, I> GclkOut<G, I>
where
    G: GclkId,
    I: GclkIo<G>,
{
    /// Consume a [`GclkOutToken`], `gpio` pin, `gclk` and the desired  receive
    /// a enabled [`GclkIn`]
    #[inline]
    pub fn enable<S>(
        token: GclkOutToken<G>,
        pin: impl AnyPin<Id = I>,
        mut gclk: S,
        polarity: bool,
    ) -> (GclkOut<G, I>, S::Inc)
    where
        S: Source<Id = G> + AsEnabledGclk + Increment,
    {
        let freq = gclk.freq();
        let pin = pin.into().into_alternate();
        gclk.as_enabled_gclk().enable_gclk_out(polarity);
        let gclk_out = GclkOut { token, freq, pin };
        (gclk_out, gclk.inc())
    }

    /// Returns the frequency as reported by the encapsulated [`super::gclk`]
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.freq
    }

    /// Deconstruct the GclkOut
    #[inline]
    pub fn disable<S>(self, mut gclk: S) -> (GclkOutToken<G>, Pin<I, AlternateM>, S::Dec)
    where
        S: Source<Id = G> + AsEnabledGclk + Decrement,
    {
        gclk.as_enabled_gclk().disable_gclk_out();
        (self.token, self.pin, gclk.dec())
    }
}

//==============================================================================
// GclkIo Tokens
//==============================================================================

seq!(N in 0..=11 {
    /// Tokens for every [`GclkIn`] and [`GclkOut`]
    pub struct Tokens {
        #( /// GclkIn #N
           pub gclk_in #N: GclkInToken<GclkId #N>, )*
        #( /// GclkOut #N
           pub gclk_out #N: GclkOutToken<GclkId #N>, )*
    }

    impl Tokens {
        // Populate the Tokens struct and return it
        #[inline]
        pub(super) unsafe fn new() -> Tokens {
            Tokens {
                #( gclk_in #N: GclkInToken::new(), )*
                #( gclk_out #N: GclkOutToken::new(), )*
            }
        }
    }
});
