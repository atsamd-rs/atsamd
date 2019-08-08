#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SERCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SERMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SERMODER {
    #[doc = "Receive"]
    RX,
    #[doc = "Transmit"]
    TX,
    #[doc = "Receive one PDM data on each serial clock edge"]
    PDM2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SERMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SERMODER::RX => 0,
            SERMODER::TX => 1,
            SERMODER::PDM2 => 2,
            SERMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SERMODER {
        match value {
            0 => SERMODER::RX,
            1 => SERMODER::TX,
            2 => SERMODER::PDM2,
            i => SERMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline]
    pub fn is_rx(&self) -> bool {
        *self == SERMODER::RX
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline]
    pub fn is_tx(&self) -> bool {
        *self == SERMODER::TX
    }
    #[doc = "Checks if the value of the field is `PDM2`"]
    #[inline]
    pub fn is_pdm2(&self) -> bool {
        *self == SERMODER::PDM2
    }
}
#[doc = "Possible values of the field `TXDEFAULT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDEFAULTR {
    #[doc = "Output Default Value is 0"]
    ZERO,
    #[doc = "Output Default Value is 1"]
    ONE,
    #[doc = "Output Default Value is high impedance"]
    HIZ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXDEFAULTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXDEFAULTR::ZERO => 0,
            TXDEFAULTR::ONE => 1,
            TXDEFAULTR::HIZ => 3,
            TXDEFAULTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXDEFAULTR {
        match value {
            0 => TXDEFAULTR::ZERO,
            1 => TXDEFAULTR::ONE,
            3 => TXDEFAULTR::HIZ,
            i => TXDEFAULTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == TXDEFAULTR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == TXDEFAULTR::ONE
    }
    #[doc = "Checks if the value of the field is `HIZ`"]
    #[inline]
    pub fn is_hiz(&self) -> bool {
        *self == TXDEFAULTR::HIZ
    }
}
#[doc = "Possible values of the field `TXSAME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSAMER {
    #[doc = "Zero data transmitted in case of underrun"]
    ZERO,
    #[doc = "Last data transmitted in case of underrun"]
    SAME,
}
impl TXSAMER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TXSAMER::ZERO => false,
            TXSAMER::SAME => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXSAMER {
        match value {
            false => TXSAMER::ZERO,
            true => TXSAMER::SAME,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == TXSAMER::ZERO
    }
    #[doc = "Checks if the value of the field is `SAME`"]
    #[inline]
    pub fn is_same(&self) -> bool {
        *self == TXSAMER::SAME
    }
}
#[doc = "Possible values of the field `CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSELR {
    #[doc = "Use Clock Unit 0"]
    CLK0,
    #[doc = "Use Clock Unit 1"]
    CLK1,
}
impl CLKSELR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CLKSELR::CLK0 => false,
            CLKSELR::CLK1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKSELR {
        match value {
            false => CLKSELR::CLK0,
            true => CLKSELR::CLK1,
        }
    }
    #[doc = "Checks if the value of the field is `CLK0`"]
    #[inline]
    pub fn is_clk0(&self) -> bool {
        *self == CLKSELR::CLK0
    }
    #[doc = "Checks if the value of the field is `CLK1`"]
    #[inline]
    pub fn is_clk1(&self) -> bool {
        *self == CLKSELR::CLK1
    }
}
#[doc = "Possible values of the field `SLOTADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOTADJR {
    #[doc = "Data is right adjusted in slot"]
    RIGHT,
    #[doc = "Data is left adjusted in slot"]
    LEFT,
}
impl SLOTADJR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SLOTADJR::RIGHT => false,
            SLOTADJR::LEFT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLOTADJR {
        match value {
            false => SLOTADJR::RIGHT,
            true => SLOTADJR::LEFT,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline]
    pub fn is_right(&self) -> bool {
        *self == SLOTADJR::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline]
    pub fn is_left(&self) -> bool {
        *self == SLOTADJR::LEFT
    }
}
#[doc = "Possible values of the field `DATASIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATASIZER {
    #[doc = "32 bits"]
    _32,
    #[doc = "24 bits"]
    _24,
    #[doc = "20 bits"]
    _20,
    #[doc = "18 bits"]
    _18,
    #[doc = "16 bits"]
    _16,
    #[doc = "16 bits compact stereo"]
    _16C,
    #[doc = "8 bits"]
    _8,
    #[doc = "8 bits compact stereo"]
    _8C,
}
impl DATASIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATASIZER::_32 => 0,
            DATASIZER::_24 => 1,
            DATASIZER::_20 => 2,
            DATASIZER::_18 => 3,
            DATASIZER::_16 => 4,
            DATASIZER::_16C => 5,
            DATASIZER::_8 => 6,
            DATASIZER::_8C => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATASIZER {
        match value {
            0 => DATASIZER::_32,
            1 => DATASIZER::_24,
            2 => DATASIZER::_20,
            3 => DATASIZER::_18,
            4 => DATASIZER::_16,
            5 => DATASIZER::_16C,
            6 => DATASIZER::_8,
            7 => DATASIZER::_8C,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == DATASIZER::_32
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline]
    pub fn is_24(&self) -> bool {
        *self == DATASIZER::_24
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline]
    pub fn is_20(&self) -> bool {
        *self == DATASIZER::_20
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline]
    pub fn is_18(&self) -> bool {
        *self == DATASIZER::_18
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == DATASIZER::_16
    }
    #[doc = "Checks if the value of the field is `_16C`"]
    #[inline]
    pub fn is_16c(&self) -> bool {
        *self == DATASIZER::_16C
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == DATASIZER::_8
    }
    #[doc = "Checks if the value of the field is `_8C`"]
    #[inline]
    pub fn is_8c(&self) -> bool {
        *self == DATASIZER::_8C
    }
}
#[doc = "Possible values of the field `WORDADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORDADJR {
    #[doc = "Data is right adjusted in word"]
    RIGHT,
    #[doc = "Data is left adjusted in word"]
    LEFT,
}
impl WORDADJR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WORDADJR::RIGHT => false,
            WORDADJR::LEFT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WORDADJR {
        match value {
            false => WORDADJR::RIGHT,
            true => WORDADJR::LEFT,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline]
    pub fn is_right(&self) -> bool {
        *self == WORDADJR::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline]
    pub fn is_left(&self) -> bool {
        *self == WORDADJR::LEFT
    }
}
#[doc = "Possible values of the field `EXTEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTENDR {
    #[doc = "Extend with zeroes"]
    ZERO,
    #[doc = "Extend with ones"]
    ONE,
    #[doc = "Extend with Most Significant Bit"]
    MSBIT,
    #[doc = "Extend with Least Significant Bit"]
    LSBIT,
}
impl EXTENDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTENDR::ZERO => 0,
            EXTENDR::ONE => 1,
            EXTENDR::MSBIT => 2,
            EXTENDR::LSBIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTENDR {
        match value {
            0 => EXTENDR::ZERO,
            1 => EXTENDR::ONE,
            2 => EXTENDR::MSBIT,
            3 => EXTENDR::LSBIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == EXTENDR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == EXTENDR::ONE
    }
    #[doc = "Checks if the value of the field is `MSBIT`"]
    #[inline]
    pub fn is_msbit(&self) -> bool {
        *self == EXTENDR::MSBIT
    }
    #[doc = "Checks if the value of the field is `LSBIT`"]
    #[inline]
    pub fn is_lsbit(&self) -> bool {
        *self == EXTENDR::LSBIT
    }
}
#[doc = "Possible values of the field `BITREV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITREVR {
    #[doc = "Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
    MSBIT,
    #[doc = "Transfer Data Least Significant Bit (LSB) first"]
    LSBIT,
}
impl BITREVR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BITREVR::MSBIT => false,
            BITREVR::LSBIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BITREVR {
        match value {
            false => BITREVR::MSBIT,
            true => BITREVR::LSBIT,
        }
    }
    #[doc = "Checks if the value of the field is `MSBIT`"]
    #[inline]
    pub fn is_msbit(&self) -> bool {
        *self == BITREVR::MSBIT
    }
    #[doc = "Checks if the value of the field is `LSBIT`"]
    #[inline]
    pub fn is_lsbit(&self) -> bool {
        *self == BITREVR::LSBIT
    }
}
#[doc = r" Value of the field"]
pub struct SLOTDIS0R {
    bits: bool,
}
impl SLOTDIS0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SLOTDIS1R {
    bits: bool,
}
impl SLOTDIS1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SLOTDIS2R {
    bits: bool,
}
impl SLOTDIS2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SLOTDIS3R {
    bits: bool,
}
impl SLOTDIS3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SLOTDIS4R {
    bits: bool,
}
impl SLOTDIS4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SLOTDIS5R {
    bits: bool,
}
impl SLOTDIS5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SLOTDIS6R {
    bits: bool,
}
impl SLOTDIS6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SLOTDIS7R {
    bits: bool,
}
impl SLOTDIS7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `MONO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOR {
    #[doc = "Normal mode"]
    STEREO,
    #[doc = "Left channel data is duplicated to right channel"]
    MONO,
}
impl MONOR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MONOR::STEREO => false,
            MONOR::MONO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONOR {
        match value {
            false => MONOR::STEREO,
            true => MONOR::MONO,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline]
    pub fn is_stereo(&self) -> bool {
        *self == MONOR::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline]
    pub fn is_mono(&self) -> bool {
        *self == MONOR::MONO
    }
}
#[doc = "Possible values of the field `DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR {
    #[doc = "Single DMA channel"]
    SINGLE,
    #[doc = "One DMA channel per data channel"]
    MULTIPLE,
}
impl DMAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DMAR::SINGLE => false,
            DMAR::MULTIPLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAR {
        match value {
            false => DMAR::SINGLE,
            true => DMAR::MULTIPLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == DMAR::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline]
    pub fn is_multiple(&self) -> bool {
        *self == DMAR::MULTIPLE
    }
}
#[doc = r" Value of the field"]
pub struct RXLOOPR {
    bits: bool,
}
impl RXLOOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `SERMODE`"]
pub enum SERMODEW {
    #[doc = "Receive"]
    RX,
    #[doc = "Transmit"]
    TX,
    #[doc = "Receive one PDM data on each serial clock edge"]
    PDM2,
}
impl SERMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SERMODEW::RX => 0,
            SERMODEW::TX => 1,
            SERMODEW::PDM2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SERMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SERMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SERMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Receive"]
    #[inline]
    pub fn rx(self) -> &'a mut W {
        self.variant(SERMODEW::RX)
    }
    #[doc = "Transmit"]
    #[inline]
    pub fn tx(self) -> &'a mut W {
        self.variant(SERMODEW::TX)
    }
    #[doc = "Receive one PDM data on each serial clock edge"]
    #[inline]
    pub fn pdm2(self) -> &'a mut W {
        self.variant(SERMODEW::PDM2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXDEFAULT`"]
pub enum TXDEFAULTW {
    #[doc = "Output Default Value is 0"]
    ZERO,
    #[doc = "Output Default Value is 1"]
    ONE,
    #[doc = "Output Default Value is high impedance"]
    HIZ,
}
impl TXDEFAULTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXDEFAULTW::ZERO => 0,
            TXDEFAULTW::ONE => 1,
            TXDEFAULTW::HIZ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDEFAULTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDEFAULTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDEFAULTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Output Default Value is 0"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(TXDEFAULTW::ZERO)
    }
    #[doc = "Output Default Value is 1"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(TXDEFAULTW::ONE)
    }
    #[doc = "Output Default Value is high impedance"]
    #[inline]
    pub fn hiz(self) -> &'a mut W {
        self.variant(TXDEFAULTW::HIZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSAME`"]
pub enum TXSAMEW {
    #[doc = "Zero data transmitted in case of underrun"]
    ZERO,
    #[doc = "Last data transmitted in case of underrun"]
    SAME,
}
impl TXSAMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSAMEW::ZERO => false,
            TXSAMEW::SAME => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSAMEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSAMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSAMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Zero data transmitted in case of underrun"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(TXSAMEW::ZERO)
    }
    #[doc = "Last data transmitted in case of underrun"]
    #[inline]
    pub fn same(self) -> &'a mut W {
        self.variant(TXSAMEW::SAME)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKSEL`"]
pub enum CLKSELW {
    #[doc = "Use Clock Unit 0"]
    CLK0,
    #[doc = "Use Clock Unit 1"]
    CLK1,
}
impl CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKSELW::CLK0 => false,
            CLKSELW::CLK1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use Clock Unit 0"]
    #[inline]
    pub fn clk0(self) -> &'a mut W {
        self.variant(CLKSELW::CLK0)
    }
    #[doc = "Use Clock Unit 1"]
    #[inline]
    pub fn clk1(self) -> &'a mut W {
        self.variant(CLKSELW::CLK1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLOTADJ`"]
