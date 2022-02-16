#[doc = "Register `GMAC_TCE` reader"]
pub struct R(crate::R<GMAC_TCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_TCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_TCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_TCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TCKER` reader - TCP Checksum Errors"]
pub struct TCKER_R(crate::FieldReader<u8, u8>);
impl TCKER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCKER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCKER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - TCP Checksum Errors"]
    #[inline(always)]
    pub fn tcker(&self) -> TCKER_R {
        TCKER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TCP Checksum Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tce](index.html) module"]
pub struct GMAC_TCE_SPEC;
impl crate::RegisterSpec for GMAC_TCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_tce::R](R) reader structure"]
impl crate::Readable for GMAC_TCE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_TCE to value 0"]
impl crate::Resettable for GMAC_TCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
