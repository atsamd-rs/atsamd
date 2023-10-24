#[doc = "Register `SYNCFR` reader"]
pub type R = crate::R<SYNCFR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SYNCFR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Synchronization Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCFR_SPEC;
impl crate::RegisterSpec for SYNCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncfr::R`](R) reader structure"]
impl crate::Readable for SYNCFR_SPEC {}
#[doc = "`reset()` method sets SYNCFR to value 0x0400"]
impl crate::Resettable for SYNCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
