#[doc = "Register `TLPITI` reader"]
pub type R = crate::R<TLPITI_SPEC>;
#[doc = "Field `TLPITI` reader - Increment once over 16 ahb clock when LPI indication bit 20 is set in tx mode"]
pub type TLPITI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Increment once over 16 ahb clock when LPI indication bit 20 is set in tx mode"]
    #[inline(always)]
    pub fn tlpiti(&self) -> TLPITI_R {
        TLPITI_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Receive LPI Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tlpiti::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TLPITI_SPEC;
impl crate::RegisterSpec for TLPITI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tlpiti::R`](R) reader structure"]
impl crate::Readable for TLPITI_SPEC {}
#[doc = "`reset()` method sets TLPITI to value 0"]
impl crate::Resettable for TLPITI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
