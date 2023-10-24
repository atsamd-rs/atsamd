#[doc = "Register `STATUSC` reader"]
pub type R = crate::R<STATUSC_SPEC>;
#[doc = "Field `STATE0` reader - Comparator 0 Current State"]
pub type STATE0_R = crate::BitReader;
#[doc = "Field `STATE1` reader - Comparator 1 Current State"]
pub type STATE1_R = crate::BitReader;
#[doc = "Field `WSTATE0` reader - Window 0 Current State"]
pub type WSTATE0_R = crate::FieldReader<WSTATE0SELECT_A>;
#[doc = "Window 0 Current State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WSTATE0SELECT_A {
    #[doc = "0: Signal is above window"]
    ABOVE = 0,
    #[doc = "1: Signal is inside window"]
    INSIDE = 1,
    #[doc = "2: Signal is below window"]
    BELOW = 2,
}
impl From<WSTATE0SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WSTATE0SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WSTATE0SELECT_A {
    type Ux = u8;
}
impl WSTATE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WSTATE0SELECT_A> {
        match self.bits {
            0 => Some(WSTATE0SELECT_A::ABOVE),
            1 => Some(WSTATE0SELECT_A::INSIDE),
            2 => Some(WSTATE0SELECT_A::BELOW),
            _ => None,
        }
    }
    #[doc = "Signal is above window"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == WSTATE0SELECT_A::ABOVE
    }
    #[doc = "Signal is inside window"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == WSTATE0SELECT_A::INSIDE
    }
    #[doc = "Signal is below window"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == WSTATE0SELECT_A::BELOW
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 0 Current State"]
    #[inline(always)]
    pub fn state0(&self) -> STATE0_R {
        STATE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Current State"]
    #[inline(always)]
    pub fn state1(&self) -> STATE1_R {
        STATE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Window 0 Current State"]
    #[inline(always)]
    pub fn wstate0(&self) -> WSTATE0_R {
        WSTATE0_R::new((self.bits >> 4) & 3)
    }
}
#[doc = "Status C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSC_SPEC;
impl crate::RegisterSpec for STATUSC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`statusc::R`](R) reader structure"]
impl crate::Readable for STATUSC_SPEC {}
#[doc = "`reset()` method sets STATUSC to value 0"]
impl crate::Resettable for STATUSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
