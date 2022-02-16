#[doc = "Register `AES_IMR` reader"]
pub struct R(crate::R<AES_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATRDY` reader - Data Ready Interrupt Mask"]
pub struct DATRDY_R(crate::FieldReader<bool, bool>);
impl DATRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URAD` reader - Unspecified Register Access Detection Interrupt Mask"]
pub struct URAD_R(crate::FieldReader<bool, bool>);
impl URAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        URAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAGRDY` reader - GCM Tag Ready Interrupt Mask"]
pub struct TAGRDY_R(crate::FieldReader<bool, bool>);
impl TAGRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAGRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAGRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn datrdy(&self) -> DATRDY_R {
        DATRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Interrupt Mask"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GCM Tag Ready Interrupt Mask"]
    #[inline(always)]
    pub fn tagrdy(&self) -> TAGRDY_R {
        TAGRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_imr](index.html) module"]
pub struct AES_IMR_SPEC;
impl crate::RegisterSpec for AES_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_imr::R](R) reader structure"]
impl crate::Readable for AES_IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AES_IMR to value 0"]
impl crate::Resettable for AES_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
