#[doc = "Register `TUR` reader"]
pub type R = crate::R<TUR_SPEC>;
#[doc = "Field `TXUNR` reader - Transmit Underruns"]
pub type TXUNR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Transmit Underruns"]
    #[inline(always)]
    pub fn txunr(&self) -> TXUNR_R {
        TXUNR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Transmit Underruns Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TUR_SPEC;
impl crate::RegisterSpec for TUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tur::R`](R) reader structure"]
impl crate::Readable for TUR_SPEC {}
#[doc = "`reset()` method sets TUR to value 0"]
impl crate::Resettable for TUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
