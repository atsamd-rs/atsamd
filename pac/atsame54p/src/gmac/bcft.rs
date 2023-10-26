#[doc = "Register `BCFT` reader"]
pub type R = crate::R<BCFT_SPEC>;
#[doc = "Field `BFTX` reader - Broadcast Frames Transmitted without Error"]
pub type BFTX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Broadcast Frames Transmitted without Error"]
    #[inline(always)]
    pub fn bftx(&self) -> BFTX_R {
        BFTX_R::new(self.bits)
    }
}
#[doc = "Broadcast Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcft::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCFT_SPEC;
impl crate::RegisterSpec for BCFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcft::R`](R) reader structure"]
impl crate::Readable for BCFT_SPEC {}
#[doc = "`reset()` method sets BCFT to value 0"]
impl crate::Resettable for BCFT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
