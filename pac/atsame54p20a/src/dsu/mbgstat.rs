#[doc = "Reader of register MBGSTAT"]
pub type R = crate::R<u32, super::MBGSTAT>;
#[doc = "Writer for register MBGSTAT"]
pub type W = crate::W<u32, super::MBGSTAT>;
#[doc = "Register MBGSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::MBGSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALLDONE`"]
pub type ALLDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALLDONE`"]
pub struct ALLDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLDONE_W<'a> {
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
#[doc = "Reader of field `FAILED`"]
pub type FAILED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAILED`"]
pub struct FAILED_W<'a> {
    w: &'a mut W,
}
impl<'a> FAILED_W<'a> {
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
#[doc = "Reader of field `ERRINFO`"]
pub type ERRINFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRINFO`"]
pub struct ERRINFO_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRINFO_W<'a> {
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
#[doc = "Reader of field `CONFIGURED`"]
pub type CONFIGURED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONFIGURED`"]
pub struct CONFIGURED_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIGURED_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MBIST Completed"]
    #[inline(always)]
    pub fn alldone(&self) -> ALLDONE_R {
        ALLDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MBIST Failed"]
    #[inline(always)]
    pub fn failed(&self) -> FAILED_R {
        FAILED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MBIST Error Info Present"]
    #[inline(always)]
    pub fn errinfo(&self) -> ERRINFO_R {
        ERRINFO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MBIST Configuration Sent"]
    #[inline(always)]
    pub fn configured(&self) -> CONFIGURED_R {
        CONFIGURED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MBIST Completed"]
    #[inline(always)]
    pub fn alldone(&mut self) -> ALLDONE_W {
        ALLDONE_W { w: self }
    }
    #[doc = "Bit 1 - MBIST Failed"]
    #[inline(always)]
    pub fn failed(&mut self) -> FAILED_W {
        FAILED_W { w: self }
    }
    #[doc = "Bit 2 - MBIST Error Info Present"]
    #[inline(always)]
    pub fn errinfo(&mut self) -> ERRINFO_W {
        ERRINFO_W { w: self }
    }
    #[doc = "Bit 3 - MBIST Configuration Sent"]
    #[inline(always)]
    pub fn configured(&mut self) -> CONFIGURED_W {
        CONFIGURED_W { w: self }
    }
}
