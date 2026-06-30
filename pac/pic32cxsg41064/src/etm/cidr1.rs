#[doc = "Register `CIDR1` reader"]
pub type R = crate::R<Cidr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Component Identification Register #1\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr1Spec;
impl crate::RegisterSpec for Cidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr1::R`](R) reader structure"]
impl crate::Readable for Cidr1Spec {}
#[doc = "`reset()` method sets CIDR1 to value 0x90"]
impl crate::Resettable for Cidr1Spec {
    const RESET_VALUE: u32 = 0x90;
}
