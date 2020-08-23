#[doc = "Reader of register CTRLBCLR"]
pub type R = crate::R<u8, super::CTRLBCLR>;
#[doc = "Writer for register CTRLBCLR"]
pub type W = crate::W<u8, super::CTRLBCLR>;
#[doc = "Register CTRLBCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLBCLR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
#[doc = "Reader of field `LUPD`"]
pub type LUPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LUPD`"]
pub struct LUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> LUPD_W<'a> {
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
#[doc = "Reader of field `ONESHOT`"]
pub type ONESHOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONESHOT`"]
pub struct ONESHOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONESHOT_W<'a> {
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
#[doc = "Ramp Index Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDXCMD_A {
    #[doc = "0: Command disabled: Index toggles between cycles A and B"]
    DISABLE = 0,
    #[doc = "1: Set index: cycle B will be forced in the next cycle"]
    SET = 1,
    #[doc = "2: Clear index: cycle A will be forced in the next cycle"]
    CLEAR = 2,
    #[doc = "3: Hold index: the next cycle will be the same as the current cycle"]
    HOLD = 3,
}
impl From<IDXCMD_A> for u8 {
    #[inline(always)]
    fn from(variant: IDXCMD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IDXCMD`"]
pub type IDXCMD_R = crate::R<u8, IDXCMD_A>;
impl IDXCMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDXCMD_A {
        match self.bits {
            0 => IDXCMD_A::DISABLE,
            1 => IDXCMD_A::SET,
            2 => IDXCMD_A::CLEAR,
            3 => IDXCMD_A::HOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDXCMD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IDXCMD_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == IDXCMD_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `HOLD`"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == IDXCMD_A::HOLD
    }
}
#[doc = "Write proxy for field `IDXCMD`"]
pub struct IDXCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> IDXCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDXCMD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Command disabled: Index toggles between cycles A and B"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDXCMD_A::DISABLE)
    }
    #[doc = "Set index: cycle B will be forced in the next cycle"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IDXCMD_A::SET)
    }
    #[doc = "Clear index: cycle A will be forced in the next cycle"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IDXCMD_A::CLEAR)
    }
    #[doc = "Hold index: the next cycle will be the same as the current cycle"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut W {
        self.variant(IDXCMD_A::HOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u8) & 0x03) << 3);
        self.w
    }
}
#[doc = "TCC Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: Clear start, restart or retrigger"]
    RETRIGGER = 1,
    #[doc = "2: Force stop"]
    STOP = 2,
    #[doc = "3: Force update or double buffered registers"]
    UPDATE = 3,
    #[doc = "4: Force COUNT read synchronization"]
    READSYNC = 4,
    #[doc = "5: One-shot DMA trigger"]
    DMAOS = 5,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<u8, CMD_A>;
impl CMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMD_A::NONE),
            1 => Val(CMD_A::RETRIGGER),
            2 => Val(CMD_A::STOP),
            3 => Val(CMD_A::UPDATE),
            4 => Val(CMD_A::READSYNC),
            5 => Val(CMD_A::DMAOS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMD_A::NONE
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == CMD_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == CMD_A::STOP
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == CMD_A::UPDATE
    }
    #[doc = "Checks if the value of the field is `READSYNC`"]
    #[inline(always)]
    pub fn is_readsync(&self) -> bool {
        *self == CMD_A::READSYNC
    }
    #[doc = "Checks if the value of the field is `DMAOS`"]
    #[inline(always)]
    pub fn is_dmaos(&self) -> bool {
        *self == CMD_A::DMAOS
    }
}
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CMD_A::NONE)
    }
    #[doc = "Clear start, restart or retrigger"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(CMD_A::RETRIGGER)
    }
    #[doc = "Force stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CMD_A::STOP)
    }
    #[doc = "Force update or double buffered registers"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(CMD_A::UPDATE)
    }
    #[doc = "Force COUNT read synchronization"]
    #[inline(always)]
    pub fn readsync(self) -> &'a mut W {
        self.variant(CMD_A::READSYNC)
    }
    #[doc = "One-shot DMA trigger"]
    #[inline(always)]
    pub fn dmaos(self) -> &'a mut W {
        self.variant(CMD_A::DMAOS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u8) & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&self) -> LUPD_R {
        LUPD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline(always)]
    pub fn idxcmd(&self) -> IDXCMD_R {
        IDXCMD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - TCC Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&mut self) -> LUPD_W {
        LUPD_W { w: self }
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> ONESHOT_W {
        ONESHOT_W { w: self }
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline(always)]
    pub fn idxcmd(&mut self) -> IDXCMD_W {
        IDXCMD_W { w: self }
    }
    #[doc = "Bits 5:7 - TCC Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
}
