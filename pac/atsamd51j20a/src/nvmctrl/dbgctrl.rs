#[doc = "Reader of register DBGCTRL"]
pub type R = crate::R<u8, super::DBGCTRL>;
#[doc = "Writer for register DBGCTRL"]
pub type W = crate::W<u8, super::DBGCTRL>;
#[doc = "Register DBGCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DBGCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ECCDIS`"]
pub type ECCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCDIS`"]
pub struct ECCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCDIS_W<'a> {
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
#[doc = "Reader of field `ECCELOG`"]
pub type ECCELOG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCELOG`"]
pub struct ECCELOG_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCELOG_W<'a> {
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
    #[doc = "Bit 0 - Debugger ECC Read Disable"]
    #[inline(always)]
    pub fn eccdis(&self) -> ECCDIS_R {
        ECCDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Debugger ECC Error Tracking Mode"]
    #[inline(always)]
    pub fn eccelog(&self) -> ECCELOG_R {
        ECCELOG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debugger ECC Read Disable"]
    #[inline(always)]
    pub fn eccdis(&mut self) -> ECCDIS_W {
        ECCDIS_W { w: self }
    }
    #[doc = "Bit 1 - Debugger ECC Error Tracking Mode"]
    #[inline(always)]
    pub fn eccelog(&mut self) -> ECCELOG_W {
        ECCELOG_W { w: self }
    }
}
