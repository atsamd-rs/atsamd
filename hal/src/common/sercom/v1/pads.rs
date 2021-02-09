//! Version 1 of the SERCOM pads module
//!
//! This module is a compatibility shim that allows existing code to use the new
//! [`v2`](crate::sercom::v2) module. This API will eventually be deprecated and
//! removed.
//!
//! Implementing this compatibility shim is somewhat complicated. The `v1` API
//! created individual `SercomXPadY` types for each [`Sercom`]/[`PadNum`] pair.
//! However, the mapping from [`Sercom`] & [`PadNum`] to GPIO [`Pin`] is not
//! unique, so the `SercomXPadY` types still have to take a type parameter.
//! The type parameter represents the corresponding [`Pin`] type stored within
//! the `SercomXPadY` struct.
//!
//! The `v2` API takes a different approach. It defines a single, unified
//! [`Pad`] type that takes three type parameters. Marker types specify the
//! [`Sercom`] and [`PadNum`], and a [`Map`] trait handles the many-valued
//! conversion between GPIO [`Pin`] and SERCOM [`Pad`].
//!
//! This compatibility shim implements the `SercomXPadY` types as type aliases
//! of the form `type SercomXPadY<Z> = Pad<SercomX, PadY, Z>`. The approach
//! mostly works, but there is one catch.
//!
//! In SAMD51 & SAME5x chips, the [`Map`] trait is implemented on `IoSet` types,
//! while in SAMD11 & SAMD21 chips, it is implemented on [`PinId`]s.
//! Unfortunately, neither of these options works for the `v1` API. In the `v1`
//! API, the `Z` type parameter is a [`Pin`], not an `IoSet` or [`PinId`].
//!
//! The problem is solved by "lifting" the [`Map`] implementations from each
//! [`PinId`] to the corresponding [`Pin`] configured to act as a [`Pad`]. For
//! the SAMD51 & SAMDE5x case, extra implementations of [`Map`] are provided for
//! [`PinId`]s, over and above the implementations for `IoSet`s.
//!
//! Thus, the `v1` pad types share a type constructor ([`Pad`]) with the `v2`
//! pad types, but they are still distinct, because they use different types for
//! [`Map`]. For convenience, [`From`] & [`Into`] conversions are provided
//! between the two.

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

// Transfer all implementations of [`Map`] on [`PinId`]s to their corresponding
// configured [`Pin`] types
//
// This is required for `v1` compatibility. See the [`v1::pads`](self) module
// for more details.
impl<S, P, I> Map<S, P> for Pin<I, I::Mode>
where
    S: Sercom,
    P: PadNum,
    I: PinId + Map<S, P>,
{
    type Id = I;
    type Mode = I::Mode;
}

/// The PadPin trait makes it more ergonomic to convert a pin into a Sercom pad.
/// You should not implement this trait for yourself; only the implementations
/// in the sercom module make sense.
pub trait PadPin<T>: Sealed {
    fn into_pad(self, port: &mut Port) -> T;
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
/// [`Map`], while `v1` [`Pad`]s use configured [`Pin`]s.
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
/// [`Map`], while `v1` [`Pad`]s use configured [`Pin`]s.
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
