#[doc = "Register `CALIB` reader"]
pub struct R(crate::R<CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TENMS` reader - Reload value to use for 10ms timing"]
pub struct TENMS_R(crate::FieldReader<u32, u32>);
impl TENMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TENMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENMS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TENMS is rounded from non-integer ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEW_A {
    #[doc = "0: 10ms calibration value is exact"]
    VALUE_0 = 0,
    #[doc = "1: 10ms calibration value is inexact, because of the clock frequency"]
    VALUE_1 = 1,
}
impl From<SKEW_A> for bool {
    #[inline(always)]
    fn from(variant: SKEW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKEW` reader - TENMS is rounded from non-integer ratio"]
pub struct SKEW_R(crate::FieldReader<bool, SKEW_A>);
impl SKEW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SKEW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKEW_A {
        match self.bits {
            false => SKEW_A::VALUE_0,
            true => SKEW_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == SKEW_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == SKEW_A::VALUE_1
    }
}
impl core::ops::Deref for SKEW_R {
    type Target = crate::FieldReader<bool, SKEW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "No Separate Reference Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOREF_A {
    #[doc = "0: The reference clock is provided"]
    VALUE_0 = 0,
    #[doc = "1: The reference clock is not provided"]
    VALUE_1 = 1,
}
impl From<NOREF_A> for bool {
    #[inline(always)]
    fn from(variant: NOREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOREF` reader - No Separate Reference Clock"]
pub struct NOREF_R(crate::FieldReader<bool, NOREF_A>);
impl NOREF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOREF_A {
        match self.bits {
            false => NOREF_A::VALUE_0,
            true => NOREF_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == NOREF_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == NOREF_A::VALUE_1
    }
}
impl core::ops::Deref for NOREF_R {
    type Target = crate::FieldReader<bool, NOREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Reload value to use for 10ms timing"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 30 - TENMS is rounded from non-integer ratio"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - No Separate Reference Clock"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "SysTick Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](index.html) module"]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calib::R](R) reader structure"]
impl crate::Readable for CALIB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