pub enum SLOTADJW {
    #[doc = "Data is right adjusted in slot"]
    RIGHT,
    #[doc = "Data is left adjusted in slot"]
    LEFT,
}
impl SLOTADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLOTADJW::RIGHT => false,
            SLOTADJW::LEFT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLOTADJW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOTADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLOTADJW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is right adjusted in slot"]
    #[inline]
    pub fn right(self) -> &'a mut W {
        self.variant(SLOTADJW::RIGHT)
    }
    #[doc = "Data is left adjusted in slot"]
    #[inline]
    pub fn left(self) -> &'a mut W {
        self.variant(SLOTADJW::LEFT)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATASIZE`"]
pub enum DATASIZEW {
    #[doc = "32 bits"]
    _32,
    #[doc = "24 bits"]
    _24,
    #[doc = "20 bits"]
    _20,
    #[doc = "18 bits"]
    _18,
    #[doc = "16 bits"]
    _16,
    #[doc = "16 bits compact stereo"]
    _16C,
    #[doc = "8 bits"]
    _8,
    #[doc = "8 bits compact stereo"]
    _8C,
}
impl DATASIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATASIZEW::_32 => 0,
            DATASIZEW::_24 => 1,
            DATASIZEW::_20 => 2,
            DATASIZEW::_18 => 3,
            DATASIZEW::_16 => 4,
            DATASIZEW::_16C => 5,
            DATASIZEW::_8 => 6,
            DATASIZEW::_8C => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATASIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DATASIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATASIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "32 bits"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(DATASIZEW::_32)
    }
    #[doc = "24 bits"]
    #[inline]
    pub fn _24(self) -> &'a mut W {
        self.variant(DATASIZEW::_24)
    }
    #[doc = "20 bits"]
    #[inline]
    pub fn _20(self) -> &'a mut W {
        self.variant(DATASIZEW::_20)
    }
    #[doc = "18 bits"]
    #[inline]
    pub fn _18(self) -> &'a mut W {
        self.variant(DATASIZEW::_18)
    }
    #[doc = "16 bits"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(DATASIZEW::_16)
    }
    #[doc = "16 bits compact stereo"]
    #[inline]
    pub fn _16c(self) -> &'a mut W {
        self.variant(DATASIZEW::_16C)
    }
    #[doc = "8 bits"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(DATASIZEW::_8)
    }
    #[doc = "8 bits compact stereo"]
    #[inline]
    pub fn _8c(self) -> &'a mut W {
        self.variant(DATASIZEW::_8C)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WORDADJ`"]
