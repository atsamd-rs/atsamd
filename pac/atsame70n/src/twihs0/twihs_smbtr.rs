#[doc = "Register `TWIHS_SMBTR` reader"]
pub struct R(crate::R<TWIHS_SMBTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWIHS_SMBTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWIHS_SMBTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWIHS_SMBTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWIHS_SMBTR` writer"]
pub struct W(crate::W<TWIHS_SMBTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWIHS_SMBTR_SPEC>;
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
impl From<crate::W<TWIHS_SMBTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWIHS_SMBTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC` reader - SMBus Clock Prescaler"]
pub struct PRESC_R(crate::FieldReader<u8, u8>);
impl PRESC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESC` writer - SMBus Clock Prescaler"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TLOWS` reader - Slave Clock Stretch Maximum Cycles"]
pub struct TLOWS_R(crate::FieldReader<u8, u8>);
impl TLOWS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TLOWS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLOWS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLOWS` writer - Slave Clock Stretch Maximum Cycles"]
pub struct TLOWS_W<'a> {
    w: &'a mut W,
}
impl<'a> TLOWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TLOWM` reader - Master Clock Stretch Maximum Cycles"]
pub struct TLOWM_R(crate::FieldReader<u8, u8>);
impl TLOWM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TLOWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLOWM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLOWM` writer - Master Clock Stretch Maximum Cycles"]
pub struct TLOWM_W<'a> {
    w: &'a mut W,
}
impl<'a> TLOWM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `THMAX` reader - Clock High Maximum Cycles"]
pub struct THMAX_R(crate::FieldReader<u8, u8>);
impl THMAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        THMAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THMAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THMAX` writer - Clock High Maximum Cycles"]
pub struct THMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> THMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SMBus Clock Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Slave Clock Stretch Maximum Cycles"]
    #[inline(always)]
    pub fn tlows(&self) -> TLOWS_R {
        TLOWS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Master Clock Stretch Maximum Cycles"]
    #[inline(always)]
    pub fn tlowm(&self) -> TLOWM_R {
        TLOWM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock High Maximum Cycles"]
    #[inline(always)]
    pub fn thmax(&self) -> THMAX_R {
        THMAX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SMBus Clock Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bits 8:15 - Slave Clock Stretch Maximum Cycles"]
    #[inline(always)]
    pub fn tlows(&mut self) -> TLOWS_W {
        TLOWS_W { w: self }
    }
    #[doc = "Bits 16:23 - Master Clock Stretch Maximum Cycles"]
    #[inline(always)]
    pub fn tlowm(&mut self) -> TLOWM_W {
        TLOWM_W { w: self }
    }
    #[doc = "Bits 24:31 - Clock High Maximum Cycles"]
    #[inline(always)]
    pub fn thmax(&mut self) -> THMAX_W {
        THMAX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMBus Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_smbtr](index.html) module"]
pub struct TWIHS_SMBTR_SPEC;
impl crate::RegisterSpec for TWIHS_SMBTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twihs_smbtr::R](R) reader structure"]
impl crate::Readable for TWIHS_SMBTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twihs_smbtr::W](W) writer structure"]
impl crate::Writable for TWIHS_SMBTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWIHS_SMBTR to value 0"]
impl crate::Resettable for TWIHS_SMBTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
