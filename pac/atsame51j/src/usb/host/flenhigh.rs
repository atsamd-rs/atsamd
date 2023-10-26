#[doc = "Register `FLENHIGH` reader"]
pub type R = crate::R<FLENHIGH_SPEC>;
#[doc = "Field `FLENHIGH` reader - Frame Length"]
pub type FLENHIGH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Frame Length"]
    #[inline(always)]
    pub fn flenhigh(&self) -> FLENHIGH_R {
        FLENHIGH_R::new(self.bits)
    }
}
#[doc = "HOST Host Frame Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flenhigh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLENHIGH_SPEC;
impl crate::RegisterSpec for FLENHIGH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`flenhigh::R`](R) reader structure"]
impl crate::Readable for FLENHIGH_SPEC {}
#[doc = "`reset()` method sets FLENHIGH to value 0"]
impl crate::Resettable for FLENHIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
