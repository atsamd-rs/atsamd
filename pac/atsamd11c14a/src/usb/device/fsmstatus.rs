#[doc = "Reader of register FSMSTATUS"]
pub type R = crate::R<u8, super::FSMSTATUS>;
#[doc = "Fine State Machine Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSMSTATE_A {
    #[doc = "1: OFF (L3). It corresponds to the powered-off, disconnected, and disabled state"]
    OFF = 1,
    #[doc = "2: ON (L0). It corresponds to the Idle and Active states"]
    ON = 2,
    #[doc = "4: SUSPEND (L2)"]
    SUSPEND = 4,
    #[doc = "8: SLEEP (L1)"]
    SLEEP = 8,
    #[doc = "16: DNRESUME. Down Stream Resume."]
    DNRESUME = 16,
    #[doc = "32: UPRESUME. Up Stream Resume."]
    UPRESUME = 32,
    #[doc = "64: RESET. USB lines Reset."]
    RESET = 64,
}
impl From<FSMSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: FSMSTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FSMSTATE`"]
pub type FSMSTATE_R = crate::R<u8, FSMSTATE_A>;
impl FSMSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FSMSTATE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(FSMSTATE_A::OFF),
            2 => Val(FSMSTATE_A::ON),
            4 => Val(FSMSTATE_A::SUSPEND),
            8 => Val(FSMSTATE_A::SLEEP),
            16 => Val(FSMSTATE_A::DNRESUME),
            32 => Val(FSMSTATE_A::UPRESUME),
            64 => Val(FSMSTATE_A::RESET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FSMSTATE_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FSMSTATE_A::ON
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == FSMSTATE_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == FSMSTATE_A::SLEEP
    }
    #[doc = "Checks if the value of the field is `DNRESUME`"]
    #[inline(always)]
    pub fn is_dnresume(&self) -> bool {
        *self == FSMSTATE_A::DNRESUME
    }
    #[doc = "Checks if the value of the field is `UPRESUME`"]
    #[inline(always)]
    pub fn is_upresume(&self) -> bool {
        *self == FSMSTATE_A::UPRESUME
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FSMSTATE_A::RESET
    }
}
impl R {
    #[doc = "Bits 0:6 - Fine State Machine Status"]
    #[inline(always)]
    pub fn fsmstate(&self) -> FSMSTATE_R {
        FSMSTATE_R::new((self.bits & 0x7f) as u8)
    }
}
