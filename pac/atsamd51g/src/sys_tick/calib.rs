#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CALIB_SPEC>;
#[doc = "Field `TENMS` reader - Reload value to use for 10ms timing"]
pub type TENMS_R = crate::FieldReader<u32>;
#[doc = "Field `SKEW` reader - TENMS is rounded from non-integer ratio"]
pub type SKEW_R = crate::BitReader<SKEWSELECT_A>;
#[doc = "TENMS is rounded from non-integer ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SKEWSELECT_A {
    #[doc = "0: 10ms calibration value is exact"]
    VALUE_0 = 0,
    #[doc = "1: 10ms calibration value is inexact, because of the clock frequency"]
    VALUE_1 = 1,
}
impl From<SKEWSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SKEWSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SKEW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SKEWSELECT_A {
        match self.bits {
            false => SKEWSELECT_A::VALUE_0,
            true => SKEWSELECT_A::VALUE_1,
        }
    }
    #[doc = "10ms calibration value is exact"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SKEWSELECT_A::VALUE_0
    }
    #[doc = "10ms calibration value is inexact, because of the clock frequency"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SKEWSELECT_A::VALUE_1
    }
}
#[doc = "Field `NOREF` reader - No Separate Reference Clock"]
pub type NOREF_R = crate::BitReader<NOREFSELECT_A>;
#[doc = "No Separate Reference Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOREFSELECT_A {
    #[doc = "0: The reference clock is provided"]
    VALUE_0 = 0,
    #[doc = "1: The reference clock is not provided"]
    VALUE_1 = 1,
}
impl From<NOREFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: NOREFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl NOREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NOREFSELECT_A {
        match self.bits {
            false => NOREFSELECT_A::VALUE_0,
            true => NOREFSELECT_A::VALUE_1,
        }
    }
    #[doc = "The reference clock is provided"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == NOREFSELECT_A::VALUE_0
    }
    #[doc = "The reference clock is not provided"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == NOREFSELECT_A::VALUE_1
    }
}
impl R {
    #[doc = "Bits 0:23 - Reload value to use for 10ms timing"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - TENMS is rounded from non-integer ratio"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - No Separate Reference Clock"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SysTick Calibration Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calib::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calib::R`](R) reader structure"]
impl crate::Readable for CALIB_SPEC {}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
