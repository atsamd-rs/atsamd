#[doc = "Register `AFEC_CDR` reader"]
pub struct R(crate::R<AFEC_CDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_CDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_CDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_CDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Converted Data"]
pub struct DATA_R(crate::FieldReader<u16, u16>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "AFEC Channel Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cdr](index.html) module"]
pub struct AFEC_CDR_SPEC;
impl crate::RegisterSpec for AFEC_CDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_cdr::R](R) reader structure"]
impl crate::Readable for AFEC_CDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AFEC_CDR to value 0"]
impl crate::Resettable for AFEC_CDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
