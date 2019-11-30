#[doc = "Reader of register STATUS"]
pub type R = crate::R<u16, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u16, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRM`"]
pub type PRM_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOAD`"]
pub type LOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOAD`"]
pub struct LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PROGE`"]
pub type PROGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROGE`"]
pub struct PROGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `LOCKE`"]
pub type LOCKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKE`"]
pub struct LOCKE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `NVME`"]
pub type NVME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NVME`"]
pub struct NVME_W<'a> {
    w: &'a mut W,
}
impl<'a> NVME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SB`"]
pub type SB_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Power Reduction Mode"]
    #[inline(always)]
    pub fn prm(&self) -> PRM_R {
        PRM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NVM Page Buffer Active Loading"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Programming Error Status"]
    #[inline(always)]
    pub fn proge(&self) -> PROGE_R {
        PROGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock Error Status"]
    #[inline(always)]
    pub fn locke(&self) -> LOCKE_R {
        LOCKE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NVM Error"]
    #[inline(always)]
    pub fn nvme(&self) -> NVME_R {
        NVME_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Security Bit Status"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - NVM Page Buffer Active Loading"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W {
        LOAD_W { w: self }
    }
    #[doc = "Bit 2 - Programming Error Status"]
    #[inline(always)]
    pub fn proge(&mut self) -> PROGE_W {
        PROGE_W { w: self }
    }
    #[doc = "Bit 3 - Lock Error Status"]
    #[inline(always)]
    pub fn locke(&mut self) -> LOCKE_W {
        LOCKE_W { w: self }
    }
    #[doc = "Bit 4 - NVM Error"]
    #[inline(always)]
    pub fn nvme(&mut self) -> NVME_W {
        NVME_W { w: self }
    }
}
