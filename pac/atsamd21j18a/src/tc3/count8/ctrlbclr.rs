#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CTRLBCLR {
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
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDR {
    #[doc = "No action"]
    NONE,
    #[doc = "Force a start, restart or retrigger"]
    RETRIGGER,
    #[doc = "Force a stop"]
    STOP,
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
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "No action"]
    NONE,
    #[doc = "Force a start, restart or retrigger"]
    RETRIGGER,
    #[doc = "Force a stop"]
    STOP,
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
    #[doc = "Force a start, restart or retrigger"]
    #[inline]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(CMDW::RETRIGGER)
    }
    #[doc = "Force a stop"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(CMDW::STOP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 6:7 - Command"]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        CMDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
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
    #[doc = "Bit 2 - One-Shot"]
    #[inline]
    pub fn oneshot(&mut self) -> _ONESHOTW {
        _ONESHOTW { w: self }
    }
    #[doc = "Bits 6:7 - Command"]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
}
