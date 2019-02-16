#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PRESCBUF {
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
#[doc = "Possible values of the field `PRESCBUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCBUFR {
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
impl PRESCBUFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCBUFR::DIV1 => 0,
            PRESCBUFR::DIV2 => 1,
            PRESCBUFR::DIV4 => 2,
            PRESCBUFR::DIV8 => 3,
            PRESCBUFR::DIV16 => 4,
            PRESCBUFR::DIV32 => 5,
            PRESCBUFR::DIV64 => 6,
            PRESCBUFR::DIV128 => 7,
            PRESCBUFR::DIV256 => 8,
            PRESCBUFR::DIV512 => 9,
            PRESCBUFR::DIV1024 => 10,
            PRESCBUFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCBUFR {
        match value {
            0 => PRESCBUFR::DIV1,
            1 => PRESCBUFR::DIV2,
            2 => PRESCBUFR::DIV4,
            3 => PRESCBUFR::DIV8,
            4 => PRESCBUFR::DIV16,
            5 => PRESCBUFR::DIV32,
            6 => PRESCBUFR::DIV64,
            7 => PRESCBUFR::DIV128,
            8 => PRESCBUFR::DIV256,
            9 => PRESCBUFR::DIV512,
            10 => PRESCBUFR::DIV1024,
            i => PRESCBUFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PRESCBUFR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PRESCBUFR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PRESCBUFR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PRESCBUFR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PRESCBUFR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PRESCBUFR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PRESCBUFR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PRESCBUFR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == PRESCBUFR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == PRESCBUFR::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCBUFR::DIV1024
    }
}
#[doc = "Values that can be written to the field `PRESCBUF`"]
pub enum PRESCBUFW {
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
impl PRESCBUFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCBUFW::DIV1 => 0,
            PRESCBUFW::DIV2 => 1,
            PRESCBUFW::DIV4 => 2,
            PRESCBUFW::DIV8 => 3,
            PRESCBUFW::DIV16 => 4,
            PRESCBUFW::DIV32 => 5,
            PRESCBUFW::DIV64 => 6,
            PRESCBUFW::DIV128 => 7,
            PRESCBUFW::DIV256 => 8,
            PRESCBUFW::DIV512 => 9,
            PRESCBUFW::DIV1024 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCBUFW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCBUFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCBUFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No division"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCBUFW::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCBUFW::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCBUFW::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCBUFW::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCBUFW::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCBUFW::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCBUFW::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCBUFW::DIV128)
    }
    #[doc = "Divide by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCBUFW::DIV256)
    }
    #[doc = "Divide by 512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESCBUFW::DIV512)
    }
    #[doc = "Divide by 1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCBUFW::DIV1024)
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
    #[doc = "Bits 0:3 - Prescaler Buffer Value"]
    #[inline]
    pub fn prescbuf(&self) -> PRESCBUFR {
        PRESCBUFR::_from({
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
    #[doc = "Bits 0:3 - Prescaler Buffer Value"]
    #[inline]
    pub fn prescbuf(&mut self) -> _PRESCBUFW {
        _PRESCBUFW { w: self }
    }
}
