//! Define a SERCOM pad type
//!
//! This module defines the [`Pad`] type, which represents a [`Pin`] configured
//! to act as a SERCOM pad.
//!
//! # Overview
//!
//! Because each SERCOM pad can usually be mapped to several possible GPIO pins,
//! a `Pad` is parameterized by three [type-level enums]: [`Sercom`], [`PadNum`]
//! and [`PinId`]. For instance, pin PA11 configured to act as pad 3 for SERCOM
//! 0 would be specified as `Pad<Sercom0, Pad3, PA11>`.
//!
//! When a `Pad` is created, it takes ownership of the corresponding `Pin` and
//! correctly configures the [`PinMode`] automatically. Users usually don't need
//! to create `Pad`s directly. The downstream consumers of `Pad` types, like
//! [`v2::spi`](super::spi), often take care of that conversion. However, if
//! necessary, `Pad`s can be created in two different ways. They can be created
//! manually, using the [`Pad::new()`] method, or they can be converted [`From`]
//! `Pin`s. The conversion from `Pin` to `Pad` is generally many-valued, so the
//! types usually can't be inferred.
//!
//! ```
//! use atsamd_hal::pac::Peripherals;
//! use atsamd_hal::gpio::v2::Pins;
//! use atsamd_hal::sercom::v2::{Pad, Sercom0, Pad0, Pad1};
//!
//! let peripherals = Peripherals::take();
//! let pins = Pins::new(peripherals.PORT);
//! let pad0 = Pad::<Sercom0, Pad0, _>::new(pins.pa08);
//! let pad1: Pad<Sercom0, Pad1, _> = pins.pa09.into();
//! ```
//!
//! On the other hand, the conversion from `Pad` to `Pin` is always unique,
//! because the `Pad` always knows which `Pin` it contains. Conversion in this
//! direction can be accomplished with the [`free`] method or the
//! [`From`]/[`Into`] traits.
//!
//! ```
//! use atsamd_hal::gpio::v2::Pin;
//!
//! let pa08 = pad0.free();
//! let pa09: Pin<_, _> = pad1.into();
//! ```
#![cfg_attr(
    feature = "min-samd51g",
    doc = "
# IOSET\n
\n
SAMx5x chips do not allow arbitrary combinations of `PinId` for a given
SERCOM. Instead, all `PinId`s must belong to the same IOSET. This module
defines a [type-level enum], [`IoSet`], to enforce this restriction. The
[`InIoSet`] [type class] is responsible for labeling each `Pad` type with
its corresponding, valid `IoSet`\\(s).\n
\n
"
)]
//! # Type-level enforcement
//!
//! This module also provides additional, type-level tools to enforce the
//! restrictions imposed by the corresponding datasheets.
//!
//! The [`PadInfo`] trait forms the core of type-level encoding for `Pad`
//! types. All other traits are derived from `PadInfo` in some way. It acts as
//! a [type-level function] mapping `PinId`s to other `Pad`-related types.
//!
//! For a given `Sercom` and `PadNum`, the type-level function
//! [`GetPadMode`] maps a `PinId` to its corresponding `PinMode`, either
//! [`AlternateC`] or [`AlternateD`], and [`PadMode`] acts a type alias for the
//! output of this function. This trait is used to automatically convert a
//! `Pin` to the correct `PinMode` for the given `Pad`.
//!
//! The [`GetPad<S>`] trait is a type-level function mapping an input type to
//! a corresponding `Pad` type for the given `Sercom`. For SAMD21 and SAMx5x
//! chips, `GetPad` is implemented on `PinId`s, while for SAMD11 chips, `GetPad`
//! is implemented on (`PadNum`, `PinId`) tuples, marked by the
#![cfg_attr(feature = "samd11", doc = "[`NITuple`]")]
#![cfg_attr(not(feature = "samd11"), doc = "`NITuple`")]
//! [type class].
//!
//! `GetPad` serves to improve the ergonomics of specifying downstream, `Pad`
//! types. With knowledge of a desired `Sercom`, `GetPad<S>` allows users to
//! specify a unique `Pad` with the minimum amount of information for the given
//! chip.
//!
//! The [`ConvertPinToPad`] trait is a type-level function theat makes it easier
//! to convert a `Pin` type to its corresponding `Sercom` and `PadNum` types.
//!
//! The [`OptionalPadNum`] and [`OptionalPad`] traits use the [`OptionalKind`]
//! pattern to act as type-level versions of [`Option`] for `PadNum` and `Pad`
//! respectively. And the [`GetOptionalPad`] trait acts as a type-level function
//! to retreive an `OptionalPad`.
//!
//! Finally, the [`AnyPad`] trait defines an [`AnyKind`] type class for all
//! `Pad` types.
//!
//! [`AlternateC`]: crate::gpio::v2::AlternateC
//! [`AlternateD`]: crate::gpio::v2::AlternateD
//! [type-level enum]: crate::typelevel#type-level-enums
//! [type-level enums]: crate::typelevel#type-level-enums
//! [type class]: crate::typelevel#type-classes
//! [type-level function]: crate::typelevel#type-level-functions
//! [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
//! [`AnyKind`]: crate::typelevel#anykind-trait-pattern
//! [`new`]: Pad::new
//! [`free`]: Pad::free

