#[doc = "Reader of register CHEVCTRL"]
pub type R = crate::R<u8, super::CHEVCTRL>;
#[doc = "Writer for register CHEVCTRL"]
pub type W = crate::W<u8, super::CHEVCTRL>;
#[doc = "Register CHEVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CHEVCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel Event Input Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVACT_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: Transfer and periodic transfer trigger"]
    TRIG = 1,
    #[doc = "2: Conditional transfer trigger"]
    CTRIG = 2,
    #[doc = "3: Conditional block transfer"]
    CBLOCK = 3,
    #[doc = "4: Channel suspend operation"]
    SUSPEND = 4,
    #[doc = "5: Channel resume operation"]
    RESUME = 5,
    #[doc = "6: Skip next block suspend action"]
    SSKIP = 6,
    #[doc = "7: Increase priority"]
    INCPRI = 7,
}
impl From<EVACT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EVACT`"]
pub type EVACT_R = crate::R<u8, EVACT_A>;
impl EVACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVACT_A {
        match self.bits {
            0 => EVACT_A::NOACT,
            1 => EVACT_A::TRIG,
            2 => EVACT_A::CTRIG,
            3 => EVACT_A::CBLOCK,
            4 => EVACT_A::SUSPEND,
            5 => EVACT_A::RESUME,
            6 => EVACT_A::SSKIP,
            7 => EVACT_A::INCPRI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == EVACT_A::NOACT
    }
    #[doc = "Checks if the value of the field is `TRIG`"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == EVACT_A::TRIG
    }
    #[doc = "Checks if the value of the field is `CTRIG`"]
    #[inline(always)]
    pub fn is_ctrig(&self) -> bool {
        *self == EVACT_A::CTRIG
    }
    #[doc = "Checks if the value of the field is `CBLOCK`"]
    #[inline(always)]
    pub fn is_cblock(&self) -> bool {
        *self == EVACT_A::CBLOCK
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == EVACT_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == EVACT_A::RESUME
    }
    #[doc = "Checks if the value of the field is `SSKIP`"]
    #[inline(always)]
    pub fn is_sskip(&self) -> bool {
        *self == EVACT_A::SSKIP
    }
    #[doc = "Checks if the value of the field is `INCPRI`"]
    #[inline(always)]
    pub fn is_incpri(&self) -> bool {
        *self == EVACT_A::INCPRI
    }
}
#[doc = "Write proxy for field `EVACT`"]
pub struct EVACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EVACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut W {
        self.variant(EVACT_A::NOACT)
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut W {
        self.variant(EVACT_A::TRIG)
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn ctrig(self) -> &'a mut W {
        self.variant(EVACT_A::CTRIG)
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn cblock(self) -> &'a mut W {
        self.variant(EVACT_A::CBLOCK)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(EVACT_A::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(EVACT_A::RESUME)
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn sskip(self) -> &'a mut W {
        self.variant(EVACT_A::SSKIP)
    }
    #[doc = "Increase priority"]
    #[inline(always)]
    pub fn incpri(self) -> &'a mut W {
        self.variant(EVACT_A::INCPRI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Channel Event Output Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVOMODE_A {
    #[doc = "0: Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    DEFAULT = 0,
    #[doc = "1: Ongoing trigger action"]
    TRIGACT = 1,
}
impl From<EVOMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EVOMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EVOMODE`"]
pub type EVOMODE_R = crate::R<u8, EVOMODE_A>;
impl EVOMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EVOMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EVOMODE_A::DEFAULT),
            1 => Val(EVOMODE_A::TRIGACT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == EVOMODE_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `TRIGACT`"]
    #[inline(always)]
    pub fn is_trigact(&self) -> bool {
        *self == EVOMODE_A::TRIGACT
    }
}
#[doc = "Write proxy for field `EVOMODE`"]
pub struct EVOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EVOMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVOMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(EVOMODE_A::DEFAULT)
    }
    #[doc = "Ongoing trigger action"]
    #[inline(always)]
    pub fn trigact(self) -> &'a mut W {
        self.variant(EVOMODE_A::TRIGACT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `EVIE`"]
pub type EVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVIE`"]
pub struct EVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EVIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EVOE`"]
pub type EVOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVOE`"]
pub struct EVOE_W<'a> {
    w: &'a mut W,
}
impl<'a> EVOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Channel Event Input Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - Channel Event Output Mode"]
    #[inline(always)]
    pub fn evomode(&self) -> EVOMODE_R {
        EVOMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&self) -> EVIE_R {
        EVIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EVOE_R {
        EVOE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel Event Input Action"]
    #[inline(always)]
    pub fn evact(&mut self) -> EVACT_W {
        EVACT_W { w: self }
    }
    #[doc = "Bits 4:5 - Channel Event Output Mode"]
    #[inline(always)]
    pub fn evomode(&mut self) -> EVOMODE_W {
        EVOMODE_W { w: self }
    }
    #[doc = "Bit 6 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&mut self) -> EVIE_W {
        EVIE_W { w: self }
    }
    #[doc = "Bit 7 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&mut self) -> EVOE_W {
        EVOE_W { w: self }
    }
}
