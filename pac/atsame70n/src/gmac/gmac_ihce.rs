#[doc = "Register `GMAC_IHCE` reader"]
pub struct R(crate::R<GMAC_IHCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_IHCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_IHCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_IHCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HCKER` reader - IP Header Checksum Errors"]
pub struct HCKER_R(crate::FieldReader<u8, u8>);
impl HCKER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HCKER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCKER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - IP Header Checksum Errors"]
    #[inline(always)]
    pub fn hcker(&self) -> HCKER_R {
        HCKER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "IP Header Checksum Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ihce](index.html) module"]
pub struct GMAC_IHCE_SPEC;
impl crate::RegisterSpec for GMAC_IHCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_ihce::R](R) reader structure"]
impl crate::Readable for GMAC_IHCE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_IHCE to value 0"]
impl crate::Resettable for GMAC_IHCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
