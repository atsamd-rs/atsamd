#[doc = "Register `PIDR0` reader"]
pub type R = crate::R<Pidr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Peripheral Identification Register #0\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr0Spec;
impl crate::RegisterSpec for Pidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr0::R`](R) reader structure"]
impl crate::Readable for Pidr0Spec {}
#[doc = "`reset()` method sets PIDR0 to value 0x25"]
impl crate::Resettable for Pidr0Spec {
    const RESET_VALUE: u32 = 0x25;
}
