#[doc = "Register `FCSE` reader"]
pub type R = crate::R<FCSE_SPEC>;
#[doc = "Field `FCKR` reader - Frame Check Sequence Errors"]
pub type FCKR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Frame Check Sequence Errors"]
    #[inline(always)]
    pub fn fckr(&self) -> FCKR_R {
        FCKR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Frame Check Sequence Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcse::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCSE_SPEC;
impl crate::RegisterSpec for FCSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcse::R`](R) reader structure"]
impl crate::Readable for FCSE_SPEC {}
#[doc = "`reset()` method sets FCSE to value 0"]
impl crate::Resettable for FCSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
