#[doc = "Register `TRIGGER` reader"]
pub struct R(crate::R<TRIGGER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGGER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGGER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGGER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRIGGER` reader - "]
pub type TRIGGER_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 1) != 0)
    }
}
#[doc = "TRIGGER\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigger](index.html) module"]
pub struct TRIGGER_SPEC;
impl crate::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigger::R](R) reader structure"]
impl crate::Readable for TRIGGER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
