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
        0x02
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
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
            REFSELR::VDDANA => 0x01,
            REFSELR::VREFPB => 0x02,
            REFSELR::INTREF => 0x03,
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
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u8) & 0x01) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[doc = "Bit 0 - Differential mode enable"]
    #[inline]
    pub fn diff(&self) -> DIFFR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        DIFFR { bits }
    }
    #[doc = "Bits 1:2 - Reference Selection for DAC0/1"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
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
