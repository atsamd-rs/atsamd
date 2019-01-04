#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BBPS {
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
#[doc = "Possible values of the field `CONF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONFR {
    #[doc = "The power switch is handled by the BOD33"]
    BOD33,
    #[doc = "In Backup Domain, the backup domain is always supplied by battery backup power"]
    FORCED,
}
impl CONFR {
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
            CONFR::BOD33 => false,
            CONFR::FORCED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONFR {
        match value {
            false => CONFR::BOD33,
            true => CONFR::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `BOD33`"]
    #[inline]
    pub fn is_bod33(&self) -> bool {
        *self == CONFR::BOD33
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline]
    pub fn is_forced(&self) -> bool {
        *self == CONFR::FORCED
    }
}
#[doc = r" Value of the field"]
pub struct WAKEENR {
    bits: bool,
}
impl WAKEENR {
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
#[doc = "Values that can be written to the field `CONF`"]
pub enum CONFW {
    #[doc = "The power switch is handled by the BOD33"]
    BOD33,
    #[doc = "In Backup Domain, the backup domain is always supplied by battery backup power"]
    FORCED,
}
impl CONFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONFW::BOD33 => false,
            CONFW::FORCED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONFW<'a> {
    w: &'a mut W,
}
impl<'a> _CONFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The power switch is handled by the BOD33"]
    #[inline]
    pub fn bod33(self) -> &'a mut W {
        self.variant(CONFW::BOD33)
    }
    #[doc = "In Backup Domain, the backup domain is always supplied by battery backup power"]
    #[inline]
    pub fn forced(self) -> &'a mut W {
        self.variant(CONFW::FORCED)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAKEENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEENW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Battery Backup Configuration"]
    #[inline]
    pub fn conf(&self) -> CONFR {
        CONFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline]
    pub fn wakeen(&self) -> WAKEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAKEENR { bits }
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Battery Backup Configuration"]
    #[inline]
    pub fn conf(&mut self) -> _CONFW {
        _CONFW { w: self }
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline]
    pub fn wakeen(&mut self) -> _WAKEENW {
        _WAKEENW { w: self }
    }
}
