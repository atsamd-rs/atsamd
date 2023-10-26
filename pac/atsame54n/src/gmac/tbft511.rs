#[doc = "Register `TBFT511` reader"]
pub type R = crate::R<TBFT511_SPEC>;
#[doc = "Field `NFTX` reader - 256 to 511 Byte Frames Transmitted without Error"]
pub type NFTX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 256 to 511 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new(self.bits)
    }
}
#[doc = "256 to 511 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbft511::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBFT511_SPEC;
impl crate::RegisterSpec for TBFT511_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbft511::R`](R) reader structure"]
impl crate::Readable for TBFT511_SPEC {}
#[doc = "`reset()` method sets TBFT511 to value 0"]
impl crate::Resettable for TBFT511_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
