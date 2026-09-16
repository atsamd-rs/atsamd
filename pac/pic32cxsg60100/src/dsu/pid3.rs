#[doc = "Register `PID3` reader"]
pub type R = crate::R<Pid3Spec>;
#[doc = "Field `CUSMOD` reader - ARM CUSMOD"]
pub type CusmodR = crate::FieldReader;
#[doc = "Field `REVAND` reader - Revision Number"]
pub type RevandR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - ARM CUSMOD"]
    #[inline(always)]
    pub fn cusmod(&self) -> CusmodR {
        CusmodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Revision Number"]
    #[inline(always)]
    pub fn revand(&self) -> RevandR {
        RevandR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid3Spec;
impl crate::RegisterSpec for Pid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid3::R`](R) reader structure"]
impl crate::Readable for Pid3Spec {}
#[doc = "`reset()` method sets PID3 to value 0"]
impl crate::Resettable for Pid3Spec {
    const RESET_VALUE: u32 = 0;
}
