#[doc = "Register `PID4` reader"]
pub type R = crate::R<Pid4Spec>;
#[doc = "Field `JEPCC` reader - JEP-106 Continuation Code"]
pub type JepccR = crate::FieldReader;
#[doc = "Field `FKBC` reader - 4KB count"]
pub type FkbcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - JEP-106 Continuation Code"]
    #[inline(always)]
    pub fn jepcc(&self) -> JepccR {
        JepccR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB count"]
    #[inline(always)]
    pub fn fkbc(&self) -> FkbcR {
        FkbcR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pid4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid4Spec;
impl crate::RegisterSpec for Pid4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid4::R`](R) reader structure"]
impl crate::Readable for Pid4Spec {}
#[doc = "`reset()` method sets PID4 to value 0"]
impl crate::Resettable for Pid4Spec {
    const RESET_VALUE: u32 = 0;
}
