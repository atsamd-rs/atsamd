#[doc = "Register `BUSYCH` reader"]
pub struct R(crate::R<BUSYCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSYCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSYCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSYCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSYCH0` reader - Busy Channel 0"]
pub struct BUSYCH0_R(crate::FieldReader<bool, bool>);
impl BUSYCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSYCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSYCH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSYCH1` reader - Busy Channel 1"]
pub struct BUSYCH1_R(crate::FieldReader<bool, bool>);
impl BUSYCH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSYCH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSYCH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSYCH2` reader - Busy Channel 2"]
pub struct BUSYCH2_R(crate::FieldReader<bool, bool>);
impl BUSYCH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSYCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSYCH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSYCH3` reader - Busy Channel 3"]
pub struct BUSYCH3_R(crate::FieldReader<bool, bool>);
impl BUSYCH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSYCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSYCH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSYCH4` reader - Busy Channel 4"]
pub struct BUSYCH4_R(crate::FieldReader<bool, bool>);
impl BUSYCH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSYCH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSYCH4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSYCH5` reader - Busy Channel 5"]
pub struct BUSYCH5_R(crate::FieldReader<bool, bool>);
impl BUSYCH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSYCH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSYCH5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Busy Channel 0"]
    #[inline(always)]
    pub fn busych0(&self) -> BUSYCH0_R {
        BUSYCH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Busy Channel 1"]
    #[inline(always)]
    pub fn busych1(&self) -> BUSYCH1_R {
        BUSYCH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Busy Channel 2"]
    #[inline(always)]
    pub fn busych2(&self) -> BUSYCH2_R {
        BUSYCH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Busy Channel 3"]
    #[inline(always)]
    pub fn busych3(&self) -> BUSYCH3_R {
        BUSYCH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Busy Channel 4"]
    #[inline(always)]
    pub fn busych4(&self) -> BUSYCH4_R {
        BUSYCH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Busy Channel 5"]
    #[inline(always)]
    pub fn busych5(&self) -> BUSYCH5_R {
        BUSYCH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Busy Channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busych](index.html) module"]
pub struct BUSYCH_SPEC;
impl crate::RegisterSpec for BUSYCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busych::R](R) reader structure"]
impl crate::Readable for BUSYCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUSYCH to value 0"]
impl crate::Resettable for BUSYCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
