//! Define a trait to track the [`CharSize`], which represents the [`Config`]
//! [`Size`] for SAMD11 and SAMD21 chips
//!
//! [`Config`]: super::Config
//! [`Size`]: super::Size

use num_traits::{AsPrimitive, PrimInt};

use crate::typelevel::Sealed;

//=============================================================================
// Character size
//=============================================================================

/// Type-level enum representing the SPI character size
///
/// This trait acts as both a [type-level enum], forming a type class for
/// character sizes, as well as a [type-level function] mapping the
/// corresponding word size.
///
/// The SPI character size affects the word size for the embedded HAL traits.
/// Eight-bit transactions use a `u8` word, while nine-bit transactions use a
/// `u16` word.
///
/// [type-level enum]: crate::typelevel#type-level-enums
/// [type-level function]: crate::typelevel#type-level-functions
pub trait CharSize: Sealed + Default {
    /// Word size for the character size
    type Word: PrimInt + AsPrimitive<u16>;

    /// Register bit pattern for the corresponding `CharSize`
    const BITS: u8;

    /// Number of bytes in an SPI transaction
    const BYTES: u8;
}

/// Type alias to recover the [`Word`](CharSize::Word) type from an
/// implementation of [`CharSize`]
pub type Word<C> = <C as CharSize>::Word;

/// [`CharSize`] variant for 8-bit transactions
#[derive(Default)]
pub struct EightBit;

/// [`CharSize`] variant for 9-bit transactions
#[derive(Default)]
pub struct NineBit;

impl Sealed for EightBit {}
impl CharSize for EightBit {
    type Word = u8;
    const BITS: u8 = 0;
    const BYTES: u8 = 1;
}

impl Sealed for NineBit {}
impl CharSize for NineBit {
    type Word = u16;
    const BITS: u8 = 1;
    const BYTES: u8 = 2;
}
