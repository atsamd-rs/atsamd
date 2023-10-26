#[doc = "Register `TBFR511` reader"]
pub type R = crate::R<TBFR511_SPEC>;
#[doc = "Field `NFRX` reader - 256 to 511 Byte Frames Received without Error"]
pub type NFRX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 256 to 511 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NFRX_R {
        NFRX_R::new(self.bits)
    }
}
#[doc = "256 to 511Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbfr511::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBFR511_SPEC;
impl crate::RegisterSpec for TBFR511_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbfr511::R`](R) reader structure"]
impl crate::Readable for TBFR511_SPEC {}
#[doc = "`reset()` method sets TBFR511 to value 0"]
impl crate::Resettable for TBFR511_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
