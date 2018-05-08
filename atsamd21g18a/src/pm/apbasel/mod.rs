#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::APBASEL {
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
#[doc = "Possible values of the field `APBADIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APBADIVR {
    #[doc = "Divide by 1"]
    DIV1,
    #[doc = "Divide by 2"]
    DIV2,
    #[doc = "Divide by 4"]
    DIV4,
    #[doc = "Divide by 8"]
    DIV8,
    #[doc = "Divide by 16"]
    DIV16,
    #[doc = "Divide by 32"]
    DIV32,
    #[doc = "Divide by 64"]
    DIV64,
    #[doc = "Divide by 128"]
    DIV128,
}
impl APBADIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            APBADIVR::DIV1 => 0,
            APBADIVR::DIV2 => 1,
            APBADIVR::DIV4 => 2,
            APBADIVR::DIV8 => 3,
            APBADIVR::DIV16 => 4,
            APBADIVR::DIV32 => 5,
            APBADIVR::DIV64 => 6,
            APBADIVR::DIV128 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> APBADIVR {
        match value {
            0 => APBADIVR::DIV1,
            1 => APBADIVR::DIV2,
            2 => APBADIVR::DIV4,
            3 => APBADIVR::DIV8,
            4 => APBADIVR::DIV16,
            5 => APBADIVR::DIV32,
            6 => APBADIVR::DIV64,
            7 => APBADIVR::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == APBADIVR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == APBADIVR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == APBADIVR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == APBADIVR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == APBADIVR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == APBADIVR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == APBADIVR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == APBADIVR::DIV128
    }
}
#[doc = "Values that can be written to the field `APBADIV`"]
pub enum APBADIVW {
    #[doc = "Divide by 1"]
    DIV1,
    #[doc = "Divide by 2"]
    DIV2,
    #[doc = "Divide by 4"]
    DIV4,
    #[doc = "Divide by 8"]
    DIV8,
    #[doc = "Divide by 16"]
    DIV16,
    #[doc = "Divide by 32"]
    DIV32,
    #[doc = "Divide by 64"]
    DIV64,
    #[doc = "Divide by 128"]
    DIV128,
}
impl APBADIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            APBADIVW::DIV1 => 0,
            APBADIVW::DIV2 => 1,
            APBADIVW::DIV4 => 2,
            APBADIVW::DIV8 => 3,
            APBADIVW::DIV16 => 4,
            APBADIVW::DIV32 => 5,
            APBADIVW::DIV64 => 6,
            APBADIVW::DIV128 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APBADIVW<'a> {
    w: &'a mut W,
}
impl<'a> _APBADIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APBADIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(APBADIVW::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(APBADIVW::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(APBADIVW::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(APBADIVW::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(APBADIVW::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(APBADIVW::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(APBADIVW::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(APBADIVW::DIV128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:2 - APBA Prescaler Selection"]
    #[inline]
    pub fn apbadiv(&self) -> APBADIVR {
        APBADIVR::_from({
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
    #[doc = "Bits 0:2 - APBA Prescaler Selection"]
    #[inline]
    pub fn apbadiv(&mut self) -> _APBADIVW {
        _APBADIVW { w: self }
    }
}
