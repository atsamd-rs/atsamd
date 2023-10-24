#[doc = "Register `ITATBCTR2` reader"]
pub type R = crate::R<ITATBCTR2_SPEC>;
#[doc = "Field `ATREADY` reader - "]
pub type ATREADY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn atready(&self) -> ATREADY_R {
        ATREADY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "ITATBCTR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itatbctr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITATBCTR2_SPEC;
impl crate::RegisterSpec for ITATBCTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itatbctr2::R`](R) reader structure"]
impl crate::Readable for ITATBCTR2_SPEC {}
#[doc = "`reset()` method sets ITATBCTR2 to value 0"]
impl crate::Resettable for ITATBCTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
