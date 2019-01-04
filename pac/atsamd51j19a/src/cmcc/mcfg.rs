#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCFG {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "cycle counter"]
    CYCLE_COUNT,
    #[doc = "instruction hit counter"]
    IHIT_COUNT,
    #[doc = "data hit counter"]
    DHIT_COUNT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::CYCLE_COUNT => 0,
            MODER::IHIT_COUNT => 1,
            MODER::DHIT_COUNT => 2,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::CYCLE_COUNT,
            1 => MODER::IHIT_COUNT,
            2 => MODER::DHIT_COUNT,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE_COUNT`"]
    #[inline]
    pub fn is_cycle_count(&self) -> bool {
        *self == MODER::CYCLE_COUNT
    }
    #[doc = "Checks if the value of the field is `IHIT_COUNT`"]
    #[inline]
    pub fn is_ihit_count(&self) -> bool {
        *self == MODER::IHIT_COUNT
    }
    #[doc = "Checks if the value of the field is `DHIT_COUNT`"]
    #[inline]
    pub fn is_dhit_count(&self) -> bool {
        *self == MODER::DHIT_COUNT
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "cycle counter"]
    CYCLE_COUNT,
    #[doc = "instruction hit counter"]
    IHIT_COUNT,
    #[doc = "data hit counter"]
    DHIT_COUNT,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::CYCLE_COUNT => 0,
            MODEW::IHIT_COUNT => 1,
            MODEW::DHIT_COUNT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "cycle counter"]
    #[inline]
    pub fn cycle_count(self) -> &'a mut W {
        self.variant(MODEW::CYCLE_COUNT)
    }
    #[doc = "instruction hit counter"]
    #[inline]
    pub fn ihit_count(self) -> &'a mut W {
        self.variant(MODEW::IHIT_COUNT)
    }
    #[doc = "data hit counter"]
    #[inline]
    pub fn dhit_count(self) -> &'a mut W {
        self.variant(MODEW::DHIT_COUNT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Cache Controller Monitor Counter Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Cache Controller Monitor Counter Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
