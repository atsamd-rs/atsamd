#[doc = "Register `PIDR2` reader"]
pub type R = crate::R<Pidr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Peripheral Identification Register #2\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr2Spec;
impl crate::RegisterSpec for Pidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr2::R`](R) reader structure"]
impl crate::Readable for Pidr2Spec {}
#[doc = "`reset()` method sets PIDR2 to value 0x0b"]
impl crate::Resettable for Pidr2Spec {
    const RESET_VALUE: u32 = 0x0b;
}