use paste::paste;
use seq_macro::seq;

use super::Sercom;
use crate::gpio::v2::{AnyPin, OptionalPinId, Pin, PinId, PinMode, SpecificPinId};
use crate::typelevel::{Is, NoneT, Sealed};

//==============================================================================
// PadNum
//==============================================================================

/// Type-level enum representing a SERCOM pad number
///
/// It has variants [`Pad0`], [`Pad1`], [`Pad2`] & [`Pad3`]. See the [type-level
/// enum] documentation for an explanation of the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait PadNum: Sealed {}

seq!(N in 0..=3 {
    paste! {
        #[doc = "Type-level variant of [`PadNum`] representing SERCOM pad " N]
        ///
        /// See the [type-level enum] documentation for an explanation of the
        /// pattern.
        ///
        /// [type-level enum]: crate::typelevel#type-level-enum
        pub enum Pad#N {}
        impl Sealed for Pad#N {}
        impl PadNum for Pad#N {}
    }
});

//==============================================================================
// OptionalPadNum
//==============================================================================

/// Type-level equivalent of `Option<PadNum>`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait OptionalPadNum: Sealed {}

impl OptionalPadNum for NoneT {}

impl<N: PadNum> OptionalPadNum for N {}

/// Type-level equivalent of `Some(PadNum)`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait SomePadNum: OptionalPadNum + PadNum {}

impl<N: PadNum> SomePadNum for N {}

//==============================================================================
// PadInfo
//==============================================================================

/// Type-level function mapping [`PinId`]s to other [`Pad`]-related types
///
/// This trait forms the basis of all type-level enforcement in this module. Its
/// implementations are defined by macros in the [`pad_info`](super::pad_info)
/// module.
///
/// For SAMD21 and SAMx5x chips, a [`Sercom`] and a [`PinId`] is enough
/// information to uniquely identify a [`Pad`], so this trait returns the
/// corresponding [`PadNum`] and [`PinMode`].
///
/// For SAMD11 chips, on the other hand, some `PinId`s can serve as two
/// different `PadNum`s for the *same* `Sercom`. For these chips, `PadInfo`
/// requires a second type parameter to specify the `PadNum` and only returns
/// the `PinMode`.
#[cfg(feature = "samd11")]
pub trait PadInfo<S, N>
where
    S: Sercom,
    Self: PinId,
{
    /// Corresponding [`PinMode`] for `Pad<S, N, Self>`
    type PinMode: PinMode;
}

