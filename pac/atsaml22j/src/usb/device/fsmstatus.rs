#[doc = "Register `FSMSTATUS` reader"]
pub struct R(crate::R<FSMSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSMSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSMSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSMSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FSMSTATE` reader - Fine State Machine Status"]
pub type FSMSTATE_R = crate::FieldReader<u8, FSMSTATESELECT_A>;
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
impl FSMSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FSMSTATESELECT_A> {
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
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FSMSTATESELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FSMSTATESELECT_A::ON
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == FSMSTATESELECT_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == FSMSTATESELECT_A::SLEEP
    }
    #[doc = "Checks if the value of the field is `DNRESUME`"]
    #[inline(always)]
    pub fn is_dnresume(&self) -> bool {
        *self == FSMSTATESELECT_A::DNRESUME
    }
    #[doc = "Checks if the value of the field is `UPRESUME`"]
    #[inline(always)]
    pub fn is_upresume(&self) -> bool {
        *self == FSMSTATESELECT_A::UPRESUME
    }
    #[doc = "Checks if the value of the field is `RESET`"]
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
#[doc = "Finite State Machine Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsmstatus](index.html) module"]
pub struct FSMSTATUS_SPEC;
impl crate::RegisterSpec for FSMSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fsmstatus::R](R) reader structure"]
impl crate::Readable for FSMSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSMSTATUS to value 0x01"]
impl crate::Resettable for FSMSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
