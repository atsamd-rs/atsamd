#[doc = "Register `CIDR0` reader"]
pub type R = crate::R<Cidr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Component Identification Register #0\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr0Spec;
impl crate::RegisterSpec for Cidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr0::R`](R) reader structure"]
impl crate::Readable for Cidr0Spec {}
#[doc = "`reset()` method sets CIDR0 to value 0x0d"]
impl crate::Resettable for Cidr0Spec {
    const RESET_VALUE: u32 = 0x0d;
}
