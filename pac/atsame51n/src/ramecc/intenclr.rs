#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u8, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u8, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SINGLEE`"]
pub type SINGLEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINGLEE`"]
pub struct SINGLEE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLEE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DUALE`"]
pub type DUALE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUALE`"]
pub struct DUALE_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Single Bit ECC Error Interrupt Enable Clear"]
    #[inline(always)]
    pub fn singlee(&self) -> SINGLEE_R {
        SINGLEE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Dual Bit ECC Error Interrupt Enable Clear"]
    #[inline(always)]
    pub fn duale(&self) -> DUALE_R {
        DUALE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Bit ECC Error Interrupt Enable Clear"]
    #[inline(always)]
    pub fn singlee(&mut self) -> SINGLEE_W {
        SINGLEE_W { w: self }
    }
    #[doc = "Bit 1 - Dual Bit ECC Error Interrupt Enable Clear"]
    #[inline(always)]
    pub fn duale(&mut self) -> DUALE_W {
        DUALE_W { w: self }
    }
}
