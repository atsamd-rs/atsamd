#[doc = "Register `HSMCI_RDR` reader"]
pub struct R(crate::R<HSMCI_RDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSMCI_RDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSMCI_RDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSMCI_RDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Data to Read"]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Data to Read"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits as u32)
    }
}
#[doc = "Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_rdr](index.html) module"]
pub struct HSMCI_RDR_SPEC;
impl crate::RegisterSpec for HSMCI_RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsmci_rdr::R](R) reader structure"]
impl crate::Readable for HSMCI_RDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSMCI_RDR to value 0"]
impl crate::Resettable for HSMCI_RDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
