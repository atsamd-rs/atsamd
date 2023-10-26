#[doc = "Register `UCE` reader"]
pub type R = crate::R<UCE_SPEC>;
#[doc = "Field `UCKER` reader - UDP Checksum Errors"]
pub type UCKER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - UDP Checksum Errors"]
    #[inline(always)]
    pub fn ucker(&self) -> UCKER_R {
        UCKER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UDP Checksum Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uce::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCE_SPEC;
impl crate::RegisterSpec for UCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uce::R`](R) reader structure"]
impl crate::Readable for UCE_SPEC {}
#[doc = "`reset()` method sets UCE to value 0"]
impl crate::Resettable for UCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
