#[doc = "Reader of register INSTRFRAME"]
pub type R = crate::R<u32, super::INSTRFRAME>;
#[doc = "Writer for register INSTRFRAME"]
pub type W = crate::W<u32, super::INSTRFRAME>;
#[doc = "Register INSTRFRAME `reset()`'s with value 0"]
impl crate::ResetValue for super::INSTRFRAME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Instruction Code, Address, Option Code and Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WIDTH_A {
    #[doc = "0: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    SINGLE_BIT_SPI = 0,
    #[doc = "1: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    DUAL_OUTPUT = 1,
    #[doc = "2: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    QUAD_OUTPUT = 2,
    #[doc = "3: Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_IO = 3,
    #[doc = "4: Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_IO = 4,
    #[doc = "5: Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_CMD = 5,
    #[doc = "6: Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_CMD = 6,
}
impl From<WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WIDTH`"]
pub type WIDTH_R = crate::R<u8, WIDTH_A>;
impl WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WIDTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WIDTH_A::SINGLE_BIT_SPI),
            1 => Val(WIDTH_A::DUAL_OUTPUT),
            2 => Val(WIDTH_A::QUAD_OUTPUT),
            3 => Val(WIDTH_A::DUAL_IO),
            4 => Val(WIDTH_A::QUAD_IO),
            5 => Val(WIDTH_A::DUAL_CMD),
            6 => Val(WIDTH_A::QUAD_CMD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_BIT_SPI`"]
    #[inline(always)]
    pub fn is_single_bit_spi(&self) -> bool {
        *self == WIDTH_A::SINGLE_BIT_SPI
    }
    #[doc = "Checks if the value of the field is `DUAL_OUTPUT`"]
    #[inline(always)]
    pub fn is_dual_output(&self) -> bool {
        *self == WIDTH_A::DUAL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `QUAD_OUTPUT`"]
    #[inline(always)]
    pub fn is_quad_output(&self) -> bool {
        *self == WIDTH_A::QUAD_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        *self == WIDTH_A::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        *self == WIDTH_A::QUAD_IO
    }
    #[doc = "Checks if the value of the field is `DUAL_CMD`"]
    #[inline(always)]
    pub fn is_dual_cmd(&self) -> bool {
        *self == WIDTH_A::DUAL_CMD
    }
    #[doc = "Checks if the value of the field is `QUAD_CMD`"]
    #[inline(always)]
    pub fn is_quad_cmd(&self) -> bool {
        *self == WIDTH_A::QUAD_CMD
    }
}
#[doc = "Write proxy for field `WIDTH`"]
pub struct WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    #[inline(always)]
    pub fn single_bit_spi(self) -> &'a mut W {
        self.variant(WIDTH_A::SINGLE_BIT_SPI)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_output(self) -> &'a mut W {
        self.variant(WIDTH_A::DUAL_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_output(self) -> &'a mut W {
        self.variant(WIDTH_A::QUAD_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(WIDTH_A::DUAL_IO)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(WIDTH_A::QUAD_IO)
    }
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_cmd(self) -> &'a mut W {
        self.variant(WIDTH_A::DUAL_CMD)
    }
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_cmd(self) -> &'a mut W {
        self.variant(WIDTH_A::QUAD_CMD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `INSTREN`"]
pub type INSTREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INSTREN`"]
pub struct INSTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTREN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADDREN`"]
pub type ADDREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDREN`"]
pub struct ADDREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDREN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `OPTCODEEN`"]
pub type OPTCODEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPTCODEEN`"]
pub struct OPTCODEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTCODEEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DATAEN`"]
pub type DATAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAEN`"]
pub struct DATAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Option Code Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPTCODELEN_A {
    #[doc = "0: 1-bit length option code"]
    _1BIT = 0,
    #[doc = "1: 2-bits length option code"]
    _2BITS = 1,
    #[doc = "2: 4-bits length option code"]
    _4BITS = 2,
    #[doc = "3: 8-bits length option code"]
    _8BITS = 3,
}
impl From<OPTCODELEN_A> for u8 {
    #[inline(always)]
    fn from(variant: OPTCODELEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPTCODELEN`"]
pub type OPTCODELEN_R = crate::R<u8, OPTCODELEN_A>;
impl OPTCODELEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPTCODELEN_A {
        match self.bits {
            0 => OPTCODELEN_A::_1BIT,
            1 => OPTCODELEN_A::_2BITS,
            2 => OPTCODELEN_A::_4BITS,
            3 => OPTCODELEN_A::_8BITS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1BIT`"]
    #[inline(always)]
    pub fn is_1bit(&self) -> bool {
        *self == OPTCODELEN_A::_1BIT
    }
    #[doc = "Checks if the value of the field is `_2BITS`"]
    #[inline(always)]
    pub fn is_2bits(&self) -> bool {
        *self == OPTCODELEN_A::_2BITS
    }
    #[doc = "Checks if the value of the field is `_4BITS`"]
    #[inline(always)]
    pub fn is_4bits(&self) -> bool {
        *self == OPTCODELEN_A::_4BITS
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == OPTCODELEN_A::_8BITS
    }
}
#[doc = "Write proxy for field `OPTCODELEN`"]
pub struct OPTCODELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTCODELEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPTCODELEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1-bit length option code"]
    #[inline(always)]
    pub fn _1bit(self) -> &'a mut W {
        self.variant(OPTCODELEN_A::_1BIT)
    }
    #[doc = "2-bits length option code"]
    #[inline(always)]
    pub fn _2bits(self) -> &'a mut W {
        self.variant(OPTCODELEN_A::_2BITS)
    }
    #[doc = "4-bits length option code"]
    #[inline(always)]
    pub fn _4bits(self) -> &'a mut W {
        self.variant(OPTCODELEN_A::_4BITS)
    }
    #[doc = "8-bits length option code"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(OPTCODELEN_A::_8BITS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Address Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRLEN_A {
    #[doc = "0: 24-bits address length"]
    _24BITS = 0,
    #[doc = "1: 32-bits address length"]
    _32BITS = 1,
}
impl From<ADDRLEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRLEN`"]
pub type ADDRLEN_R = crate::R<bool, ADDRLEN_A>;
impl ADDRLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRLEN_A {
        match self.bits {
            false => ADDRLEN_A::_24BITS,
            true => ADDRLEN_A::_32BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_24BITS`"]
    #[inline(always)]
    pub fn is_24bits(&self) -> bool {
        *self == ADDRLEN_A::_24BITS
    }
    #[doc = "Checks if the value of the field is `_32BITS`"]
    #[inline(always)]
    pub fn is_32bits(&self) -> bool {
        *self == ADDRLEN_A::_32BITS
    }
}
#[doc = "Write proxy for field `ADDRLEN`"]
pub struct ADDRLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "24-bits address length"]
    #[inline(always)]
    pub fn _24bits(self) -> &'a mut W {
        self.variant(ADDRLEN_A::_24BITS)
    }
    #[doc = "32-bits address length"]
    #[inline(always)]
    pub fn _32bits(self) -> &'a mut W {
        self.variant(ADDRLEN_A::_32BITS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Data Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TFRTYPE_A {
    #[doc = "0: Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial flash memory is not possible."]
    READ = 0,
    #[doc = "1: Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial flash memory is possible."]
    READMEMORY = 1,
    #[doc = "2: Write transfer into the serial memory.Scrambling is not performed."]
    WRITE = 2,
    #[doc = "3: Write data transfer into the serial memory.If enabled, scrambling is performed."]
    WRITEMEMORY = 3,
}
impl From<TFRTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TFRTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TFRTYPE`"]
pub type TFRTYPE_R = crate::R<u8, TFRTYPE_A>;
impl TFRTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFRTYPE_A {
        match self.bits {
            0 => TFRTYPE_A::READ,
            1 => TFRTYPE_A::READMEMORY,
            2 => TFRTYPE_A::WRITE,
            3 => TFRTYPE_A::WRITEMEMORY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == TFRTYPE_A::READ
    }
    #[doc = "Checks if the value of the field is `READMEMORY`"]
    #[inline(always)]
    pub fn is_readmemory(&self) -> bool {
        *self == TFRTYPE_A::READMEMORY
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == TFRTYPE_A::WRITE
    }
    #[doc = "Checks if the value of the field is `WRITEMEMORY`"]
    #[inline(always)]
    pub fn is_writememory(&self) -> bool {
        *self == TFRTYPE_A::WRITEMEMORY
    }
}
#[doc = "Write proxy for field `TFRTYPE`"]
pub struct TFRTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFRTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFRTYPE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial flash memory is not possible."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(TFRTYPE_A::READ)
    }
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial flash memory is possible."]
    #[inline(always)]
    pub fn readmemory(self) -> &'a mut W {
        self.variant(TFRTYPE_A::READMEMORY)
    }
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(TFRTYPE_A::WRITE)
    }
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    #[inline(always)]
    pub fn writememory(self) -> &'a mut W {
        self.variant(TFRTYPE_A::WRITEMEMORY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `CRMODE`"]
pub type CRMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRMODE`"]
pub struct CRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRMODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DDREN`"]
pub type DDREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDREN`"]
pub struct DDREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDREN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DUMMYLEN`"]
pub type DUMMYLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DUMMYLEN`"]
pub struct DUMMYLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DUMMYLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Instruction Code, Address, Option Code and Data Width"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    pub fn instren(&self) -> INSTREN_R {
        INSTREN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    pub fn addren(&self) -> ADDREN_R {
        ADDREN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    pub fn optcodeen(&self) -> OPTCODEEN_R {
        OPTCODEEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    pub fn dataen(&self) -> DATAEN_R {
        DATAEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    pub fn optcodelen(&self) -> OPTCODELEN_R {
        OPTCODELEN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    pub fn addrlen(&self) -> ADDRLEN_R {
        ADDRLEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    pub fn tfrtype(&self) -> TFRTYPE_R {
        TFRTYPE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    pub fn crmode(&self) -> CRMODE_R {
        CRMODE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Double Data Rate Enable"]
    #[inline(always)]
    pub fn ddren(&self) -> DDREN_R {
        DDREN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Dummy Cycles Length"]
    #[inline(always)]
    pub fn dummylen(&self) -> DUMMYLEN_R {
        DUMMYLEN_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Instruction Code, Address, Option Code and Data Width"]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W {
        WIDTH_W { w: self }
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    pub fn instren(&mut self) -> INSTREN_W {
        INSTREN_W { w: self }
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    pub fn addren(&mut self) -> ADDREN_W {
        ADDREN_W { w: self }
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    pub fn optcodeen(&mut self) -> OPTCODEEN_W {
        OPTCODEEN_W { w: self }
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    pub fn dataen(&mut self) -> DATAEN_W {
        DATAEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    pub fn optcodelen(&mut self) -> OPTCODELEN_W {
        OPTCODELEN_W { w: self }
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    pub fn addrlen(&mut self) -> ADDRLEN_W {
        ADDRLEN_W { w: self }
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    pub fn tfrtype(&mut self) -> TFRTYPE_W {
        TFRTYPE_W { w: self }
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    pub fn crmode(&mut self) -> CRMODE_W {
        CRMODE_W { w: self }
    }
    #[doc = "Bit 15 - Double Data Rate Enable"]
    #[inline(always)]
    pub fn ddren(&mut self) -> DDREN_W {
        DDREN_W { w: self }
    }
    #[doc = "Bits 16:20 - Dummy Cycles Length"]
    #[inline(always)]
    pub fn dummylen(&mut self) -> DUMMYLEN_W {
        DUMMYLEN_W { w: self }
    }
}
