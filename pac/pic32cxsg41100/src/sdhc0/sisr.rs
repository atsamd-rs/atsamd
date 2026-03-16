#[doc = "Register `SISR` reader"]
pub type R = crate::R<SisrSpec>;
#[doc = "Field `INTSSL` reader - Interrupt Signal for Each SDHC Slot"]
pub type IntsslR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Interrupt Signal for Each SDHC Slot"]
    #[inline(always)]
    pub fn intssl(&self) -> IntsslR {
        IntsslR::new((self.bits & 3) as u8)
    }
}
#[doc = "Slot Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SisrSpec;
impl crate::RegisterSpec for SisrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sisr::R`](R) reader structure"]
impl crate::Readable for SisrSpec {}
#[doc = "`reset()` method sets SISR to value 0"]
impl crate::Resettable for SisrSpec {
    const RESET_VALUE: u16 = 0;
}
