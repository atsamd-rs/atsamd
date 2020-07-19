#[doc = "Reader of register AIRCR"]
pub type R = crate::R<u32, super::AIRCR>;
#[doc = "Writer for register AIRCR"]
pub type W = crate::W<u32, super::AIRCR>;
#[doc = "Register AIRCR `reset()`'s with value 0xfa05_0000"]
impl crate::ResetValue for super::AIRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfa05_0000
    }
}
#[doc = "Reader of field `VECTRESET`"]
pub type VECTRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VECTRESET`"]
pub struct VECTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `VECTCLRACTIVE`"]
pub type VECTCLRACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VECTCLRACTIVE`"]
pub struct VECTCLRACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTCLRACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRESETREQ_A {
    #[doc = "0: No system reset request"]
    VALUE_0 = 0,
    #[doc = "1: Asserts a signal to the outer system that requests a reset"]
    VALUE_1 = 1,
}
impl From<SYSRESETREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRESETREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSRESETREQ`"]
pub type SYSRESETREQ_R = crate::R<bool, SYSRESETREQ_A>;
impl SYSRESETREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRESETREQ_A {
        match self.bits {
            false => SYSRESETREQ_A::VALUE_0,
            true => SYSRESETREQ_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SYSRESETREQ_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SYSRESETREQ_A::VALUE_1
    }
}
#[doc = "Write proxy for field `SYSRESETREQ`"]
pub struct SYSRESETREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRESETREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRESETREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No system reset request"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SYSRESETREQ_A::VALUE_0)
    }
    #[doc = "Asserts a signal to the outer system that requests a reset"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(SYSRESETREQ_A::VALUE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PRIGROUP`"]
pub type PRIGROUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIGROUP`"]
pub struct PRIGROUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIGROUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Data endianness, 0=little, 1=big\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANNESS_A {
    #[doc = "0: Little-endian"]
    VALUE_0 = 0,
    #[doc = "1: Big-endian"]
    VALUE_1 = 1,
}
impl From<ENDIANNESS_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIANNESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDIANNESS`"]
pub type ENDIANNESS_R = crate::R<bool, ENDIANNESS_A>;
impl ENDIANNESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIANNESS_A {
        match self.bits {
            false => ENDIANNESS_A::VALUE_0,
            true => ENDIANNESS_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == ENDIANNESS_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == ENDIANNESS_A::VALUE_1
    }
}
#[doc = "Write proxy for field `ENDIANNESS`"]
pub struct ENDIANNESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIANNESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDIANNESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(ENDIANNESS_A::VALUE_0)
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(ENDIANNESS_A::VALUE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `VECTKEY`"]
pub type VECTKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VECTKEY`"]
pub struct VECTKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Must write 0"]
    #[inline(always)]
    pub fn vectreset(&self) -> VECTRESET_R {
        VECTRESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Must write 0"]
    #[inline(always)]
    pub fn vectclractive(&self) -> VECTCLRACTIVE_R {
        VECTCLRACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System Reset Request"]
    #[inline(always)]
    pub fn sysresetreq(&self) -> SYSRESETREQ_R {
        SYSRESETREQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping"]
    #[inline(always)]
    pub fn prigroup(&self) -> PRIGROUP_R {
        PRIGROUP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Data endianness, 0=little, 1=big"]
    #[inline(always)]
    pub fn endianness(&self) -> ENDIANNESS_R {
        ENDIANNESS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Must write 0"]
    #[inline(always)]
    pub fn vectreset(&mut self) -> VECTRESET_W {
        VECTRESET_W { w: self }
    }
    #[doc = "Bit 1 - Must write 0"]
    #[inline(always)]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W {
        VECTCLRACTIVE_W { w: self }
    }
    #[doc = "Bit 2 - System Reset Request"]
    #[inline(always)]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W {
        SYSRESETREQ_W { w: self }
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping"]
    #[inline(always)]
    pub fn prigroup(&mut self) -> PRIGROUP_W {
        PRIGROUP_W { w: self }
    }
    #[doc = "Bit 15 - Data endianness, 0=little, 1=big"]
    #[inline(always)]
    pub fn endianness(&mut self) -> ENDIANNESS_W {
        ENDIANNESS_W { w: self }
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&mut self) -> VECTKEY_W {
        VECTKEY_W { w: self }
    }
}
