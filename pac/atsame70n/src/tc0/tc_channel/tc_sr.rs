#[doc = "Register `TC_SR` reader"]
pub struct R(crate::R<TC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COVFS` reader - Counter Overflow Status (cleared on read)"]
pub struct COVFS_R(crate::FieldReader<bool, bool>);
impl COVFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COVFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COVFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOVRS` reader - Load Overrun Status (cleared on read)"]
pub struct LOVRS_R(crate::FieldReader<bool, bool>);
impl LOVRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOVRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOVRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPAS` reader - RA Compare Status (cleared on read)"]
pub struct CPAS_R(crate::FieldReader<bool, bool>);
impl CPAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPBS` reader - RB Compare Status (cleared on read)"]
pub struct CPBS_R(crate::FieldReader<bool, bool>);
impl CPBS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPBS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPCS` reader - RC Compare Status (cleared on read)"]
pub struct CPCS_R(crate::FieldReader<bool, bool>);
impl CPCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDRAS` reader - RA Loading Status (cleared on read)"]
pub struct LDRAS_R(crate::FieldReader<bool, bool>);
impl LDRAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDRAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDRAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDRBS` reader - RB Loading Status (cleared on read)"]
pub struct LDRBS_R(crate::FieldReader<bool, bool>);
impl LDRBS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDRBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDRBS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETRGS` reader - External Trigger Status (cleared on read)"]
pub struct ETRGS_R(crate::FieldReader<bool, bool>);
impl ETRGS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETRGS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETRGS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSTA` reader - Clock Enabling Status"]
pub struct CLKSTA_R(crate::FieldReader<bool, bool>);
impl CLKSTA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKSTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKSTA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTIOA` reader - TIOAx Mirror"]
pub struct MTIOA_R(crate::FieldReader<bool, bool>);
impl MTIOA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MTIOA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTIOA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTIOB` reader - TIOBx Mirror"]
pub struct MTIOB_R(crate::FieldReader<bool, bool>);
impl MTIOB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MTIOB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTIOB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Counter Overflow Status (cleared on read)"]
    #[inline(always)]
    pub fn covfs(&self) -> COVFS_R {
        COVFS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Load Overrun Status (cleared on read)"]
    #[inline(always)]
    pub fn lovrs(&self) -> LOVRS_R {
        LOVRS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RA Compare Status (cleared on read)"]
    #[inline(always)]
    pub fn cpas(&self) -> CPAS_R {
        CPAS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RB Compare Status (cleared on read)"]
    #[inline(always)]
    pub fn cpbs(&self) -> CPBS_R {
        CPBS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RC Compare Status (cleared on read)"]
    #[inline(always)]
    pub fn cpcs(&self) -> CPCS_R {
        CPCS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RA Loading Status (cleared on read)"]
    #[inline(always)]
    pub fn ldras(&self) -> LDRAS_R {
        LDRAS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RB Loading Status (cleared on read)"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LDRBS_R {
        LDRBS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Trigger Status (cleared on read)"]
    #[inline(always)]
    pub fn etrgs(&self) -> ETRGS_R {
        ETRGS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Clock Enabling Status"]
    #[inline(always)]
    pub fn clksta(&self) -> CLKSTA_R {
        CLKSTA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIOAx Mirror"]
    #[inline(always)]
    pub fn mtioa(&self) -> MTIOA_R {
        MTIOA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIOBx Mirror"]
    #[inline(always)]
    pub fn mtiob(&self) -> MTIOB_R {
        MTIOB_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
#[doc = "Status Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_sr](index.html) module"]
pub struct TC_SR_SPEC;
impl crate::RegisterSpec for TC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tc_sr::R](R) reader structure"]
impl crate::Readable for TC_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TC_SR to value 0"]
impl crate::Resettable for TC_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
