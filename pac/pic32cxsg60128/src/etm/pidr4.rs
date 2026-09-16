#[doc = "Register `PIDR4` reader"]
pub type R = crate::R<Pidr4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Peripheral Identification Register #4\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr4Spec;
impl crate::RegisterSpec for Pidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr4::R`](R) reader structure"]
impl crate::Readable for Pidr4Spec {}
#[doc = "`reset()` method sets PIDR4 to value 0x04"]
impl crate::Resettable for Pidr4Spec {
    const RESET_VALUE: u32 = 0x04;
}
