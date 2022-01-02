#[doc = "Register `CSE` reader"]
pub struct R(crate::R<CSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSR` reader - Carrier Sense Error"]
pub struct CSR_R(crate::FieldReader<u16, u16>);
impl CSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Carrier Sense Error"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Carrier Sense Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cse](index.html) module"]
pub struct CSE_SPEC;
impl crate::RegisterSpec for CSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cse::R](R) reader structure"]
impl crate::Readable for CSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSE to value 0"]
impl crate::Resettable for CSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
