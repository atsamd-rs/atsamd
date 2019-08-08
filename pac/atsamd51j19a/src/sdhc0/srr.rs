#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SRR {
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
#[doc = "Possible values of the field `SWRSTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTALLR {
    #[doc = "Work"]
    WORK,
    #[doc = "Reset"]
    RESET,
}
impl SWRSTALLR {
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
            SWRSTALLR::WORK => false,
            SWRSTALLR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRSTALLR {
        match value {
            false => SWRSTALLR::WORK,
            true => SWRSTALLR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `WORK`"]
    #[inline]
    pub fn is_work(&self) -> bool {
        *self == SWRSTALLR::WORK
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTALLR::RESET
    }
}
#[doc = "Possible values of the field `SWRSTCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTCMDR {
    #[doc = "Work"]
    WORK,
    #[doc = "Reset"]
    RESET,
}
impl SWRSTCMDR {
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
            SWRSTCMDR::WORK => false,
            SWRSTCMDR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRSTCMDR {
        match value {
            false => SWRSTCMDR::WORK,
            true => SWRSTCMDR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `WORK`"]
    #[inline]
    pub fn is_work(&self) -> bool {
        *self == SWRSTCMDR::WORK
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTCMDR::RESET
    }
}
#[doc = "Possible values of the field `SWRSTDAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTDATR {
    #[doc = "Work"]
    WORK,
    #[doc = "Reset"]
    RESET,
}
impl SWRSTDATR {
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
            SWRSTDATR::WORK => false,
            SWRSTDATR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRSTDATR {
        match value {
            false => SWRSTDATR::WORK,
            true => SWRSTDATR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `WORK`"]
    #[inline]
    pub fn is_work(&self) -> bool {
        *self == SWRSTDATR::WORK
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTDATR::RESET
    }
}
#[doc = "Values that can be written to the field `SWRSTALL`"]
pub enum SWRSTALLW {
    #[doc = "Work"]
    WORK,
    #[doc = "Reset"]
    RESET,
}
impl SWRSTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRSTALLW::WORK => false,
            SWRSTALLW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRSTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Work"]
    #[inline]
    pub fn work(self) -> &'a mut W {
        self.variant(SWRSTALLW::WORK)
    }
    #[doc = "Reset"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTALLW::RESET)
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
#[doc = "Values that can be written to the field `SWRSTCMD`"]
pub enum SWRSTCMDW {
    #[doc = "Work"]
    WORK,
    #[doc = "Reset"]
    RESET,
}
impl SWRSTCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRSTCMDW::WORK => false,
            SWRSTCMDW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRSTCMDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Work"]
    #[inline]
    pub fn work(self) -> &'a mut W {
        self.variant(SWRSTCMDW::WORK)
    }
    #[doc = "Reset"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTCMDW::RESET)
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
#[doc = "Values that can be written to the field `SWRSTDAT`"]
pub enum SWRSTDATW {
    #[doc = "Work"]
    WORK,
    #[doc = "Reset"]
    RESET,
}
impl SWRSTDATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRSTDATW::WORK => false,
            SWRSTDATW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTDATW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTDATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRSTDATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Work"]
    #[inline]
    pub fn work(self) -> &'a mut W {
        self.variant(SWRSTDATW::WORK)
    }
    #[doc = "Reset"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTDATW::RESET)
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
        const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Software Reset For All"]
    #[inline]
    pub fn swrstall(&self) -> SWRSTALLR {
        SWRSTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Software Reset For CMD Line"]
    #[inline]
    pub fn swrstcmd(&self) -> SWRSTCMDR {
        SWRSTCMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Software Reset For DAT Line"]
    #[inline]
    pub fn swrstdat(&self) -> SWRSTDATR {
        SWRSTDATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Software Reset For All"]
    #[inline]
    pub fn swrstall(&mut self) -> _SWRSTALLW {
        _SWRSTALLW { w: self }
    }
    #[doc = "Bit 1 - Software Reset For CMD Line"]
    #[inline]
    pub fn swrstcmd(&mut self) -> _SWRSTCMDW {
        _SWRSTCMDW { w: self }
    }
    #[doc = "Bit 2 - Software Reset For DAT Line"]
    #[inline]
    pub fn swrstdat(&mut self) -> _SWRSTDATW {
        _SWRSTDATW { w: self }
    }
}
