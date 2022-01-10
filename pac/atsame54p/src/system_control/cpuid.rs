#[doc = "Register `CPUID` reader"]
pub struct R(crate::R<CPUID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REVISION` reader - Processor revision number"]
pub struct REVISION_R(crate::FieldReader<u8, u8>);
impl REVISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARTNO` reader - Process Part Number, 0xC24=Cortex-M4"]
pub struct PARTNO_R(crate::FieldReader<u16, u16>);
impl PARTNO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PARTNO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTNO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONSTANT` reader - Constant"]
pub struct CONSTANT_R(crate::FieldReader<u8, u8>);
impl CONSTANT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CONSTANT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONSTANT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VARIANT` reader - Variant number"]
pub struct VARIANT_R(crate::FieldReader<u8, u8>);
impl VARIANT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VARIANT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VARIANT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMPLEMENTER` reader - Implementer code, 0x41=ARM"]
pub struct IMPLEMENTER_R(crate::FieldReader<u8, u8>);
impl IMPLEMENTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IMPLEMENTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMPLEMENTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Processor revision number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Process Part Number, 0xC24=Cortex-M4"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Constant"]
    #[inline(always)]
    pub fn constant(&self) -> CONSTANT_R {
        CONSTANT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Variant number"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Implementer code, 0x41=ARM"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "CPUID Base Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuid](index.html) module"]
pub struct CPUID_SPEC;
impl crate::RegisterSpec for CPUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuid::R](R) reader structure"]
impl crate::Readable for CPUID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPUID to value 0x410f_c240"]
impl crate::Resettable for CPUID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x410f_c240
    }
}
