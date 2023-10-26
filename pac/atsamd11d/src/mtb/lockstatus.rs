#[doc = "Register `LOCKSTATUS` reader"]
pub type R = crate::R<LOCKSTATUS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<LOCKSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MTB Lock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCKSTATUS_SPEC;
impl crate::RegisterSpec for LOCKSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockstatus::R`](R) reader structure"]
impl crate::Readable for LOCKSTATUS_SPEC {}
#[doc = "`reset()` method sets LOCKSTATUS to value 0"]
impl crate::Resettable for LOCKSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
