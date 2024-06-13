#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BODVDDRDY` reader - BODVDD Ready"]
pub struct BODVDDRDY_R(crate::FieldReader<bool, bool>);
impl BODVDDRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODVDDRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODVDDRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODVDDDET` reader - BODVDD Detection"]
pub struct BODVDDDET_R(crate::FieldReader<bool, bool>);
impl BODVDDDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODVDDDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODVDDDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BVDDSRDY` reader - BODVDD Synchronization Ready"]
pub struct BVDDSRDY_R(crate::FieldReader<bool, bool>);
impl BVDDSRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BVDDSRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BVDDSRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODCORERDY` reader - BODCORE Ready"]
pub struct BODCORERDY_R(crate::FieldReader<bool, bool>);
impl BODCORERDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODCORERDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODCORERDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODCOREDET` reader - BODCORE Detection"]
pub struct BODCOREDET_R(crate::FieldReader<bool, bool>);
impl BODCOREDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODCOREDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODCOREDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCORESRDY` reader - BODCORE Synchronization Ready"]
pub struct BCORESRDY_R(crate::FieldReader<bool, bool>);
impl BCORESRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BCORESRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCORESRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - BODVDD Ready"]
    #[inline(always)]
    pub fn bodvddrdy(&self) -> BODVDDRDY_R {
        BODVDDRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BODVDD Detection"]
    #[inline(always)]
    pub fn bodvdddet(&self) -> BODVDDDET_R {
        BODVDDDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BODVDD Synchronization Ready"]
    #[inline(always)]
    pub fn bvddsrdy(&self) -> BVDDSRDY_R {
        BVDDSRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BODCORE Ready"]
    #[inline(always)]
    pub fn bodcorerdy(&self) -> BODCORERDY_R {
        BODCORERDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BODCORE Detection"]
    #[inline(always)]
    pub fn bodcoredet(&self) -> BODCOREDET_R {
        BODCOREDET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BODCORE Synchronization Ready"]
    #[inline(always)]
    pub fn bcoresrdy(&self) -> BCORESRDY_R {
        BCORESRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Power and Clocks Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
