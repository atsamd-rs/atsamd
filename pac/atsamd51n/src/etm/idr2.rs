#[doc = "Register `IDR2` reader"]
pub type R = crate::R<Idr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM ID Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`idr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idr2Spec;
impl crate::RegisterSpec for Idr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr2::R`](R) reader structure"]
impl crate::Readable for Idr2Spec {}
#[doc = "`reset()` method sets IDR2 to value 0"]
impl crate::Resettable for Idr2Spec {}
