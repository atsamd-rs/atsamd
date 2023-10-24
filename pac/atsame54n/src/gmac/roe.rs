#[doc = "Register `ROE` reader"]
pub type R = crate::R<ROE_SPEC>;
#[doc = "Field `RXOVR` reader - Receive Overruns"]
pub type RXOVR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Receive Overruns"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Receive Overrun Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`roe::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROE_SPEC;
impl crate::RegisterSpec for ROE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`roe::R`](R) reader structure"]
impl crate::Readable for ROE_SPEC {}
#[doc = "`reset()` method sets ROE to value 0"]
impl crate::Resettable for ROE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
