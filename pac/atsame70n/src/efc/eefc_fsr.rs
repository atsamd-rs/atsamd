#[doc = "Register `EEFC_FSR` reader"]
pub struct R(crate::R<EEFC_FSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFC_FSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFC_FSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFC_FSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRDY` reader - Flash Ready Status (cleared when Flash is busy)"]
pub struct FRDY_R(crate::FieldReader<bool, bool>);
impl FRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCMDE` reader - Flash Command Error Status (cleared on read or by writing EEFC_FCR)"]
pub struct FCMDE_R(crate::FieldReader<bool, bool>);
impl FCMDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCMDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCMDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLOCKE` reader - Flash Lock Error Status (cleared on read)"]
pub struct FLOCKE_R(crate::FieldReader<bool, bool>);
impl FLOCKE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLOCKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLOCKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLERR` reader - Flash Error Status (cleared when a programming operation starts)"]
pub struct FLERR_R(crate::FieldReader<bool, bool>);
impl FLERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UECCELSB` reader - Unique ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
pub struct UECCELSB_R(crate::FieldReader<bool, bool>);
impl UECCELSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UECCELSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UECCELSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MECCELSB` reader - Multiple ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
pub struct MECCELSB_R(crate::FieldReader<bool, bool>);
impl MECCELSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MECCELSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MECCELSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UECCEMSB` reader - Unique ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
pub struct UECCEMSB_R(crate::FieldReader<bool, bool>);
impl UECCEMSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UECCEMSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UECCEMSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MECCEMSB` reader - Multiple ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
pub struct MECCEMSB_R(crate::FieldReader<bool, bool>);
impl MECCEMSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MECCEMSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MECCEMSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Flash Ready Status (cleared when Flash is busy)"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash Command Error Status (cleared on read or by writing EEFC_FCR)"]
    #[inline(always)]
    pub fn fcmde(&self) -> FCMDE_R {
        FCMDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flash Lock Error Status (cleared on read)"]
    #[inline(always)]
    pub fn flocke(&self) -> FLOCKE_R {
        FLOCKE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flash Error Status (cleared when a programming operation starts)"]
    #[inline(always)]
    pub fn flerr(&self) -> FLERR_R {
        FLERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Unique ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn ueccelsb(&self) -> UECCELSB_R {
        UECCELSB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Multiple ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn meccelsb(&self) -> MECCELSB_R {
        MECCELSB_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Unique ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn ueccemsb(&self) -> UECCEMSB_R {
        UECCEMSB_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Multiple ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn meccemsb(&self) -> MECCEMSB_R {
        MECCEMSB_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
#[doc = "EEFC Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefc_fsr](index.html) module"]
pub struct EEFC_FSR_SPEC;
impl crate::RegisterSpec for EEFC_FSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eefc_fsr::R](R) reader structure"]
impl crate::Readable for EEFC_FSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EEFC_FSR to value 0"]
impl crate::Resettable for EEFC_FSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
