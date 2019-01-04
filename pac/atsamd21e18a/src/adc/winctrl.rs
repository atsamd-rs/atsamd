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
#[doc = "Possible values of the field `WINMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINMODER {
    #[doc = "No window mode (default)"]
    DISABLE,
    #[doc = "Mode 1: RESULT > WINLT"]
    MODE1,
    #[doc = "Mode 2: RESULT < WINUT"]
    MODE2,
    #[doc = "Mode 3: WINLT < RESULT < WINUT"]
    MODE3,
    #[doc = "Mode 4: !(WINLT < RESULT < WINUT)"]
    MODE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WINMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WINMODER::DISABLE => 0,
            WINMODER::MODE1 => 1,
            WINMODER::MODE2 => 2,
            WINMODER::MODE3 => 3,
            WINMODER::MODE4 => 4,
            WINMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WINMODER {
        match value {
            0 => WINMODER::DISABLE,
            1 => WINMODER::MODE1,
            2 => WINMODER::MODE2,
            3 => WINMODER::MODE3,
            4 => WINMODER::MODE4,
            i => WINMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WINMODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline]
    pub fn is_mode1(&self) -> bool {
        *self == WINMODER::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline]
    pub fn is_mode2(&self) -> bool {
        *self == WINMODER::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline]
    pub fn is_mode3(&self) -> bool {
        *self == WINMODER::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline]
    pub fn is_mode4(&self) -> bool {
        *self == WINMODER::MODE4
    }
}
#[doc = "Values that can be written to the field `WINMODE`"]
pub enum WINMODEW {
    #[doc = "No window mode (default)"]
    DISABLE,
    #[doc = "Mode 1: RESULT > WINLT"]
    MODE1,
    #[doc = "Mode 2: RESULT < WINUT"]
    MODE2,
    #[doc = "Mode 3: WINLT < RESULT < WINUT"]
    MODE3,
    #[doc = "Mode 4: !(WINLT < RESULT < WINUT)"]
    MODE4,
}
impl WINMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WINMODEW::DISABLE => 0,
            WINMODEW::MODE1 => 1,
            WINMODEW::MODE2 => 2,
            WINMODEW::MODE3 => 3,
            WINMODEW::MODE4 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WINMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WINMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WINMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No window mode (default)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WINMODEW::DISABLE)
    }
    #[doc = "Mode 1: RESULT > WINLT"]
    #[inline]
    pub fn mode1(self) -> &'a mut W {
        self.variant(WINMODEW::MODE1)
    }
    #[doc = "Mode 2: RESULT < WINUT"]
    #[inline]
    pub fn mode2(self) -> &'a mut W {
        self.variant(WINMODEW::MODE2)
    }
    #[doc = "Mode 3: WINLT < RESULT < WINUT"]
    #[inline]
    pub fn mode3(self) -> &'a mut W {
        self.variant(WINMODEW::MODE3)
    }
    #[doc = "Mode 4: !(WINLT < RESULT < WINUT)"]
    #[inline]
    pub fn mode4(self) -> &'a mut W {
        self.variant(WINMODEW::MODE4)
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
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline]
    pub fn winmode(&self) -> WINMODER {
        WINMODER::_from({
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
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline]
    pub fn winmode(&mut self) -> _WINMODEW {
        _WINMODEW { w: self }
    }
}
