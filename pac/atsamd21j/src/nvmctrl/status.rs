#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRM` reader - Power Reduction Mode"]
pub struct PRM_R(crate::FieldReader<bool, bool>);
impl PRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOAD` reader - NVM Page Buffer Active Loading"]
pub struct LOAD_R(crate::FieldReader<bool, bool>);
impl LOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOAD` writer - NVM Page Buffer Active Loading"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PROGE` reader - Programming Error Status"]
pub struct PROGE_R(crate::FieldReader<bool, bool>);
impl PROGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROGE` writer - Programming Error Status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `LOCKE` reader - Lock Error Status"]
pub struct LOCKE_R(crate::FieldReader<bool, bool>);
impl LOCKE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKE` writer - Lock Error Status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `NVME` reader - NVM Error"]
pub struct NVME_R(crate::FieldReader<bool, bool>);
impl NVME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NVME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVME` writer - NVM Error"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SB` reader - Security Bit Status"]
pub struct SB_R(crate::FieldReader<bool, bool>);
impl SB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
