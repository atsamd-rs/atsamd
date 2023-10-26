#[doc = "Register `PIDR7` reader"]
pub type R = crate::R<PIDR7_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PIDR7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Peripheral Identification Register #7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR7_SPEC;
impl crate::RegisterSpec for PIDR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr7::R`](R) reader structure"]
impl crate::Readable for PIDR7_SPEC {}
#[doc = "`reset()` method sets PIDR7 to value 0"]
impl crate::Resettable for PIDR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
