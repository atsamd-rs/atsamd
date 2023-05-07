#[doc = "Register `OTHI` reader"]
pub struct R(crate::R<OTHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXO` reader - Transmitted Octets"]
pub type TXO_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmitted Octets"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Octets Transmitted \\[47:32\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [othi](index.html) module"]
pub struct OTHI_SPEC;
impl crate::RegisterSpec for OTHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [othi::R](R) reader structure"]
impl crate::Readable for OTHI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTHI to value 0"]
impl crate::Resettable for OTHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
