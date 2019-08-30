#[doc = "Reader of register DFLLCTRLB"]
pub type R = crate::R<u8, super::DFLLCTRLB>;
#[doc = "Writer for register DFLLCTRLB"]
pub type W = crate::W<u8, super::DFLLCTRLB>;
#[doc = "Register DFLLCTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::DFLLCTRLB {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
#[doc = "Reader of field `STABLE`"]
pub type STABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STABLE`"]
pub struct STABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> STABLE_W<'a> {
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
#[doc = "Reader of field `LLAW`"]
pub type LLAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LLAW`"]
pub struct LLAW_W<'a> {
    w: &'a mut W,
}
impl<'a> LLAW_W<'a> {
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
#[doc = "Reader of field `USBCRM`"]
pub type USBCRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBCRM`"]
pub struct USBCRM_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCRM_W<'a> {
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
#[doc = "Reader of field `CCDIS`"]
pub type CCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCDIS`"]
pub struct CCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCDIS_W<'a> {
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
#[doc = "Reader of field `QLDIS`"]
pub type QLDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QLDIS`"]
pub struct QLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> QLDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `BPLCKC`"]
pub type BPLCKC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BPLCKC`"]
pub struct BPLCKC_W<'a> {
    w: &'a mut W,
}
impl<'a> BPLCKC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `WAITLOCK`"]
pub type WAITLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAITLOCK`"]
pub struct WAITLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Operating Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Stable DFLL Frequency"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&self) -> LLAW_R {
        LLAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB Clock Recovery Mode"]
    #[inline(always)]
    pub fn usbcrm(&self) -> USBCRM_R {
        USBCRM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&self) -> CCDIS_R {
        CCDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&self) -> QLDIS_R {
        QLDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bypass Coarse Lock"]
    #[inline(always)]
    pub fn bplckc(&self) -> BPLCKC_R {
        BPLCKC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wait Lock"]
    #[inline(always)]
    pub fn waitlock(&self) -> WAITLOCK_R {
        WAITLOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operating Mode Selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 1 - Stable DFLL Frequency"]
    #[inline(always)]
    pub fn stable(&mut self) -> STABLE_W {
        STABLE_W { w: self }
    }
    #[doc = "Bit 2 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&mut self) -> LLAW_W {
        LLAW_W { w: self }
    }
    #[doc = "Bit 3 - USB Clock Recovery Mode"]
    #[inline(always)]
    pub fn usbcrm(&mut self) -> USBCRM_W {
        USBCRM_W { w: self }
    }
    #[doc = "Bit 4 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&mut self) -> CCDIS_W {
        CCDIS_W { w: self }
    }
    #[doc = "Bit 5 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&mut self) -> QLDIS_W {
        QLDIS_W { w: self }
    }
    #[doc = "Bit 6 - Bypass Coarse Lock"]
    #[inline(always)]
    pub fn bplckc(&mut self) -> BPLCKC_W {
        BPLCKC_W { w: self }
    }
    #[doc = "Bit 7 - Wait Lock"]
    #[inline(always)]
    pub fn waitlock(&mut self) -> WAITLOCK_W {
        WAITLOCK_W { w: self }
    }
}
