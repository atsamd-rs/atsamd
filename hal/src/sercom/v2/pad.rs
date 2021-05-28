//! Define a SERCOM pad type
//!
//! This module helps configure [`Pin`]s as SERCOM pads. It does not define a
//! pad type directly. Rather, it provides type-level tools to convert `Pin`s
//! to the correct [`PinMode`] and enforce type-level constraints at
//! compile-time.
//!
//! # Overview
//!
//! A SERCOM pad is defined by two types, its corresponding [`Sercom`] instance
//! and its [`PadNum`], from [`Pad0`] to [`Pad3`]. However, a given SERCOM pad
//! can usually be mapped to several possible [`PinId`]s.
//!
//! There are two primary traits defined in this module:
//! - The [`IsPad`] trait is implemented on `Pin` types that are properly
//!   configured as SERCOM pads, with `PinMode` [`AlternateC`] or
//!   [`AlternateD`]. It acts as both a [type class] for SERCOM pads and as a
//!   [type-level function] to recover the corresponding [`Sercom`] and
//!   [`PadNum`] types from the `Pin`.
//! - The [`PadLookup`] trait is used primarily to map [`PinId`]s to the
//!   corresponding `PinMode` for a given SERCOM pad. This relieves a burden on
//!   users, who no longer need to manually look up the corresponding `PinMode`.
//!
//! # The `GetPad` trait
//!
//! The [`GetPad<S>`] trait is a type-level function mapping an input type to
//! a configured `Pin` type for the given `Sercom`. For SAMD21 and SAMx5x
//! chips, `GetPad` is implemented on `PinId`s, while for SAMD11 chips, `GetPad`
//! is implemented on (`PadNum`, `PinId`) tuples, marked by the
#![cfg_attr(feature = "samd11", doc = "[`NITuple`]")]
#![cfg_attr(not(feature = "samd11"), doc = "`NITuple`")]
//! [type class].
//!
//! `GetPad` serves to improve the ergonomics of specifying pad types. With
//! knowledge of a desired `Sercom`, `GetPad<S>` allows users to specify a
//! unique pad with the minimum amount of information for the given chip.
//!
//! [`AlternateC`]: crate::gpio::v2::AlternateC
//! [`AlternateD`]: crate::gpio::v2::AlternateD
//! [type class]: crate::typelevel#type-classes
//! [type-level function]: crate::typelevel#type-level-functions
#![cfg_attr(
    feature = "min-samd51g",
    doc = "
# IOSET\n
\n
SAMx5x chips do not allow arbitrary combinations of `PinId` for a given
SERCOM. Instead, all `PinId`s must belong to the same IOSET. This module
defines a [type-level enum], [`IoSet`], to enforce this restriction. The
[`InIoSet`] [type class] is responsible for labeling each `IsPad` type with
its corresponding, valid `IoSet`\\(s).\n
\n
"
)]

use paste::paste;
use seq_macro::seq;

use super::Sercom;
use crate::gpio::v2::{AnyPin, OptionalPin, Pin, PinId, PinMode};
use crate::typelevel::{NoneT, Sealed};

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

//==============================================================================
// NITuple
//==============================================================================

/// Type-class for (`PadNum`, `PinId`) tuples
///
/// This trait serves as both a [type class] for ([`PadNum`], [`PinId`]) tuples
/// and as a [type-level function] mapping each `NITuple` to its constituent
/// `PadNum` and `PinId` types.
///
/// [type class]: crate::typelevel#type-classes
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
// OptionalNITuple
//==============================================================================

/// Type-level equivalent of `Option<NITuple>`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
#[cfg(feature = "samd11")]
pub trait OptionalNITuple: Sealed {}

#[cfg(feature = "samd11")]
impl OptionalNITuple for NoneT {}

#[cfg(feature = "samd11")]
impl<N: NITuple> OptionalNITuple for N {}

//==============================================================================
// GetNITuple
//==============================================================================

