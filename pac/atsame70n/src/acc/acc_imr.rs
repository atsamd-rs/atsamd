#[doc = "Register `ACC_IMR` reader"]
pub struct R(crate::R<ACC_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACC_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACC_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACC_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CE` reader - Comparison Edge"]
pub struct CE_R(crate::FieldReader<bool, bool>);
impl CE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Comparison Edge"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_imr](index.html) module"]
pub struct ACC_IMR_SPEC;
impl crate::RegisterSpec for ACC_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acc_imr::R](R) reader structure"]
impl crate::Readable for ACC_IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACC_IMR to value 0"]
impl crate::Resettable for ACC_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
