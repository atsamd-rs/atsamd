//! Version 1 of the SERCOM pads module
//!
//! This module is a compatibility shim that allows existing code to use the new
//! [`v2`](crate::sercom::v2) module. This API will eventually be deprecated and
//! removed.
//!
//! In the previous API, a macro defined implementations of [`PadPin`] between
//! each valid pin/pad pair. And each pad type used a type parameter to indicate
//! the pin type stored within it.
//!
//! In the new API, the [`Map`] trait is used to map [`Pin`]s to [`Pad`]s, and
//! the stored [`Pin`] type is derived from it. To implement the old API using
//! the new API, we must transfer the implementations of [`Map`] to the
//! configured [`Pin`] types.
//!
//! The remaining documentation in this module comes from the original API.

use paste::paste;

use crate::gpio::v1::{IntoFunction, Pin};
use crate::gpio::v2::{PinId, PinMode};
use crate::gpio::Port;
use crate::typelevel::Sealed;

pub use crate::sercom::v2::pads::*;

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
pub trait PadPin<T>: Sealed {
    fn into_pad(self, port: &mut Port) -> T;
}

impl<I, M> Sealed for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
}

// Transfer `Map` from each `PinId` to its corresponding configured `Pin`
impl<S, P, I> Map<S, P> for Pin<I, I::Mode>
where
    S: Sercom,
    P: PadNum,
    I: PinId + Map<S, P>,
{
    type Id = I;
    type Mode = I::Mode;
}

impl<S, P, I, M> PadPin<Pad<S, P, Pin<I::Id, I::Mode>>> for Pin<I, M>
where
    S: Sercom,
    P: PadNum,
    I: PinId + Map<S, P>,
    M: PinMode,
    Pin<I, M>: IntoFunction<Pin<I::Id, I::Mode>>,
    Pin<I::Id, I::Mode>: Map<S, P, Id = I::Id, Mode = I::Mode>,
{
    #[allow(unused_variables)]
    fn into_pad(self, port: &mut Port) -> Pad<S, P, Pin<I::Id, I::Mode>> {
        let pin: Pin<I::Id, I::Mode> = self.into_function(port);
        Pad::new(pin.pin)
    }
}

/// Convert from a `v2` [`Pad`] to a `v1` [`Pad`]
///
/// The difference here is the [`Map`] type. `v2` [`Pad`]s use [`PinId`]s for
/// [`Map`], while `v1` [`Pad`]s use configured [`Pin`]s. This conversion is
/// usually only applicable to thumbv6m chips. The thumbv7em chips usually
/// implement [`Map`] on `IoSet`
impl<S, P, I> From<Pad<S, P, I>> for Pad<S, P, Pin<I::Id, I::Mode>>
where
    S: Sercom,
    P: PadNum,
    I: PinId + Map<S, P>,
    Pin<I::Id, I::Mode>: Map<S, P, Id = I::Id, Mode = I::Mode>,
{
    fn from(pad: Pad<S, P, I>) -> Pad<S, P, Pin<I::Id, I::Mode>> {
        Pad { pin: pad.pin }
    }
}

/// Convert from a `v1` [`Pad`] to a `v2` [`Pad`]
///
/// The difference here is the [`Map`] type. `v2` [`Pad`]s use [`PinId`]s for
/// [`Map`], while `v1` [`Pad`]s use configured [`Pin`]s. This conversion is
/// usually only applicable to thumbv6m chips. The thumbv7em chips usually
/// implement [`Map`] on `IoSet`.
impl<S, P, I> From<Pad<S, P, Pin<I::Id, I::Mode>>> for Pad<S, P, I>
where
    S: Sercom,
    P: PadNum,
    I: PinId + Map<S, P>,
    Pin<I::Id, I::Mode>: Map<S, P, Id = I::Id, Mode = I::Mode>,
{
    fn from(pad: Pad<S, P, Pin<I::Id, I::Mode>>) -> Pad<S, P, I> {
        Pad { pin: pad.pin }
    }
}
