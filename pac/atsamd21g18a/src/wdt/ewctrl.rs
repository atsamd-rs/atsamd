#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::EWCTRL {
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
        0x0b
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `EWOFFSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWOFFSETR {
    #[doc = "8 clock cycles"]
    _8,
    #[doc = "16 clock cycles"]
    _16,
    #[doc = "32 clock cycles"]
    _32,
    #[doc = "64 clock cycles"]
    _64,
    #[doc = "128 clock cycles"]
    _128,
    #[doc = "256 clock cycles"]
    _256,
    #[doc = "512 clock cycles"]
    _512,
    #[doc = "1024 clock cycles"]
    _1K,
    #[doc = "2048 clock cycles"]
    _2K,
    #[doc = "4096 clock cycles"]
    _4K,
    #[doc = "8192 clock cycles"]
    _8K,
    #[doc = "16384 clock cycles"]
    _16K,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EWOFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EWOFFSETR::_8 => 0,
            EWOFFSETR::_16 => 0x01,
            EWOFFSETR::_32 => 0x02,
            EWOFFSETR::_64 => 0x03,
            EWOFFSETR::_128 => 0x04,
            EWOFFSETR::_256 => 0x05,
            EWOFFSETR::_512 => 0x06,
            EWOFFSETR::_1K => 0x07,
            EWOFFSETR::_2K => 0x08,
            EWOFFSETR::_4K => 0x09,
            EWOFFSETR::_8K => 0x0a,
            EWOFFSETR::_16K => 0x0b,
            EWOFFSETR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EWOFFSETR {
        match value {
            0 => EWOFFSETR::_8,
            1 => EWOFFSETR::_16,
            2 => EWOFFSETR::_32,
            3 => EWOFFSETR::_64,
            4 => EWOFFSETR::_128,
            5 => EWOFFSETR::_256,
            6 => EWOFFSETR::_512,
            7 => EWOFFSETR::_1K,
            8 => EWOFFSETR::_2K,
            9 => EWOFFSETR::_4K,
            10 => EWOFFSETR::_8K,
            11 => EWOFFSETR::_16K,
            i => EWOFFSETR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == EWOFFSETR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == EWOFFSETR::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == EWOFFSETR::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == EWOFFSETR::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == EWOFFSETR::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == EWOFFSETR::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == EWOFFSETR::_512
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline]
    pub fn is_1k(&self) -> bool {
        *self == EWOFFSETR::_1K
    }
    #[doc = "Checks if the value of the field is `_2K`"]
    #[inline]
    pub fn is_2k(&self) -> bool {
        *self == EWOFFSETR::_2K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline]
    pub fn is_4k(&self) -> bool {
        *self == EWOFFSETR::_4K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline]
    pub fn is_8k(&self) -> bool {
        *self == EWOFFSETR::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline]
    pub fn is_16k(&self) -> bool {
        *self == EWOFFSETR::_16K
    }
}
#[doc = "Values that can be written to the field `EWOFFSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWOFFSETW {
    #[doc = "8 clock cycles"]
    _8,
    #[doc = "16 clock cycles"]
    _16,
    #[doc = "32 clock cycles"]
    _32,
    #[doc = "64 clock cycles"]
    _64,
    #[doc = "128 clock cycles"]
    _128,
    #[doc = "256 clock cycles"]
    _256,
    #[doc = "512 clock cycles"]
    _512,
    #[doc = "1024 clock cycles"]
    _1K,
    #[doc = "2048 clock cycles"]
    _2K,
    #[doc = "4096 clock cycles"]
    _4K,
    #[doc = "8192 clock cycles"]
    _8K,
    #[doc = "16384 clock cycles"]
    _16K,
}
impl EWOFFSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EWOFFSETW::_8 => 0,
            EWOFFSETW::_16 => 1,
            EWOFFSETW::_32 => 2,
            EWOFFSETW::_64 => 3,
            EWOFFSETW::_128 => 4,
            EWOFFSETW::_256 => 5,
            EWOFFSETW::_512 => 6,
            EWOFFSETW::_1K => 7,
            EWOFFSETW::_2K => 8,
            EWOFFSETW::_4K => 9,
            EWOFFSETW::_8K => 10,
            EWOFFSETW::_16K => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EWOFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _EWOFFSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EWOFFSETW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 clock cycles"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(EWOFFSETW::_8)
    }
    #[doc = "16 clock cycles"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(EWOFFSETW::_16)
    }
    #[doc = "32 clock cycles"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(EWOFFSETW::_32)
    }
    #[doc = "64 clock cycles"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(EWOFFSETW::_64)
    }
    #[doc = "128 clock cycles"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(EWOFFSETW::_128)
    }
    #[doc = "256 clock cycles"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(EWOFFSETW::_256)
    }
    #[doc = "512 clock cycles"]
    #[inline]
    pub fn _512(self) -> &'a mut W {
        self.variant(EWOFFSETW::_512)
    }
    #[doc = "1024 clock cycles"]
    #[inline]
    pub fn _1k(self) -> &'a mut W {
        self.variant(EWOFFSETW::_1K)
    }
    #[doc = "2048 clock cycles"]
    #[inline]
    pub fn _2k(self) -> &'a mut W {
        self.variant(EWOFFSETW::_2K)
    }
    #[doc = "4096 clock cycles"]
    #[inline]
    pub fn _4k(self) -> &'a mut W {
        self.variant(EWOFFSETW::_4K)
    }
    #[doc = "8192 clock cycles"]
    #[inline]
    pub fn _8k(self) -> &'a mut W {
        self.variant(EWOFFSETW::_8K)
    }
    #[doc = "16384 clock cycles"]
    #[inline]
    pub fn _16k(self) -> &'a mut W {
        self.variant(EWOFFSETW::_16K)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 0);
        self.w.bits |= ((value as u8) & 0x0f) << 0;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
    #[inline]
    pub fn ewoffset(&self) -> EWOFFSETR {
        EWOFFSETR::_from(((self.bits >> 0) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
    #[inline]
    pub fn ewoffset(&mut self) -> _EWOFFSETW {
        _EWOFFSETW { w: self }
    }
}
