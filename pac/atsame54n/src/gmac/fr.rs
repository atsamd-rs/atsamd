#[doc = "Register `FR` reader"]
pub type R = crate::R<FR_SPEC>;
#[doc = "Field `FRX` reader - Frames Received without Error"]
pub type FRX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Frames Received without Error"]
    #[inline(always)]
    pub fn frx(&self) -> FRX_R {
        FRX_R::new(self.bits)
    }
}
#[doc = "Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr::R`](R) reader structure"]
impl crate::Readable for FR_SPEC {}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