/// Type-level function mapping [`PinId`]s to other [`Pad`]-related types
///
/// This trait forms the basis of all type-level enforcement in this module. Its
/// implementations are defined by macros in the [`pad_info`](super::pad_info)
/// module.
///
/// For SAMD21 and SAMx5x chips, a [`Sercom`] and a [`PinId`] is enough
/// information to uniquely identify a [`Pad`], so this trait returns the
/// corresponding [`PadNum`] and [`PinMode`].
///
/// For SAMD11 chips, on the other hand, some `PinId`s can serve as two
/// different `PadNum`s for the *same* `Sercom`. For these chips, `PadInfo`
/// requires a second type parameter to specify the `PadNum` and only returns
/// the `PinMode`.
#[cfg(not(feature = "samd11"))]
pub trait PadInfo<S>
where
    S: Sercom,
    Self: PinId,
{
    /// Corresponding [`PadNum`] for the combination of `S` and `Self`
    type PadNum: PadNum;
    /// Corresponding [`PinMode`] for `Pad<S, Self::PadNum, Self>`
    type PinMode: PinMode;
}

//==============================================================================
// ConvertPinToPad
//==============================================================================

/// Type-level function mapping a configured [`Pin`] to its corresponding
/// [`Sercom`] and [`PadNum`]
///
/// This trait is a [type-level function] theat makes it easier to convert a
/// [`Pin`] type in a [`PinMode`] of [`AlternateC`] or [`AlternateD`] to its
/// corresponding [`Sercom`] and [`PadNum`] types.
///
/// The type aliases [`PinToSercom`], [`PinToPadNum`],
#[cfg_attr(feature = "samd11", doc = "[`PinToNITuple`]")]
#[cfg_attr(not(feature = "samd11"), doc = "`PinToNITuple`")]
/// and [`PinToPad`] use this trait to recover their respective types.
///
/// [type-level function]: crate::typelevel#type-level-functions
pub trait ConvertPinToPad: AnyPin {
    type Sercom: Sercom;
    type PadNum: PadNum;
}

/// Type alias to recover the corresponding [`Sercom`] type from a [`Pin`] using
/// the [`ConvertPinToPad`] trait.
pub type PinToSercom<P> = <P as ConvertPinToPad>::Sercom;

/// Type alias to recover the corresponding [`PadNum`] type from a [`Pin`] using
/// the [`ConvertPinToPad`] trait.
pub type PinToPadNum<P> = <P as ConvertPinToPad>::PadNum;

/// Type alias to recover the corresponding [`Pad`] type from a [`Pin`] using
/// the [`ConvertPinToPad`] trait.
pub type PinToPad<P> = Pad<PinToSercom<P>, PinToPadNum<P>, SpecificPinId<P>>;

/// Type alias to recover the corresponding [`NITuple`] type from a [`Pin`]
/// using the [`ConvertPinToPad`] trait.
#[cfg(feature = "samd11")]
pub type PinToNITuple<P> = (PinToPadNum<P>, SpecificPinId<P>);

//==============================================================================
// GetPadMode
//==============================================================================

/// Type-level function mapping [`PinId`]s to [`PinMode`]s for a given
/// [`Sercom`] and [`PadNum`]
///
/// This trait acts as a [type-level function]. It is implemented on [`PinId`]s
/// and it takes two additional types as arguments, a [`Sercom`] and a
/// [`PadNum`]. It returns the correct [`PinMode`] for the corresponding [`Pin`]
/// configured as a [`Pad`].
///
/// [type-level function]: crate::typelevel#type-level-functions
pub trait GetPadMode<S, N>
where
    Self: PinId,
    S: Sercom,
    N: PadNum,
{
    /// Corresponding [`PinMode`] for `Pad<S, N, Self>`
    type Mode: PinMode;
}

/// Alias for the return type of [`GetPadMode`]
///
/// See the documentation on [type-level functions] for an explanation of the
/// pattern.
///
/// [type-level functions]: crate::typelevel#type-level-functions
pub type PadMode<S, N, I> = <I as GetPadMode<S, N>>::Mode;

