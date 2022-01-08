#[doc = "Register `FLENHIGH` reader"]
pub struct R(crate::R<FLENHIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLENHIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLENHIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLENHIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLENHIGH` reader - Frame Length"]
pub struct FLENHIGH_R(crate::FieldReader<u8, u8>);
impl FLENHIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLENHIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLENHIGH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Frame Length"]
    #[inline(always)]
    pub fn flenhigh(&self) -> FLENHIGH_R {
        FLENHIGH_R::new(self.bits as u8)
    }
}
#[doc = "HOST Host Frame Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flenhigh](index.html) module"]
pub struct FLENHIGH_SPEC;
impl crate::RegisterSpec for FLENHIGH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [flenhigh::R](R) reader structure"]
impl crate::Readable for FLENHIGH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLENHIGH to value 0"]
impl crate::Resettable for FLENHIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
