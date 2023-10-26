#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSY_SPEC>;
#[doc = "Field `SWRST` reader - Software Reset Synchroniation Busy bit"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `GENCTRL` reader - Generic Clock Generator Control n Synchronization Busy bits"]
pub type GENCTRL_R = crate::FieldReader<GENCTRLSELECT_A>;
#[doc = "Generic Clock Generator Control n Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GENCTRLSELECT_A {
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
impl From<GENCTRLSELECT_A> for u16 {
    #[inline(always)]
    fn from(variant: GENCTRLSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GENCTRLSELECT_A {
    type Ux = u16;
}
impl GENCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GENCTRLSELECT_A> {
        match self.bits {
            1 => Some(GENCTRLSELECT_A::GCLK0),
            2 => Some(GENCTRLSELECT_A::GCLK1),
            4 => Some(GENCTRLSELECT_A::GCLK2),
            8 => Some(GENCTRLSELECT_A::GCLK3),
            16 => Some(GENCTRLSELECT_A::GCLK4),
            32 => Some(GENCTRLSELECT_A::GCLK5),
            64 => Some(GENCTRLSELECT_A::GCLK6),
            128 => Some(GENCTRLSELECT_A::GCLK7),
            256 => Some(GENCTRLSELECT_A::GCLK8),
            512 => Some(GENCTRLSELECT_A::GCLK9),
            1024 => Some(GENCTRLSELECT_A::GCLK10),
            2048 => Some(GENCTRLSELECT_A::GCLK11),
            _ => None,
        }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRLSELECT_A::GCLK0
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRLSELECT_A::GCLK1
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRLSELECT_A::GCLK2
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRLSELECT_A::GCLK3
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRLSELECT_A::GCLK4
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRLSELECT_A::GCLK5
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRLSELECT_A::GCLK6
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRLSELECT_A::GCLK7
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRLSELECT_A::GCLK8
    }
    #[doc = "Generic clock generator 9"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRLSELECT_A::GCLK9
    }
    #[doc = "Generic clock generator 10"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRLSELECT_A::GCLK10
    }
    #[doc = "Generic clock generator 11"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRLSELECT_A::GCLK11
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Synchroniation Busy bit"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:13 - Generic Clock Generator Control n Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl(&self) -> GENCTRL_R {
        GENCTRL_R::new(((self.bits >> 2) & 0x0fff) as u16)
    }
}
#[doc = "Synchronization Busy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
