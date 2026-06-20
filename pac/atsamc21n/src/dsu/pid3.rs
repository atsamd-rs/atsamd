#[doc = "Register `PID3` reader"]
pub struct R(crate::R<PID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CUSMOD` reader - ARM CUSMOD"]
pub struct CUSMOD_R(crate::FieldReader<u8, u8>);
impl CUSMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CUSMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUSMOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVAND` reader - Revision Number"]
pub struct REVAND_R(crate::FieldReader<u8, u8>);
impl REVAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REVAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVAND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - ARM CUSMOD"]
    #[inline(always)]
    pub fn cusmod(&self) -> CUSMOD_R {
        CUSMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Revision Number"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid3](index.html) module"]
pub struct PID3_SPEC;
impl crate::RegisterSpec for PID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid3::R](R) reader structure"]
impl crate::Readable for PID3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PID3 to value 0"]
impl crate::Resettable for PID3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
