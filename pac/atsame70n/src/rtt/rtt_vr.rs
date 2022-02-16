#[doc = "Register `RTT_VR` reader"]
pub struct R(crate::R<RTT_VR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTT_VR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTT_VR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTT_VR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRTV` reader - Current Real-time Value"]
pub struct CRTV_R(crate::FieldReader<u32, u32>);
impl CRTV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CRTV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRTV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Current Real-time Value"]
    #[inline(always)]
    pub fn crtv(&self) -> CRTV_R {
        CRTV_R::new(self.bits as u32)
    }
}
#[doc = "Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtt_vr](index.html) module"]
pub struct RTT_VR_SPEC;
impl crate::RegisterSpec for RTT_VR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtt_vr::R](R) reader structure"]
impl crate::Readable for RTT_VR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTT_VR to value 0"]
impl crate::Resettable for RTT_VR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
