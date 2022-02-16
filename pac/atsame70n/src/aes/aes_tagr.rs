#[doc = "Register `AES_TAGR[%s]` reader"]
pub struct R(crate::R<AES_TAGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_TAGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_TAGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_TAGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAG` reader - GCM Authentication Tag x"]
pub struct TAG_R(crate::FieldReader<u32, u32>);
impl TAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - GCM Authentication Tag x"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(self.bits as u32)
    }
}
#[doc = "GCM Authentication Tag Word Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_tagr](index.html) module"]
pub struct AES_TAGR_SPEC;
impl crate::RegisterSpec for AES_TAGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_tagr::R](R) reader structure"]
impl crate::Readable for AES_TAGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AES_TAGR[%s]
to value 0"]
impl crate::Resettable for AES_TAGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
