#[doc = "Register `TBFR1518` reader"]
pub type R = crate::R<TBFR1518_SPEC>;
#[doc = "Field `NFRX` reader - 1024 to 1518 Byte Frames Received without Error"]
pub type NFRX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 1024 to 1518 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NFRX_R {
        NFRX_R::new(self.bits)
    }
}
#[doc = "1024 to 1518 Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbfr1518::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBFR1518_SPEC;
impl crate::RegisterSpec for TBFR1518_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbfr1518::R`](R) reader structure"]
impl crate::Readable for TBFR1518_SPEC {}
#[doc = "`reset()` method sets TBFR1518 to value 0"]
impl crate::Resettable for TBFR1518_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
