#[doc = "Register `TLPITR` reader"]
pub struct R(crate::R<TLPITR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TLPITR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TLPITR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TLPITR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TLPITR` reader - Count number of times enable LPI tx bit 20 goes from low to high"]
pub struct TLPITR_R(crate::FieldReader<u16, u16>);
impl TLPITR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TLPITR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLPITR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Count number of times enable LPI tx bit 20 goes from low to high"]
    #[inline(always)]
    pub fn tlpitr(&self) -> TLPITR_R {
        TLPITR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive LPI transition Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlpitr](index.html) module"]
pub struct TLPITR_SPEC;
impl crate::RegisterSpec for TLPITR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tlpitr::R](R) reader structure"]
impl crate::Readable for TLPITR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TLPITR to value 0"]
impl crate::Resettable for TLPITR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
