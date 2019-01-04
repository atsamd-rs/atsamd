#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCHCTRL {
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
#[doc = "Possible values of the field `GEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENR {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GENR::GCLK0 => 0,
            GENR::GCLK1 => 1,
            GENR::GCLK2 => 2,
            GENR::GCLK3 => 3,
            GENR::GCLK4 => 4,
            GENR::GCLK5 => 5,
            GENR::GCLK6 => 6,
            GENR::GCLK7 => 7,
            GENR::GCLK8 => 8,
            GENR::GCLK9 => 9,
            GENR::GCLK10 => 10,
            GENR::GCLK11 => 11,
            GENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GENR {
        match value {
            0 => GENR::GCLK0,
            1 => GENR::GCLK1,
            2 => GENR::GCLK2,
            3 => GENR::GCLK3,
            4 => GENR::GCLK4,
            5 => GENR::GCLK5,
            6 => GENR::GCLK6,
            7 => GENR::GCLK7,
            8 => GENR::GCLK8,
            9 => GENR::GCLK9,
            10 => GENR::GCLK10,
            11 => GENR::GCLK11,
            i => GENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENR::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENR::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENR::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENR::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENR::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENR::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENR::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENR::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENR::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENR::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENR::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENR::GCLK11
    }
}
#[doc = r" Value of the field"]
pub struct CHENR {
    bits: bool,
}
impl CHENR {
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
#[doc = r" Value of the field"]
pub struct WRTLOCKR {
    bits: bool,
}
impl WRTLOCKR {
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
#[doc = "Values that can be written to the field `GEN`"]
pub enum GENW {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
}
impl GENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GENW::GCLK0 => 0,
            GENW::GCLK1 => 1,
            GENW::GCLK2 => 2,
            GENW::GCLK3 => 3,
            GENW::GCLK4 => 4,
            GENW::GCLK5 => 5,
            GENW::GCLK6 => 6,
            GENW::GCLK7 => 7,
            GENW::GCLK8 => 8,
            GENW::GCLK9 => 9,
            GENW::GCLK10 => 10,
            GENW::GCLK11 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GENW<'a> {
    w: &'a mut W,
}
impl<'a> _GENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Generic clock generator 0"]
    #[inline]
    pub fn gclk0(self) -> &'a mut W {
        self.variant(GENW::GCLK0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline]
    pub fn gclk1(self) -> &'a mut W {
        self.variant(GENW::GCLK1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline]
    pub fn gclk2(self) -> &'a mut W {
        self.variant(GENW::GCLK2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline]
    pub fn gclk3(self) -> &'a mut W {
        self.variant(GENW::GCLK3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline]
    pub fn gclk4(self) -> &'a mut W {
        self.variant(GENW::GCLK4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline]
    pub fn gclk5(self) -> &'a mut W {
        self.variant(GENW::GCLK5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline]
    pub fn gclk6(self) -> &'a mut W {
        self.variant(GENW::GCLK6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline]
    pub fn gclk7(self) -> &'a mut W {
        self.variant(GENW::GCLK7)
    }
    #[doc = "Generic clock generator 8"]
    #[inline]
    pub fn gclk8(self) -> &'a mut W {
        self.variant(GENW::GCLK8)
    }
    #[doc = "Generic clock generator 9"]
    #[inline]
    pub fn gclk9(self) -> &'a mut W {
        self.variant(GENW::GCLK9)
    }
    #[doc = "Generic clock generator 10"]
    #[inline]
    pub fn gclk10(self) -> &'a mut W {
        self.variant(GENW::GCLK10)
    }
    #[doc = "Generic clock generator 11"]
    #[inline]
    pub fn gclk11(self) -> &'a mut W {
        self.variant(GENW::GCLK11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHENW<'a> {
    w: &'a mut W,
}
impl<'a> _CHENW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRTLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _WRTLOCKW<'a> {
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bits 0:3 - Generic Clock Generator"]
    #[inline]
    pub fn gen(&self) -> GENR {
        GENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Channel Enable"]
    #[inline]
    pub fn chen(&self) -> CHENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHENR { bits }
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline]
    pub fn wrtlock(&self) -> WRTLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRTLOCKR { bits }
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
    #[doc = "Bits 0:3 - Generic Clock Generator"]
    #[inline]
    pub fn gen(&mut self) -> _GENW {
        _GENW { w: self }
    }
    #[doc = "Bit 6 - Channel Enable"]
    #[inline]
    pub fn chen(&mut self) -> _CHENW {
        _CHENW { w: self }
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline]
    pub fn wrtlock(&mut self) -> _WRTLOCKW {
        _WRTLOCKW { w: self }
    }
}
