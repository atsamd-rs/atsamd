#[doc = "Register `TBFR255` reader"]
pub type R = crate::R<TBFR255_SPEC>;
#[doc = "Field `NFRX` reader - 128 to 255 Byte Frames Received without Error"]
pub type NFRX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 128 to 255 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NFRX_R {
        NFRX_R::new(self.bits)
    }
}
#[doc = "128 to 255 Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbfr255::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBFR255_SPEC;
impl crate::RegisterSpec for TBFR255_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbfr255::R`](R) reader structure"]
impl crate::Readable for TBFR255_SPEC {}
#[doc = "`reset()` method sets TBFR255 to value 0"]
impl crate::Resettable for TBFR255_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