#[cfg(feature = "samd11")]
impl<S, N, I> GetPadMode<S, N> for I
where
    S: Sercom,
    N: PadNum,
    I: PadInfo<S, N>,
{
    type Mode = I::PinMode;
}

#[cfg(not(feature = "samd11"))]
impl<S, I> GetPadMode<S, I::PadNum> for I
where
    S: Sercom,
    I: PadInfo<S>,
{
    type Mode = I::PinMode;
}

//==============================================================================
// Pad
//==============================================================================

/// A GPIO pin configured as a SERCOM pad
///
/// Each `Pad` is parameterized by three [type-level enums], [`Sercom`],
/// [`PadNum`], and [`PinId`]. When created, a `Pad` takes ownership of the
/// corresponding [`Pin`]. `Pad`s usually don't need to be created by users
/// directly. But if required, they can be created with the [`new`](Self::new())
/// function or converted [`From`] `Pin`s. The corresponding `Pin` can be
/// recovered with the [`free`](Self::free()) function or again with
/// [`From`]/[`Into`].
///
/// See the [module-level documentation](self) for examples.
///
/// [type-level enums]: crate::typelevel#type-level-enum
/// [type-level function]: crate::typelevel#type-level-functions
pub struct Pad<S, N, I>
where
    S: Sercom,
    N: PadNum,
    I: GetPadMode<S, N>,
{
    pin: Pin<I, I::Mode>,
}

impl<S, N, I> Pad<S, N, I>
where
    S: Sercom,
    N: PadNum,
    I: GetPadMode<S, N>,
{
    /// Create a new SERCOM [`Pad`] from a [`Pin`]
    #[inline]
    pub fn new(pin: impl AnyPin<Id = I>) -> Self {
        let pin = pin.into().into_mode();
        Pad { pin }
    }

    /// Consume the [`Pad`] and release the corresponding [`Pin`]
    #[inline]
    pub fn free(self) -> Pin<I, I::Mode> {
        self.pin
    }
}

//==============================================================================
// Convert between Pin and Pad
//==============================================================================

impl<S, N, P> From<P> for Pad<S, N, P::Id>
where
    S: Sercom,
    N: PadNum,
    P: AnyPin,
    P::Id: GetPadMode<S, N>,
{
    /// Convert from a [`Pin`] to its corresponding [`Pad`]
    ///
    /// This conversion is not necessarily unique for a given [`Pin`].
    #[inline]
    fn from(pin: P) -> Self {
        Pad::new(pin)
    }
}

impl<S, N, I> From<Pad<S, N, I>> for Pin<I, I::Mode>
where
    S: Sercom,
    N: PadNum,
    I: GetPadMode<S, N>,
{
    /// Convert from a [`Pad`] to its corresponding [`Pin`]
    ///
    /// This transformation is unique for a given [`Pad`].
    #[inline]
    fn from(pad: Pad<S, N, I>) -> Self {
        pad.pin
    }
}

//==============================================================================
// AnyPad
//==============================================================================

/// Type class for [`Pad`] types
///
/// This trait uses the [`AnyKind`] trait pattern to create a [type class] for
/// [`Pad`] types. See the `AnyKind` documentation for more details on the
/// pattern.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait AnyPad: Is<Type = SpecificPad<Self>> {
    type Sercom: Sercom;
    type PadNum: PadNum;
    type PinId: GetPadMode<Self::Sercom, Self::PadNum>;
}

/// Type constructor to recover the specific [`Pad`] from an implementation of
/// [`AnyPad`]
///
/// See the [`AnyKind`] documentation for more details on the pattern.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
pub type SpecificPad<P> = Pad<<P as AnyPad>::Sercom, <P as AnyPad>::PadNum, <P as AnyPad>::PinId>;

impl<S, N, I> AsRef<Self> for Pad<S, N, I>
where
    S: Sercom,
    N: PadNum,
    I: GetPadMode<S, N>,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<S, N, I> AsMut<Self> for Pad<S, N, I>
