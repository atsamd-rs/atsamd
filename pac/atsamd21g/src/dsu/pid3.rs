#[doc = "Register `PID3` reader"]
pub type R = crate::R<PID3_SPEC>;
#[doc = "Field `CUSMOD` reader - ARM CUSMOD"]
pub type CUSMOD_R = crate::FieldReader;
#[doc = "Field `REVAND` reader - Revision Number"]
pub type REVAND_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - ARM CUSMOD"]
    #[inline(always)]
    pub fn cusmod(&self) -> CUSMOD_R {
        CUSMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Revision Number"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PID3_SPEC;
impl crate::RegisterSpec for PID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid3::R`](R) reader structure"]
impl crate::Readable for PID3_SPEC {}
#[doc = "`reset()` method sets PID3 to value 0"]
impl crate::Resettable for PID3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
