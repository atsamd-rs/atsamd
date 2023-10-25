#[doc = "Register `PIDR0` reader"]
pub type R = crate::R<PIDR0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PIDR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Peripheral Identification Register #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR0_SPEC;
impl crate::RegisterSpec for PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr0::R`](R) reader structure"]
impl crate::Readable for PIDR0_SPEC {}
#[doc = "`reset()` method sets PIDR0 to value 0x25"]
impl crate::Resettable for PIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x25;
}
