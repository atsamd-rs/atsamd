#[doc = "Register `TBFR511` reader"]
pub struct R(crate::R<TBFR511_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBFR511_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBFR511_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBFR511_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NFRX` reader - 256 to 511 Byte Frames Received without Error"]
pub type NFRX_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 256 to 511 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NFRX_R {
        NFRX_R::new(self.bits)
    }
}
#[doc = "256 to 511Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr511](index.html) module"]
pub struct TBFR511_SPEC;
impl crate::RegisterSpec for TBFR511_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbfr511::R](R) reader structure"]
impl crate::Readable for TBFR511_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TBFR511 to value 0"]
impl crate::Resettable for TBFR511_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
