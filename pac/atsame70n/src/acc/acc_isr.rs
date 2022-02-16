#[doc = "Register `ACC_ISR` reader"]
pub struct R(crate::R<ACC_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACC_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACC_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACC_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CE` reader - Comparison Edge (cleared on read)"]
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
#[doc = "Field `SCO` reader - Synchronized Comparator Output"]
pub struct SCO_R(crate::FieldReader<bool, bool>);
impl SCO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` reader - Flag Mask"]
pub struct MASK_R(crate::FieldReader<bool, bool>);
impl MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Comparison Edge (cleared on read)"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Synchronized Comparator Output"]
    #[inline(always)]
    pub fn sco(&self) -> SCO_R {
        SCO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Flag Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_isr](index.html) module"]
pub struct ACC_ISR_SPEC;
impl crate::RegisterSpec for ACC_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acc_isr::R](R) reader structure"]
impl crate::Readable for ACC_ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACC_ISR to value 0"]
impl crate::Resettable for ACC_ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
