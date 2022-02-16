#[doc = "Register `PIO_PCISR` reader"]
pub struct R(crate::R<PIO_PCISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO_PCISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO_PCISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO_PCISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DRDY` reader - Parallel Capture Mode Data Ready"]
pub struct DRDY_R(crate::FieldReader<bool, bool>);
impl DRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE` reader - Parallel Capture Mode Overrun Error"]
pub struct OVRE_R(crate::FieldReader<bool, bool>);
impl OVRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "Parallel Capture Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pcisr](index.html) module"]
pub struct PIO_PCISR_SPEC;
impl crate::RegisterSpec for PIO_PCISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio_pcisr::R](R) reader structure"]
impl crate::Readable for PIO_PCISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIO_PCISR to value 0"]
impl crate::Resettable for PIO_PCISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
