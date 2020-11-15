//! Version 2 of the SERCOM pads module
//!
//! This module implements the [`Pad`] type, which represents a [`Pin`]
//! configured to act as a SERCOM pad. A [`Pad`] is parameterized by three
//! types. The first two types identify the pad by its [`Sercom`] and
//! [`PadNum`]. However, each SERCOM pad can usually be mapped to several
//! possible GPIO pins. The third type must implement the [`Map`] trait, which
//! identifies a corresponding [`PinId`] and [`PinMode`]. The [`PinMode`] is
//! usually [`AlternateC`] or [`AlternateD`].
//!
//! To create a [`Pad`], use the [`From`]/[`Into`] traits. Upon creation, the
//! [`Pad`] takes ownership of the [`Pin`]. The conversion from [`Pin`] to
//! [`Pad`] is potentially many-valued, so it usually must be constrained. On
//! the other hand, the conversion from [`Pad`] to [`Pin`] is always unique,
//! because the [`Pad`] always knows which [`Pin`] it contains.
//!
//! ```rust
//! let pad: Pad<Sercom0, Pad0, IoSet1> = pins.pa08.into();
//! let pin: Pin<_, _> = pad.into();
//! ```
//!
//! Because of differences in the way pins are mapped to SERCOM pads, the
//! [`Map`] trait is implemented on different types, depending on the chip
//! series. See the [`Map`] documentation for more details.
//!
//! As a result, the actual implementations of [`Map`] are not found in this
//! module. They are included in the [`pad_map`] module.
//!
//! [`pad_map`]: crate::sercom::v2::pad_map
//! [`Pin`]: crate::gpio::v2::Pin
//! [`PinId`]: crate::gpio::v2::PinId
//! [`PinMode`]: crate::gpio::v2::PinMode
//! [`AlternateC`]: crate::gpio::v2::AlternateC
//! [`AlternateD`]: crate::gpio::v2::AlternateD
//! [`From`]: core::convert::From
//! [`Into`]: core::convert::Into

use core::convert::From;
use core::ops::Deref;

use crate::paste::paste;

use crate::target_device::sercom0;
use crate::target_device::{SERCOM0, SERCOM1};
#[cfg(any(feature = "samd21", feature = "min-samd51g"))]
use crate::target_device::{SERCOM2, SERCOM3};
#[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
use crate::target_device::{SERCOM4, SERCOM5};
#[cfg(feature = "min-samd51n")]
use crate::target_device::{SERCOM6, SERCOM7};

use crate::gpio::v2::*;
use crate::typelevel::*;

//==============================================================================
//  Sercom
//==============================================================================

/// Type-level `enum` representing a Serial Communication Interface (SERCOM)
pub trait Sercom: Sealed {
    /// Corresponding [PAC](crate::target_device) SERCOM type
    type SERCOM: Deref<Target = sercom0::RegisterBlock>;
}

/// Type alias to extract the correct [PAC](crate::target_device) SERCOM type
/// from the [`Sercom`] instance
pub type SERCOM<S> = <S as Sercom>::SERCOM;

macro_rules! sercom {
    ( $($Sercom:ident),+ ) => {
        paste! {
            $(
                /// Represents the corresponding SERCOM instance
                pub enum $Sercom {}
                impl Sealed for $Sercom {}
                impl Sercom for $Sercom { type SERCOM = [<$Sercom:upper>]; }
            )+
        }
    };
}

sercom!(Sercom0, Sercom1);
#[cfg(any(feature = "samd21", feature = "min-samd51g"))]
sercom!(Sercom2, Sercom3);
#[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
sercom!(Sercom4, Sercom5);
#[cfg(feature = "min-samd51n")]
sercom!(Sercom6, Sercom7);

//==============================================================================
//  PadNum
//==============================================================================

/// Type-level `enum` representing a SERCOM pad number
pub trait PadNum: Sealed {}

macro_rules! padnum {
    ( $( $PadNum:ident),+ ) => {
        $(
            /// Represents the corresponding SERCOM pad number
            pub enum $PadNum {}
            impl Sealed for $PadNum {}
            impl PadNum for $PadNum {}
        )+
    };
}

