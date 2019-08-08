#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INSTRFRAME {
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
#[doc = "Possible values of the field `WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIDTHR {
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    SINGLE_BIT_SPI,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    DUAL_OUTPUT,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    QUAD_OUTPUT,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_IO,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_IO,
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_CMD,
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_CMD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WIDTHR::SINGLE_BIT_SPI => 0,
            WIDTHR::DUAL_OUTPUT => 1,
            WIDTHR::QUAD_OUTPUT => 2,
            WIDTHR::DUAL_IO => 3,
            WIDTHR::QUAD_IO => 4,
            WIDTHR::DUAL_CMD => 5,
            WIDTHR::QUAD_CMD => 6,
            WIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WIDTHR {
        match value {
            0 => WIDTHR::SINGLE_BIT_SPI,
            1 => WIDTHR::DUAL_OUTPUT,
            2 => WIDTHR::QUAD_OUTPUT,
            3 => WIDTHR::DUAL_IO,
            4 => WIDTHR::QUAD_IO,
            5 => WIDTHR::DUAL_CMD,
            6 => WIDTHR::QUAD_CMD,
            i => WIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_BIT_SPI`"]
    #[inline]
    pub fn is_single_bit_spi(&self) -> bool {
        *self == WIDTHR::SINGLE_BIT_SPI
    }
    #[doc = "Checks if the value of the field is `DUAL_OUTPUT`"]
    #[inline]
    pub fn is_dual_output(&self) -> bool {
        *self == WIDTHR::DUAL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `QUAD_OUTPUT`"]
    #[inline]
    pub fn is_quad_output(&self) -> bool {
        *self == WIDTHR::QUAD_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline]
    pub fn is_dual_io(&self) -> bool {
        *self == WIDTHR::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline]
    pub fn is_quad_io(&self) -> bool {
        *self == WIDTHR::QUAD_IO
    }
    #[doc = "Checks if the value of the field is `DUAL_CMD`"]
    #[inline]
    pub fn is_dual_cmd(&self) -> bool {
        *self == WIDTHR::DUAL_CMD
    }
    #[doc = "Checks if the value of the field is `QUAD_CMD`"]
    #[inline]
    pub fn is_quad_cmd(&self) -> bool {
        *self == WIDTHR::QUAD_CMD
    }
}
#[doc = r" Value of the field"]
pub struct INSTRENR {
    bits: bool,
}
impl INSTRENR {
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
pub struct ADDRENR {
    bits: bool,
}
impl ADDRENR {
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
pub struct OPTCODEENR {
    bits: bool,
}
impl OPTCODEENR {
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
pub struct DATAENR {
    bits: bool,
}
impl DATAENR {
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
#[doc = "Possible values of the field `OPTCODELEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTCODELENR {
    #[doc = "1-bit length option code"]
    _1BIT,
    #[doc = "2-bits length option code"]
    _2BITS,
    #[doc = "4-bits length option code"]
    _4BITS,
    #[doc = "8-bits length option code"]
    _8BITS,
}
impl OPTCODELENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPTCODELENR::_1BIT => 0,
            OPTCODELENR::_2BITS => 1,
            OPTCODELENR::_4BITS => 2,
            OPTCODELENR::_8BITS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPTCODELENR {
        match value {
            0 => OPTCODELENR::_1BIT,
            1 => OPTCODELENR::_2BITS,
            2 => OPTCODELENR::_4BITS,
            3 => OPTCODELENR::_8BITS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1BIT`"]
    #[inline]
    pub fn is_1bit(&self) -> bool {
        *self == OPTCODELENR::_1BIT
    }
    #[doc = "Checks if the value of the field is `_2BITS`"]
    #[inline]
    pub fn is_2bits(&self) -> bool {
        *self == OPTCODELENR::_2BITS
    }
    #[doc = "Checks if the value of the field is `_4BITS`"]
    #[inline]
    pub fn is_4bits(&self) -> bool {
        *self == OPTCODELENR::_4BITS
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline]
    pub fn is_8bits(&self) -> bool {
        *self == OPTCODELENR::_8BITS
    }
}
#[doc = "Possible values of the field `ADDRLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRLENR {
    #[doc = "24-bits address length"]
    _24BITS,
    #[doc = "32-bits address length"]
    _32BITS,
}
impl ADDRLENR {
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
            ADDRLENR::_24BITS => false,
            ADDRLENR::_32BITS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRLENR {
        match value {
            false => ADDRLENR::_24BITS,
            true => ADDRLENR::_32BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_24BITS`"]
    #[inline]
    pub fn is_24bits(&self) -> bool {
        *self == ADDRLENR::_24BITS
    }
    #[doc = "Checks if the value of the field is `_32BITS`"]
    #[inline]
    pub fn is_32bits(&self) -> bool {
        *self == ADDRLENR::_32BITS
    }
}
#[doc = "Possible values of the field `TFRTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFRTYPER {
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial flash memory is not possible."]
    READ,
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial flash memory is possible."]
    READMEMORY,
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    WRITE,
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    WRITEMEMORY,
}
impl TFRTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TFRTYPER::READ => 0,
            TFRTYPER::READMEMORY => 1,
            TFRTYPER::WRITE => 2,
            TFRTYPER::WRITEMEMORY => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TFRTYPER {
        match value {
            0 => TFRTYPER::READ,
            1 => TFRTYPER::READMEMORY,
            2 => TFRTYPER::WRITE,
            3 => TFRTYPER::WRITEMEMORY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == TFRTYPER::READ
    }
    #[doc = "Checks if the value of the field is `READMEMORY`"]
    #[inline]
    pub fn is_readmemory(&self) -> bool {
        *self == TFRTYPER::READMEMORY
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline]
    pub fn is_write(&self) -> bool {
        *self == TFRTYPER::WRITE
    }
    #[doc = "Checks if the value of the field is `WRITEMEMORY`"]
    #[inline]
    pub fn is_writememory(&self) -> bool {
        *self == TFRTYPER::WRITEMEMORY
    }
}
#[doc = r" Value of the field"]
pub struct CRMODER {
    bits: bool,
}
impl CRMODER {
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
pub struct DDRENR {
    bits: bool,
}
impl DDRENR {
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
pub struct DUMMYLENR {
    bits: u8,
}
impl DUMMYLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `WIDTH`"]
pub enum WIDTHW {
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    SINGLE_BIT_SPI,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    DUAL_OUTPUT,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    QUAD_OUTPUT,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_IO,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_IO,
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_CMD,
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_CMD,
}
impl WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WIDTHW::SINGLE_BIT_SPI => 0,
            WIDTHW::DUAL_OUTPUT => 1,
            WIDTHW::QUAD_OUTPUT => 2,
            WIDTHW::DUAL_IO => 3,
            WIDTHW::QUAD_IO => 4,
            WIDTHW::DUAL_CMD => 5,
            WIDTHW::QUAD_CMD => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    #[inline]
    pub fn single_bit_spi(self) -> &'a mut W {
        self.variant(WIDTHW::SINGLE_BIT_SPI)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    #[inline]
    pub fn dual_output(self) -> &'a mut W {
        self.variant(WIDTHW::DUAL_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    #[inline]
    pub fn quad_output(self) -> &'a mut W {
        self.variant(WIDTHW::QUAD_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(WIDTHW::DUAL_IO)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(WIDTHW::QUAD_IO)
    }
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline]
    pub fn dual_cmd(self) -> &'a mut W {
        self.variant(WIDTHW::DUAL_CMD)
    }
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline]
    pub fn quad_cmd(self) -> &'a mut W {
        self.variant(WIDTHW::QUAD_CMD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INSTRENW<'a> {
    w: &'a mut W,
}
impl<'a> _INSTRENW<'a> {
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
#[doc = r" Proxy"]
pub struct _ADDRENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRENW<'a> {
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
#[doc = r" Proxy"]
pub struct _OPTCODEENW<'a> {
    w: &'a mut W,
}
impl<'a> _OPTCODEENW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAENW<'a> {
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
#[doc = "Values that can be written to the field `OPTCODELEN`"]
pub enum OPTCODELENW {
    #[doc = "1-bit length option code"]
    _1BIT,
    #[doc = "2-bits length option code"]
    _2BITS,
    #[doc = "4-bits length option code"]
    _4BITS,
    #[doc = "8-bits length option code"]
    _8BITS,
}
impl OPTCODELENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPTCODELENW::_1BIT => 0,
            OPTCODELENW::_2BITS => 1,
            OPTCODELENW::_4BITS => 2,
            OPTCODELENW::_8BITS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPTCODELENW<'a> {
    w: &'a mut W,
}
impl<'a> _OPTCODELENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPTCODELENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1-bit length option code"]
    #[inline]
    pub fn _1bit(self) -> &'a mut W {
        self.variant(OPTCODELENW::_1BIT)
    }
    #[doc = "2-bits length option code"]
    #[inline]
    pub fn _2bits(self) -> &'a mut W {
        self.variant(OPTCODELENW::_2BITS)
    }
    #[doc = "4-bits length option code"]
    #[inline]
    pub fn _4bits(self) -> &'a mut W {
        self.variant(OPTCODELENW::_4BITS)
    }
    #[doc = "8-bits length option code"]
    #[inline]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(OPTCODELENW::_8BITS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADDRLEN`"]
pub enum ADDRLENW {
    #[doc = "24-bits address length"]
    _24BITS,
    #[doc = "32-bits address length"]
    _32BITS,
}
impl ADDRLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRLENW::_24BITS => false,
            ADDRLENW::_32BITS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRLENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRLENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "24-bits address length"]
    #[inline]
    pub fn _24bits(self) -> &'a mut W {
        self.variant(ADDRLENW::_24BITS)
    }
    #[doc = "32-bits address length"]
    #[inline]
    pub fn _32bits(self) -> &'a mut W {
        self.variant(ADDRLENW::_32BITS)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TFRTYPE`"]
pub enum TFRTYPEW {
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial flash memory is not possible."]
    READ,
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial flash memory is possible."]
    READMEMORY,
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    WRITE,
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    WRITEMEMORY,
}
impl TFRTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TFRTYPEW::READ => 0,
            TFRTYPEW::READMEMORY => 1,
            TFRTYPEW::WRITE => 2,
            TFRTYPEW::WRITEMEMORY => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFRTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TFRTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFRTYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial flash memory is not possible."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(TFRTYPEW::READ)
    }
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial flash memory is possible."]
    #[inline]
    pub fn readmemory(self) -> &'a mut W {
        self.variant(TFRTYPEW::READMEMORY)
    }
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    #[inline]
    pub fn write(self) -> &'a mut W {
        self.variant(TFRTYPEW::WRITE)
    }
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    #[inline]
    pub fn writememory(self) -> &'a mut W {
        self.variant(TFRTYPEW::WRITEMEMORY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CRMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CRMODEW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DDRENW<'a> {
    w: &'a mut W,
}
impl<'a> _DDRENW<'a> {
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
pub struct _DUMMYLENW<'a> {
    w: &'a mut W,
}
impl<'a> _DUMMYLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:2 - Instruction Code, Address, Option Code and Data Width"]
    #[inline]
    pub fn width(&self) -> WIDTHR {
        WIDTHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline]
    pub fn instren(&self) -> INSTRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INSTRENR { bits }
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline]
    pub fn addren(&self) -> ADDRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADDRENR { bits }
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline]
    pub fn optcodeen(&self) -> OPTCODEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPTCODEENR { bits }
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline]
    pub fn dataen(&self) -> DATAENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATAENR { bits }
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline]
    pub fn optcodelen(&self) -> OPTCODELENR {
        OPTCODELENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline]
    pub fn addrlen(&self) -> ADDRLENR {
        ADDRLENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline]
    pub fn tfrtype(&self) -> TFRTYPER {
        TFRTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline]
    pub fn crmode(&self) -> CRMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CRMODER { bits }
    }
    #[doc = "Bit 15 - Double Data Rate Enable"]
    #[inline]
    pub fn ddren(&self) -> DDRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDRENR { bits }
    }
    #[doc = "Bits 16:20 - Dummy Cycles Length"]
    #[inline]
    pub fn dummylen(&self) -> DUMMYLENR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DUMMYLENR { bits }
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
    #[doc = "Bits 0:2 - Instruction Code, Address, Option Code and Data Width"]
    #[inline]
    pub fn width(&mut self) -> _WIDTHW {
        _WIDTHW { w: self }
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline]
    pub fn instren(&mut self) -> _INSTRENW {
        _INSTRENW { w: self }
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline]
    pub fn addren(&mut self) -> _ADDRENW {
        _ADDRENW { w: self }
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline]
    pub fn optcodeen(&mut self) -> _OPTCODEENW {
        _OPTCODEENW { w: self }
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline]
    pub fn dataen(&mut self) -> _DATAENW {
        _DATAENW { w: self }
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline]
    pub fn optcodelen(&mut self) -> _OPTCODELENW {
        _OPTCODELENW { w: self }
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline]
    pub fn addrlen(&mut self) -> _ADDRLENW {
        _ADDRLENW { w: self }
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline]
    pub fn tfrtype(&mut self) -> _TFRTYPEW {
        _TFRTYPEW { w: self }
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline]
    pub fn crmode(&mut self) -> _CRMODEW {
        _CRMODEW { w: self }
    }
    #[doc = "Bit 15 - Double Data Rate Enable"]
    #[inline]
    pub fn ddren(&mut self) -> _DDRENW {
        _DDRENW { w: self }
    }
    #[doc = "Bits 16:20 - Dummy Cycles Length"]
    #[inline]
    pub fn dummylen(&mut self) -> _DUMMYLENW {
        _DUMMYLENW { w: self }
    }
}
