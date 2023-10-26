#[doc = "Register `FSCR` reader"]
pub type R = crate::R<FSCR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<FSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Formatter Synchronization Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSCR_SPEC;
impl crate::RegisterSpec for FSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fscr::R`](R) reader structure"]
impl crate::Readable for FSCR_SPEC {}
#[doc = "`reset()` method sets FSCR to value 0"]
impl crate::Resettable for FSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
