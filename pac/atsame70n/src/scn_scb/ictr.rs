#[doc = "Register `ICTR` reader"]
pub struct R(crate::R<ICTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTLINESNUM` reader - Total number of interrupt lines supported by an implementation, defined in groups of 32"]
pub struct INTLINESNUM_R(crate::FieldReader<u8, u8>);
impl INTLINESNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTLINESNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTLINESNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Total number of interrupt lines supported by an implementation, defined in groups of 32"]
    #[inline(always)]
    pub fn intlinesnum(&self) -> INTLINESNUM_R {
        INTLINESNUM_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Interrupt Controller Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ictr](index.html) module"]
pub struct ICTR_SPEC;
impl crate::RegisterSpec for ICTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ictr::R](R) reader structure"]
impl crate::Readable for ICTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ICTR to value 0"]
impl crate::Resettable for ICTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
