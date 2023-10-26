#[doc = "Register `IHCE` reader"]
pub type R = crate::R<IHCE_SPEC>;
#[doc = "Field `HCKER` reader - IP Header Checksum Errors"]
pub type HCKER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - IP Header Checksum Errors"]
    #[inline(always)]
    pub fn hcker(&self) -> HCKER_R {
        HCKER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "IP Header Checksum Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ihce::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IHCE_SPEC;
impl crate::RegisterSpec for IHCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ihce::R`](R) reader structure"]
impl crate::Readable for IHCE_SPEC {}
#[doc = "`reset()` method sets IHCE to value 0"]
impl crate::Resettable for IHCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
