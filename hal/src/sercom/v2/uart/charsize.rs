//! Character size definitions

use super::DataReg;
use crate::typelevel::Sealed;
use num_traits::{AsPrimitive, PrimInt};

/// Type-level `enum` representing the UART character size
///
/// The UART character size affects the word size for the embedded HAL traits.
/// Eight or less bit transactions use a `u8` word, while nine-bit transactions
/// use a `u16` word.
pub trait CharSize: Sealed {
    /// Word size for the character size
    type Word: 'static + PrimInt + AsPrimitive<DataReg>;
}

/// Type-level `enum` indicating a [`CharSize`] that is not dynamic
pub trait FixedCharSize: CharSize {
    /// Bits to write into the `LENGTH` register
    const SIZE: CharSizeEnum;
}

/// Type alias to recover the `Word` type from an implementation of [`CharSize`]
pub type Word<C> = <C as CharSize>::Word;

/// [`CharSize`] variant for 5-bit transactions
pub enum FiveBit {}

/// [`CharSize`] variant for 6-bit transactions
pub enum SixBit {}

/// [`CharSize`] variant for 7-bit transactions
pub enum SevenBit {}

/// [`CharSize`] variant for 8-bit transactions
pub enum EightBit {}

/// [`CharSize`] variant for 9-bit transactions
pub enum NineBit {}

/// Dynamic [`CharSize`] that can be changed on the fly
pub enum DynCharSize {}

/// `enum` version of [`CharSize`]
#[repr(u8)]
pub enum CharSizeEnum {
    FiveBit = 0x5,
    SixBit = 0x6,
    SevenBit = 0x7,
    EightBit = 0x0,
    NineBit = 0x1,
}

impl Sealed for FiveBit {}
impl CharSize for FiveBit {
    type Word = u8;
}
impl FixedCharSize for FiveBit {
    const SIZE: CharSizeEnum = CharSizeEnum::FiveBit;
}

impl Sealed for SixBit {}
impl CharSize for SixBit {
    type Word = u8;
}
impl FixedCharSize for SixBit {
    const SIZE: CharSizeEnum = CharSizeEnum::SixBit;
}

impl Sealed for SevenBit {}
impl CharSize for SevenBit {
    type Word = u8;
}
impl FixedCharSize for SevenBit {
    const SIZE: CharSizeEnum = CharSizeEnum::SevenBit;
}

impl Sealed for EightBit {}
impl CharSize for EightBit {
    type Word = u8;
}
impl FixedCharSize for EightBit {
    const SIZE: CharSizeEnum = CharSizeEnum::EightBit;
}

impl Sealed for NineBit {}
impl CharSize for NineBit {
    type Word = u16;
}
impl FixedCharSize for NineBit {
    const SIZE: CharSizeEnum = CharSizeEnum::NineBit;
}

impl Sealed for DynCharSize {}
impl CharSize for DynCharSize {
    type Word = u16;
}
