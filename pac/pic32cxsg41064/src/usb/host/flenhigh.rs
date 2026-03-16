#[doc = "Register `FLENHIGH` reader"]
pub type R = crate::R<FlenhighSpec>;
#[doc = "Field `FLENHIGH` reader - Frame Length"]
pub type FlenhighR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Frame Length"]
    #[inline(always)]
    pub fn flenhigh(&self) -> FlenhighR {
        FlenhighR::new(self.bits)
    }
}
#[doc = "HOST Host Frame Length\n\nYou can [`read`](crate::Reg::read) this register and get [`flenhigh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlenhighSpec;
impl crate::RegisterSpec for FlenhighSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`flenhigh::R`](R) reader structure"]
impl crate::Readable for FlenhighSpec {}
#[doc = "`reset()` method sets FLENHIGH to value 0"]
impl crate::Resettable for FlenhighSpec {
    const RESET_VALUE: u8 = 0;
}
