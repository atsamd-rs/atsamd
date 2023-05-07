#[doc = "Register `PID4` reader"]
pub struct R(crate::R<PID4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JEPCC` reader - JEP-106 Continuation Code"]
pub type JEPCC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FKBC` reader - 4KB Count"]
pub type FKBC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - JEP-106 Continuation Code"]
    #[inline(always)]
    pub fn jepcc(&self) -> JEPCC_R {
        JEPCC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB Count"]
    #[inline(always)]
    pub fn fkbc(&self) -> FKBC_R {
        FKBC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid4](index.html) module"]
pub struct PID4_SPEC;
impl crate::RegisterSpec for PID4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid4::R](R) reader structure"]
impl crate::Readable for PID4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PID4 to value 0"]
impl crate::Resettable for PID4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
