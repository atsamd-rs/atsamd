#[doc = "Register `TBFR1023` reader"]
pub type R = crate::R<TBFR1023_SPEC>;
#[doc = "Field `NFRX` reader - 512 to 1023 Byte Frames Received without Error"]
pub type NFRX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 512 to 1023 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NFRX_R {
        NFRX_R::new(self.bits)
    }
}
#[doc = "512 to 1023 Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbfr1023::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBFR1023_SPEC;
impl crate::RegisterSpec for TBFR1023_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbfr1023::R`](R) reader structure"]
impl crate::Readable for TBFR1023_SPEC {}
#[doc = "`reset()` method sets TBFR1023 to value 0"]
impl crate::Resettable for TBFR1023_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
