#[doc = "Reader of register CTRLC"]
pub type R = crate::R<u32, super::CTRLC>;
#[doc = "Writer for register CTRLC"]
pub type W = crate::W<u32, super::CTRLC>;
#[doc = "Register CTRLC `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICSPACE`"]
pub type ICSPACE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICSPACE`"]
pub struct ICSPACE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICSPACE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Data 32 Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA32B_A {
    #[doc = "0: Transaction from and to DATA register are 8-bit"]
    DATA_TRANS_8BIT = 0,
    #[doc = "1: Transaction from and to DATA register are 32-bit"]
    DATA_TRANS_32BIT = 1,
}
impl From<DATA32B_A> for bool {
    #[inline(always)]
    fn from(variant: DATA32B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATA32B`"]
pub type DATA32B_R = crate::R<bool, DATA32B_A>;
impl DATA32B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA32B_A {
        match self.bits {
            false => DATA32B_A::DATA_TRANS_8BIT,
            true => DATA32B_A::DATA_TRANS_32BIT,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_TRANS_8BIT`"]
    #[inline(always)]
    pub fn is_data_trans_8bit(&self) -> bool {
        *self == DATA32B_A::DATA_TRANS_8BIT
    }
    #[doc = "Checks if the value of the field is `DATA_TRANS_32BIT`"]
    #[inline(always)]
    pub fn is_data_trans_32bit(&self) -> bool {
        *self == DATA32B_A::DATA_TRANS_32BIT
    }
}
#[doc = "Write proxy for field `DATA32B`"]
pub struct DATA32B_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA32B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA32B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transaction from and to DATA register are 8-bit"]
    #[inline(always)]
    pub fn data_trans_8bit(self) -> &'a mut W {
        self.variant(DATA32B_A::DATA_TRANS_8BIT)
    }
    #[doc = "Transaction from and to DATA register are 32-bit"]
    #[inline(always)]
    pub fn data_trans_32bit(self) -> &'a mut W {
        self.variant(DATA32B_A::DATA_TRANS_32BIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Inter-Character Spacing"]
    #[inline(always)]
    pub fn icspace(&self) -> ICSPACE_R {
        ICSPACE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&self) -> DATA32B_R {
        DATA32B_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Inter-Character Spacing"]
    #[inline(always)]
    pub fn icspace(&mut self) -> ICSPACE_W {
        ICSPACE_W { w: self }
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&mut self) -> DATA32B_W {
        DATA32B_W { w: self }
    }
}
