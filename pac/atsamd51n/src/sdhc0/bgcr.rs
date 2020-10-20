#[doc = "Reader of register BGCR"]
pub type R = crate::R<u8, super::BGCR>;
#[doc = "Writer for register BGCR"]
pub type W = crate::W<u8, super::BGCR>;
#[doc = "Register BGCR `reset()`'s with value 0"]
impl crate::ResetValue for super::BGCR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Stop at Block Gap Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPBGR_A {
    #[doc = "0: Transfer"]
    TRANSFER = 0,
    #[doc = "1: Stop"]
    STOP = 1,
}
impl From<STPBGR_A> for bool {
    #[inline(always)]
    fn from(variant: STPBGR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STPBGR`"]
pub type STPBGR_R = crate::R<bool, STPBGR_A>;
impl STPBGR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPBGR_A {
        match self.bits {
            false => STPBGR_A::TRANSFER,
            true => STPBGR_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSFER`"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == STPBGR_A::TRANSFER
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STPBGR_A::STOP
    }
}
#[doc = "Write proxy for field `STPBGR`"]
pub struct STPBGR_W<'a> {
    w: &'a mut W,
}
impl<'a> STPBGR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPBGR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut W {
        self.variant(STPBGR_A::TRANSFER)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STPBGR_A::STOP)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Continue Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTR_A {
    #[doc = "0: Not affected"]
    GO_ON = 0,
    #[doc = "1: Restart"]
    RESTART = 1,
}
impl From<CONTR_A> for bool {
    #[inline(always)]
    fn from(variant: CONTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONTR`"]
pub type CONTR_R = crate::R<bool, CONTR_A>;
impl CONTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONTR_A {
        match self.bits {
            false => CONTR_A::GO_ON,
            true => CONTR_A::RESTART,
        }
    }
    #[doc = "Checks if the value of the field is `GO_ON`"]
    #[inline(always)]
    pub fn is_go_on(&self) -> bool {
        *self == CONTR_A::GO_ON
    }
    #[doc = "Checks if the value of the field is `RESTART`"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == CONTR_A::RESTART
    }
}
#[doc = "Write proxy for field `CONTR`"]
pub struct CONTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not affected"]
    #[inline(always)]
    pub fn go_on(self) -> &'a mut W {
        self.variant(CONTR_A::GO_ON)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut W {
        self.variant(CONTR_A::RESTART)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Read Wait Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWCTRL_A {
    #[doc = "0: Disable Read Wait Control"]
    DISABLE = 0,
    #[doc = "1: Enable Read Wait Control"]
    ENABLE = 1,
}
impl From<RWCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RWCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RWCTRL`"]
pub type RWCTRL_R = crate::R<bool, RWCTRL_A>;
impl RWCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWCTRL_A {
        match self.bits {
            false => RWCTRL_A::DISABLE,
            true => RWCTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RWCTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RWCTRL_A::ENABLE
    }
}
#[doc = "Write proxy for field `RWCTRL`"]
pub struct RWCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RWCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWCTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RWCTRL_A::DISABLE)
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RWCTRL_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Interrupt at Block Gap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTBG_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<INTBG_A> for bool {
    #[inline(always)]
    fn from(variant: INTBG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTBG`"]
pub type INTBG_R = crate::R<bool, INTBG_A>;
impl INTBG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTBG_A {
        match self.bits {
            false => INTBG_A::DISABLED,
            true => INTBG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTBG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTBG_A::ENABLED
    }
}
#[doc = "Write proxy for field `INTBG`"]
pub struct INTBG_W<'a> {
    w: &'a mut W,
}
impl<'a> INTBG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTBG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTBG_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTBG_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Stop at Block Gap Request"]
    #[inline(always)]
    pub fn stpbgr(&self) -> STPBGR_R {
        STPBGR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn contr(&self) -> CONTR_R {
        CONTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctrl(&self) -> RWCTRL_R {
        RWCTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt at Block Gap"]
    #[inline(always)]
    pub fn intbg(&self) -> INTBG_R {
        INTBG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop at Block Gap Request"]
    #[inline(always)]
    pub fn stpbgr(&mut self) -> STPBGR_W {
        STPBGR_W { w: self }
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn contr(&mut self) -> CONTR_W {
        CONTR_W { w: self }
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctrl(&mut self) -> RWCTRL_W {
        RWCTRL_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt at Block Gap"]
    #[inline(always)]
    pub fn intbg(&mut self) -> INTBG_W {
        INTBG_W { w: self }
    }
}