/// Type-level function to get the corresponding [`NITuple`] for a [`Pin`]
/// configured as a SERCOM pad
///
/// This trait acts as a [type-level function]. As input, it takes a `Pin`
/// configured as a SERCOM pad, and it uses the [`AnyPin`] and [`IsPad`] traits
/// to return an `NITuple`.
///
/// [type-level function]: crate::typelevel#type-level-functions
#[cfg(feature = "samd11")]
pub trait GetNITuple: AnyPin + IsPad {
    type NITuple: NITuple;
}

#[cfg(feature = "samd11")]
impl<P: AnyPin + IsPad> GetNITuple for P {
    type NITuple = (<P as IsPad>::PadNum, <P as AnyPin>::Id);
}

//==============================================================================
// GetOptionalNITuple
//==============================================================================

/// Type-level function to recover an [`OptionalNITuple`] from an
/// [`OptionalPin`]
///
/// This trait acts as a [type-level function]. In pseudo-Rust, it is the
/// type-level equivalent of starting with an `Option<Pin>` and calling
/// `.map(GetNITuple)` to recover an `Option<NITuple>`.
///
/// [type-level function]: crate::typelevel#type-level-functions
#[cfg(feature = "samd11")]
pub trait GetOptionalNITuple: OptionalPin {
    type NITuple: OptionalNITuple;
}

#[cfg(feature = "samd11")]
impl GetOptionalNITuple for NoneT {
    type NITuple = NoneT;
}

#[cfg(feature = "samd11")]
impl<P: GetNITuple> GetOptionalNITuple for P {
    type NITuple = P::NITuple;
}

//==============================================================================
// IsPad
//==============================================================================

/// Type class for [`Pin`]s configured as SERCOM pads
///
/// This trait serves as both a [type class] for `Pin`s configured as SERCOM
/// pads and as a [type-level function] mapping each `Pin` type to its
/// corresponding [`Sercom`] and [`PadNum`].
///
/// [type class]: crate::typelevel#type-classes
/// [type-level function]: crate::typelevel#type-level-functions
pub trait IsPad {
    type Sercom: Sercom;
    type PadNum: PadNum;
}

//==============================================================================
// PadLookup
//==============================================================================

/// Type-level function mapping [`PinId`]s to other pad-related types
///
/// For SAMD21 and SAMx5x chips, a [`Sercom`] and a [`PinId`] is enough
/// information to uniquely identify a pad, so this trait returns the
/// corresponding [`PadNum`] and [`PinMode`].
///
/// For SAMD11 chips, on the other hand, some `PinId`s can serve as two
/// different `PadNum`s for the *same* `Sercom`. For these chips, `PadLookup`
/// requires a second type parameter to specify the `PadNum` and only returns
/// the `PinMode`.
#[cfg(feature = "samd11")]
pub trait PadLookup<S, N>
where
    S: Sercom,
    Self: PinId,
{
    type PinMode: PinMode;
}

/// Type-level function mapping [`PinId`]s to other pad-related types
///
/// For SAMD21 and SAMx5x chips, a [`Sercom`] and a [`PinId`] is enough
/// information to uniquely identify a pad, so this trait returns the
/// corresponding [`PadNum`] and [`PinMode`].
///
/// For SAMD11 chips, on the other hand, some `PinId`s can serve as two
/// different `PadNum`s for the *same* `Sercom`. For these chips, `PadLookup`
/// requires a second type parameter to specify the `PadNum` and only returns
/// the `PinMode`.
#[cfg(not(feature = "samd11"))]
pub trait PadLookup<S>
where
    S: Sercom,
    Self: PinId,
{
    type PadNum: PadNum;
    type PinMode: PinMode;
}

//==============================================================================
// GetPadKey
//==============================================================================

