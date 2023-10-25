#[doc = "Register `PIDR4` reader"]
pub type R = crate::R<PIDR4_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PIDR4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Peripheral Identification Register #4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR4_SPEC;
impl crate::RegisterSpec for PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr4::R`](R) reader structure"]
impl crate::Readable for PIDR4_SPEC {}
#[doc = "`reset()` method sets PIDR4 to value 0x04"]
impl crate::Resettable for PIDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
