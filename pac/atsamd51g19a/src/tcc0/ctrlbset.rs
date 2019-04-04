#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CTRLBSET {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct DIRR {
    bits: bool,
}
impl DIRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct LUPDR {
    bits: bool,
}
impl LUPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ONESHOTR {
    bits: bool,
}
impl ONESHOTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `IDXCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDXCMDR {
    #[doc = "Command disabled: Index toggles between cycles A and B"]
    DISABLE,
    #[doc = "Set index: cycle B will be forced in the next cycle"]
    SET,
    #[doc = "Clear index: cycle A will be forced in the next cycle"]
    CLEAR,
    #[doc = "Hold index: the next cycle will be the same as the current cycle"]
    HOLD,
}
impl IDXCMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDXCMDR::DISABLE => 0,
            IDXCMDR::SET => 1,
            IDXCMDR::CLEAR => 2,
            IDXCMDR::HOLD => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDXCMDR {
        match value {
            0 => IDXCMDR::DISABLE,
            1 => IDXCMDR::SET,
            2 => IDXCMDR::CLEAR,
            3 => IDXCMDR::HOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IDXCMDR::DISABLE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == IDXCMDR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == IDXCMDR::CLEAR
    }
    #[doc = "Checks if the value of the field is `HOLD`"]
    #[inline]
    pub fn is_hold(&self) -> bool {
        *self == IDXCMDR::HOLD
    }
}
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDR {
    #[doc = "No action"]
    NONE,
    #[doc = "Clear start, restart or retrigger"]
    RETRIGGER,
    #[doc = "Force stop"]
    STOP,
    #[doc = "Force update or double buffered registers"]
    UPDATE,
    #[doc = "Force COUNT read synchronization"]
    READSYNC,
    #[doc = "One-shot DMA trigger"]
    DMAOS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDR::NONE => 0,
            CMDR::RETRIGGER => 1,
            CMDR::STOP => 2,
            CMDR::UPDATE => 3,
            CMDR::READSYNC => 4,
            CMDR::DMAOS => 5,
            CMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDR {
        match value {
            0 => CMDR::NONE,
            1 => CMDR::RETRIGGER,
            2 => CMDR::STOP,
            3 => CMDR::UPDATE,
            4 => CMDR::READSYNC,
            5 => CMDR::DMAOS,
            i => CMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == CMDR::NONE
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline]
    pub fn is_retrigger(&self) -> bool {
        *self == CMDR::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == CMDR::STOP
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline]
    pub fn is_update(&self) -> bool {
        *self == CMDR::UPDATE
    }
    #[doc = "Checks if the value of the field is `READSYNC`"]
    #[inline]
    pub fn is_readsync(&self) -> bool {
        *self == CMDR::READSYNC
    }
    #[doc = "Checks if the value of the field is `DMAOS`"]
    #[inline]
    pub fn is_dmaos(&self) -> bool {
        *self == CMDR::DMAOS
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _LUPDW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ONESHOTW<'a> {
    w: &'a mut W,
}
impl<'a> _ONESHOTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDXCMD`"]
pub enum IDXCMDW {
    #[doc = "Command disabled: Index toggles between cycles A and B"]
    DISABLE,
    #[doc = "Set index: cycle B will be forced in the next cycle"]
    SET,
    #[doc = "Clear index: cycle A will be forced in the next cycle"]
    CLEAR,
    #[doc = "Hold index: the next cycle will be the same as the current cycle"]
    HOLD,
}
impl IDXCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDXCMDW::DISABLE => 0,
            IDXCMDW::SET => 1,
            IDXCMDW::CLEAR => 2,
            IDXCMDW::HOLD => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDXCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _IDXCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDXCMDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Command disabled: Index toggles between cycles A and B"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDXCMDW::DISABLE)
    }
    #[doc = "Set index: cycle B will be forced in the next cycle"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(IDXCMDW::SET)
    }
    #[doc = "Clear index: cycle A will be forced in the next cycle"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(IDXCMDW::CLEAR)
    }
    #[doc = "Hold index: the next cycle will be the same as the current cycle"]
    #[inline]
    pub fn hold(self) -> &'a mut W {
        self.variant(IDXCMDW::HOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "No action"]
    NONE,
    #[doc = "Clear start, restart or retrigger"]
    RETRIGGER,
    #[doc = "Force stop"]
    STOP,
    #[doc = "Force update or double buffered registers"]
    UPDATE,
    #[doc = "Force COUNT read synchronization"]
    READSYNC,
    #[doc = "One-shot DMA trigger"]
    DMAOS,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::NONE => 0,
            CMDW::RETRIGGER => 1,
            CMDW::STOP => 2,
            CMDW::UPDATE => 3,
            CMDW::READSYNC => 4,
            CMDW::DMAOS => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No action"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(CMDW::NONE)
    }
    #[doc = "Clear start, restart or retrigger"]
    #[inline]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(CMDW::RETRIGGER)
    }
    #[doc = "Force stop"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(CMDW::STOP)
    }
    #[doc = "Force update or double buffered registers"]
    #[inline]
    pub fn update(self) -> &'a mut W {
        self.variant(CMDW::UPDATE)
    }
    #[doc = "Force COUNT read synchronization"]
    #[inline]
    pub fn readsync(self) -> &'a mut W {
        self.variant(CMDW::READSYNC)
    }
    #[doc = "One-shot DMA trigger"]
    #[inline]
    pub fn dmaos(self) -> &'a mut W {
        self.variant(CMDW::DMAOS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Counter Direction"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        DIRR { bits }
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline]
    pub fn lupd(&self) -> LUPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        LUPDR { bits }
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline]
    pub fn oneshot(&self) -> ONESHOTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        ONESHOTR { bits }
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline]
    pub fn idxcmd(&self) -> IDXCMDR {
        IDXCMDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 5:7 - TCC Command"]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        CMDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Counter Direction"]
    #[inline]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline]
    pub fn lupd(&mut self) -> _LUPDW {
        _LUPDW { w: self }
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline]
    pub fn oneshot(&mut self) -> _ONESHOTW {
        _ONESHOTW { w: self }
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline]
    pub fn idxcmd(&mut self) -> _IDXCMDW {
        _IDXCMDW { w: self }
    }
    #[doc = "Bits 5:7 - TCC Command"]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
}
