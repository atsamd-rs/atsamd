#[doc = "Reader of register STATUSA"]
pub type R = crate::R<u8, super::STATUSA>;
#[doc = "Writer for register STATUSA"]
pub type W = crate::W<u8, super::STATUSA>;
#[doc = "Register STATUSA `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUSA {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
#[doc = "Reader of field `CRSTEXT`"]
pub type CRSTEXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRSTEXT`"]
pub struct CRSTEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSTEXT_W<'a> {
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
#[doc = "Reader of field `BERR`"]
pub type BERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BERR`"]
pub struct BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `FAIL`"]
pub type FAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAIL`"]
pub struct FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> FAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PERR`"]
pub type PERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERR`"]
pub struct PERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU Reset Phase Extension"]
    #[inline(always)]
    pub fn crstext(&self) -> CRSTEXT_R {
        CRSTEXT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Failure"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Protection Error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Done"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bit 1 - CPU Reset Phase Extension"]
    #[inline(always)]
    pub fn crstext(&mut self) -> CRSTEXT_W {
        CRSTEXT_W { w: self }
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W { w: self }
    }
    #[doc = "Bit 3 - Failure"]
    #[inline(always)]
    pub fn fail(&mut self) -> FAIL_W {
        FAIL_W { w: self }
    }
    #[doc = "Bit 4 - Protection Error"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W { w: self }
    }
}
