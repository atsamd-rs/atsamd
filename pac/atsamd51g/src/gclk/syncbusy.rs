#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `GENCTRL`"]
pub type GENCTRL_R = crate::R<u16, GENCTRL_A>;
impl GENCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, GENCTRL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(GENCTRL_A::GCLK0),
            2 => Val(GENCTRL_A::GCLK1),
            4 => Val(GENCTRL_A::GCLK2),
            8 => Val(GENCTRL_A::GCLK3),
            16 => Val(GENCTRL_A::GCLK4),
            32 => Val(GENCTRL_A::GCLK5),
            64 => Val(GENCTRL_A::GCLK6),
            128 => Val(GENCTRL_A::GCLK7),
            256 => Val(GENCTRL_A::GCLK8),
            512 => Val(GENCTRL_A::GCLK9),
            1024 => Val(GENCTRL_A::GCLK10),
            2048 => Val(GENCTRL_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL_A::GCLK11
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
