#[doc = "Register `UCE` reader"]
pub struct R(crate::R<UCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UCKER` reader - UDP Checksum Errors"]
pub struct UCKER_R(crate::FieldReader<u8, u8>);
impl UCKER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UCKER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCKER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - UDP Checksum Errors"]
    #[inline(always)]
    pub fn ucker(&self) -> UCKER_R {
        UCKER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UDP Checksum Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uce](index.html) module"]
pub struct UCE_SPEC;
impl crate::RegisterSpec for UCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uce::R](R) reader structure"]
impl crate::Readable for UCE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UCE to value 0"]
impl crate::Resettable for UCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
