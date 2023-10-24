#[doc = "Register `CIDR0` reader"]
pub type R = crate::R<CIDR0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CIDR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Component Identification Register #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR0_SPEC;
impl crate::RegisterSpec for CIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr0::R`](R) reader structure"]
impl crate::Readable for CIDR0_SPEC {}
#[doc = "`reset()` method sets CIDR0 to value 0x0d"]
impl crate::Resettable for CIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
