#[doc = "Register `EFRSH` reader"]
pub struct R(crate::R<EFRSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFRSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFRSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFRSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RUD` reader - Register Update"]
pub struct RUD_R(crate::FieldReader<u16, u16>);
impl RUD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RUD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Event Frame Received Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efrsh](index.html) module"]
pub struct EFRSH_SPEC;
impl crate::RegisterSpec for EFRSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efrsh::R](R) reader structure"]
impl crate::Readable for EFRSH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EFRSH to value 0"]
impl crate::Resettable for EFRSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
