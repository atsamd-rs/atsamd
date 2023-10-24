#[doc = "Register `FSMSTATUS` reader"]
pub type R = crate::R<FSMSTATUS_SPEC>;
#[doc = "Field `FSMSTATE` reader - Fine State Machine Status"]
pub type FSMSTATE_R = crate::FieldReader<FSMSTATESELECT_A>;
#[doc = "Fine State Machine Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSMSTATESELECT_A {
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
impl From<FSMSTATESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FSMSTATESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSMSTATESELECT_A {
    type Ux = u8;
}
impl FSMSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FSMSTATESELECT_A> {
        match self.bits {
            1 => Some(FSMSTATESELECT_A::OFF),
            2 => Some(FSMSTATESELECT_A::ON),
            4 => Some(FSMSTATESELECT_A::SUSPEND),
            8 => Some(FSMSTATESELECT_A::SLEEP),
            16 => Some(FSMSTATESELECT_A::DNRESUME),
            32 => Some(FSMSTATESELECT_A::UPRESUME),
            64 => Some(FSMSTATESELECT_A::RESET),
            _ => None,
        }
    }
    #[doc = "OFF (L3). It corresponds to the powered-off, disconnected, and disabled state"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FSMSTATESELECT_A::OFF
    }
    #[doc = "ON (L0). It corresponds to the Idle and Active states"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FSMSTATESELECT_A::ON
    }
    #[doc = "SUSPEND (L2)"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == FSMSTATESELECT_A::SUSPEND
    }
    #[doc = "SLEEP (L1)"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == FSMSTATESELECT_A::SLEEP
    }
    #[doc = "DNRESUME. Down Stream Resume."]
    #[inline(always)]
    pub fn is_dnresume(&self) -> bool {
        *self == FSMSTATESELECT_A::DNRESUME
    }
    #[doc = "UPRESUME. Up Stream Resume."]
    #[inline(always)]
    pub fn is_upresume(&self) -> bool {
        *self == FSMSTATESELECT_A::UPRESUME
    }
    #[doc = "RESET. USB lines Reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FSMSTATESELECT_A::RESET
    }
}
impl R {
    #[doc = "Bits 0:6 - Fine State Machine Status"]
    #[inline(always)]
    pub fn fsmstate(&self) -> FSMSTATE_R {
        FSMSTATE_R::new(self.bits & 0x7f)
    }
}
#[doc = "Finite State Machine Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsmstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSMSTATUS_SPEC;
impl crate::RegisterSpec for FSMSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fsmstatus::R`](R) reader structure"]
impl crate::Readable for FSMSTATUS_SPEC {}
#[doc = "`reset()` method sets FSMSTATUS to value 0x01"]
impl crate::Resettable for FSMSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
