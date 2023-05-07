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
pub type REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PARTNO` reader - Process Part Number, 0xC24=Cortex-M4"]
pub type PARTNO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONSTANT` reader - Constant"]
pub type CONSTANT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VARIANT` reader - Variant number"]
pub type VARIANT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IMPLEMENTER` reader - Implementer code, 0x41=ARM"]
pub type IMPLEMENTER_R = crate::FieldReader<u8, u8>;
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
    const RESET_VALUE: Self::Ux = 0x410f_c240;
}
