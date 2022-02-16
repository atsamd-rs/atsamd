#[doc = "Register `PWM_WPSR` reader"]
pub struct R(crate::R<PWM_WPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_WPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_WPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_WPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WPSWS0` reader - Write Protect SW Status"]
pub struct WPSWS0_R(crate::FieldReader<bool, bool>);
impl WPSWS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPSWS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPSWS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPSWS1` reader - Write Protect SW Status"]
pub struct WPSWS1_R(crate::FieldReader<bool, bool>);
impl WPSWS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPSWS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPSWS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPSWS2` reader - Write Protect SW Status"]
pub struct WPSWS2_R(crate::FieldReader<bool, bool>);
impl WPSWS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPSWS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPSWS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPSWS3` reader - Write Protect SW Status"]
pub struct WPSWS3_R(crate::FieldReader<bool, bool>);
impl WPSWS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPSWS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPSWS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPSWS4` reader - Write Protect SW Status"]
pub struct WPSWS4_R(crate::FieldReader<bool, bool>);
impl WPSWS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPSWS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPSWS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPSWS5` reader - Write Protect SW Status"]
pub struct WPSWS5_R(crate::FieldReader<bool, bool>);
impl WPSWS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPSWS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPSWS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPVS` reader - Write Protect Violation Status"]
pub struct WPVS_R(crate::FieldReader<bool, bool>);
impl WPVS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPVS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPHWS0` reader - Write Protect HW Status"]
pub struct WPHWS0_R(crate::FieldReader<bool, bool>);
impl WPHWS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPHWS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPHWS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPHWS1` reader - Write Protect HW Status"]
pub struct WPHWS1_R(crate::FieldReader<bool, bool>);
impl WPHWS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPHWS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPHWS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPHWS2` reader - Write Protect HW Status"]
pub struct WPHWS2_R(crate::FieldReader<bool, bool>);
impl WPHWS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPHWS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPHWS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPHWS3` reader - Write Protect HW Status"]
pub struct WPHWS3_R(crate::FieldReader<bool, bool>);
impl WPHWS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPHWS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPHWS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPHWS4` reader - Write Protect HW Status"]
pub struct WPHWS4_R(crate::FieldReader<bool, bool>);
impl WPHWS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPHWS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPHWS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPHWS5` reader - Write Protect HW Status"]
pub struct WPHWS5_R(crate::FieldReader<bool, bool>);
impl WPHWS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPHWS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPHWS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPVSRC` reader - Write Protect Violation Source"]
pub struct WPVSRC_R(crate::FieldReader<u16, u16>);
impl WPVSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WPVSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPVSRC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws0(&self) -> WPSWS0_R {
        WPSWS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws1(&self) -> WPSWS1_R {
        WPSWS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws2(&self) -> WPSWS2_R {
        WPSWS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws3(&self) -> WPSWS3_R {
        WPSWS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws4(&self) -> WPSWS4_R {
        WPSWS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws5(&self) -> WPSWS5_R {
        WPSWS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write Protect Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws0(&self) -> WPHWS0_R {
        WPHWS0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws1(&self) -> WPHWS1_R {
        WPHWS1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws2(&self) -> WPHWS2_R {
        WPHWS2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws3(&self) -> WPHWS3_R {
        WPHWS3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws4(&self) -> WPHWS4_R {
        WPHWS4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws5(&self) -> WPHWS5_R {
        WPHWS5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Write Protect Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "PWM Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_wpsr](index.html) module"]
pub struct PWM_WPSR_SPEC;
impl crate::RegisterSpec for PWM_WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_wpsr::R](R) reader structure"]
impl crate::Readable for PWM_WPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_WPSR to value 0"]
impl crate::Resettable for PWM_WPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
