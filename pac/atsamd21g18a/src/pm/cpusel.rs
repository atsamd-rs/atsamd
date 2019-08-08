#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CPUSEL {
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
#[doc = "Possible values of the field `CPUDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPUDIVR {
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
impl CPUDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CPUDIVR::DIV1 => 0,
            CPUDIVR::DIV2 => 1,
            CPUDIVR::DIV4 => 2,
            CPUDIVR::DIV8 => 3,
            CPUDIVR::DIV16 => 4,
            CPUDIVR::DIV32 => 5,
            CPUDIVR::DIV64 => 6,
            CPUDIVR::DIV128 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CPUDIVR {
        match value {
            0 => CPUDIVR::DIV1,
            1 => CPUDIVR::DIV2,
            2 => CPUDIVR::DIV4,
            3 => CPUDIVR::DIV8,
            4 => CPUDIVR::DIV16,
            5 => CPUDIVR::DIV32,
            6 => CPUDIVR::DIV64,
            7 => CPUDIVR::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == CPUDIVR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == CPUDIVR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == CPUDIVR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == CPUDIVR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == CPUDIVR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == CPUDIVR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == CPUDIVR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == CPUDIVR::DIV128
    }
}
#[doc = "Values that can be written to the field `CPUDIV`"]
pub enum CPUDIVW {
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
impl CPUDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CPUDIVW::DIV1 => 0,
            CPUDIVW::DIV2 => 1,
            CPUDIVW::DIV4 => 2,
            CPUDIVW::DIV8 => 3,
            CPUDIVW::DIV16 => 4,
            CPUDIVW::DIV32 => 5,
            CPUDIVW::DIV64 => 6,
            CPUDIVW::DIV128 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPUDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CPUDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPUDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(CPUDIVW::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(CPUDIVW::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(CPUDIVW::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(CPUDIVW::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(CPUDIVW::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(CPUDIVW::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(CPUDIVW::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(CPUDIVW::DIV128)
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
    #[doc = "Bits 0:2 - CPU Prescaler Selection"]
    #[inline]
    pub fn cpudiv(&self) -> CPUDIVR {
        CPUDIVR::_from({
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
    #[doc = "Bits 0:2 - CPU Prescaler Selection"]
    #[inline]
    pub fn cpudiv(&mut self) -> _CPUDIVW {
        _CPUDIVW { w: self }
    }
}
