#[doc = "Register `TBFR127` reader"]
pub type R = crate::R<TBFR127_SPEC>;
#[doc = "Field `NFRX` reader - 65 to 127 Byte Frames Received without Error"]
pub type NFRX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 65 to 127 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NFRX_R {
        NFRX_R::new(self.bits)
    }
}
#[doc = "65 to 127 Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbfr127::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBFR127_SPEC;
impl crate::RegisterSpec for TBFR127_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbfr127::R`](R) reader structure"]
impl crate::Readable for TBFR127_SPEC {}
#[doc = "`reset()` method sets TBFR127 to value 0"]
impl crate::Resettable for TBFR127_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
