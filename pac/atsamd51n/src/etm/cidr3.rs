#[doc = "Register `CIDR3` reader"]
pub type R = crate::R<Cidr3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Component Identification Register #3\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr3Spec;
impl crate::RegisterSpec for Cidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr3::R`](R) reader structure"]
impl crate::Readable for Cidr3Spec {}
#[doc = "`reset()` method sets CIDR3 to value 0xb1"]
impl crate::Resettable for Cidr3Spec {
    const RESET_VALUE: u32 = 0xb1;
}
