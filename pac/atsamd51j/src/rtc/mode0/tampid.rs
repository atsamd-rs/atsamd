#[doc = "Reader of register TAMPID"]
pub type R = crate::R<u32, super::TAMPID>;
#[doc = "Writer for register TAMPID"]
pub type W = crate::W<u32, super::TAMPID>;
#[doc = "Register TAMPID `reset()`'s with value 0"]
impl crate::ResetValue for super::TAMPID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAMPID0`"]
pub type TAMPID0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMPID0`"]
pub struct TAMPID0_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPID0_W<'a> {
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
#[doc = "Reader of field `TAMPID1`"]
pub type TAMPID1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMPID1`"]
pub struct TAMPID1_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPID1_W<'a> {
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
#[doc = "Reader of field `TAMPID2`"]
pub type TAMPID2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMPID2`"]
pub struct TAMPID2_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPID2_W<'a> {
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
#[doc = "Reader of field `TAMPID3`"]
pub type TAMPID3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMPID3`"]
pub struct TAMPID3_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPID3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TAMPID4`"]
pub type TAMPID4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMPID4`"]
pub struct TAMPID4_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPID4_W<'a> {
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
#[doc = "Reader of field `TAMPEVT`"]
pub type TAMPEVT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMPEVT`"]
pub struct TAMPEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPEVT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Tamper Input 0 Detected"]
    #[inline(always)]
    pub fn tampid0(&self) -> TAMPID0_R {
        TAMPID0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tamper Input 1 Detected"]
    #[inline(always)]
    pub fn tampid1(&self) -> TAMPID1_R {
        TAMPID1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tamper Input 2 Detected"]
    #[inline(always)]
    pub fn tampid2(&self) -> TAMPID2_R {
        TAMPID2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Tamper Input 3 Detected"]
    #[inline(always)]
    pub fn tampid3(&self) -> TAMPID3_R {
        TAMPID3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Tamper Input 4 Detected"]
    #[inline(always)]
    pub fn tampid4(&self) -> TAMPID4_R {
        TAMPID4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Tamper Event Detected"]
    #[inline(always)]
    pub fn tampevt(&self) -> TAMPEVT_R {
        TAMPEVT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper Input 0 Detected"]
    #[inline(always)]
    pub fn tampid0(&mut self) -> TAMPID0_W {
        TAMPID0_W { w: self }
    }
    #[doc = "Bit 1 - Tamper Input 1 Detected"]
    #[inline(always)]
    pub fn tampid1(&mut self) -> TAMPID1_W {
        TAMPID1_W { w: self }
    }
    #[doc = "Bit 2 - Tamper Input 2 Detected"]
    #[inline(always)]
    pub fn tampid2(&mut self) -> TAMPID2_W {
        TAMPID2_W { w: self }
    }
    #[doc = "Bit 3 - Tamper Input 3 Detected"]
    #[inline(always)]
    pub fn tampid3(&mut self) -> TAMPID3_W {
        TAMPID3_W { w: self }
    }
    #[doc = "Bit 4 - Tamper Input 4 Detected"]
    #[inline(always)]
    pub fn tampid4(&mut self) -> TAMPID4_W {
        TAMPID4_W { w: self }
    }
    #[doc = "Bit 31 - Tamper Event Detected"]
    #[inline(always)]
    pub fn tampevt(&mut self) -> TAMPEVT_W {
        TAMPEVT_W { w: self }
    }
}