where
    S: Sercom,
    N: PadNum,
    I: GetPadMode<S, N>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<S, N, I> Sealed for Pad<S, N, I>
where
    S: Sercom,
    N: PadNum,
    I: GetPadMode<S, N>,
{
}

impl<S, N, I> AnyPad for Pad<S, N, I>
where
    S: Sercom,
    N: PadNum,
    I: GetPadMode<S, N>,
{
    type Sercom = S;
    type PadNum = N;
    type PinId = I;
}

//==============================================================================
// OptionalPad
//==============================================================================

/// Type-level equivalent of `Option<Pad>`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait OptionalPad: Sealed {}
impl OptionalPad for NoneT {}
impl<P: AnyPad> OptionalPad for P {}

/// Type-level equivalent of `Some(Pad)`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait SomePad: OptionalPad + AnyPad {}
impl<P: AnyPad> SomePad for P {}

//==============================================================================
// NITuple
//==============================================================================

/// Type-class for (`PadNum`, `PinId`) tuples
///
/// This trait forms a type-class for all (`PadNum`, `PinId`) tuples, and it
/// allows you to recover the constituate types from a generic type implementing
/// `NITuple`.
///
/// This type is used by the [type-level function] [`GetPad`], through the
/// [`GetPadMarker`] trait. It allows the implementation of [`GetPad`] to be
/// fully generic, which helps type inference in downstream modules.
///
/// [type-level function]: crate::typelevel#type-level-functions
#[cfg(feature = "samd11")]
pub trait NITuple: Sealed {
    type Num: PadNum;
    type Id: PinId;
}

#[cfg(feature = "samd11")]
impl<N: PadNum, I: PinId> Sealed for (N, I) {}

#[cfg(feature = "samd11")]
impl<N: PadNum, I: PinId> NITuple for (N, I) {
    type Num = N;
    type Id = I;
}

//==============================================================================
// GetPadMarker
//==============================================================================

/// Marker trait for [`GetPad`]
///
/// This trait has two purposes. First, it papers over differences between the
/// SAMD11 chips and all other chips. And second, it acts to prevent
/// conflicting-implementation errors.
///
/// The latter purpose is a limitation of the Rust trait system. For some
/// reason, when a local trait takes type parameters, the compiler stops
/// understanding that a local trait can never be implemented for a local type
/// in some other crate. As a workaround, you can use a marker trait like this
/// one to make it possible.
#[cfg(feature = "samd11")]
pub trait GetPadMarker: NITuple {}

#[cfg(feature = "samd11")]
impl<P: NITuple> GetPadMarker for P {}

/// Marker trait for [`GetPad`]
///
/// This trait has two purposes. First, it papers over differences between the
/// SAMD11 chips and all other chips. And second, it acts to prevent
/// conflicting-implementation errors.
///
/// The latter purpose is a limitation of the Rust trait system. For some
/// reason, when a local trait takes type parameters, the compiler stops
/// understanding that a local trait can never be implemented for a local type
/// in some other crate. As a workaround, you can use a marker trait like this
/// one to make it possible.
#[cfg(not(feature = "samd11"))]
pub trait GetPadMarker: PinId {}

#[cfg(not(feature = "samd11"))]
impl<I: PinId> GetPadMarker for I {}

//==============================================================================
// GetPad
//==============================================================================

