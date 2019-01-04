#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PRESC {
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
#[doc = "Possible values of the field `PRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCR {
    #[doc = "No division"]
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
    #[doc = "Divide by 256"]
    DIV256,
    #[doc = "Divide by 512"]
    DIV512,
    #[doc = "Divide by 1024"]
    DIV1024,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCR::DIV1 => 0,
            PRESCR::DIV2 => 1,
            PRESCR::DIV4 => 2,
            PRESCR::DIV8 => 3,
            PRESCR::DIV16 => 4,
            PRESCR::DIV32 => 5,
            PRESCR::DIV64 => 6,
            PRESCR::DIV128 => 7,
            PRESCR::DIV256 => 8,
            PRESCR::DIV512 => 9,
            PRESCR::DIV1024 => 10,
            PRESCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCR {
        match value {
            0 => PRESCR::DIV1,
            1 => PRESCR::DIV2,
            2 => PRESCR::DIV4,
            3 => PRESCR::DIV8,
            4 => PRESCR::DIV16,
            5 => PRESCR::DIV32,
            6 => PRESCR::DIV64,
            7 => PRESCR::DIV128,
            8 => PRESCR::DIV256,
            9 => PRESCR::DIV512,
            10 => PRESCR::DIV1024,
            i => PRESCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PRESCR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PRESCR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PRESCR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PRESCR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PRESCR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == PRESCR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == PRESCR::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCR::DIV1024
    }
}
#[doc = "Values that can be written to the field `PRESC`"]
pub enum PRESCW {
    #[doc = "No division"]
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
    #[doc = "Divide by 256"]
    DIV256,
    #[doc = "Divide by 512"]
    DIV512,
    #[doc = "Divide by 1024"]
    DIV1024,
}
impl PRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCW::DIV1 => 0,
            PRESCW::DIV2 => 1,
            PRESCW::DIV4 => 2,
            PRESCW::DIV8 => 3,
            PRESCW::DIV16 => 4,
            PRESCW::DIV32 => 5,
            PRESCW::DIV64 => 6,
            PRESCW::DIV128 => 7,
            PRESCW::DIV256 => 8,
            PRESCW::DIV512 => 9,
            PRESCW::DIV1024 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No division"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCW::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCW::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCW::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCW::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCW::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCW::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCW::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCW::DIV128)
    }
    #[doc = "Divide by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCW::DIV256)
    }
    #[doc = "Divide by 512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESCW::DIV512)
    }
    #[doc = "Divide by 1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCW::DIV1024)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - Prescaler Value"]
    #[inline]
    pub fn presc(&self) -> PRESCR {
        PRESCR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Prescaler Value"]
    #[inline]
    pub fn presc(&mut self) -> _PRESCW {
        _PRESCW { w: self }
    }
}
