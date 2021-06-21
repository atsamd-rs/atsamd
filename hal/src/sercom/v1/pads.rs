//! Version 1 of the SERCOM pads module
//!
//! This module is a compatibility shim that allows existing code to use the new
//! [`sercom::v2`](crate::sercom::v2) module. This API will eventually be
//! deprecated and removed.
//!
//! To recreate the `v1` API with `v2` types, this module defines its own [Pad]
//! type. The `v1::Pad` type is actually just a wrapper around a [`v2::Pad`]
//! with modified type parameters. Where the `v2::Pad` takes a `PinId` as its
//! third type parameter, a `v1::Pad` takes a configured [`Pin`]. The
//! `SercomXPadY` types of the original, `v1` API are recreated as type aliases
//! of the form `type SercomXPadY<Z> = Pad<SercomX, PadY, Z>`.
//!
//! The [`PadPin`] trait is kept intact, but additional options are also
//! provided to create `v1::Pad`s. They can now be created directly from both
//! `v1` and `v2` `Pin`s and they can be converted [`From`]/[`Into`] `v2::Pad`s.

use core::marker::PhantomData;

use paste::paste;

use crate::gpio::v1::{IntoFunction, Pin};
use crate::gpio::v2::{AnyPin, PinMode, SpecificPinId};
use crate::gpio::Port;
use crate::typelevel::Sealed;

pub use crate::sercom::v2::{self, *};

/// A GPIO pin configured to act as a SERCOM pad
///
/// This type is actually just a wrapper around a [`v2::Pad`] with modified type
/// parameters. Where the `v2::Pad` takes a `PinId` as its third type parameter,
/// a `v1::Pad` takes a configured [`Pin`]. The `SercomXPadY` types of the
/// original, `v1` API are recreated as type aliases of the form
/// `type SercomXPadY<Z> = Pad<SercomX, PadY, Z>`.
pub struct Pad<S, N, P>
where
    S: Sercom,
    N: PadNum,
    P: AnyPin,
    P::Id: GetPadMode<S, N>,
{
    pad: v2::Pad<S, N, P::Id>,
    pin: PhantomData<P>,
}

macro_rules! define_pads {
    ( $($Sercom:ty),+ ) => {
        $(
            define_pads!($Sercom: Pad0);
            define_pads!($Sercom: Pad1);
            define_pads!($Sercom: Pad2);
            define_pads!($Sercom: Pad3);
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

define_pads!(Sercom0, Sercom1);
#[cfg(any(feature = "samd21", feature = "min-samd51g"))]
define_pads!(Sercom2, Sercom3);
#[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
define_pads!(Sercom4, Sercom5);
#[cfg(feature = "min-samd51n")]
define_pads!(Sercom6, Sercom7);

/// The PadPin trait makes it more ergonomic to convert a pin into a Sercom pad.
/// You should not implement this trait for yourself; only the implementations
/// in the sercom module make sense.
pub trait PadPin<P>: Sealed {
    fn into_pad(self, port: &mut Port) -> P;
}

impl<S, N, I, M> PadPin<Pad<S, N, Pin<I, I::Mode>>> for Pin<I, M>
where
    S: Sercom,
    N: PadNum,
    I: GetPadMode<S, N>,
    M: PinMode,
    Pin<I, M>: IntoFunction<Pin<I, I::Mode>>,
{
    fn into_pad(self, port: &mut Port) -> Pad<S, N, Pin<I, I::Mode>> {
        let pin: Pin<I, I::Mode> = self.into_function(port);
        Pad {
            pad: v2::Pad::new(pin.pin),
            pin: PhantomData,
        }
    }
}

type ConfiguredPin<S, N, P> = Pin<SpecificPinId<P>, PadMode<S, N, SpecificPinId<P>>>;

/// Convert any pin into a [`v1::Pad`](Pad)
impl<S, N, P> From<P> for Pad<S, N, ConfiguredPin<S, N, P>>
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
        v2::Pad::<S, N, P::Id>::from(pin).into()
    }
}

/// Convert from a [`v2::Pad`] to a [`v1::Pad`](Pad)
impl<S, N, I> From<v2::Pad<S, N, I>> for Pad<S, N, Pin<I, I::Mode>>
where
    S: Sercom,
    N: PadNum,
    I: GetPadMode<S, N>,
{
    fn from(pad: v2::Pad<S, N, I>) -> Self {
        Pad {
            pad,
            pin: PhantomData,
        }
    }
}

/// Convert from a [`v1::Pad`](Pad) to a [`v2::Pad`]
impl<S, N, I> From<Pad<S, N, Pin<I, I::Mode>>> for v2::Pad<S, N, I>
where
    S: Sercom,
    N: PadNum,
    I: GetPadMode<S, N>,
{
    fn from(pad: Pad<S, N, Pin<I, I::Mode>>) -> Self {
        pad.pad
    }
}
