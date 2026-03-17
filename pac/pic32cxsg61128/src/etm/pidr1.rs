#[doc = "Register `PIDR1` reader"]
pub type R = crate::R<Pidr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Peripheral Identification Register #1\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr1Spec;
impl crate::RegisterSpec for Pidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr1::R`](R) reader structure"]
impl crate::Readable for Pidr1Spec {}
#[doc = "`reset()` method sets PIDR1 to value 0xb9"]
impl crate::Resettable for Pidr1Spec {
    const RESET_VALUE: u32 = 0xb9;
}
