#[doc = "Register `HSMCI_RSPR[%s]` reader"]
pub struct R(crate::R<HSMCI_RSPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSMCI_RSPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSMCI_RSPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSMCI_RSPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSP` reader - Response"]
pub struct RSP_R(crate::FieldReader<u32, u32>);
impl RSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Response"]
    #[inline(always)]
    pub fn rsp(&self) -> RSP_R {
        RSP_R::new(self.bits as u32)
    }
}
#[doc = "Response Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_rspr](index.html) module"]
pub struct HSMCI_RSPR_SPEC;
impl crate::RegisterSpec for HSMCI_RSPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsmci_rspr::R](R) reader structure"]
impl crate::Readable for HSMCI_RSPR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSMCI_RSPR[%s]
to value 0"]
impl crate::Resettable for HSMCI_RSPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