padnum!(Pad0, Pad1, Pad2, Pad3);

//==============================================================================
//  IoSet
//==============================================================================

/// Type-level `enum` representing a SERCOM IOSET configuration
#[cfg(feature = "min-samd51g")]
pub trait IoSet: Sealed {}

#[cfg(feature = "min-samd51g")]
macro_rules! ioset {
    ( $($IoSet:ident),+ ) => {
        $(
            /// Represents the corresponding IOSET
            pub enum $IoSet {}
            impl Sealed for $IoSet {}
            impl IoSet for $IoSet {}
        )+
    };
}

#[cfg(feature = "min-samd51g")]
ioset!(IoSet1, IoSet2, IoSet3, IoSet4);
#[cfg(feature = "min-samd51j")]
ioset!(IoSet5);
#[cfg(feature = "min-samd51g")]
ioset!(IoSet6);

//==============================================================================
//  Four-pad tuple struct
//==============================================================================

/// Tuple struct containing all four [`Pad`]s for a given [`Sercom`] and
/// [`IoSet`]
#[cfg(feature = "min-samd51g")]
pub struct Pads<S, I>(
    pub Pad<S, Pad0, I>,
    pub Pad<S, Pad1, I>,
    pub Pad<S, Pad2, I>,
    pub Pad<S, Pad3, I>,
)
where
    S: Sercom,
    I: IoSet + Map<S, Pad0> + Map<S, Pad1> + Map<S, Pad2> + Map<S, Pad3>;

//==============================================================================
//  Pin-to-pad mapping
//==============================================================================

/// Type-level function mapping [`Pad`]s to [`Pin`]s
///
/// This trait acts as a type-level function. It takes two types as arguments,
/// the [`Sercom`] and [`PadNum`] of a [`Pad`], and returns the [`PinId`] and
/// [`PinMode`] for the corresponding [`Pin`].
///
/// For the SAMD51 and SAME5x series chips, all pins for a given SERCOM must
/// come from the same IOSET. To account for this, we introduce a new
#[cfg_attr(feature = "min-samd51g", doc = "[`IoSet`]")]
#[cfg_attr(not(feature = "min-samd51g"), doc = "`IoSet`")]
/// meta-type and implement [`Map`] on its instances. For a given [`Sercom`] and
/// [`PadNum`], the
#[cfg_attr(feature = "min-samd51g", doc = "[`IoSet`]")]
#[cfg_attr(not(feature = "min-samd51g"), doc = "`IoSet`")]
/// uniquely identifies a corresponding [`PinId`] and
/// [`PinMode`].
///
/// The SAMD11 and SAMD21 series chips are not limited by IOSET. Any combination
/// of valid pins for a given SERCOM is acceptable. Thus, the [`Map`] trait is
/// implemented directly on [`PinId`]s. Because the same [`Pin`] can often be
/// used for two different [`Pad`]s, the [`Map`] trait acts to map a
/// [`Sercom`]/[`PadNum`] pair to the correct [`PinMode`] for the [`PinId`].
///
/// [`Pin`]: crate::gpio::v2::Pin
pub trait Map<S, P>
where
    S: Sercom,
    P: PadNum,
{
    /// The [`PinId`](crate::gpio::v2::PinId) for the corresponding pin
    type Id: PinId;
    /// The [`PinMode`](crate::gpio::v2::PinMode) for the corresponding pin
    type Mode: PinMode;
}

//==============================================================================
//  Pad struct
//==============================================================================

