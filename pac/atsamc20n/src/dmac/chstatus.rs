#[doc = "Register `CHSTATUS` reader"]
pub struct R(crate::R<CHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PEND` reader - Channel Pending"]
pub struct PEND_R(crate::FieldReader<bool, bool>);
impl PEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - Channel Busy"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FERR` reader - Channel Fetch Error"]
pub struct FERR_R(crate::FieldReader<bool, bool>);
impl FERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Channel Pending"]
    #[inline(always)]
    pub fn pend(&self) -> PEND_R {
        PEND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel Fetch Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "Channel Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chstatus](index.html) module"]
pub struct CHSTATUS_SPEC;
impl crate::RegisterSpec for CHSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chstatus::R](R) reader structure"]
impl crate::Readable for CHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHSTATUS to value 0"]
impl crate::Resettable for CHSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
