#[doc = "Register `DPLLSYNCBUSY` reader"]
pub struct R(crate::R<DPLLSYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLLSYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLLSYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLLSYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENABLE` reader - DPLL Enable Synchronization Status"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLRATIO` reader - DPLL Loop Divider Ratio Synchronization Status"]
pub struct DPLLRATIO_R(crate::FieldReader<bool, bool>);
impl DPLLRATIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLRATIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLRATIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - DPLL Enable Synchronization Status"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DPLL Loop Divider Ratio Synchronization Status"]
    #[inline(always)]
    pub fn dpllratio(&self) -> DPLLRATIO_R {
        DPLLRATIO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "DPLL Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllsyncbusy](index.html) module"]
pub struct DPLLSYNCBUSY_SPEC;
impl crate::RegisterSpec for DPLLSYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpllsyncbusy::R](R) reader structure"]
impl crate::Readable for DPLLSYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DPLLSYNCBUSY to value 0"]
impl crate::Resettable for DPLLSYNCBUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
