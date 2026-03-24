#[doc = "Register `FCSE` reader"]
pub type R = crate::R<FcseSpec>;
#[doc = "Field `FCKR` reader - Frame Check Sequence Errors"]
pub type FckrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Frame Check Sequence Errors"]
    #[inline(always)]
    pub fn fckr(&self) -> FckrR {
        FckrR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Frame Check Sequence Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcse::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcseSpec;
impl crate::RegisterSpec for FcseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcse::R`](R) reader structure"]
impl crate::Readable for FcseSpec {}
#[doc = "`reset()` method sets FCSE to value 0"]
impl crate::Resettable for FcseSpec {
    const RESET_VALUE: u32 = 0;
}
