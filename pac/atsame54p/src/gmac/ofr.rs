#[doc = "Register `OFR` reader"]
pub type R = crate::R<OFR_SPEC>;
#[doc = "Field `OFRX` reader - Oversized Frames Received"]
pub type OFRX_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Oversized Frames Received"]
    #[inline(always)]
    pub fn ofrx(&self) -> OFRX_R {
        OFRX_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Oversize Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OFR_SPEC;
impl crate::RegisterSpec for OFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr::R`](R) reader structure"]
impl crate::Readable for OFR_SPEC {}
#[doc = "`reset()` method sets OFR to value 0"]
impl crate::Resettable for OFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
