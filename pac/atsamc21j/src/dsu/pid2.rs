#[doc = "Register `PID2` reader"]
pub struct R(crate::R<PID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JEPIDCH` reader - JEP-106 Identity Code High"]
pub struct JEPIDCH_R(crate::FieldReader<u8, u8>);
impl JEPIDCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        JEPIDCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEPIDCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEPU` reader - JEP-106 Identity Code is used"]
pub struct JEPU_R(crate::FieldReader<bool, bool>);
impl JEPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JEPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVISION` reader - Revision Number"]
pub struct REVISION_R(crate::FieldReader<u8, u8>);
impl REVISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - JEP-106 Identity Code High"]
    #[inline(always)]
    pub fn jepidch(&self) -> JEPIDCH_R {
        JEPIDCH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - JEP-106 Identity Code is used"]
    #[inline(always)]
    pub fn jepu(&self) -> JEPU_R {
        JEPU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Revision Number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid2](index.html) module"]
pub struct PID2_SPEC;
impl crate::RegisterSpec for PID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid2::R](R) reader structure"]
impl crate::Readable for PID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PID2 to value 0x09"]
impl crate::Resettable for PID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x09
    }
}
