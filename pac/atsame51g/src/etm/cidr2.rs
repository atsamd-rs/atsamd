#[doc = "Register `CIDR2` reader"]
pub type R = crate::R<Cidr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Component Identification Register #2\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr2Spec;
impl crate::RegisterSpec for Cidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr2::R`](R) reader structure"]
impl crate::Readable for Cidr2Spec {}
#[doc = "`reset()` method sets CIDR2 to value 0x05"]
impl crate::Resettable for Cidr2Spec {
    const RESET_VALUE: u32 = 0x05;
}
