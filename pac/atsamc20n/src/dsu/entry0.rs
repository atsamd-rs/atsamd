#[doc = "Register `ENTRY0` reader"]
pub struct R(crate::R<ENTRY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENTRY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENTRY0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENTRY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPRES` reader - Entry Present"]
pub struct EPRES_R(crate::FieldReader<bool, bool>);
impl EPRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMT` reader - Format"]
pub struct FMT_R(crate::FieldReader<bool, bool>);
impl FMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDOFF` reader - Address Offset"]
pub struct ADDOFF_R(crate::FieldReader<u32, u32>);
impl ADDOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ADDOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDOFF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Entry Present"]
    #[inline(always)]
    pub fn epres(&self) -> EPRES_R {
        EPRES_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Format"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 12:31 - Address Offset"]
    #[inline(always)]
    pub fn addoff(&self) -> ADDOFF_R {
        ADDOFF_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
#[doc = "CoreSight ROM Table Entry 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [entry0](index.html) module"]
pub struct ENTRY0_SPEC;
impl crate::RegisterSpec for ENTRY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [entry0::R](R) reader structure"]
impl crate::Readable for ENTRY0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ENTRY0 to value 0x9f0f_c002"]
impl crate::Resettable for ENTRY0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x9f0f_c002
    }
}
