#[doc = "Register `PIDR3` reader"]
pub type R = crate::R<Pidr3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Peripheral Identification Register #3\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr3Spec;
impl crate::RegisterSpec for Pidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr3::R`](R) reader structure"]
impl crate::Readable for Pidr3Spec {}
#[doc = "`reset()` method sets PIDR3 to value 0"]
impl crate::Resettable for Pidr3Spec {
    const RESET_VALUE: u32 = 0;
}
