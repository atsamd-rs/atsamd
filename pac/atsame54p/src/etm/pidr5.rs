#[doc = "Register `PIDR5` reader"]
pub type R = crate::R<Pidr5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Peripheral Identification Register #5\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr5Spec;
impl crate::RegisterSpec for Pidr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr5::R`](R) reader structure"]
impl crate::Readable for Pidr5Spec {}
#[doc = "`reset()` method sets PIDR5 to value 0"]
impl crate::Resettable for Pidr5Spec {}
