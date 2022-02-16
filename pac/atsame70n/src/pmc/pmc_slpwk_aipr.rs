#[doc = "Register `PMC_SLPWK_AIPR` reader"]
pub struct R(crate::R<PMC_SLPWK_AIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_SLPWK_AIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_SLPWK_AIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_SLPWK_AIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AIP` reader - Activity In Progress"]
pub struct AIP_R(crate::FieldReader<bool, bool>);
impl AIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Activity In Progress"]
    #[inline(always)]
    pub fn aip(&self) -> AIP_R {
        AIP_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "SleepWalking Activity In Progress Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_slpwk_aipr](index.html) module"]
pub struct PMC_SLPWK_AIPR_SPEC;
impl crate::RegisterSpec for PMC_SLPWK_AIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_slpwk_aipr::R](R) reader structure"]
impl crate::Readable for PMC_SLPWK_AIPR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMC_SLPWK_AIPR to value 0"]
impl crate::Resettable for PMC_SLPWK_AIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
