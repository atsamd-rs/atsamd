#[doc = "Register `AES_CTRR` reader"]
pub struct R(crate::R<AES_CTRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_CTRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_CTRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_CTRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTR` reader - GCM Encryption Counter"]
pub struct CTR_R(crate::FieldReader<u32, u32>);
impl CTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - GCM Encryption Counter"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(self.bits as u32)
    }
}
#[doc = "GCM Encryption Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ctrr](index.html) module"]
pub struct AES_CTRR_SPEC;
impl crate::RegisterSpec for AES_CTRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_ctrr::R](R) reader structure"]
impl crate::Readable for AES_CTRR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AES_CTRR to value 0"]
impl crate::Resettable for AES_CTRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
