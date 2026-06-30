#[doc = "Register `FSMSTATUS` reader"]
pub type R = crate::R<FsmstatusSpec>;
#[doc = "Fine State Machine Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsmstateselect {
    #[doc = "1: OFF (L3). It corresponds to the powered-off, disconnected, and disabled state"]
    Off = 1,
    #[doc = "2: ON (L0). It corresponds to the Idle and Active states"]
    On = 2,
    #[doc = "4: SUSPEND (L2)"]
    Suspend = 4,
    #[doc = "8: SLEEP (L1)"]
    Sleep = 8,
    #[doc = "16: DNRESUME. Down Stream Resume."]
    Dnresume = 16,
    #[doc = "32: UPRESUME. Up Stream Resume."]
    Upresume = 32,
    #[doc = "64: RESET. USB lines Reset."]
    Reset = 64,
}
impl From<Fsmstateselect> for u8 {
    #[inline(always)]
    fn from(variant: Fsmstateselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsmstateselect {
    type Ux = u8;
}
impl crate::IsEnum for Fsmstateselect {}
#[doc = "Field `FSMSTATE` reader - Fine State Machine Status"]
pub type FsmstateR = crate::FieldReader<Fsmstateselect>;
impl FsmstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fsmstateselect> {
        match self.bits {
            1 => Some(Fsmstateselect::Off),
            2 => Some(Fsmstateselect::On),
            4 => Some(Fsmstateselect::Suspend),
            8 => Some(Fsmstateselect::Sleep),
            16 => Some(Fsmstateselect::Dnresume),
            32 => Some(Fsmstateselect::Upresume),
            64 => Some(Fsmstateselect::Reset),
            _ => None,
        }
    }
    #[doc = "OFF (L3). It corresponds to the powered-off, disconnected, and disabled state"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Fsmstateselect::Off
    }
    #[doc = "ON (L0). It corresponds to the Idle and Active states"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Fsmstateselect::On
    }
    #[doc = "SUSPEND (L2)"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Fsmstateselect::Suspend
    }
    #[doc = "SLEEP (L1)"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == Fsmstateselect::Sleep
    }
    #[doc = "DNRESUME. Down Stream Resume."]
    #[inline(always)]
    pub fn is_dnresume(&self) -> bool {
        *self == Fsmstateselect::Dnresume
    }
    #[doc = "UPRESUME. Up Stream Resume."]
    #[inline(always)]
    pub fn is_upresume(&self) -> bool {
        *self == Fsmstateselect::Upresume
    }
    #[doc = "RESET. USB lines Reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Fsmstateselect::Reset
    }
}
impl R {
    #[doc = "Bits 0:6 - Fine State Machine Status"]
    #[inline(always)]
    pub fn fsmstate(&self) -> FsmstateR {
        FsmstateR::new(self.bits & 0x7f)
    }
}
#[doc = "Finite State Machine Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fsmstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmstatusSpec;
impl crate::RegisterSpec for FsmstatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fsmstatus::R`](R) reader structure"]
impl crate::Readable for FsmstatusSpec {}
#[doc = "`reset()` method sets FSMSTATUS to value 0x01"]
impl crate::Resettable for FsmstatusSpec {
    const RESET_VALUE: u8 = 0x01;
}
