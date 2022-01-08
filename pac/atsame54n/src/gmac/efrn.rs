#[doc = "Register `EFRN` reader"]
pub struct R(crate::R<EFRN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFRN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFRN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFRN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RUD` reader - Register Update"]
pub struct RUD_R(crate::FieldReader<u32, u32>);
impl RUD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RUD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:29 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
#[doc = "PTP Event Frame Received Nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efrn](index.html) module"]
pub struct EFRN_SPEC;
impl crate::RegisterSpec for EFRN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efrn::R](R) reader structure"]
impl crate::Readable for EFRN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EFRN to value 0"]
impl crate::Resettable for EFRN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
