//! Version 1 of the SERCOM pads module
//!
//! This module is a compatibility shim that allows existing code to use the new
//! [`sercom::v2`](crate::sercom::v2) module. This API will eventually be
//! deprecated and removed.
//!
//! To recreate the `v1` API with `v2` types, this module defines its own
//! [`Pad`] type, which is just a wrapper around a [`Pin`] configured as a
//! SERCOM pad. The `SercomXPadY` types of the original, `v1` API are recreated
//! as type aliases of the form
//!
//! ```
//! type SercomXPadY<Z> = Pad<SercomX, PadY, Z>
//! ```
//!
//! Use the [`PadPin`] trait to construct `Pad`s. The corresponding `Pin` can be
//! recovered using the [`free`] method.
//!
//! ```
//! use atsamd_hal::pac::Peripherals;
//! use atsamd_hal::gpio::v1::GpioExt;
//! use atsamd_hal::sercom::v1::{PadPin, Sercom0Pad0};
//!
//! let peripherals = Peripherals::take().unwrap();
//! let mut parts = peripherals.PORT.split();
//! let pad: Sercom0Pad0<_> = parts.pa8.into_pad(&mut parts.port);
//! let pin = pad.free();
//! ```
//!
//! [`free`]: Pad::free

#![allow(deprecated)]

use core::marker::PhantomData;

use paste::paste;

use crate::gpio::{self, AnyPin, PinId, PinMode};
use crate::sercom::v2::*;
use crate::typelevel::Sealed;

//==============================================================================
// IsPad
//==============================================================================

/// Extend implementations of [`IsPad`] from [`v2::Pin`]s to [`v1::Pin`]s
impl<I, M> IsPad for v1::Pin<I, M>
where
    I: PinId,
    M: PinMode,
    v1::Pin<I, M>: AnyPin,
    v2::Pin<I, M>: IsPad,
{
    type Sercom = <v2::Pin<I, M> as IsPad>::Sercom;
    type PadNum = <v2::Pin<I, M> as IsPad>::PadNum;
}

//==============================================================================
// Pad
//==============================================================================

/// A GPIO [`Pin`] configured to act as a SERCOM [`Pad`]
///
/// This type is just a wrapper around a correctly-configured [`Pin`]. The
/// `SercomXPadY` types of the original, `v1` API are recreated as type aliases
/// of the form
///
/// ```
/// type SercomXPadY<Z> = Pad<SercomX, PadY, Z>
/// ```
pub struct Pad<S, N, P>
where
    S: Sercom,
    N: PadNum,
    P: IsPad<Sercom = S, PadNum = N>,
{
    sercom: PhantomData<S>,
    padnum: PhantomData<N>,
    pin: P,
}

impl<S, N, P> Pad<S, N, P>
where
    S: Sercom,
    N: PadNum,
    P: IsPad<Sercom = S, PadNum = N>,
{
    /// Consume the [`Pad`] and recover the corresponding [`Pin`]
    #[inline]
    pub fn free(self) -> P {
        self.pin
    }
}

impl<S, N, P> Sealed for Pad<S, N, P>
where
    S: Sercom,
    N: PadNum,
    P: IsPad<Sercom = S, PadNum = N>,
{
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
// CompatiblePad
//==============================================================================

/// Type class to improve compatibility between `v1` and `v2` SERCOM pad types
///
/// The `sercom::v1::pads` module uses a wrapper [`Pad`] type to represent
/// SERCOM pads. The `v2::pad` module, on the other hand, does not use a
/// wrapper. Instead, it labels each correctly-configured [`v2::Pin`] with the
/// [`IsPad`] trait.
///
/// This trait forms a [type class] over both. It allows the [`v1::uart`],
/// [`v1::spi`] and [`v1::i2c`] modules to accept both `v1` and `v2` pad types.
///
/// [`v1::uart`]: super::uart
/// [`v1::spi`]: super::spi
/// [`v1::i2c`]: super::i2c
/// [type class]: crate::typelevel#type-classes
pub trait CompatiblePad: Sealed {
    type Sercom: Sercom;
    type PadNum: PadNum;
}

impl<S, N, P> CompatiblePad for Pad<S, N, P>
where
    S: Sercom,
    N: PadNum,
    P: IsPad<Sercom = S, PadNum = N>,
{
    type Sercom = S;
    type PadNum = N;
}

impl<P: IsPad> CompatiblePad for P {
    type Sercom = P::Sercom;
    type PadNum = P::PadNum;
}

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
impl<S, N, I, M> PadPin<Pad<S, N, Pin<I, I::PinMode>>> for Pin<I, M>
where
    S: Sercom,
    N: PadNum,
    I: GetPad<S, N>,
    M: PinMode,
    Pin<I, M>: IntoFunction<Pin<I, I::PinMode>>,
    Pin<I, I::PinMode>: IsPad<Sercom = S, PadNum = N>,
{
    #[inline]
    fn into_pad(self, port: &mut Port) -> Pad<S, N, Pin<I, I::PinMode>> {
        let pin = self.into_function(port);
        Pad {
            sercom: PhantomData,
            padnum: PhantomData,
            pin,
        }
    }
}

#[cfg(not(feature = "samd11"))]
impl<S, N, I, M> PadPin<Pad<S, N, Pin<I, I::PinMode>>> for Pin<I, M>
where
    S: Sercom,
    N: PadNum,
    I: GetPad<S>,
    M: PinMode,
    Pin<I, M>: IntoFunction<Pin<I, I::PinMode>>,
    Pin<I, I::PinMode>: IsPad<Sercom = S, PadNum = N>,
{
    #[inline]
    fn into_pad(self, port: &mut Port) -> Pad<S, N, Pin<I, I::PinMode>> {
        let pin = self.into_function(port);
        Pad {
            sercom: PhantomData,
            padnum: PhantomData,
            pin,
        }
    }
}
