#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MASK {
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
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "Alarm Disabled"]
    OFF,
    #[doc = "Match seconds only"]
    SS,
    #[doc = "Match seconds and minutes only"]
    MMSS,
    #[doc = "Match seconds, minutes, and hours only"]
    HHMMSS,
    #[doc = "Match seconds, minutes, hours, and days only"]
    DDHHMMSS,
    #[doc = "Match seconds, minutes, hours, days, and months only"]
    MMDDHHMMSS,
    #[doc = "Match seconds, minutes, hours, days, months, and years"]
    YYMMDDHHMMSS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELR::OFF => 0,
            SELR::SS => 1,
            SELR::MMSS => 2,
            SELR::HHMMSS => 3,
            SELR::DDHHMMSS => 4,
            SELR::MMDDHHMMSS => 5,
            SELR::YYMMDDHHMMSS => 6,
            SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELR {
        match value {
            0 => SELR::OFF,
            1 => SELR::SS,
            2 => SELR::MMSS,
            3 => SELR::HHMMSS,
            4 => SELR::DDHHMMSS,
            5 => SELR::MMDDHHMMSS,
            6 => SELR::YYMMDDHHMMSS,
            i => SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == SELR::OFF
    }
    #[doc = "Checks if the value of the field is `SS`"]
    #[inline]
    pub fn is_ss(&self) -> bool {
        *self == SELR::SS
    }
    #[doc = "Checks if the value of the field is `MMSS`"]
    #[inline]
    pub fn is_mmss(&self) -> bool {
        *self == SELR::MMSS
    }
    #[doc = "Checks if the value of the field is `HHMMSS`"]
    #[inline]
    pub fn is_hhmmss(&self) -> bool {
        *self == SELR::HHMMSS
    }
    #[doc = "Checks if the value of the field is `DDHHMMSS`"]
    #[inline]
    pub fn is_ddhhmmss(&self) -> bool {
        *self == SELR::DDHHMMSS
    }
    #[doc = "Checks if the value of the field is `MMDDHHMMSS`"]
    #[inline]
    pub fn is_mmddhhmmss(&self) -> bool {
        *self == SELR::MMDDHHMMSS
    }
    #[doc = "Checks if the value of the field is `YYMMDDHHMMSS`"]
    #[inline]
    pub fn is_yymmddhhmmss(&self) -> bool {
        *self == SELR::YYMMDDHHMMSS
    }
}
#[doc = "Values that can be written to the field `SEL`"]
pub enum SELW {
    #[doc = "Alarm Disabled"]
    OFF,
    #[doc = "Match seconds only"]
    SS,
    #[doc = "Match seconds and minutes only"]
    MMSS,
    #[doc = "Match seconds, minutes, and hours only"]
    HHMMSS,
    #[doc = "Match seconds, minutes, hours, and days only"]
    DDHHMMSS,
    #[doc = "Match seconds, minutes, hours, days, and months only"]
    MMDDHHMMSS,
    #[doc = "Match seconds, minutes, hours, days, months, and years"]
    YYMMDDHHMMSS,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::OFF => 0,
            SELW::SS => 1,
            SELW::MMSS => 2,
            SELW::HHMMSS => 3,
            SELW::DDHHMMSS => 4,
            SELW::MMDDHHMMSS => 5,
            SELW::YYMMDDHHMMSS => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alarm Disabled"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(SELW::OFF)
    }
    #[doc = "Match seconds only"]
    #[inline]
    pub fn ss(self) -> &'a mut W {
        self.variant(SELW::SS)
    }
    #[doc = "Match seconds and minutes only"]
    #[inline]
    pub fn mmss(self) -> &'a mut W {
        self.variant(SELW::MMSS)
    }
    #[doc = "Match seconds, minutes, and hours only"]
    #[inline]
    pub fn hhmmss(self) -> &'a mut W {
        self.variant(SELW::HHMMSS)
    }
    #[doc = "Match seconds, minutes, hours, and days only"]
    #[inline]
    pub fn ddhhmmss(self) -> &'a mut W {
        self.variant(SELW::DDHHMMSS)
    }
    #[doc = "Match seconds, minutes, hours, days, and months only"]
    #[inline]
    pub fn mmddhhmmss(self) -> &'a mut W {
        self.variant(SELW::MMDDHHMMSS)
    }
    #[doc = "Match seconds, minutes, hours, days, months, and years"]
    #[inline]
    pub fn yymmddhhmmss(self) -> &'a mut W {
        self.variant(SELW::YYMMDDHHMMSS)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:2 - Alarm Mask Selection"]
    #[inline]
    pub fn sel(&self) -> SELR {
        SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - Alarm Mask Selection"]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
