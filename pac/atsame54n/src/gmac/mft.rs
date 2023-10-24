#[doc = "Register `MFT` reader"]
pub type R = crate::R<MFT_SPEC>;
#[doc = "Field `MFTX` reader - Multicast Frames Transmitted without Error"]
pub type MFTX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast Frames Transmitted without Error"]
    #[inline(always)]
    pub fn mftx(&self) -> MFTX_R {
        MFTX_R::new(self.bits)
    }
}
#[doc = "Multicast Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mft::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MFT_SPEC;
impl crate::RegisterSpec for MFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mft::R`](R) reader structure"]
impl crate::Readable for MFT_SPEC {}
#[doc = "`reset()` method sets MFT to value 0"]
impl crate::Resettable for MFT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
