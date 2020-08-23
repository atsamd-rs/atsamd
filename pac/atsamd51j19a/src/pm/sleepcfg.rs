#[doc = "Reader of register SLEEPCFG"]
pub type R = crate::R<u8, super::SLEEPCFG>;
#[doc = "Writer for register SLEEPCFG"]
pub type W = crate::W<u8, super::SLEEPCFG>;
#[doc = "Register SLEEPCFG `reset()`'s with value 0x02"]
impl crate::ResetValue for super::SLEEPCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Sleep Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLEEPMODE_A {
    #[doc = "2: CPU, AHBx, and APBx clocks are OFF"]
    IDLE = 2,
    #[doc = "4: All Clocks are OFF"]
    STANDBY = 4,
    #[doc = "5: Backup domain is ON as well as some PDRAMs"]
    HIBERNATE = 5,
    #[doc = "6: Only Backup domain is powered ON"]
    BACKUP = 6,
    #[doc = "7: All power domains are powered OFF"]
    OFF = 7,
}
impl From<SLEEPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SLEEPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SLEEPMODE`"]
pub type SLEEPMODE_R = crate::R<u8, SLEEPMODE_A>;
impl SLEEPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLEEPMODE_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(SLEEPMODE_A::IDLE),
            4 => Val(SLEEPMODE_A::STANDBY),
            5 => Val(SLEEPMODE_A::HIBERNATE),
            6 => Val(SLEEPMODE_A::BACKUP),
            7 => Val(SLEEPMODE_A::OFF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SLEEPMODE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == SLEEPMODE_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `HIBERNATE`"]
    #[inline(always)]
    pub fn is_hibernate(&self) -> bool {
        *self == SLEEPMODE_A::HIBERNATE
    }
    #[doc = "Checks if the value of the field is `BACKUP`"]
    #[inline(always)]
    pub fn is_backup(&self) -> bool {
        *self == SLEEPMODE_A::BACKUP
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SLEEPMODE_A::OFF
    }
}
#[doc = "Write proxy for field `SLEEPMODE`"]
pub struct SLEEPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CPU, AHBx, and APBx clocks are OFF"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::IDLE)
    }
    #[doc = "All Clocks are OFF"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::STANDBY)
    }
    #[doc = "Backup domain is ON as well as some PDRAMs"]
    #[inline(always)]
    pub fn hibernate(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::HIBERNATE)
    }
    #[doc = "Only Backup domain is powered ON"]
    #[inline(always)]
    pub fn backup(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::BACKUP)
    }
    #[doc = "All power domains are powered OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::OFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline(always)]
    pub fn sleepmode(&self) -> SLEEPMODE_R {
        SLEEPMODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline(always)]
    pub fn sleepmode(&mut self) -> SLEEPMODE_W {
        SLEEPMODE_W { w: self }
    }
}
