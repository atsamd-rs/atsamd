#[doc = "Register `TBFT255` reader"]
pub type R = crate::R<TBFT255_SPEC>;
#[doc = "Field `NFTX` reader - 128 to 255 Byte Frames Transmitted without Error"]
pub type NFTX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 128 to 255 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new(self.bits)
    }
}
#[doc = "128 to 255 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbft255::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBFT255_SPEC;
impl crate::RegisterSpec for TBFT255_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbft255::R`](R) reader structure"]
impl crate::Readable for TBFT255_SPEC {}
#[doc = "`reset()` method sets TBFT255 to value 0"]
impl crate::Resettable for TBFT255_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
