#[doc = "Register `SISR` reader"]
pub struct R(crate::R<SISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTSSL` reader - Interrupt Signal for Each Slot"]
pub type INTSSL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Interrupt Signal for Each Slot"]
    #[inline(always)]
    pub fn intssl(&self) -> INTSSL_R {
        INTSSL_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Slot Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sisr](index.html) module"]
pub struct SISR_SPEC;
impl crate::RegisterSpec for SISR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sisr::R](R) reader structure"]
impl crate::Readable for SISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SISR to value 0"]
impl crate::Resettable for SISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
