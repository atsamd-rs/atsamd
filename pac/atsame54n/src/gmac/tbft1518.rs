#[doc = "Register `TBFT1518` reader"]
pub type R = crate::R<TBFT1518_SPEC>;
#[doc = "Field `NFTX` reader - 1024 to 1518 Byte Frames Transmitted without Error"]
pub type NFTX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 1024 to 1518 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new(self.bits)
    }
}
#[doc = "1024 to 1518 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbft1518::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBFT1518_SPEC;
impl crate::RegisterSpec for TBFT1518_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbft1518::R`](R) reader structure"]
impl crate::Readable for TBFT1518_SPEC {}
#[doc = "`reset()` method sets TBFT1518 to value 0"]
impl crate::Resettable for TBFT1518_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
