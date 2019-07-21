#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::WINCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
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
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u8 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct WEN0R {
    bits: bool,
}
impl WEN0R {
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
#[doc = "Possible values of the field `WINTSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINTSEL0R {
    #[doc = "Interrupt on signal above window"]
    ABOVE,
    #[doc = "Interrupt on signal inside window"]
    INSIDE,
    #[doc = "Interrupt on signal below window"]
    BELOW,
    #[doc = "Interrupt on signal outside window"]
    OUTSIDE,
}
impl WINTSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WINTSEL0R::ABOVE => 0,
            WINTSEL0R::INSIDE => 0x01,
            WINTSEL0R::BELOW => 0x02,
            WINTSEL0R::OUTSIDE => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WINTSEL0R {
        match value {
            0 => WINTSEL0R::ABOVE,
            1 => WINTSEL0R::INSIDE,
            2 => WINTSEL0R::BELOW,
            3 => WINTSEL0R::OUTSIDE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline]
    pub fn is_above(&self) -> bool {
        *self == WINTSEL0R::ABOVE
    }
    #[doc = "Checks if the value of the field is `INSIDE`"]
    #[inline]
    pub fn is_inside(&self) -> bool {
        *self == WINTSEL0R::INSIDE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline]
    pub fn is_below(&self) -> bool {
        *self == WINTSEL0R::BELOW
    }
    #[doc = "Checks if the value of the field is `OUTSIDE`"]
    #[inline]
    pub fn is_outside(&self) -> bool {
        *self == WINTSEL0R::OUTSIDE
    }
}
#[doc = r" Proxy"]
pub struct _WEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _WEN0W<'a> {
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
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u8) & 0x01) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `WINTSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINTSEL0W {
    #[doc = "Interrupt on signal above window"]
    ABOVE,
    #[doc = "Interrupt on signal inside window"]
    INSIDE,
    #[doc = "Interrupt on signal below window"]
    BELOW,
    #[doc = "Interrupt on signal outside window"]
    OUTSIDE,
}
impl WINTSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WINTSEL0W::ABOVE => 0,
            WINTSEL0W::INSIDE => 1,
            WINTSEL0W::BELOW => 2,
            WINTSEL0W::OUTSIDE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WINTSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _WINTSEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WINTSEL0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt on signal above window"]
    #[inline]
    pub fn above(self) -> &'a mut W {
        self.variant(WINTSEL0W::ABOVE)
    }
    #[doc = "Interrupt on signal inside window"]
    #[inline]
    pub fn inside(self) -> &'a mut W {
        self.variant(WINTSEL0W::INSIDE)
    }
    #[doc = "Interrupt on signal below window"]
    #[inline]
    pub fn below(self) -> &'a mut W {
        self.variant(WINTSEL0W::BELOW)
    }
    #[doc = "Interrupt on signal outside window"]
    #[inline]
    pub fn outside(self) -> &'a mut W {
        self.variant(WINTSEL0W::OUTSIDE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 1);
        self.w.bits |= ((value as u8) & 0x03) << 1;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Window 0 Mode Enable"]
    #[inline]
    pub fn wen0(&self) -> WEN0R {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        WEN0R { bits }
    }
    #[doc = "Bits 1:2 - Window 0 Interrupt Selection"]
    #[inline]
    pub fn wintsel0(&self) -> WINTSEL0R {
        WINTSEL0R::_from(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Window 0 Mode Enable"]
    #[inline]
    pub fn wen0(&mut self) -> _WEN0W {
        _WEN0W { w: self }
    }
    #[doc = "Bits 1:2 - Window 0 Interrupt Selection"]
    #[inline]
    pub fn wintsel0(&mut self) -> _WINTSEL0W {
        _WINTSEL0W { w: self }
    }
}
