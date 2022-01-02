#[doc = "Register `RESS` reader"]
pub struct R(crate::R<RESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESS` reader - Last ADC conversion result"]
pub struct RESS_R(crate::FieldReader<u16, u16>);
impl RESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Last ADC conversion result"]
    #[inline(always)]
    pub fn ress(&self) -> RESS_R {
        RESS_R::new(self.bits as u16)
    }
}
#[doc = "Last Sample Result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ress](index.html) module"]
pub struct RESS_SPEC;
impl crate::RegisterSpec for RESS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ress::R](R) reader structure"]
impl crate::Readable for RESS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESS to value 0"]
impl crate::Resettable for RESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
