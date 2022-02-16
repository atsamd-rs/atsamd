#[doc = "Register `QSPI_ICR` reader"]
pub struct R(crate::R<QSPI_ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_ICR` writer"]
pub struct W(crate::W<QSPI_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_ICR_SPEC>;
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
impl From<crate::W<QSPI_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INST` reader - Instruction Code"]
pub struct INST_R(crate::FieldReader<u8, u8>);
impl INST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INST` writer - Instruction Code"]
pub struct INST_W<'a> {
    w: &'a mut W,
}
impl<'a> INST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `OPT` reader - Option Code"]
pub struct OPT_R(crate::FieldReader<u8, u8>);
impl OPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPT` writer - Option Code"]
pub struct OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    pub fn inst(&self) -> INST_R {
        INST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    pub fn opt(&self) -> OPT_R {
        OPT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    pub fn inst(&mut self) -> INST_W {
        INST_W { w: self }
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    pub fn opt(&mut self) -> OPT_W {
        OPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction Code Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_icr](index.html) module"]
pub struct QSPI_ICR_SPEC;
impl crate::RegisterSpec for QSPI_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_icr::R](R) reader structure"]
impl crate::Readable for QSPI_ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_icr::W](W) writer structure"]
impl crate::Writable for QSPI_ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_ICR to value 0"]
impl crate::Resettable for QSPI_ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
