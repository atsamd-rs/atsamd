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

use crate::clock::v2::{
    types::{Counter, Decrement, Enabled, Increment},
    Source, SourceMarker,
};
use crate::gpio::v2::{self as gpio, AlternateM, AnyPin, Pin, PinId};
use crate::time::Hertz;
use crate::typelevel::*;

use super::gclk::*;

//==============================================================================
// GclkIo
//==============================================================================

/// Trait for binding [`gpio`] pins to specific [`Gclk`][`super::gclk]
pub trait GclkIo<G: GclkNum>: PinId {}

impl GclkIo<marker::Gclk4> for gpio::PA10 {}
impl GclkIo<marker::Gclk5> for gpio::PA11 {}

impl GclkIo<marker::Gclk0> for gpio::PA14 {}
impl GclkIo<marker::Gclk1> for gpio::PA15 {}
impl GclkIo<marker::Gclk2> for gpio::PA16 {}
impl GclkIo<marker::Gclk3> for gpio::PA17 {}

impl GclkIo<marker::Gclk1> for gpio::PA27 {}
impl GclkIo<marker::Gclk0> for gpio::PA30 {}

impl GclkIo<marker::Gclk4> for gpio::PB10 {}
impl GclkIo<marker::Gclk5> for gpio::PB11 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<marker::Gclk6> for gpio::PB12 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<marker::Gclk7> for gpio::PB13 {}

#[cfg(feature = "min-samd51j")]
impl GclkIo<marker::Gclk0> for gpio::PB14 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<marker::Gclk1> for gpio::PB15 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<marker::Gclk2> for gpio::PB16 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<marker::Gclk3> for gpio::PB17 {}
#[cfg(feature = "min-samd51n")]
impl GclkIo<marker::Gclk4> for gpio::PB18 {}
#[cfg(feature = "min-samd51n")]
impl GclkIo<marker::Gclk5> for gpio::PB19 {}
#[cfg(feature = "min-samd51n")]
impl GclkIo<marker::Gclk6> for gpio::PB20 {}
#[cfg(feature = "min-samd51n")]
impl GclkIo<marker::Gclk7> for gpio::PB21 {}

impl GclkIo<marker::Gclk0> for gpio::PB22 {}
impl GclkIo<marker::Gclk1> for gpio::PB23 {}

//==============================================================================
// GclkInToken
//==============================================================================

/// [`GclkInToken`] are singular for each `Gclk`, ensuring that
/// inputs are not multiply constructed
pub struct GclkInToken<G: GclkNum> {
    gen: PhantomData<G>,
}

