#[doc = "Reader of register CTICTRLA%s"]
pub type R = crate::R<u8, super::CTICTRLA>;
#[doc = "Writer for register CTICTRLA%s"]
pub type W = crate::W<u8, super::CTICTRLA>;
#[doc = "Register CTICTRLA%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CTICTRLA {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Action when global break issued\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRK_A {
    #[doc = "0: Break when requested"]
    BREAK = 0,
    #[doc = "1: Trigger DBG interrupt instead of break"]
    INTERRUPT = 1,
    #[doc = "2: Ignore break request"]
    IGNORE = 2,
}
impl From<BRK_A> for u8 {
    #[inline(always)]
    fn from(variant: BRK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BRK`"]
pub type BRK_R = crate::R<u8, BRK_A>;
impl BRK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BRK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BRK_A::BREAK),
            1 => Val(BRK_A::INTERRUPT),
            2 => Val(BRK_A::IGNORE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BREAK`"]
    #[inline(always)]
    pub fn is_break_(&self) -> bool {
        *self == BRK_A::BREAK
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == BRK_A::INTERRUPT
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == BRK_A::IGNORE
    }
}
#[doc = "Write proxy for field `BRK`"]
pub struct BRK_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Break when requested"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(BRK_A::BREAK)
    }
    #[doc = "Trigger DBG interrupt instead of break"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(BRK_A::INTERRUPT)
    }
    #[doc = "Ignore break request"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(BRK_A::IGNORE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Action when global restart issued\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESTART_A {
    #[doc = "0: Restart when requested"]
    RESTART = 0,
    #[doc = "1: Ignore restart request"]
    IGNORE = 1,
}
impl From<RESTART_A> for bool {
    #[inline(always)]
    fn from(variant: RESTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESTART`"]
pub type RESTART_R = crate::R<bool, RESTART_A>;
impl RESTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESTART_A {
        match self.bits {
            false => RESTART_A::RESTART,
            true => RESTART_A::IGNORE,
        }
    }
    #[doc = "Checks if the value of the field is `RESTART`"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == RESTART_A::RESTART
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == RESTART_A::IGNORE
    }
}
#[doc = "Write proxy for field `RESTART`"]
pub struct RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Restart when requested"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut W {
        self.variant(RESTART_A::RESTART)
    }
    #[doc = "Ignore restart request"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(RESTART_A::IGNORE)
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
#[doc = "Action when inter-process resource freed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPS_A {
    #[doc = "0: Generate CPU Event when awaited resource is freed."]
    EVENT = 0,
    #[doc = "1: Generate Interrupt when awaited resource is freed."]
    INTERRUPT = 1,
}
impl From<IPS_A> for bool {
    #[inline(always)]
    fn from(variant: IPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IPS`"]
pub type IPS_R = crate::R<bool, IPS_A>;
impl IPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPS_A {
        match self.bits {
            false => IPS_A::EVENT,
            true => IPS_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == IPS_A::EVENT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == IPS_A::INTERRUPT
    }
}
#[doc = "Write proxy for field `IPS`"]
pub struct IPS_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generate CPU Event when awaited resource is freed."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(IPS_A::EVENT)
    }
    #[doc = "Generate Interrupt when awaited resource is freed."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(IPS_A::INTERRUPT)
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
    #[doc = "Bits 0:1 - Action when global break issued"]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Action when global restart issued"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Action when inter-process resource freed"]
    #[inline(always)]
    pub fn ips(&self) -> IPS_R {
        IPS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Action when global break issued"]
    #[inline(always)]
    pub fn brk(&mut self) -> BRK_W {
        BRK_W { w: self }
    }
    #[doc = "Bit 2 - Action when global restart issued"]
    #[inline(always)]
    pub fn restart(&mut self) -> RESTART_W {
        RESTART_W { w: self }
    }
    #[doc = "Bit 3 - Action when inter-process resource freed"]
    #[inline(always)]
    pub fn ips(&mut self) -> IPS_W {
        IPS_W { w: self }
    }
}
