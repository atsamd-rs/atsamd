#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::REFCTRL {
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
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "1.0V voltage reference"]
    INT1V,
    #[doc = "1/1.48 VDDANA"]
    INTVCC0,
    #[doc = "1/2 VDDANA (only for VDDANA > 2.0V)"]
    INTVCC1,
    #[doc = "External reference"]
    AREFA,
    #[doc = "External reference"]
    AREFB,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::INT1V => 0,
            REFSELR::INTVCC0 => 1,
            REFSELR::INTVCC1 => 2,
            REFSELR::AREFA => 3,
            REFSELR::AREFB => 4,
            REFSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::INT1V,
            1 => REFSELR::INTVCC0,
            2 => REFSELR::INTVCC1,
            3 => REFSELR::AREFA,
            4 => REFSELR::AREFB,
            i => REFSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INT1V`"]
    #[inline]
    pub fn is_int1v(&self) -> bool {
        *self == REFSELR::INT1V
    }
    #[doc = "Checks if the value of the field is `INTVCC0`"]
    #[inline]
    pub fn is_intvcc0(&self) -> bool {
        *self == REFSELR::INTVCC0
    }
    #[doc = "Checks if the value of the field is `INTVCC1`"]
    #[inline]
    pub fn is_intvcc1(&self) -> bool {
        *self == REFSELR::INTVCC1
    }
    #[doc = "Checks if the value of the field is `AREFA`"]
    #[inline]
    pub fn is_arefa(&self) -> bool {
        *self == REFSELR::AREFA
    }
    #[doc = "Checks if the value of the field is `AREFB`"]
    #[inline]
    pub fn is_arefb(&self) -> bool {
        *self == REFSELR::AREFB
    }
}
#[doc = r" Value of the field"]
pub struct REFCOMPR {
    bits: bool,
}
impl REFCOMPR {
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
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "1.0V voltage reference"]
    INT1V,
    #[doc = "1/1.48 VDDANA"]
    INTVCC0,
    #[doc = "1/2 VDDANA (only for VDDANA > 2.0V)"]
    INTVCC1,
    #[doc = "External reference"]
    AREFA,
    #[doc = "External reference"]
    AREFB,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::INT1V => 0,
            REFSELW::INTVCC0 => 1,
            REFSELW::INTVCC1 => 2,
            REFSELW::AREFA => 3,
            REFSELW::AREFB => 4,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1.0V voltage reference"]
    #[inline]
    pub fn int1v(self) -> &'a mut W {
        self.variant(REFSELW::INT1V)
    }
    #[doc = "1/1.48 VDDANA"]
    #[inline]
    pub fn intvcc0(self) -> &'a mut W {
        self.variant(REFSELW::INTVCC0)
    }
    #[doc = "1/2 VDDANA (only for VDDANA > 2.0V)"]
    #[inline]
    pub fn intvcc1(self) -> &'a mut W {
        self.variant(REFSELW::INTVCC1)
    }
    #[doc = "External reference"]
    #[inline]
    pub fn arefa(self) -> &'a mut W {
        self.variant(REFSELW::AREFA)
    }
    #[doc = "External reference"]
    #[inline]
    pub fn arefb(self) -> &'a mut W {
        self.variant(REFSELW::AREFB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REFCOMPW<'a> {
    w: &'a mut W,
}
impl<'a> _REFCOMPW<'a> {
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
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline]
    pub fn refcomp(&self) -> REFCOMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        REFCOMPR { bits }
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
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline]
    pub fn refcomp(&mut self) -> _REFCOMPW {
        _REFCOMPW { w: self }
    }
}