impl<G> GclkInToken<G>
where
    G: GclkNum,
{
    /// Create a new [`GclkInToken`] associated to the given
    /// [`Gclk`][`super::gclk]
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
    G: GclkNum,
    I: GclkIo<G>,
{
    token: GclkInToken<G>,
    pin: Pin<I, AlternateM>,
    freq: Hertz,
}

impl<G, I> GclkIn<G, I>
where
    G: GclkNum,
    I: GclkIo<G>,
{
    /// Consume a [`GclkInToken`], `gpio` pin and a provided frequency to
    /// receive an enabled [`GclkIn`]
    pub fn enable<F>(token: GclkInToken<G>, pin: impl AnyPin<Id = I>, freq: F) -> Enabled<Self, U0>
    where
        F: Into<Hertz>,
    {
        let pin = pin.into().into_alternate();
        let freq = freq.into();
        Enabled::new(GclkIn { token, pin, freq })
    }

    /// Deconstruct the [`GclkIn`] and return the [`GclkInToken`] and `gpio` pin
    fn disable(self) -> (GclkInToken<G>, Pin<I, AlternateM>) {
        (self.token, self.pin)
    }
}

impl<G, I> Sealed for GclkIn<G, I>
where
    G: GclkNum,
    I: GclkIo<G>,
{
}

impl<G, I> Enabled<GclkIn<G, I>, U0>
where
    G: GclkNum,
    I: GclkIo<G>,
{
    /// Disable the [`GclkIn`], deconstruct it and return the [`GclkInToken`]
    /// and `gpio` pin
    pub fn disable(self) -> (GclkInToken<G>, Pin<I, AlternateM>) {
        self.0.disable()
    }
}

//==============================================================================
// GclkSource
//==============================================================================

/// Used to ensure a [`Gclk`] either acts as [`GclkIn`] or [`GclkOut`]
///
/// [`GclkOut`] cannot be constructed for a [`Gclk`] that is powered from a
/// [`GclkIn`] because of HW limitations (tested empirically; documentation does
/// not mention it).
///
/// As negated trait bounds are not available in Rust, a _negated_
/// [`NotGclkInput`] trait was introduced which is implemented by a subset of
/// [`GclkSourceMarkers`](GclkSourceMarker) that are not [`GclkIns`](GclkIn).
pub trait NotGclkInput: GclkSourceMarker {}

/// A [`GclkIn`] can act as a clock source for a [`Gclk`]
pub enum GclkInput {}

impl Sealed for GclkInput {}

impl GclkSourceMarker for GclkInput {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::GCLKIN;
}

impl SourceMarker for GclkInput {}

impl<G, I, N> GclkSource<G> for Enabled<GclkIn<G, I>, N>
where
    G: GclkNum,
    I: GclkIo<G>,
    N: Counter,
{
    type Type = GclkInput;
}

impl<G, I, N> Source for Enabled<GclkIn<G, I>, N>
where
    G: GclkNum,
    I: GclkIo<G>,
    N: Counter,
{
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
pub struct GclkOutToken<G: GclkNum> {
    gen: PhantomData<G>,
}

impl<G: GclkNum> GclkOutToken<G> {
    /// Create a new [`GclkOutToken`] associated to the given
    /// [`Gclk`][`super::gclk]
    unsafe fn new() -> GclkOutToken<G> {
        GclkOutToken { gen: PhantomData }
    }
}

//==============================================================================
// GclkOutSource
//==============================================================================

/// A [`GclkOut`] is associated with a [`Gclk`]
pub trait GclkOutSourceMarker: GclkNum + SourceMarker {}

impl<G: GclkNum> GclkOutSourceMarker for G {}

mod private {
    use super::*;
    pub trait GclkOutSource: Source {
        fn enable_gclk_out(&mut self, polarity: bool);
        fn disable_gclk_out(&mut self);
    }
}

pub(crate) use private::GclkOutSource as PrivateGclkOutSource;

/// [`GclkOutSource`] is the clock source for a [`GclkOut`]
pub trait GclkOutSource: PrivateGclkOutSource {
    /// Associated type
    type Type: GclkOutSourceMarker;
}

impl<G, T, N> GclkOutSource for Enabled<Gclk<G, T>, N>
where
    G: GclkOutSourceMarker,
    T: GclkSourceMarker + NotGclkInput,
    N: Counter,
{
    type Type = G;
}

impl<G, T, N> PrivateGclkOutSource for Enabled<Gclk<G, T>, N>
where
    G: GclkOutSourceMarker,
    T: GclkSourceMarker + NotGclkInput,
    N: Counter,
{
    /// Enable the gclk_out
    ///
    /// See [Enabled<Gclk>::enable_gclk_out][super::gclk::Gclk::enable_gclk_out]
    fn enable_gclk_out(&mut self, polarity: bool) {
        self.enable_gclk_out(polarity);
    }

    /// Disable the gclk_out
    ///
    /// See [Enabled<Gclk>::disable_gclk_out][super::gclk::Gclk::
    /// disable_gclk_out]
    fn disable_gclk_out(&mut self) {
        self.disable_gclk_out();
    }
}

//==============================================================================
// GclkOut
//==============================================================================

/// [`GclkOut`] requires a [`GclkOutToken<G>`] and a compatible [`gpio`] pin
/// and will assume the frequency from the source [`Gclk`]
pub struct GclkOut<G, I>
where
    G: GclkNum,
    I: GclkIo<G>,
{
    token: GclkOutToken<G>,
    freq: Hertz,
    pin: Pin<I, AlternateM>,
}

impl<G, I> GclkOut<G, I>
where
    G: GclkNum,
    I: GclkIo<G>,
{
    /// Consume a [`GclkOutToken`], `gpio` pin, `gclk` and the desired  receive
    /// a enabled [`GclkIn`]
    pub fn enable<S>(
        token: GclkOutToken<G>,
        pin: impl AnyPin<Id = I>,
        mut gclk: S,
        polarity: bool,
    ) -> (GclkOut<G, I>, S::Inc)
    where
        S: GclkOutSource<Type = G> + Increment,
    {
        let freq = gclk.freq();
        let pin = pin.into().into_alternate();
        gclk.enable_gclk_out(polarity);
        let gclk_out = GclkOut { token, freq, pin };
        (gclk_out, gclk.inc())
    }

    /// Returns the frequency as reported by the encapsulated [`super::gclk`]
    pub fn freq(&self) -> Hertz {
        self.freq
    }

    /// Deconstruct the GclkOut
    pub fn disable<S>(self, mut gclk: S) -> (GclkOutToken<G>, Pin<I, AlternateM>, S::Dec)
    where
        S: GclkOutSource<Type = G> + Decrement,
    {
        gclk.disable_gclk_out();
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
           pub gclk_in#N: GclkInToken<marker::Gclk#N>, )*
        #( /// GclkOut #N
           pub gclk_out#N: GclkOutToken<marker::Gclk#N>, )*
    }

    impl Tokens {
        // Populate the Tokens struct and return it
        pub(super) unsafe fn new() -> Tokens {
            Tokens {
                #( gclk_in#N: GclkInToken::new(), )*
                #( gclk_out#N: GclkOutToken::new(), )*
            }
        }
    }
});
