#[doc = "Register `EPSTATUS%s` reader"]
pub struct R(crate::R<EPSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DTGLOUT` reader - Data Toggle Out"]
pub struct DTGLOUT_R(crate::FieldReader<bool, bool>);
impl DTGLOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTGLOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTGLOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTGLIN` reader - Data Toggle In"]
pub struct DTGLIN_R(crate::FieldReader<bool, bool>);
impl DTGLIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTGLIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTGLIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURBK` reader - Current Bank"]
pub struct CURBK_R(crate::FieldReader<bool, bool>);
impl CURBK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CURBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURBK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALLRQ0` reader - Stall 0 Request"]
pub struct STALLRQ0_R(crate::FieldReader<bool, bool>);
impl STALLRQ0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALLRQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLRQ0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALLRQ1` reader - Stall 1 Request"]
pub struct STALLRQ1_R(crate::FieldReader<bool, bool>);
impl STALLRQ1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALLRQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLRQ1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK0RDY` reader - Bank 0 ready"]
pub struct BK0RDY_R(crate::FieldReader<bool, bool>);
impl BK0RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BK0RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK0RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK1RDY` reader - Bank 1 ready"]
pub struct BK1RDY_R(crate::FieldReader<bool, bool>);
impl BK1RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BK1RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK1RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Data Toggle Out"]
    #[inline(always)]
    pub fn dtglout(&self) -> DTGLOUT_R {
        DTGLOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Toggle In"]
    #[inline(always)]
    pub fn dtglin(&self) -> DTGLIN_R {
        DTGLIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Current Bank"]
    #[inline(always)]
    pub fn curbk(&self) -> CURBK_R {
        CURBK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stall 0 Request"]
    #[inline(always)]
    pub fn stallrq0(&self) -> STALLRQ0_R {
        STALLRQ0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stall 1 Request"]
    #[inline(always)]
    pub fn stallrq1(&self) -> STALLRQ1_R {
        STALLRQ1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bank 0 ready"]
    #[inline(always)]
    pub fn bk0rdy(&self) -> BK0RDY_R {
        BK0RDY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bank 1 ready"]
    #[inline(always)]
    pub fn bk1rdy(&self) -> BK1RDY_R {
        BK1RDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "DEVICE End Point Pipe Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstatus](index.html) module"]
pub struct EPSTATUS_SPEC;
impl crate::RegisterSpec for EPSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [epstatus::R](R) reader structure"]
impl crate::Readable for EPSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPSTATUS%s to value 0"]
impl crate::Resettable for EPSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
