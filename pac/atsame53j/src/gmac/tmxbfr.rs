#[doc = "Register `TMXBFR` reader"]
pub type R = crate::R<TmxbfrSpec>;
#[doc = "Field `NFRX` reader - 1519 to Maximum Byte Frames Received without Error"]
pub type NfrxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 1519 to Maximum Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NfrxR {
        NfrxR::new(self.bits)
    }
}
#[doc = "1519 to Maximum Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmxbfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmxbfrSpec;
impl crate::RegisterSpec for TmxbfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmxbfr::R`](R) reader structure"]
impl crate::Readable for TmxbfrSpec {}
#[doc = "`reset()` method sets TMXBFR to value 0"]
impl crate::Resettable for TmxbfrSpec {
    const RESET_VALUE: u32 = 0;
}
