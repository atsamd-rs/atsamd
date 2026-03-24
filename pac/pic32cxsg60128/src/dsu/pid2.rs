#[doc = "Register `PID2` reader"]
pub type R = crate::R<Pid2Spec>;
#[doc = "Field `JEPIDCH` reader - JEP-106 Identity Code High"]
pub type JepidchR = crate::FieldReader;
#[doc = "Field `JEPU` reader - JEP-106 Identity Code is used"]
pub type JepuR = crate::BitReader;
#[doc = "Field `REVISION` reader - Revision Number"]
pub type RevisionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - JEP-106 Identity Code High"]
    #[inline(always)]
    pub fn jepidch(&self) -> JepidchR {
        JepidchR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - JEP-106 Identity Code is used"]
    #[inline(always)]
    pub fn jepu(&self) -> JepuR {
        JepuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Revision Number"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid2Spec;
impl crate::RegisterSpec for Pid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid2::R`](R) reader structure"]
impl crate::Readable for Pid2Spec {}
#[doc = "`reset()` method sets PID2 to value 0x09"]
impl crate::Resettable for Pid2Spec {
    const RESET_VALUE: u32 = 0x09;
}
