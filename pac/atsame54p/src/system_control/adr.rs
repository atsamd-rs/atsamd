#[doc = "Register `ADR` reader"]
pub type R = crate::R<ADR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ADR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Auxiliary Feature Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADR_SPEC;
impl crate::RegisterSpec for ADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adr::R`](R) reader structure"]
impl crate::Readable for ADR_SPEC {}
#[doc = "`reset()` method sets ADR to value 0"]
impl crate::Resettable for ADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
