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
pub struct SWRST_R(crate::FieldReader<bool, bool>);
impl SWRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Generic Clock Generator Control n Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GENCTRL_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL_A> for u16 {
    #[inline(always)]
    fn from(variant: GENCTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GENCTRL` reader - Generic Clock Generator Control n Synchronization Busy bits"]
pub struct GENCTRL_R(crate::FieldReader<u16, GENCTRL_A>);
impl GENCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GENCTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL_A> {
        match self.bits {
            1 => Some(GENCTRL_A::GCLK0),
            2 => Some(GENCTRL_A::GCLK1),
            4 => Some(GENCTRL_A::GCLK2),
            8 => Some(GENCTRL_A::GCLK3),
            16 => Some(GENCTRL_A::GCLK4),
            32 => Some(GENCTRL_A::GCLK5),
            64 => Some(GENCTRL_A::GCLK6),
            128 => Some(GENCTRL_A::GCLK7),
            256 => Some(GENCTRL_A::GCLK8),
            512 => Some(GENCTRL_A::GCLK9),
            1024 => Some(GENCTRL_A::GCLK10),
            2048 => Some(GENCTRL_A::GCLK11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        **self == GENCTRL_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        **self == GENCTRL_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        **self == GENCTRL_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        **self == GENCTRL_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        **self == GENCTRL_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        **self == GENCTRL_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        **self == GENCTRL_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        **self == GENCTRL_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        **self == GENCTRL_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        **self == GENCTRL_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        **self == GENCTRL_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        **self == GENCTRL_A::GCLK11
    }
}
impl core::ops::Deref for GENCTRL_R {
    type Target = crate::FieldReader<u16, GENCTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Synchroniation Busy bit"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:13 - Generic Clock Generator Control n Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl(&self) -> GENCTRL_R {
        GENCTRL_R::new(((self.bits >> 2) & 0x0fff) as u16)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
