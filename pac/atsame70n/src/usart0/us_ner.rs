#[doc = "Register `US_NER` reader"]
pub struct R(crate::R<US_NER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_NER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_NER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_NER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NB_ERRORS` reader - Number of Errors"]
pub struct NB_ERRORS_R(crate::FieldReader<u8, u8>);
impl NB_ERRORS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NB_ERRORS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NB_ERRORS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Errors"]
    #[inline(always)]
    pub fn nb_errors(&self) -> NB_ERRORS_R {
        NB_ERRORS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_ner](index.html) module"]
pub struct US_NER_SPEC;
impl crate::RegisterSpec for US_NER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_ner::R](R) reader structure"]
impl crate::Readable for US_NER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_NER to value 0"]
impl crate::Resettable for US_NER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
