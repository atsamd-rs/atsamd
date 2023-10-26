#[doc = "Register `BFR64` reader"]
pub type R = crate::R<BFR64_SPEC>;
#[doc = "Field `NFRX` reader - 64 Byte Frames Received without Error"]
pub type NFRX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 64 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NFRX_R {
        NFRX_R::new(self.bits)
    }
}
#[doc = "64 Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfr64::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BFR64_SPEC;
impl crate::RegisterSpec for BFR64_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bfr64::R`](R) reader structure"]
impl crate::Readable for BFR64_SPEC {}
#[doc = "`reset()` method sets BFR64 to value 0"]
impl crate::Resettable for BFR64_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