/// Type-level function to uniquely specify a [`Pad`] type
///
/// This trait acts as a [type-level function] to uniquely specify a [`Pad`]
/// type with as little information as possible.
///
/// SAMD21 and SAMx5x chips only ever map a given [`PinId`] to a single
/// [`PadNum`] for a given [`Sercom`]. SAMD11 chips, on the other hand,
/// sometimes have the same `PinId` mapped to two different `PadNum`s for the
/// *same* `Sercom`. As a result, a `Sercom` and a `PinId` is enough information
/// to uniquely specify a `Pad` for SAMD21 and SAMDx5x chips, but you need to
/// know the `PadNum` for SAMD11 chips.
///
/// For SAMD21 and SAMx5x chips, this trait is implemented on `PinId`s directly.
/// For SAMD11 chips, it is implemented on (`PadNum`, `PinId`) tupes, also known
/// as
#[cfg_attr(feature = "samd11", doc = "[`NITuple`]s")]
#[cfg_attr(not(feature = "samd11"), doc = "`NITuple`s")]
///
/// This trait improves the ergonomics of fully specifying a `Pad` in downstream
/// modules, like the [`spi::Pads`](super::spi::Pads) type.
///
/// [type-level function]: crate::typelevel#type-level-functions
pub trait GetPad<S>
where
    S: Sercom,
    Self: GetPadMarker,
{
    type PadNum: PadNum;
    type PinId: GetPadMode<S, Self::PadNum>;
    type Pad: AnyPad<Sercom = S, PadNum = Self::PadNum, PinId = Self::PinId>;
}

#[cfg(feature = "samd11")]
impl<S, T> GetPad<S> for T
where
    S: Sercom,
    T: NITuple,
    T::Id: PadInfo<S, T::Num>,
{
    type PadNum = T::Num;
    type PinId = T::Id;
    type Pad = Pad<S, T::Num, T::Id>;
}

#[cfg(not(feature = "samd11"))]
impl<S, I> GetPad<S> for I
where
    S: Sercom,
    I: PadInfo<S>,
{
    type PadNum = I::PadNum;
    type PinId = I;
    type Pad = Pad<S, I::PadNum, I>;
}

//==============================================================================
// GetOptionalPad
//==============================================================================

/// Type-level function to recover an [`OptionalPad`] from an optional
/// implementor of [`GetPad`]
///
/// This trait acts as a [type-level function]. In pseudo-Rust, it is the
/// type-level equivalent of starting with an `Option<GetPadMarker>` and calling
/// `.map(GetPad)` to recover an `Option<Pad>`.
///
/// [type-level function]: crate::typelevel#type-level-functions
pub trait GetOptionalPad<S: Sercom>: Sealed {
    type PadNum: OptionalPadNum;
    type PinId: OptionalPinId;
    type Pad: OptionalPad;
}

impl<S: Sercom> GetOptionalPad<S> for NoneT {
    type PadNum = NoneT;
    type PinId = NoneT;
    type Pad = NoneT;
}

impl<S, T> GetOptionalPad<S> for T
where
    S: Sercom,
    T: GetPad<S> + GetPadMarker,
{
    type PadNum = T::PadNum;
    type PinId = T::PinId;
    type Pad = T::Pad;
}

//==============================================================================
// IoSet
//==============================================================================

/// Type-level enum representing a SERCOM IOSET
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
#[cfg(feature = "min-samd51g")]
pub trait IoSet: Sealed {}

#[cfg(feature = "min-samd51g")]
seq!(N in 1..=6 {
    paste! {
        #[doc = "Type-level variant of [`IoSet`] representing SERCOM IOSET " N]
        ///
        /// See the [type-level enum] documentation for more details on the
        /// pattern.
        ///
        /// [type-level enum]: crate::typelevel#type-level-enum
        pub enum IoSet#N {}
        impl Sealed for IoSet#N {}
        impl IoSet for IoSet#N {}
    }
});

/// Type class for [`Pad`]s in a given [`IoSet`]
///
/// This trait is used to label each [`Pad`] with its corresponding
/// [`IoSet`]\(s). Downstream types can use this trait as a [type class] to
/// restrict [`Pad`]s to a given [`IoSet`]. See the [type class] documentation
/// for more details on the pattern.
///
/// [type class]: crate::typelevel#type-classes
#[cfg(feature = "min-samd51g")]
pub trait InIoSet<I>
where
    Self: AnyPad,
    I: IoSet,
{
}
