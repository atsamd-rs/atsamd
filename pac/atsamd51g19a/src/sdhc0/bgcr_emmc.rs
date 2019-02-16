#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::BGCR_EMMC {
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
#[doc = "Possible values of the field `STPBGR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPBGRR {
    #[doc = "Transfer"]
    TRANSFER,
    #[doc = "Stop"]
    STOP,
}
impl STPBGRR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            STPBGRR::TRANSFER => false,
            STPBGRR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STPBGRR {
        match value {
            false => STPBGRR::TRANSFER,
            true => STPBGRR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSFER`"]
    #[inline]
    pub fn is_transfer(&self) -> bool {
        *self == STPBGRR::TRANSFER
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == STPBGRR::STOP
    }
}
#[doc = "Possible values of the field `CONTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTRR {
    #[doc = "Not affected"]
    GO_ON,
    #[doc = "Restart"]
    RESTART,
}
impl CONTRR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CONTRR::GO_ON => false,
            CONTRR::RESTART => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONTRR {
        match value {
            false => CONTRR::GO_ON,
            true => CONTRR::RESTART,
        }
    }
    #[doc = "Checks if the value of the field is `GO_ON`"]
    #[inline]
    pub fn is_go_on(&self) -> bool {
        *self == CONTRR::GO_ON
    }
    #[doc = "Checks if the value of the field is `RESTART`"]
    #[inline]
    pub fn is_restart(&self) -> bool {
        *self == CONTRR::RESTART
    }
}
#[doc = "Values that can be written to the field `STPBGR`"]
pub enum STPBGRW {
    #[doc = "Transfer"]
    TRANSFER,
    #[doc = "Stop"]
    STOP,
}
impl STPBGRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STPBGRW::TRANSFER => false,
            STPBGRW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STPBGRW<'a> {
    w: &'a mut W,
}
impl<'a> _STPBGRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STPBGRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer"]
    #[inline]
    pub fn transfer(self) -> &'a mut W {
        self.variant(STPBGRW::TRANSFER)
    }
    #[doc = "Stop"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(STPBGRW::STOP)
    }
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
#[doc = "Values that can be written to the field `CONTR`"]
pub enum CONTRW {
    #[doc = "Not affected"]
    GO_ON,
    #[doc = "Restart"]
    RESTART,
}
impl CONTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONTRW::GO_ON => false,
            CONTRW::RESTART => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONTRW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not affected"]
    #[inline]
    pub fn go_on(self) -> &'a mut W {
        self.variant(CONTRW::GO_ON)
    }
    #[doc = "Restart"]
    #[inline]
    pub fn restart(self) -> &'a mut W {
        self.variant(CONTRW::RESTART)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Stop at Block Gap Request"]
    #[inline]
    pub fn stpbgr(&self) -> STPBGRR {
        STPBGRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline]
    pub fn contr(&self) -> CONTRR {
        CONTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    #[doc = "Bit 0 - Stop at Block Gap Request"]
    #[inline]
    pub fn stpbgr(&mut self) -> _STPBGRW {
        _STPBGRW { w: self }
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline]
    pub fn contr(&mut self) -> _CONTRW {
        _CONTRW { w: self }
    }
}
