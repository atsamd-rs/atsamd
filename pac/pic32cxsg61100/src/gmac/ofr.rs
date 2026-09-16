#[doc = "Register `OFR` reader"]
pub type R = crate::R<OfrSpec>;
#[doc = "Field `OFRX` reader - Oversized Frames Received"]
pub type OfrxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Oversized Frames Received"]
    #[inline(always)]
    pub fn ofrx(&self) -> OfrxR {
        OfrxR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Oversize Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OfrSpec;
impl crate::RegisterSpec for OfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr::R`](R) reader structure"]
impl crate::Readable for OfrSpec {}
#[doc = "`reset()` method sets OFR to value 0"]
impl crate::Resettable for OfrSpec {
    const RESET_VALUE: u32 = 0;
}
