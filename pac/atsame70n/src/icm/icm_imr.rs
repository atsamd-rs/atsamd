#[doc = "Register `ICM_IMR` reader"]
pub struct R(crate::R<ICM_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICM_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICM_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICM_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RHC` reader - Region Hash Completed Interrupt Mask"]
pub struct RHC_R(crate::FieldReader<u8, u8>);
impl RHC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RHC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RHC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDM` reader - Region Digest Mismatch Interrupt Mask"]
pub struct RDM_R(crate::FieldReader<u8, u8>);
impl RDM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBE` reader - Region Bus Error Interrupt Mask"]
pub struct RBE_R(crate::FieldReader<u8, u8>);
impl RBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWC` reader - Region Wrap Condition Detected Interrupt Mask"]
pub struct RWC_R(crate::FieldReader<u8, u8>);
impl RWC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RWC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REC` reader - Region End bit Condition Detected Interrupt Mask"]
pub struct REC_R(crate::FieldReader<u8, u8>);
impl REC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSU` reader - Region Status Updated Interrupt Mask"]
pub struct RSU_R(crate::FieldReader<u8, u8>);
impl RSU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RSU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URAD` reader - Undefined Register Access Detection Interrupt Mask"]
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
impl R {
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Mask"]
    #[inline(always)]
    pub fn rhc(&self) -> RHC_R {
        RHC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn rdm(&self) -> RDM_R {
        RDM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Mask"]
    #[inline(always)]
    pub fn rbe(&self) -> RBE_R {
        RBE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected Interrupt Mask"]
    #[inline(always)]
    pub fn rwc(&self) -> RWC_R {
        RWC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Region End bit Condition Detected Interrupt Mask"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Mask"]
    #[inline(always)]
    pub fn rsu(&self) -> RSU_R {
        RSU_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Mask"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_imr](index.html) module"]
pub struct ICM_IMR_SPEC;
impl crate::RegisterSpec for ICM_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icm_imr::R](R) reader structure"]
impl crate::Readable for ICM_IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ICM_IMR to value 0"]
impl crate::Resettable for ICM_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
