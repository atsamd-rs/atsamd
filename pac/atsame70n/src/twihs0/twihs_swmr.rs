#[doc = "Register `TWIHS_SWMR` reader"]
pub struct R(crate::R<TWIHS_SWMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWIHS_SWMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWIHS_SWMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWIHS_SWMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWIHS_SWMR` writer"]
pub struct W(crate::W<TWIHS_SWMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWIHS_SWMR_SPEC>;
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
impl From<crate::W<TWIHS_SWMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWIHS_SWMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADR1` reader - Slave Address 1"]
pub struct SADR1_R(crate::FieldReader<u8, u8>);
impl SADR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SADR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADR1` writer - Slave Address 1"]
pub struct SADR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `SADR2` reader - Slave Address 2"]
pub struct SADR2_R(crate::FieldReader<u8, u8>);
impl SADR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SADR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADR2` writer - Slave Address 2"]
pub struct SADR2_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `SADR3` reader - Slave Address 3"]
pub struct SADR3_R(crate::FieldReader<u8, u8>);
impl SADR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SADR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADR3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADR3` writer - Slave Address 3"]
pub struct SADR3_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `DATAM` reader - Data Match"]
pub struct DATAM_R(crate::FieldReader<u8, u8>);
impl DATAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAM` writer - Data Match"]
pub struct DATAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Slave Address 1"]
    #[inline(always)]
    pub fn sadr1(&self) -> SADR1_R {
        SADR1_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Slave Address 2"]
    #[inline(always)]
    pub fn sadr2(&self) -> SADR2_R {
        SADR2_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Slave Address 3"]
    #[inline(always)]
    pub fn sadr3(&self) -> SADR3_R {
        SADR3_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - Data Match"]
    #[inline(always)]
    pub fn datam(&self) -> DATAM_R {
        DATAM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave Address 1"]
    #[inline(always)]
    pub fn sadr1(&mut self) -> SADR1_W {
        SADR1_W { w: self }
    }
    #[doc = "Bits 8:14 - Slave Address 2"]
    #[inline(always)]
    pub fn sadr2(&mut self) -> SADR2_W {
        SADR2_W { w: self }
    }
    #[doc = "Bits 16:22 - Slave Address 3"]
    #[inline(always)]
    pub fn sadr3(&mut self) -> SADR3_W {
        SADR3_W { w: self }
    }
    #[doc = "Bits 24:31 - Data Match"]
    #[inline(always)]
    pub fn datam(&mut self) -> DATAM_W {
        DATAM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SleepWalking Matching Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_swmr](index.html) module"]
pub struct TWIHS_SWMR_SPEC;
impl crate::RegisterSpec for TWIHS_SWMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twihs_swmr::R](R) reader structure"]
impl crate::Readable for TWIHS_SWMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twihs_swmr::W](W) writer structure"]
impl crate::Writable for TWIHS_SWMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWIHS_SWMR to value 0"]
impl crate::Resettable for TWIHS_SWMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
