#[doc = "Register `PMC_PMMR` reader"]
pub struct R(crate::R<PMC_PMMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_PMMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_PMMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_PMMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_PMMR` writer"]
pub struct W(crate::W<PMC_PMMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_PMMR_SPEC>;
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
impl From<crate::W<PMC_PMMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_PMMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLA_MMAX` reader - PLLA Maximum Allowed Multiplier Value"]
pub struct PLLA_MMAX_R(crate::FieldReader<u16, u16>);
impl PLLA_MMAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PLLA_MMAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLA_MMAX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLA_MMAX` writer - PLLA Maximum Allowed Multiplier Value"]
pub struct PLLA_MMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLA_MMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - PLLA Maximum Allowed Multiplier Value"]
    #[inline(always)]
    pub fn plla_mmax(&self) -> PLLA_MMAX_R {
        PLLA_MMAX_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - PLLA Maximum Allowed Multiplier Value"]
    #[inline(always)]
    pub fn plla_mmax(&mut self) -> PLLA_MMAX_W {
        PLLA_MMAX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Maximum Multiplier Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pmmr](index.html) module"]
pub struct PMC_PMMR_SPEC;
impl crate::RegisterSpec for PMC_PMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_pmmr::R](R) reader structure"]
impl crate::Readable for PMC_PMMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_pmmr::W](W) writer structure"]
impl crate::Writable for PMC_PMMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_PMMR to value 0"]
impl crate::Resettable for PMC_PMMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