/// Marker trait for implementers of [`GetPad`]
///
/// This trait has two purposes. First, it papers over differences between the
/// SAMD11 chips and all other chips. And second, it acts to prevent
/// conflicting-implementation errors.
///
/// The latter purpose is a limitation of the Rust trait system. For some
/// reason, when a trait takes type parameters, the compiler can't properly
/// enforce the trait orphan rules. As a workaround, you can use a marker trait
/// without type parameters, like this one, one to make it possible.
#[cfg(feature = "samd11")]
pub trait GetPadKey: NITuple {}

#[cfg(feature = "samd11")]
impl<P: NITuple> GetPadKey for P {}

/// Marker trait for implementers of [`GetPad`]
///
/// This trait has two purposes. First, it papers over differences between the
/// SAMD11 chips and all other chips. And second, it acts to prevent
/// conflicting-implementation errors.
///
/// The latter purpose is a limitation of the Rust trait system. For some
/// reason, when a trait takes type parameters, the compiler can't properly
/// enforce the trait orphan rules. As a workaround, you can use a marker trait
/// without type parameters, like this one, one to make it possible.
#[cfg(not(feature = "samd11"))]
pub trait GetPadKey: PinId {}

#[cfg(not(feature = "samd11"))]
impl<I: PinId> GetPadKey for I {}

//==============================================================================
// GetPad
//==============================================================================

/// Type-level function to uniquely specify a SERCOM [`Pad`] type
///
/// This trait acts as a [type-level function] to uniquely specify a `Pad`
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
    Self: GetPadKey,
{
    type PadNum: PadNum;
    type Pad: AnyPin;
}

/// Type alias to recover a [`Pin`] configured as a SERCOM pad using [`GetPad`]
pub type Pad<S, T> = <T as GetPad<S>>::Pad;

/// Type alias to recover the corresponding [`PinMode`] for a given
/// combination of [`Sercom`], [`PadNum`] and [`PinId`]
#[cfg(feature = "samd11")]
pub type PadMode<S, N, I> = <I as PadLookup<S, N>>::PinMode;

#[cfg(feature = "samd11")]
impl<S, T> GetPad<S> for T
where
    S: Sercom,
    T: NITuple,
    T::Id: PadLookup<S, T::Num>,
{
    type PadNum = T::Num;
    type Pad = Pin<T::Id, PadMode<S, T::Num, T::Id>>;
}

#[cfg(not(feature = "samd11"))]
impl<S, I> GetPad<S> for I
where
    S: Sercom,
    I: PadLookup<S>,
{
    type PadNum = I::PadNum;
    type Pad = Pin<I, I::PinMode>;
}

//==============================================================================
// GetOptionalPad
//==============================================================================

/// Type-level function to recover an [`OptionalPin`], configured as a SERCOM
/// pad, from an optional implementer of [`GetPad`]
///
/// This trait acts as a [type-level function]. In pseudo-Rust, it is the
/// type-level equivalent of starting with an `Option<GetPadKey>` and calling
/// `.map(GetPad)` to recover an `Option<Pad>`.
///
/// [type-level function]: crate::typelevel#type-level-functions
pub trait GetOptionalPad<S: Sercom>: Sealed {
    type PadNum: OptionalPadNum;
    type Pad: OptionalPin;
}

impl<S: Sercom> GetOptionalPad<S> for NoneT {
    type PadNum = NoneT;
    type Pad = NoneT;
}

impl<S, T> GetOptionalPad<S> for T
where
    S: Sercom,
    T: GetPad<S> + GetPadKey,
{
    type PadNum = T::PadNum;
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

/// Type class for SERCOM pads in a given [`IoSet`]
///
/// This trait is used to label each [`Pin`] implementing [`IsPad`] with its
/// corresponding [`IoSet`]\(s). Downstream types can use this trait as a
/// [type class] to restrict [`Pin`]s to a given [`IoSet`]. See the [type class]
/// documentation for more details on the pattern.
///
/// [type class]: crate::typelevel#type-classes
#[cfg(feature = "min-samd51g")]
pub trait InIoSet<I>
where
    Self: IsPad,
    I: IoSet,
{
}
