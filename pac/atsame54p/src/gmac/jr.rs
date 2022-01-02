#[doc = "Register `JR` reader"]
pub struct R(crate::R<JR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JRX` reader - Jabbers Received"]
pub struct JRX_R(crate::FieldReader<u16, u16>);
impl JRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        JRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JRX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Jabbers Received"]
    #[inline(always)]
    pub fn jrx(&self) -> JRX_R {
        JRX_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Jabbers Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jr](index.html) module"]
pub struct JR_SPEC;
impl crate::RegisterSpec for JR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jr::R](R) reader structure"]
impl crate::Readable for JR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JR to value 0"]
impl crate::Resettable for JR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
