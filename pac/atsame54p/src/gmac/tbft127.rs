#[doc = "Register `TBFT127` reader"]
pub type R = crate::R<TBFT127_SPEC>;
#[doc = "Field `NFTX` reader - 65 to 127 Byte Frames Transmitted without Error"]
pub type NFTX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 65 to 127 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new(self.bits)
    }
}
#[doc = "65 to 127 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbft127::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBFT127_SPEC;
impl crate::RegisterSpec for TBFT127_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbft127::R`](R) reader structure"]
impl crate::Readable for TBFT127_SPEC {}
#[doc = "`reset()` method sets TBFT127 to value 0"]
impl crate::Resettable for TBFT127_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
