#[doc = "Register `PIDR6` reader"]
pub type R = crate::R<PIDR6_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PIDR6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Peripheral Identification Register #6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR6_SPEC;
impl crate::RegisterSpec for PIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr6::R`](R) reader structure"]
impl crate::Readable for PIDR6_SPEC {}
#[doc = "`reset()` method sets PIDR6 to value 0"]
impl crate::Resettable for PIDR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
