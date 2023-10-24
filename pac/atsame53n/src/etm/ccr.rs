#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Configuration Code Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`reset()` method sets CCR to value 0x8c80_2000"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8c80_2000;
}
