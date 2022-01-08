#[doc = "Register `PSTATUS` reader"]
pub struct R(crate::R<PSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DTGL` reader - Data Toggle"]
pub struct DTGL_R(crate::FieldReader<bool, bool>);
impl DTGL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTGL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTGL_R {
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
#[doc = "Field `PFREEZE` reader - Pipe Freeze"]
pub struct PFREEZE_R(crate::FieldReader<bool, bool>);
impl PFREEZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PFREEZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFREEZE_R {
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
    #[doc = "Bit 0 - Data Toggle"]
    #[inline(always)]
    pub fn dtgl(&self) -> DTGL_R {
        DTGL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Current Bank"]
    #[inline(always)]
    pub fn curbk(&self) -> CURBK_R {
        CURBK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pipe Freeze"]
    #[inline(always)]
    pub fn pfreeze(&self) -> PFREEZE_R {
        PFREEZE_R::new(((self.bits >> 4) & 0x01) != 0)
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
#[doc = "HOST_PIPE End Point Pipe Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstatus](index.html) module"]
pub struct PSTATUS_SPEC;
impl crate::RegisterSpec for PSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pstatus::R](R) reader structure"]
impl crate::Readable for PSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSTATUS to value 0"]
impl crate::Resettable for PSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
