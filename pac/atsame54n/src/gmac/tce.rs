#[doc = "Register `TCE` reader"]
pub type R = crate::R<TCE_SPEC>;
#[doc = "Field `TCKER` reader - TCP Checksum Errors"]
pub type TCKER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - TCP Checksum Errors"]
    #[inline(always)]
    pub fn tcker(&self) -> TCKER_R {
        TCKER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TCP Checksum Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tce::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCE_SPEC;
impl crate::RegisterSpec for TCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tce::R`](R) reader structure"]
impl crate::Readable for TCE_SPEC {}
#[doc = "`reset()` method sets TCE to value 0"]
impl crate::Resettable for TCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
