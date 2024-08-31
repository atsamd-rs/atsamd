#[doc = "Register `ITATBCTR0` reader"]
pub type R = crate::R<Itatbctr0Spec>;
#[doc = "Field `ATREADY` reader - "]
pub type AtreadyR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn atready(&self) -> AtreadyR {
        AtreadyR::new((self.bits & 1) != 0)
    }
}
#[doc = "ITATBCTR0\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itatbctr0Spec;
impl crate::RegisterSpec for Itatbctr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itatbctr0::R`](R) reader structure"]
impl crate::Readable for Itatbctr0Spec {}
#[doc = "`reset()` method sets ITATBCTR0 to value 0"]
impl crate::Resettable for Itatbctr0Spec {
    const RESET_VALUE: u32 = 0;
}
