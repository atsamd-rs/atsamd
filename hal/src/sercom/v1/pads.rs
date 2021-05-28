//! Version 1 of the SERCOM pads module
//!
//! This module is a compatibility shim that allows existing code to use the new
//! [`sercom::v2`](crate::sercom::v2) module. This API will eventually be
//! deprecated and removed.
//!
//! To recreate the `v1` API with `v2` types, this module defines its own
//! [`Pad`] type. The `v1::Pad` type is actually just a wrapper around a
//! [`v2::Pin`] configured as a SERCOM pad. The `SercomXPadY` types of the
//! original, `v1` API are recreated as type aliases of the form
//! `type SercomXPadY<Z> = Pad<SercomX, PadY, Z>`.
//!
//! The [`PadPin`] trait is kept intact, and it is the only way to construct
//! [`Pad`]s.

use core::marker::PhantomData;

use paste::paste;

use crate::gpio::v1;
use crate::gpio::v2::{self, AnyPin, PinId, PinMode};
use crate::gpio::Port;
use crate::typelevel::Sealed;

pub use crate::sercom::v2::*;

//==============================================================================
// Pad
//==============================================================================

/// A GPIO pin configured to act as a SERCOM pad
///
/// This type is actually just a wrapper around a [`v2::Pin`]. The `SercomXPadY`
/// types of the original, `v1` API are recreated as type aliases of the form
/// `type SercomXPadY<Z> = Pad<SercomX, PadY, Z>`.
pub struct Pad<S, N, P>
where
    S: Sercom,
    N: PadNum,
    P: AnyPin,
    v2::SpecificPin<P>: IsPad<Sercom = S, PadNum = N>,
{
    sercom: PhantomData<S>,
    padnum: PhantomData<N>,
    _pin: P,
}

/// Implement [`IsPad`] for the `v1` [`Pad`] type
///
/// This implementation helps simplify compatibility between `v1` and `v2` for
/// the existing, `v1` SERCOM peripheral modules.
impl<S, N, I, M> IsPad for Pad<S, N, v1::Pin<I, M>>
where
    S: Sercom,
    N: PadNum,
    I: PinId,
    M: PinMode,
    v2::Pin<I, M>: IsPad<Sercom = S, PadNum = N>,
{
    type Sercom = S;
    type PadNum = N;
}

//==============================================================================
// Pad aliases
//==============================================================================

macro_rules! pad_alias {
    ( $($Sercom:ty),+ ) => {
        $(
            pad_alias!($Sercom: Pad0);
            pad_alias!($Sercom: Pad1);
            pad_alias!($Sercom: Pad2);
            pad_alias!($Sercom: Pad3);
        )+
    };
    ($Sercom:ty: $Pad:ty) => {
        paste! {
            /// Represents a numbered pad for the associated sercom instance. The pad is
            /// generic over any pin, only the PadPin implementations in this the sercom
            /// module make sense.
            pub type [<$Sercom $Pad>]<P> = Pad<$Sercom, $Pad, P>;
        }
    }
}

pad_alias!(Sercom0, Sercom1);
#[cfg(any(feature = "samd21", feature = "min-samd51g"))]
pad_alias!(Sercom2, Sercom3);
#[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
pad_alias!(Sercom4, Sercom5);
#[cfg(feature = "min-samd51n")]
pad_alias!(Sercom6, Sercom7);

//==============================================================================
// Internal aliases
//==============================================================================

#[cfg(feature = "samd11")]
type PadMode<S, N, I> = <I as PadLookup<S, N>>::PinMode;

#[cfg(not(feature = "samd11"))]
type PadMode<S, I> = <I as PadLookup<S>>::PinMode;

#[cfg(feature = "samd11")]
type V1ConfiguredPin<S, N, I> = v1::Pin<I, PadMode<S, N, I>>;

#[cfg(not(feature = "samd11"))]
type V1ConfiguredPin<S, I> = v1::Pin<I, PadMode<S, I>>;

#[cfg(feature = "samd11")]
type V2ConfiguredPin<S, N, I> = v2::Pin<I, PadMode<S, N, I>>;

#[cfg(not(feature = "samd11"))]
type V2ConfiguredPin<S, I> = v2::Pin<I, PadMode<S, I>>;

//==============================================================================
// PadPin
//==============================================================================

/// The PadPin trait makes it more ergonomic to convert a pin into a Sercom pad.
/// You should not implement this trait for yourself; only the implementations
/// in the sercom module make sense.
pub trait PadPin<P>: Sealed {
    fn into_pad(self, port: &mut Port) -> P;
}

#[cfg(feature = "samd11")]
impl<S, N, I, M> PadPin<Pad<S, N, V1ConfiguredPin<S, N, I>>> for v1::Pin<I, M>
where
    S: Sercom,
    N: PadNum,
    I: PadLookup<S, N>,
    M: PinMode,
    V2ConfiguredPin<S, N, I>: IsPad<Sercom = S, PadNum = N>,
{
    #[inline]
    fn into_pad(self, _port: &mut Port) -> Pad<S, N, V1ConfiguredPin<S, N, I>> {
        let v2_pin = v2::Pin::<I, M>::from(self);
        let v1_configured_pin = v2_pin.into_mode().into();
        Pad {
            sercom: PhantomData,
            padnum: PhantomData,
            _pin: v1_configured_pin,
        }
    }
}

#[cfg(not(feature = "samd11"))]
impl<S, N, I, M> PadPin<Pad<S, N, V1ConfiguredPin<S, I>>> for v1::Pin<I, M>
where
    S: Sercom,
    N: PadNum,
    I: PadLookup<S>,
    M: PinMode,
    V2ConfiguredPin<S, I>: IsPad<Sercom = S, PadNum = N>,
{
    #[inline]
    fn into_pad(self, _port: &mut Port) -> Pad<S, N, V1ConfiguredPin<S, I>> {
        let v2_pin = v2::Pin::<I, M>::from(self);
        let v1_configured_pin = v2_pin.into_mode().into();
        Pad {
            sercom: PhantomData,
            padnum: PhantomData,
            _pin: v1_configured_pin,
        }
    }
}
