#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::APBBSEL {
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
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `APBBDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APBBDIVR {
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
impl APBBDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            APBBDIVR::DIV1 => 0,
            APBBDIVR::DIV2 => 0x01,
            APBBDIVR::DIV4 => 0x02,
            APBBDIVR::DIV8 => 0x03,
            APBBDIVR::DIV16 => 0x04,
            APBBDIVR::DIV32 => 0x05,
            APBBDIVR::DIV64 => 0x06,
            APBBDIVR::DIV128 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> APBBDIVR {
        match value {
            0 => APBBDIVR::DIV1,
            1 => APBBDIVR::DIV2,
            2 => APBBDIVR::DIV4,
            3 => APBBDIVR::DIV8,
            4 => APBBDIVR::DIV16,
            5 => APBBDIVR::DIV32,
            6 => APBBDIVR::DIV64,
            7 => APBBDIVR::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == APBBDIVR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == APBBDIVR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == APBBDIVR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == APBBDIVR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == APBBDIVR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == APBBDIVR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == APBBDIVR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == APBBDIVR::DIV128
    }
}
#[doc = "Values that can be written to the field `APBBDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APBBDIVW {
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
impl APBBDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            APBBDIVW::DIV1 => 0,
            APBBDIVW::DIV2 => 1,
            APBBDIVW::DIV4 => 2,
            APBBDIVW::DIV8 => 3,
            APBBDIVW::DIV16 => 4,
            APBBDIVW::DIV32 => 5,
            APBBDIVW::DIV64 => 6,
            APBBDIVW::DIV128 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APBBDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _APBBDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APBBDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(APBBDIVW::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(APBBDIVW::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(APBBDIVW::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(APBBDIVW::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(APBBDIVW::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(APBBDIVW::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(APBBDIVW::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(APBBDIVW::DIV128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 0);
        self.w.bits |= ((value as u8) & 0x07) << 0;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:2 - APBB Prescaler Selection"]
    #[inline]
    pub fn apbbdiv(&self) -> APBBDIVR {
        APBBDIVR::_from(((self.bits >> 0) & 0x07) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - APBB Prescaler Selection"]
    #[inline]
    pub fn apbbdiv(&mut self) -> _APBBDIVW {
        _APBBDIVW { w: self }
    }
}
