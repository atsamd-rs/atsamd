#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SWRST` reader - Software Reset Synchroniation Busy bit"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `GENCTRL0` reader - Generic Clock Generator Control 0 Synchronization Busy bits"]
pub type GENCTRL0_R = crate::BitReader<GENCTRL0SELECT_A>;
#[doc = "Generic Clock Generator Control 0 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENCTRL0SELECT_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
}
impl From<GENCTRL0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl GENCTRL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL0SELECT_A> {
        match self.bits {
            true => Some(GENCTRL0SELECT_A::GCLK0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL0SELECT_A::GCLK0
    }
}
#[doc = "Field `GENCTRL1` reader - Generic Clock Generator Control 1 Synchronization Busy bits"]
pub type GENCTRL1_R = crate::BitReader<GENCTRL1SELECT_A>;
#[doc = "Generic Clock Generator Control 1 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENCTRL1SELECT_A {
    #[doc = "1: Generic clock generator 1"]
    GCLK1 = 1,
}
impl From<GENCTRL1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl GENCTRL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL1SELECT_A> {
        match self.bits {
            true => Some(GENCTRL1SELECT_A::GCLK1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL1SELECT_A::GCLK1
    }
}
#[doc = "Field `GENCTRL2` reader - Generic Clock Generator Control 2 Synchronization Busy bits"]
pub type GENCTRL2_R = crate::BitReader<GENCTRL2SELECT_A>;
#[doc = "Generic Clock Generator Control 2 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENCTRL2SELECT_A {
    #[doc = "1: Generic clock generator 2"]
    GCLK2 = 1,
}
impl From<GENCTRL2SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL2SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl GENCTRL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL2SELECT_A> {
        match self.bits {
            true => Some(GENCTRL2SELECT_A::GCLK2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL2SELECT_A::GCLK2
    }
}
#[doc = "Field `GENCTRL3` reader - Generic Clock Generator Control 3 Synchronization Busy bits"]
pub type GENCTRL3_R = crate::BitReader<GENCTRL3SELECT_A>;
#[doc = "Generic Clock Generator Control 3 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENCTRL3SELECT_A {
    #[doc = "1: Generic clock generator 3"]
    GCLK3 = 1,
}
impl From<GENCTRL3SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL3SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl GENCTRL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL3SELECT_A> {
        match self.bits {
            true => Some(GENCTRL3SELECT_A::GCLK3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL3SELECT_A::GCLK3
    }
}
#[doc = "Field `GENCTRL4` reader - Generic Clock Generator Control 4 Synchronization Busy bits"]
pub type GENCTRL4_R = crate::BitReader<GENCTRL4SELECT_A>;
#[doc = "Generic Clock Generator Control 4 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENCTRL4SELECT_A {
    #[doc = "1: Generic clock generator 4"]
    GCLK4 = 1,
}
impl From<GENCTRL4SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL4SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl GENCTRL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL4SELECT_A> {
        match self.bits {
            true => Some(GENCTRL4SELECT_A::GCLK4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL4SELECT_A::GCLK4
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Synchroniation Busy bit"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Generic Clock Generator Control 0 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl0(&self) -> GENCTRL0_R {
        GENCTRL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generic Clock Generator Control 1 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl1(&self) -> GENCTRL1_R {
        GENCTRL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generic Clock Generator Control 2 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl2(&self) -> GENCTRL2_R {
        GENCTRL2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generic Clock Generator Control 3 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl3(&self) -> GENCTRL3_R {
        GENCTRL3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generic Clock Generator Control 4 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl4(&self) -> GENCTRL4_R {
        GENCTRL4_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
