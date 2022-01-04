#[doc = "Register `RUNLOCK` reader"]
pub struct R(crate::R<RUNLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RUNLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RUNLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RUNLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RUNLOCK` reader - Region Un-Lock Bits"]
pub struct RUNLOCK_R(crate::FieldReader<u32, u32>);
impl RUNLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RUNLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNLOCK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Region Un-Lock Bits"]
    #[inline(always)]
    pub fn runlock(&self) -> RUNLOCK_R {
        RUNLOCK_R::new(self.bits as u32)
    }
}
#[doc = "Lock Section\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [runlock](index.html) module"]
pub struct RUNLOCK_SPEC;
impl crate::RegisterSpec for RUNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [runlock::R](R) reader structure"]
impl crate::Readable for RUNLOCK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RUNLOCK to value 0"]
impl crate::Resettable for RUNLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