pub enum WORDADJW {
    #[doc = "Data is right adjusted in word"]
    RIGHT,
    #[doc = "Data is left adjusted in word"]
    LEFT,
}
impl WORDADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WORDADJW::RIGHT => false,
            WORDADJW::LEFT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WORDADJW<'a> {
    w: &'a mut W,
}
impl<'a> _WORDADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WORDADJW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is right adjusted in word"]
    #[inline]
    pub fn right(self) -> &'a mut W {
        self.variant(WORDADJW::RIGHT)
    }
    #[doc = "Data is left adjusted in word"]
    #[inline]
    pub fn left(self) -> &'a mut W {
        self.variant(WORDADJW::LEFT)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTEND`"]
pub enum EXTENDW {
    #[doc = "Extend with zeroes"]
    ZERO,
    #[doc = "Extend with ones"]
    ONE,
    #[doc = "Extend with Most Significant Bit"]
    MSBIT,
    #[doc = "Extend with Least Significant Bit"]
    LSBIT,
}
impl EXTENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTENDW::ZERO => 0,
            EXTENDW::ONE => 1,
            EXTENDW::MSBIT => 2,
            EXTENDW::LSBIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTENDW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTENDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Extend with zeroes"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(EXTENDW::ZERO)
    }
    #[doc = "Extend with ones"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(EXTENDW::ONE)
    }
    #[doc = "Extend with Most Significant Bit"]
    #[inline]
    pub fn msbit(self) -> &'a mut W {
        self.variant(EXTENDW::MSBIT)
    }
    #[doc = "Extend with Least Significant Bit"]
    #[inline]
    pub fn lsbit(self) -> &'a mut W {
        self.variant(EXTENDW::LSBIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BITREV`"]
