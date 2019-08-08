#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::NMICTRL {
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
#[doc = "Possible values of the field `NMISENSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMISENSER {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising-edge detection"]
    RISE,
    #[doc = "Falling-edge detection"]
    FALL,
    #[doc = "Both-edges detection"]
    BOTH,
    #[doc = "High-level detection"]
    HIGH,
    #[doc = "Low-level detection"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NMISENSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NMISENSER::NONE => 0,
            NMISENSER::RISE => 1,
            NMISENSER::FALL => 2,
            NMISENSER::BOTH => 3,
            NMISENSER::HIGH => 4,
            NMISENSER::LOW => 5,
            NMISENSER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NMISENSER {
        match value {
            0 => NMISENSER::NONE,
            1 => NMISENSER::RISE,
            2 => NMISENSER::FALL,
            3 => NMISENSER::BOTH,
            4 => NMISENSER::HIGH,
            5 => NMISENSER::LOW,
            i => NMISENSER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == NMISENSER::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == NMISENSER::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == NMISENSER::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == NMISENSER::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == NMISENSER::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == NMISENSER::LOW
    }
}
#[doc = r" Value of the field"]
pub struct NMIFILTENR {
    bits: bool,
}
impl NMIFILTENR {
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
#[doc = "Values that can be written to the field `NMISENSE`"]
pub enum NMISENSEW {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising-edge detection"]
    RISE,
    #[doc = "Falling-edge detection"]
    FALL,
    #[doc = "Both-edges detection"]
    BOTH,
    #[doc = "High-level detection"]
    HIGH,
    #[doc = "Low-level detection"]
    LOW,
}
impl NMISENSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NMISENSEW::NONE => 0,
            NMISENSEW::RISE => 1,
            NMISENSEW::FALL => 2,
            NMISENSEW::BOTH => 3,
            NMISENSEW::HIGH => 4,
            NMISENSEW::LOW => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NMISENSEW<'a> {
    w: &'a mut W,
}
impl<'a> _NMISENSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NMISENSEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No detection"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(NMISENSEW::NONE)
    }
    #[doc = "Rising-edge detection"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(NMISENSEW::RISE)
    }
    #[doc = "Falling-edge detection"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(NMISENSEW::FALL)
    }
    #[doc = "Both-edges detection"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(NMISENSEW::BOTH)
    }
    #[doc = "High-level detection"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(NMISENSEW::HIGH)
    }
    #[doc = "Low-level detection"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(NMISENSEW::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NMIFILTENW<'a> {
    w: &'a mut W,
}
impl<'a> _NMIFILTENW<'a> {
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bits 0:2 - Non-Maskable Interrupt Sense"]
    #[inline]
    pub fn nmisense(&self) -> NMISENSER {
        NMISENSER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 3 - Non-Maskable Interrupt Filter Enable"]
    #[inline]
    pub fn nmifilten(&self) -> NMIFILTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        NMIFILTENR { bits }
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
    #[doc = "Bits 0:2 - Non-Maskable Interrupt Sense"]
    #[inline]
    pub fn nmisense(&mut self) -> _NMISENSEW {
        _NMISENSEW { w: self }
    }
    #[doc = "Bit 3 - Non-Maskable Interrupt Filter Enable"]
    #[inline]
    pub fn nmifilten(&mut self) -> _NMIFILTENW {
        _NMIFILTENW { w: self }
    }
}
