#[doc = "Register `CCFG_CAN0` reader"]
pub struct R(crate::R<CCFG_CAN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_CAN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_CAN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_CAN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_CAN0` writer"]
pub struct W(crate::W<CCFG_CAN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_CAN0_SPEC>;
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
impl From<crate::W<CCFG_CAN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_CAN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN0DMABA` reader - CAN0 DMA Base Address"]
pub struct CAN0DMABA_R(crate::FieldReader<u16, u16>);
impl CAN0DMABA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CAN0DMABA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN0DMABA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN0DMABA` writer - CAN0 DMA Base Address"]
pub struct CAN0DMABA_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0DMABA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - CAN0 DMA Base Address"]
    #[inline(always)]
    pub fn can0dmaba(&self) -> CAN0DMABA_R {
        CAN0DMABA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - CAN0 DMA Base Address"]
    #[inline(always)]
    pub fn can0dmaba(&mut self) -> CAN0DMABA_W {
        CAN0DMABA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_can0](index.html) module"]
pub struct CCFG_CAN0_SPEC;
impl crate::RegisterSpec for CCFG_CAN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_can0::R](R) reader structure"]
impl crate::Readable for CCFG_CAN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_can0::W](W) writer structure"]
impl crate::Writable for CCFG_CAN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_CAN0 to value 0"]
impl crate::Resettable for CCFG_CAN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
