#[doc = "Register `PID4` reader"]
pub type R = crate::R<PID4_SPEC>;
#[doc = "Field `JEPCC` reader - JEP-106 Continuation Code"]
pub type JEPCC_R = crate::FieldReader;
#[doc = "Field `FKBC` reader - 4KB count"]
pub type FKBC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - JEP-106 Continuation Code"]
    #[inline(always)]
    pub fn jepcc(&self) -> JEPCC_R {
        JEPCC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB count"]
    #[inline(always)]
    pub fn fkbc(&self) -> FKBC_R {
        FKBC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PID4_SPEC;
impl crate::RegisterSpec for PID4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid4::R`](R) reader structure"]
impl crate::Readable for PID4_SPEC {}
#[doc = "`reset()` method sets PID4 to value 0"]
impl crate::Resettable for PID4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
