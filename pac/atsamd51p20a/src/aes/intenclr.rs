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
#[doc = "Reader of field `ENCCMP`"]
pub type ENCCMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENCCMP`"]
pub struct ENCCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCCMP_W<'a> {
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
#[doc = "Reader of field `GFMCMP`"]
pub type GFMCMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GFMCMP`"]
pub struct GFMCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> GFMCMP_W<'a> {
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
    #[doc = "Bit 0 - Encryption Complete Interrupt Enable"]
    #[inline(always)]
    pub fn enccmp(&self) -> ENCCMP_R {
        ENCCMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GF Multiplication Complete Interrupt Enable"]
    #[inline(always)]
    pub fn gfmcmp(&self) -> GFMCMP_R {
        GFMCMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Encryption Complete Interrupt Enable"]
    #[inline(always)]
    pub fn enccmp(&mut self) -> ENCCMP_W {
        ENCCMP_W { w: self }
    }
    #[doc = "Bit 1 - GF Multiplication Complete Interrupt Enable"]
    #[inline(always)]
    pub fn gfmcmp(&mut self) -> GFMCMP_W {
        GFMCMP_W { w: self }
    }
}
