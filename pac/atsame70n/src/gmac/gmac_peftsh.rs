#[doc = "Register `GMAC_PEFTSH` reader"]
pub struct R(crate::R<GMAC_PEFTSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_PEFTSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_PEFTSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_PEFTSH_SPEC>) -> Self {
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
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_peftsh](index.html) module"]
pub struct GMAC_PEFTSH_SPEC;
impl crate::RegisterSpec for GMAC_PEFTSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_peftsh::R](R) reader structure"]
impl crate::Readable for GMAC_PEFTSH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_PEFTSH to value 0"]
impl crate::Resettable for GMAC_PEFTSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