pub enum BITREVW {
    #[doc = "Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
    MSBIT,
    #[doc = "Transfer Data Least Significant Bit (LSB) first"]
    LSBIT,
}
impl BITREVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BITREVW::MSBIT => false,
            BITREVW::LSBIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BITREVW<'a> {
    w: &'a mut W,
}
impl<'a> _BITREVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BITREVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
    #[inline]
    pub fn msbit(self) -> &'a mut W {
        self.variant(BITREVW::MSBIT)
    }
    #[doc = "Transfer Data Least Significant Bit (LSB) first"]
    #[inline]
    pub fn lsbit(self) -> &'a mut W {
        self.variant(BITREVW::LSBIT)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLOTDIS0W<'a> {
    w: &'a mut W,
}
impl<'a> _SLOTDIS0W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLOTDIS1W<'a> {
    w: &'a mut W,
}
impl<'a> _SLOTDIS1W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLOTDIS2W<'a> {
    w: &'a mut W,
}
impl<'a> _SLOTDIS2W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLOTDIS3W<'a> {
    w: &'a mut W,
}
impl<'a> _SLOTDIS3W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLOTDIS4W<'a> {
    w: &'a mut W,
}
impl<'a> _SLOTDIS4W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLOTDIS5W<'a> {
    w: &'a mut W,
}
impl<'a> _SLOTDIS5W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLOTDIS6W<'a> {
    w: &'a mut W,
}
impl<'a> _SLOTDIS6W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLOTDIS7W<'a> {
    w: &'a mut W,
}
impl<'a> _SLOTDIS7W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONO`"]
pub enum MONOW {
    #[doc = "Normal mode"]
    STEREO,
    #[doc = "Left channel data is duplicated to right channel"]
    MONO,
}
impl MONOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONOW::STEREO => false,
            MONOW::MONO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONOW<'a> {
    w: &'a mut W,
}
impl<'a> _MONOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn stereo(self) -> &'a mut W {
        self.variant(MONOW::STEREO)
    }
    #[doc = "Left channel data is duplicated to right channel"]
    #[inline]
    pub fn mono(self) -> &'a mut W {
        self.variant(MONOW::MONO)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA`"]
pub enum DMAW {
    #[doc = "Single DMA channel"]
    SINGLE,
    #[doc = "One DMA channel per data channel"]
    MULTIPLE,
}
impl DMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAW::SINGLE => false,
            DMAW::MULTIPLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single DMA channel"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(DMAW::SINGLE)
    }
    #[doc = "One DMA channel per data channel"]
    #[inline]
    pub fn multiple(self) -> &'a mut W {
        self.variant(DMAW::MULTIPLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXLOOPW<'a> {
    w: &'a mut W,
}
impl<'a> _RXLOOPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Serializer Mode"]
    #[inline]
    pub fn sermode(&self) -> SERMODER {
        SERMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Line Default Line when Slot Disabled"]
    #[inline]
    pub fn txdefault(&self) -> TXDEFAULTR {
        TXDEFAULTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Transmit Data when Underrun"]
    #[inline]
    pub fn txsame(&self) -> TXSAMER {
        TXSAMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Clock Unit Selection"]
    #[inline]
    pub fn clksel(&self) -> CLKSELR {
        CLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Data Slot Formatting Adjust"]
    #[inline]
    pub fn slotadj(&self) -> SLOTADJR {
        SLOTADJR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Data Word Size"]
    #[inline]
    pub fn datasize(&self) -> DATASIZER {
        DATASIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Data Word Formatting Adjust"]
    #[inline]
    pub fn wordadj(&self) -> WORDADJR {
        WORDADJR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:14 - Data Formatting Bit Extension"]
    #[inline]
    pub fn extend(&self) -> EXTENDR {
        EXTENDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Data Formatting Bit Reverse"]
    #[inline]
    pub fn bitrev(&self) -> BITREVR {
        BITREVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Slot 0 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis0(&self) -> SLOTDIS0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLOTDIS0R { bits }
    }
    #[doc = "Bit 17 - Slot 1 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis1(&self) -> SLOTDIS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLOTDIS1R { bits }
    }
    #[doc = "Bit 18 - Slot 2 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis2(&self) -> SLOTDIS2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLOTDIS2R { bits }
    }
    #[doc = "Bit 19 - Slot 3 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis3(&self) -> SLOTDIS3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLOTDIS3R { bits }
    }
    #[doc = "Bit 20 - Slot 4 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis4(&self) -> SLOTDIS4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLOTDIS4R { bits }
    }
    #[doc = "Bit 21 - Slot 5 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis5(&self) -> SLOTDIS5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLOTDIS5R { bits }
    }
    #[doc = "Bit 22 - Slot 6 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis6(&self) -> SLOTDIS6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLOTDIS6R { bits }
    }
    #[doc = "Bit 23 - Slot 7 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis7(&self) -> SLOTDIS7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLOTDIS7R { bits }
    }
    #[doc = "Bit 24 - Mono Mode"]
    #[inline]
    pub fn mono(&self) -> MONOR {
        MONOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Single or Multiple DMA Channels"]
    #[inline]
    pub fn dma(&self) -> DMAR {
        DMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Loop-back Test Mode"]
    #[inline]
    pub fn rxloop(&self) -> RXLOOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXLOOPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Serializer Mode"]
    #[inline]
    pub fn sermode(&mut self) -> _SERMODEW {
        _SERMODEW { w: self }
    }
    #[doc = "Bits 2:3 - Line Default Line when Slot Disabled"]
    #[inline]
    pub fn txdefault(&mut self) -> _TXDEFAULTW {
        _TXDEFAULTW { w: self }
    }
    #[doc = "Bit 4 - Transmit Data when Underrun"]
    #[inline]
    pub fn txsame(&mut self) -> _TXSAMEW {
        _TXSAMEW { w: self }
    }
    #[doc = "Bit 5 - Clock Unit Selection"]
    #[inline]
    pub fn clksel(&mut self) -> _CLKSELW {
        _CLKSELW { w: self }
    }
    #[doc = "Bit 7 - Data Slot Formatting Adjust"]
    #[inline]
    pub fn slotadj(&mut self) -> _SLOTADJW {
        _SLOTADJW { w: self }
    }
    #[doc = "Bits 8:10 - Data Word Size"]
    #[inline]
    pub fn datasize(&mut self) -> _DATASIZEW {
        _DATASIZEW { w: self }
    }
    #[doc = "Bit 12 - Data Word Formatting Adjust"]
    #[inline]
    pub fn wordadj(&mut self) -> _WORDADJW {
        _WORDADJW { w: self }
    }
    #[doc = "Bits 13:14 - Data Formatting Bit Extension"]
    #[inline]
    pub fn extend(&mut self) -> _EXTENDW {
        _EXTENDW { w: self }
    }
    #[doc = "Bit 15 - Data Formatting Bit Reverse"]
    #[inline]
    pub fn bitrev(&mut self) -> _BITREVW {
        _BITREVW { w: self }
    }
    #[doc = "Bit 16 - Slot 0 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis0(&mut self) -> _SLOTDIS0W {
        _SLOTDIS0W { w: self }
    }
    #[doc = "Bit 17 - Slot 1 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis1(&mut self) -> _SLOTDIS1W {
        _SLOTDIS1W { w: self }
    }
    #[doc = "Bit 18 - Slot 2 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis2(&mut self) -> _SLOTDIS2W {
        _SLOTDIS2W { w: self }
    }
    #[doc = "Bit 19 - Slot 3 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis3(&mut self) -> _SLOTDIS3W {
        _SLOTDIS3W { w: self }
    }
    #[doc = "Bit 20 - Slot 4 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis4(&mut self) -> _SLOTDIS4W {
        _SLOTDIS4W { w: self }
    }
    #[doc = "Bit 21 - Slot 5 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis5(&mut self) -> _SLOTDIS5W {
        _SLOTDIS5W { w: self }
    }
    #[doc = "Bit 22 - Slot 6 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis6(&mut self) -> _SLOTDIS6W {
        _SLOTDIS6W { w: self }
    }
    #[doc = "Bit 23 - Slot 7 Disabled for this Serializer"]
    #[inline]
    pub fn slotdis7(&mut self) -> _SLOTDIS7W {
        _SLOTDIS7W { w: self }
    }
    #[doc = "Bit 24 - Mono Mode"]
    #[inline]
    pub fn mono(&mut self) -> _MONOW {
        _MONOW { w: self }
    }
    #[doc = "Bit 25 - Single or Multiple DMA Channels"]
    #[inline]
    pub fn dma(&mut self) -> _DMAW {
        _DMAW { w: self }
    }
    #[doc = "Bit 26 - Loop-back Test Mode"]
    #[inline]
    pub fn rxloop(&mut self) -> _RXLOOPW {
        _RXLOOPW { w: self }
    }
}
