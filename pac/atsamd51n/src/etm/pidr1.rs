#[doc = "Register `PIDR1` reader"]
pub type R = crate::R<PIDR1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PIDR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Peripheral Identification Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR1_SPEC;
impl crate::RegisterSpec for PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr1::R`](R) reader structure"]
impl crate::Readable for PIDR1_SPEC {}
#[doc = "`reset()` method sets PIDR1 to value 0xb9"]
impl crate::Resettable for PIDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xb9;
}
