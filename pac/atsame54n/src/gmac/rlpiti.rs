#[doc = "Register `RLPITI` reader"]
pub type R = crate::R<RLPITI_SPEC>;
#[doc = "Field `RLPITI` reader - Increment once over 16 ahb clock when LPI indication bit 20 is set in rx mode"]
pub type RLPITI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Increment once over 16 ahb clock when LPI indication bit 20 is set in rx mode"]
    #[inline(always)]
    pub fn rlpiti(&self) -> RLPITI_R {
        RLPITI_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Receive LPI Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlpiti::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RLPITI_SPEC;
impl crate::RegisterSpec for RLPITI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlpiti::R`](R) reader structure"]
impl crate::Readable for RLPITI_SPEC {}
#[doc = "`reset()` method sets RLPITI to value 0"]
impl crate::Resettable for RLPITI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
