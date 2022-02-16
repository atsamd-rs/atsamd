#[doc = "Register `PWM_CMPM` reader"]
pub struct R(crate::R<PWM_CMPM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CMPM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CMPM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CMPM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CMPM` writer"]
pub struct W(crate::W<PWM_CMPM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CMPM_SPEC>;
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
impl From<crate::W<PWM_CMPM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CMPM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEN` reader - Comparison x Enable"]
pub struct CEN_R(crate::FieldReader<bool, bool>);
impl CEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEN` writer - Comparison x Enable"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CTR` reader - Comparison x Trigger"]
pub struct CTR_R(crate::FieldReader<u8, u8>);
impl CTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR` writer - Comparison x Trigger"]
pub struct CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CPR` reader - Comparison x Period"]
pub struct CPR_R(crate::FieldReader<u8, u8>);
impl CPR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPR` writer - Comparison x Period"]
pub struct CPR_W<'a> {
    w: &'a mut W,
}
impl<'a> CPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CPRCNT` reader - Comparison x Period Counter"]
pub struct CPRCNT_R(crate::FieldReader<u8, u8>);
impl CPRCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPRCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPRCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPRCNT` writer - Comparison x Period Counter"]
pub struct CPRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `CUPR` reader - Comparison x Update Period"]
pub struct CUPR_R(crate::FieldReader<u8, u8>);
impl CUPR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CUPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUPR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CUPR` writer - Comparison x Update Period"]
pub struct CUPR_W<'a> {
    w: &'a mut W,
}
impl<'a> CUPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `CUPRCNT` reader - Comparison x Update Period Counter"]
pub struct CUPRCNT_R(crate::FieldReader<u8, u8>);
impl CUPRCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CUPRCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUPRCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CUPRCNT` writer - Comparison x Update Period Counter"]
pub struct CUPRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CUPRCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparison x Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Comparison x Period"]
    #[inline(always)]
    pub fn cpr(&self) -> CPR_R {
        CPR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Comparison x Period Counter"]
    #[inline(always)]
    pub fn cprcnt(&self) -> CPRCNT_R {
        CPRCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period"]
    #[inline(always)]
    pub fn cupr(&self) -> CUPR_R {
        CUPR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Comparison x Update Period Counter"]
    #[inline(always)]
    pub fn cuprcnt(&self) -> CUPRCNT_R {
        CUPRCNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison x Enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    #[doc = "Bits 4:7 - Comparison x Trigger"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CTR_W {
        CTR_W { w: self }
    }
    #[doc = "Bits 8:11 - Comparison x Period"]
    #[inline(always)]
    pub fn cpr(&mut self) -> CPR_W {
        CPR_W { w: self }
    }
    #[doc = "Bits 12:15 - Comparison x Period Counter"]
    #[inline(always)]
    pub fn cprcnt(&mut self) -> CPRCNT_W {
        CPRCNT_W { w: self }
    }
    #[doc = "Bits 16:19 - Comparison x Update Period"]
    #[inline(always)]
    pub fn cupr(&mut self) -> CUPR_W {
        CUPR_W { w: self }
    }
    #[doc = "Bits 20:23 - Comparison x Update Period Counter"]
    #[inline(always)]
    pub fn cuprcnt(&mut self) -> CUPRCNT_W {
        CUPRCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 0 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmpm](index.html) module"]
pub struct PWM_CMPM_SPEC;
impl crate::RegisterSpec for PWM_CMPM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_cmpm::R](R) reader structure"]
impl crate::Readable for PWM_CMPM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_cmpm::W](W) writer structure"]
impl crate::Writable for PWM_CMPM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CMPM to value 0"]
impl crate::Resettable for PWM_CMPM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
