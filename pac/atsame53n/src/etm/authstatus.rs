#[doc = "Register `AUTHSTATUS` reader"]
pub type R = crate::R<AUTHSTATUS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<AUTHSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Authentication Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`authstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUTHSTATUS_SPEC;
impl crate::RegisterSpec for AUTHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`authstatus::R`](R) reader structure"]
impl crate::Readable for AUTHSTATUS_SPEC {}
#[doc = "`reset()` method sets AUTHSTATUS to value 0"]
impl crate::Resettable for AUTHSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
