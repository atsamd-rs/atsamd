#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CTRLB {
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
pub struct DIFFR {
    bits: bool,
}
impl DIFFR {
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
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "External reference unbuffered"]
    VREFPU,
    #[doc = "Analog supply"]
    VDDANA,
    #[doc = "External reference buffered"]
    VREFPB,
    #[doc = "Internal bandgap reference"]
    INTREF,
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::VREFPU => 0,
            REFSELR::VDDANA => 1,
            REFSELR::VREFPB => 2,
            REFSELR::INTREF => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::VREFPU,
            1 => REFSELR::VDDANA,
            2 => REFSELR::VREFPB,
            3 => REFSELR::INTREF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VREFPU`"]
    #[inline]
    pub fn is_vrefpu(&self) -> bool {
        *self == REFSELR::VREFPU
    }
    #[doc = "Checks if the value of the field is `VDDANA`"]
    #[inline]
    pub fn is_vddana(&self) -> bool {
        *self == REFSELR::VDDANA
    }
    #[doc = "Checks if the value of the field is `VREFPB`"]
    #[inline]
    pub fn is_vrefpb(&self) -> bool {
        *self == REFSELR::VREFPB
    }
    #[doc = "Checks if the value of the field is `INTREF`"]
    #[inline]
    pub fn is_intref(&self) -> bool {
        *self == REFSELR::INTREF
    }
}
#[doc = r" Proxy"]
pub struct _DIFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFFW<'a> {
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
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "External reference unbuffered"]
    VREFPU,
    #[doc = "Analog supply"]
    VDDANA,
    #[doc = "External reference buffered"]
    VREFPB,
    #[doc = "Internal bandgap reference"]
    INTREF,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::VREFPU => 0,
            REFSELW::VDDANA => 1,
            REFSELW::VREFPB => 2,
            REFSELW::INTREF => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External reference unbuffered"]
    #[inline]
    pub fn vrefpu(self) -> &'a mut W {
        self.variant(REFSELW::VREFPU)
    }
    #[doc = "Analog supply"]
    #[inline]
    pub fn vddana(self) -> &'a mut W {
        self.variant(REFSELW::VDDANA)
    }
    #[doc = "External reference buffered"]
    #[inline]
    pub fn vrefpb(self) -> &'a mut W {
        self.variant(REFSELW::VREFPB)
    }
    #[doc = "Internal bandgap reference"]
    #[inline]
    pub fn intref(self) -> &'a mut W {
        self.variant(REFSELW::INTREF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 0 - Differential mode enable"]
    #[inline]
    pub fn diff(&self) -> DIFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        DIFFR { bits }
    }
    #[doc = "Bits 1:2 - Reference Selection for DAC0/1"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Differential mode enable"]
    #[inline]
    pub fn diff(&mut self) -> _DIFFW {
        _DIFFW { w: self }
    }
    #[doc = "Bits 1:2 - Reference Selection for DAC0/1"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
}