/// Represents a SERCOM Pad configured to use a particular pin
///
/// Each [`Pad`] is parameterized by a [`Sercom`], a [`PadNum`], and a third
/// type that implements [`Map`], which is used to determine the corresponding
/// [`Pin`] and its configuration.
///
/// For the SAMD51 and SAME5x chips, [`Map`] is implemented on instances of
#[cfg_attr(feature = "min-samd51g", doc = "[`IoSet`]")]
#[cfg_attr(not(feature = "min-samd51g"), doc = "`IoSet`")]
/// . The SAMD11 and SAMD21 do not have any concept of IOSET, so
/// [`Map`] is instead implemented directly on the corresponding [`PinId`].
///
/// Each [`Pad`] takes ownership of the corresponding [`Pin`] for the duration
/// of its lifetime. [`Pad`]s can be converted to and from [`Pin`]s using the
/// [`Into`] and [`From`] traits.
///
/// [`Pin`]: crate::gpio::v2::Pin
/// [`PinId`]: crate::gpio::v2::PinId
/// [`Into`]: core::convert::Into
/// [`From`]: core::convert::From
pub struct Pad<S, P, M>
where
    S: Sercom,
    P: PadNum,
    M: Map<S, P>,
{
    pin: Pin<M::Id, M::Mode>,
}

impl<S, P, M> Pad<S, P, M>
where
    S: Sercom,
    P: PadNum,
    M: Map<S, P>,
{
    #[inline]
    pub fn new<O: PinMode>(pin: Pin<M::Id, O>) -> Self
    where
        Pin<M::Id, O>: Into<Pin<M::Id, M::Mode>>,
    {
        Pad { pin: pin.into() }
    }
}

impl<S, P, M> Sealed for Pad<S, P, M>
where
    S: Sercom,
    P: PadNum,
    M: Map<S, P>,
{
}

//==============================================================================
//  AnyPad meta-type
//==============================================================================

/// Meta-type representing any `Pad`.
///
/// All instances of [`Pad`] implement this trait. When used as a trait bound,
/// it acts to encapsulate a [`Pad`]. Without this trait, a completely generic
/// [`Pad`] would require three type parameters. When using this trait as a
/// bound, only one type parameter is required, yet you can still recover each
/// type parameter of the corresponding [`Pad`] through the associated types.
pub trait AnyPad: Sealed {
    type Sercom: Sercom;
    type PadNum: PadNum;
    type Map: Map<Self::Sercom, Self::PadNum>;
}

impl<S, P, M> AnyPad for Pad<S, P, M>
where
    S: Sercom,
    P: PadNum,
    M: Map<S, P>,
{
    type Sercom = S;
    type PadNum = P;
    type Map = M;
}

//==============================================================================
//  Optional pads
//==============================================================================

/// Meta-type representing an optional [`Pad`].
///
/// This trait is implemented for every [`Pad`], as well as for
/// [`NoneT`](crate::typelevel::NoneT).
pub trait OptionalPad {}
impl OptionalPad for NoneT {}
impl<P: AnyPad> OptionalPad for P {}

/// Meta-type representing a valid [`Pad`].
///
/// When used as a bound, this trait allows you to exclude
/// [`NoneT`](crate::typelevel::NoneT) and limit the type to valid [`Pad`]s.
pub trait SomePad: OptionalPad + AnyPad {}
impl<P: AnyPad> SomePad for P {}

//==============================================================================
//  Convert between pin and pad
//==============================================================================

impl<S, P, M> From<Pad<S, P, M>> for Pin<M::Id, M::Mode>
where
    S: Sercom,
    P: PadNum,
    M: Map<S, P>,
{
    /// Convert from a [`Pad`] to its corresponding
    /// [`Pin`](crate::gpio::v2::Pin).
    ///
    /// This transformation is unique for a given [`Pad`].
    #[inline]
    fn from(pad: Pad<S, P, M>) -> Self {
        pad.pin
    }
}

impl<S, P, M, O> From<Pin<M::Id, O>> for Pad<S, P, M>
where
    S: Sercom,
    P: PadNum,
    M: Map<S, P>,
    O: PinMode,
    Pin<M::Id, O>: Into<Pin<M::Id, M::Mode>>,
{
    /// Convert from a [`Pin`](crate::gpio::v2::Pin) to its corresponding
    /// [`Pad`].
    ///
    /// This conversion is not necessarily unique for a given
    /// [`Pin`](crate::gpio::v2::Pin).
    #[inline]
    fn from(pin: Pin<M::Id, O>) -> Self {
        Pad::new(pin)
    }
}
