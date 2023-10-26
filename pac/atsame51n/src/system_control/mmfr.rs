#[doc = "Register `MMFR[%s]` reader"]
pub type R = crate::R<MMFR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MMFR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Memory Model Feature Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMFR_SPEC;
impl crate::RegisterSpec for MMFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmfr::R`](R) reader structure"]
impl crate::Readable for MMFR_SPEC {}
#[doc = "`reset()` method sets MMFR[%s]
to value 0"]
impl crate::Resettable for MMFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
