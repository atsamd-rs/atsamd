#[doc = "Register `SEQSTATUS` reader"]
pub struct R(crate::R<SEQSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEQSTATE` reader - Sequence State"]
pub struct SEQSTATE_R(crate::FieldReader<u8, u8>);
impl SEQSTATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEQSTATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQSTATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQBUSY` reader - Sequence Busy"]
pub struct SEQBUSY_R(crate::FieldReader<bool, bool>);
impl SEQBUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEQBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Sequence State"]
    #[inline(always)]
    pub fn seqstate(&self) -> SEQSTATE_R {
        SEQSTATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Sequence Busy"]
    #[inline(always)]
    pub fn seqbusy(&self) -> SEQBUSY_R {
        SEQBUSY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Sequence Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqstatus](index.html) module"]
pub struct SEQSTATUS_SPEC;
impl crate::RegisterSpec for SEQSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [seqstatus::R](R) reader structure"]
impl crate::Readable for SEQSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SEQSTATUS to value 0"]
impl crate::Resettable for SEQSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
