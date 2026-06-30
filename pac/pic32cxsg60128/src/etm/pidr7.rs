#[doc = "Register `PIDR7` reader"]
pub type R = crate::R<Pidr7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Peripheral Identification Register #7\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr7Spec;
impl crate::RegisterSpec for Pidr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr7::R`](R) reader structure"]
impl crate::Readable for Pidr7Spec {}
#[doc = "`reset()` method sets PIDR7 to value 0"]
impl crate::Resettable for Pidr7Spec {
    const RESET_VALUE: u32 = 0;
}
