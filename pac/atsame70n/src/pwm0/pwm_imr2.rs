#[doc = "Register `PWM_IMR2` reader"]
pub struct R(crate::R<PWM_IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRDY` reader - Write Ready for Synchronous Channels Update Interrupt Mask"]
pub struct WRDY_R(crate::FieldReader<bool, bool>);
impl WRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNRE` reader - Synchronous Channels Update Underrun Error Interrupt Mask"]
pub struct UNRE_R(crate::FieldReader<bool, bool>);
impl UNRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPM0` reader - Comparison 0 Match Interrupt Mask"]
pub struct CMPM0_R(crate::FieldReader<bool, bool>);
impl CMPM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPM1` reader - Comparison 1 Match Interrupt Mask"]
pub struct CMPM1_R(crate::FieldReader<bool, bool>);
impl CMPM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPM2` reader - Comparison 2 Match Interrupt Mask"]
pub struct CMPM2_R(crate::FieldReader<bool, bool>);
impl CMPM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPM3` reader - Comparison 3 Match Interrupt Mask"]
pub struct CMPM3_R(crate::FieldReader<bool, bool>);
impl CMPM3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPM4` reader - Comparison 4 Match Interrupt Mask"]
pub struct CMPM4_R(crate::FieldReader<bool, bool>);
impl CMPM4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPM5` reader - Comparison 5 Match Interrupt Mask"]
pub struct CMPM5_R(crate::FieldReader<bool, bool>);
impl CMPM5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPM6` reader - Comparison 6 Match Interrupt Mask"]
pub struct CMPM6_R(crate::FieldReader<bool, bool>);
impl CMPM6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPM6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPM7` reader - Comparison 7 Match Interrupt Mask"]
pub struct CMPM7_R(crate::FieldReader<bool, bool>);
impl CMPM7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPM7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPU0` reader - Comparison 0 Update Interrupt Mask"]
pub struct CMPU0_R(crate::FieldReader<bool, bool>);
impl CMPU0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPU0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPU0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPU1` reader - Comparison 1 Update Interrupt Mask"]
pub struct CMPU1_R(crate::FieldReader<bool, bool>);
impl CMPU1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPU1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPU1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPU2` reader - Comparison 2 Update Interrupt Mask"]
pub struct CMPU2_R(crate::FieldReader<bool, bool>);
impl CMPU2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPU2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPU2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPU3` reader - Comparison 3 Update Interrupt Mask"]
pub struct CMPU3_R(crate::FieldReader<bool, bool>);
impl CMPU3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPU3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPU3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPU4` reader - Comparison 4 Update Interrupt Mask"]
pub struct CMPU4_R(crate::FieldReader<bool, bool>);
impl CMPU4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPU4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPU4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPU5` reader - Comparison 5 Update Interrupt Mask"]
pub struct CMPU5_R(crate::FieldReader<bool, bool>);
impl CMPU5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPU5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPU5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPU6` reader - Comparison 6 Update Interrupt Mask"]
pub struct CMPU6_R(crate::FieldReader<bool, bool>);
impl CMPU6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPU6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPU6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPU7` reader - Comparison 7 Update Interrupt Mask"]
pub struct CMPU7_R(crate::FieldReader<bool, bool>);
impl CMPU7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPU7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPU7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Write Ready for Synchronous Channels Update Interrupt Mask"]
    #[inline(always)]
    pub fn wrdy(&self) -> WRDY_R {
        WRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Synchronous Channels Update Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comparison 0 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm0(&self) -> CMPM0_R {
        CMPM0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comparison 1 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm1(&self) -> CMPM1_R {
        CMPM1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Comparison 2 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm2(&self) -> CMPM2_R {
        CMPM2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Comparison 3 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm3(&self) -> CMPM3_R {
        CMPM3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comparison 4 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm4(&self) -> CMPM4_R {
        CMPM4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Comparison 5 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm5(&self) -> CMPM5_R {
        CMPM5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Comparison 6 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm6(&self) -> CMPM6_R {
        CMPM6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparison 7 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm7(&self) -> CMPM7_R {
        CMPM7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Comparison 0 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu0(&self) -> CMPU0_R {
        CMPU0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Comparison 1 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu1(&self) -> CMPU1_R {
        CMPU1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Comparison 2 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu2(&self) -> CMPU2_R {
        CMPU2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Comparison 3 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu3(&self) -> CMPU3_R {
        CMPU3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Comparison 4 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu4(&self) -> CMPU4_R {
        CMPU4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Comparison 5 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu5(&self) -> CMPU5_R {
        CMPU5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Comparison 6 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu6(&self) -> CMPU6_R {
        CMPU6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Comparison 7 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu7(&self) -> CMPU7_R {
        CMPU7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "PWM Interrupt Mask Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_imr2](index.html) module"]
pub struct PWM_IMR2_SPEC;
impl crate::RegisterSpec for PWM_IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_imr2::R](R) reader structure"]
impl crate::Readable for PWM_IMR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_IMR2 to value 0"]
impl crate::Resettable for PWM_IMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
