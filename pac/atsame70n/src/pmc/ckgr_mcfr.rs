#[doc = "Register `CKGR_MCFR` reader"]
pub struct R(crate::R<CKGR_MCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKGR_MCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKGR_MCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKGR_MCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKGR_MCFR` writer"]
pub struct W(crate::W<CKGR_MCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKGR_MCFR_SPEC>;
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
impl From<crate::W<CKGR_MCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKGR_MCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAINF` reader - Main Clock Frequency"]
pub struct MAINF_R(crate::FieldReader<u16, u16>);
impl MAINF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MAINF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAINF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAINF` writer - Main Clock Frequency"]
pub struct MAINF_W<'a> {
    w: &'a mut W,
}
impl<'a> MAINF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `MAINFRDY` reader - Main Clock Frequency Measure Ready"]
pub struct MAINFRDY_R(crate::FieldReader<bool, bool>);
impl MAINFRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAINFRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAINFRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAINFRDY` writer - Main Clock Frequency Measure Ready"]
pub struct MAINFRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> MAINFRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `RCMEAS` reader - RC Oscillator Frequency Measure (write-only)"]
pub struct RCMEAS_R(crate::FieldReader<bool, bool>);
impl RCMEAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCMEAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCMEAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCMEAS` writer - RC Oscillator Frequency Measure (write-only)"]
pub struct RCMEAS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCMEAS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CCSS` reader - Counter Clock Source Selection"]
pub struct CCSS_R(crate::FieldReader<bool, bool>);
impl CCSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCSS` writer - Counter Clock Source Selection"]
pub struct CCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCSS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Main Clock Frequency"]
    #[inline(always)]
    pub fn mainf(&self) -> MAINF_R {
        MAINF_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Main Clock Frequency Measure Ready"]
    #[inline(always)]
    pub fn mainfrdy(&self) -> MAINFRDY_R {
        MAINFRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - RC Oscillator Frequency Measure (write-only)"]
    #[inline(always)]
    pub fn rcmeas(&self) -> RCMEAS_R {
        RCMEAS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Counter Clock Source Selection"]
    #[inline(always)]
    pub fn ccss(&self) -> CCSS_R {
        CCSS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Main Clock Frequency"]
    #[inline(always)]
    pub fn mainf(&mut self) -> MAINF_W {
        MAINF_W { w: self }
    }
    #[doc = "Bit 16 - Main Clock Frequency Measure Ready"]
    #[inline(always)]
    pub fn mainfrdy(&mut self) -> MAINFRDY_W {
        MAINFRDY_W { w: self }
    }
    #[doc = "Bit 20 - RC Oscillator Frequency Measure (write-only)"]
    #[inline(always)]
    pub fn rcmeas(&mut self) -> RCMEAS_W {
        RCMEAS_W { w: self }
    }
    #[doc = "Bit 24 - Counter Clock Source Selection"]
    #[inline(always)]
    pub fn ccss(&mut self) -> CCSS_W {
        CCSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Clock Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_mcfr](index.html) module"]
pub struct CKGR_MCFR_SPEC;
impl crate::RegisterSpec for CKGR_MCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgr_mcfr::R](R) reader structure"]
impl crate::Readable for CKGR_MCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckgr_mcfr::W](W) writer structure"]
impl crate::Writable for CKGR_MCFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CKGR_MCFR to value 0"]
impl crate::Resettable for CKGR_MCFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
