#[doc = "Register `PIDR2` reader"]
pub type R = crate::R<PIDR2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PIDR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Peripheral Identification Register #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR2_SPEC;
impl crate::RegisterSpec for PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr2::R`](R) reader structure"]
impl crate::Readable for PIDR2_SPEC {}
#[doc = "`reset()` method sets PIDR2 to value 0x0b"]
impl crate::Resettable for PIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
