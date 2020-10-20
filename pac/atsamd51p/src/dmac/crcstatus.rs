#[doc = "Reader of register CRCSTATUS"]
pub type R = crate::R<u8, super::CRCSTATUS>;
#[doc = "Writer for register CRCSTATUS"]
pub type W = crate::W<u8, super::CRCSTATUS>;
#[doc = "Register CRCSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::CRCSTATUS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRCBUSY`"]
pub type CRCBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCBUSY`"]
pub struct CRCBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCBUSY_W<'a> {
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
#[doc = "Reader of field `CRCZERO`"]
pub type CRCZERO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCZERO`"]
pub struct CRCZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCZERO_W<'a> {
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
#[doc = "Reader of field `CRCERR`"]
pub type CRCERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCERR`"]
pub struct CRCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCERR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CRC Module Busy"]
    #[inline(always)]
    pub fn crcbusy(&self) -> CRCBUSY_R {
        CRCBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CRC Zero"]
    #[inline(always)]
    pub fn crczero(&self) -> CRCZERO_R {
        CRCZERO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CRC Error"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Module Busy"]
    #[inline(always)]
    pub fn crcbusy(&mut self) -> CRCBUSY_W {
        CRCBUSY_W { w: self }
    }
    #[doc = "Bit 1 - CRC Zero"]
    #[inline(always)]
    pub fn crczero(&mut self) -> CRCZERO_W {
        CRCZERO_W { w: self }
    }
    #[doc = "Bit 2 - CRC Error"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W {
        CRCERR_W { w: self }
    }
}
