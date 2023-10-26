#[doc = "Register `FT` reader"]
pub type R = crate::R<FT_SPEC>;
#[doc = "Field `FTX` reader - Frames Transmitted without Error"]
pub type FTX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Frames Transmitted without Error"]
    #[inline(always)]
    pub fn ftx(&self) -> FTX_R {
        FTX_R::new(self.bits)
    }
}
#[doc = "Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ft::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FT_SPEC;
impl crate::RegisterSpec for FT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ft::R`](R) reader structure"]
impl crate::Readable for FT_SPEC {}
#[doc = "`reset()` method sets FT to value 0"]
impl crate::Resettable for FT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
