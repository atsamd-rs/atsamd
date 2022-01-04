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
#[doc = "Field `BOD33RDY` reader - BOD33 Ready"]
pub struct BOD33RDY_R(crate::FieldReader<bool, bool>);
impl BOD33RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOD33RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD33RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD33DET` reader - BOD33 Detection"]
pub struct BOD33DET_R(crate::FieldReader<bool, bool>);
impl BOD33DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOD33DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD33DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready"]
pub struct B33SRDY_R(crate::FieldReader<bool, bool>);
impl B33SRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        B33SRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B33SRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREGRDY` reader - Voltage Regulator Ready"]
pub struct VREGRDY_R(crate::FieldReader<bool, bool>);
impl VREGRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VREGRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREGRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCORERDY` reader - VDDCORE Ready"]
pub struct VCORERDY_R(crate::FieldReader<bool, bool>);
impl VCORERDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCORERDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCORERDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> BOD33RDY_R {
        BOD33RDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33SRDY_R {
        B33SRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    pub fn vregrdy(&self) -> VREGRDY_R {
        VREGRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline(always)]
    pub fn vcorerdy(&self) -> VCORERDY_R {
        VCORERDY_R::new(((self.bits >> 10) & 0x01) != 0)
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
