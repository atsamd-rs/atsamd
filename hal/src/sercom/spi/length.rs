//! Define a trait to track the transaction [`Length`], which represents the
//! [`Config`] [`Size`] for SAMx5x chips
//!
//! This module defines the `Size` trait for SAMx5x chips. These chips always
//! operate in 32-bit extension mode and use the hardware `LENGTH` counter to
//! track the length of each transaction, in bytes. See the [`Length`]
//! documentation for more details.
//!
//! [`Config`]: super::Config
//! [`Size`]: super::Size

use core::num::NonZeroU8;

use num_traits::{AsPrimitive, PrimInt};
use seq_macro::seq;
use typenum::{Unsigned, U1, U2, U3, U4};

use crate::typelevel::Sealed;

use super::NonAtomicSize;

//=============================================================================
// Transaction length
//=============================================================================

/// Type-level enum representing the SPI transaction length, in bytes
///
/// This trait acts as both a [type-level enum], forming a type class for
/// transaction lengths, as well as a [type-level function] mapping the `Length`
/// to the corresponding [`Word`] size.
///
/// The SPI transaction length is represented in the type domain using
/// [`Unsigned`] types from the [`typenum`] crate. The length can be set
/// statically, using a length from `U1` to `U255`, or it can be set
/// dynamically, using the [`DynLength`] marker type. All valid `Length` types
/// are re-exported in this module.
///
/// The SPI transaction length affects the word size for the embedded HAL
/// traits, as well as other aspects of the SPI API. Transaction lengths of 1-4
/// only require a single read/write of the `DATA` register, so they have an
/// [`AtomicSize`] behave differently than longer transaction lengths.
///
/// [type-level enum]: crate::typelevel#type-level-enums
/// [type-level function]: crate::typelevel#type-level-functions
/// [`OpMode`]: super::OpMode
/// [`AtomicSize`]: super::AtomicSize
pub trait Length: Sealed + 'static {
    fn length(&self) -> u8;
    fn static_length() -> Option<NonZeroU8>;
    /// Word size for the transaction length
    ///
    /// For lengths 1-4, this type is `u8`, `u16` or `u32`. For longer
    /// transactions, this type is `[u8, Self::USIZE]`.
    type Word: PrimInt + AsPrimitive<u32>;
}

/// Type alias to recover the [`Word`](Length::Word) type from an
/// implementation of [`Length`]
pub type Word<L> = <L as Length>::Word;

/// Marker trait for statically known transaction [`Length`]s
pub trait StaticLength: Length + Unsigned {
    const LENGTH: u8;
}

macro_rules! impl_atomic_length {
    ( $( ($Unsigned: ty, $Word: ty) ),+ ) => {
        $(
            impl Length for $Unsigned {
                #[inline]
                fn length(&self) -> u8 {
                    Self::U8
                }
                #[inline]
                fn static_length() -> Option<NonZeroU8> {
                    NonZeroU8::new(Self::U8)
                }
                type Word = $Word;
            }
            impl StaticLength for $Unsigned {
                const LENGTH: u8 = Self::U8;
            }
        )+
    };
}

impl_atomic_length!((U1, u8), (U2, u16), (U3, u32), (U4, u32));

seq!(N in 5..=255 {
    impl Length for typenum::U~N {
        #[inline]
        fn length(&self) -> u8 {
            Self::U8
        }
        #[inline]
        fn static_length() -> Option<NonZeroU8> {
            NonZeroU8::new(Self::U8)
        }
        type Word = u8;
    }
    impl StaticLength for typenum::U~N {
        const LENGTH: u8 = Self::U8;
    }
});

/// Marker type for a run-time dynamic [`Length`]
pub struct DynLength(u8);

impl DynLength {
    pub(super) fn new(length: u8) -> Self {
        let length = if length > 0 { length } else { 1 };
        Self(length)
    }
}

impl Sealed for DynLength {}

impl Length for DynLength {
    #[inline]
    fn length(&self) -> u8 {
        self.0
    }
    #[inline]
    fn static_length() -> Option<NonZeroU8> {
        None
    }
    type Word = u8;
}

impl NonAtomicSize for DynLength {}
