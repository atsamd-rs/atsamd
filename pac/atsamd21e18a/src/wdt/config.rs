#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CONFIG {
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
#[doc = "Possible values of the field `PER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERR {
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
impl PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PERR::_8 => 0,
            PERR::_16 => 1,
            PERR::_32 => 2,
            PERR::_64 => 3,
            PERR::_128 => 4,
            PERR::_256 => 5,
            PERR::_512 => 6,
            PERR::_1K => 7,
            PERR::_2K => 8,
            PERR::_4K => 9,
            PERR::_8K => 10,
            PERR::_16K => 11,
            PERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PERR {
        match value {
            0 => PERR::_8,
            1 => PERR::_16,
            2 => PERR::_32,
            3 => PERR::_64,
            4 => PERR::_128,
            5 => PERR::_256,
            6 => PERR::_512,
            7 => PERR::_1K,
            8 => PERR::_2K,
            9 => PERR::_4K,
            10 => PERR::_8K,
            11 => PERR::_16K,
            i => PERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == PERR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == PERR::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == PERR::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == PERR::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == PERR::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == PERR::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == PERR::_512
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline]
    pub fn is_1k(&self) -> bool {
        *self == PERR::_1K
    }
    #[doc = "Checks if the value of the field is `_2K`"]
    #[inline]
    pub fn is_2k(&self) -> bool {
        *self == PERR::_2K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline]
    pub fn is_4k(&self) -> bool {
        *self == PERR::_4K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline]
    pub fn is_8k(&self) -> bool {
        *self == PERR::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline]
    pub fn is_16k(&self) -> bool {
        *self == PERR::_16K
    }
}
#[doc = "Possible values of the field `WINDOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINDOWR {
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
impl WINDOWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WINDOWR::_8 => 0,
            WINDOWR::_16 => 1,
            WINDOWR::_32 => 2,
            WINDOWR::_64 => 3,
            WINDOWR::_128 => 4,
            WINDOWR::_256 => 5,
            WINDOWR::_512 => 6,
            WINDOWR::_1K => 7,
            WINDOWR::_2K => 8,
            WINDOWR::_4K => 9,
            WINDOWR::_8K => 10,
            WINDOWR::_16K => 11,
            WINDOWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WINDOWR {
        match value {
            0 => WINDOWR::_8,
            1 => WINDOWR::_16,
            2 => WINDOWR::_32,
            3 => WINDOWR::_64,
            4 => WINDOWR::_128,
            5 => WINDOWR::_256,
            6 => WINDOWR::_512,
            7 => WINDOWR::_1K,
            8 => WINDOWR::_2K,
            9 => WINDOWR::_4K,
            10 => WINDOWR::_8K,
            11 => WINDOWR::_16K,
            i => WINDOWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == WINDOWR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == WINDOWR::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == WINDOWR::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == WINDOWR::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == WINDOWR::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == WINDOWR::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == WINDOWR::_512
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline]
    pub fn is_1k(&self) -> bool {
        *self == WINDOWR::_1K
    }
    #[doc = "Checks if the value of the field is `_2K`"]
    #[inline]
    pub fn is_2k(&self) -> bool {
        *self == WINDOWR::_2K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline]
    pub fn is_4k(&self) -> bool {
        *self == WINDOWR::_4K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline]
    pub fn is_8k(&self) -> bool {
        *self == WINDOWR::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline]
    pub fn is_16k(&self) -> bool {
        *self == WINDOWR::_16K
    }
}
#[doc = "Values that can be written to the field `PER`"]
pub enum PERW {
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
impl PERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERW::_8 => 0,
            PERW::_16 => 1,
            PERW::_32 => 2,
            PERW::_64 => 3,
            PERW::_128 => 4,
            PERW::_256 => 5,
            PERW::_512 => 6,
            PERW::_1K => 7,
            PERW::_2K => 8,
            PERW::_4K => 9,
            PERW::_8K => 10,
            PERW::_16K => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERW<'a> {
    w: &'a mut W,
}
impl<'a> _PERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 clock cycles"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(PERW::_8)
    }
    #[doc = "16 clock cycles"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(PERW::_16)
    }
    #[doc = "32 clock cycles"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(PERW::_32)
    }
    #[doc = "64 clock cycles"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(PERW::_64)
    }
    #[doc = "128 clock cycles"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(PERW::_128)
    }
    #[doc = "256 clock cycles"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(PERW::_256)
    }
    #[doc = "512 clock cycles"]
    #[inline]
    pub fn _512(self) -> &'a mut W {
        self.variant(PERW::_512)
    }
    #[doc = "1024 clock cycles"]
    #[inline]
    pub fn _1k(self) -> &'a mut W {
        self.variant(PERW::_1K)
    }
    #[doc = "2048 clock cycles"]
    #[inline]
    pub fn _2k(self) -> &'a mut W {
        self.variant(PERW::_2K)
    }
    #[doc = "4096 clock cycles"]
    #[inline]
    pub fn _4k(self) -> &'a mut W {
        self.variant(PERW::_4K)
    }
    #[doc = "8192 clock cycles"]
    #[inline]
    pub fn _8k(self) -> &'a mut W {
        self.variant(PERW::_8K)
    }
    #[doc = "16384 clock cycles"]
    #[inline]
    pub fn _16k(self) -> &'a mut W {
        self.variant(PERW::_16K)
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
#[doc = "Values that can be written to the field `WINDOW`"]
pub enum WINDOWW {
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
impl WINDOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WINDOWW::_8 => 0,
            WINDOWW::_16 => 1,
            WINDOWW::_32 => 2,
            WINDOWW::_64 => 3,
            WINDOWW::_128 => 4,
            WINDOWW::_256 => 5,
            WINDOWW::_512 => 6,
            WINDOWW::_1K => 7,
            WINDOWW::_2K => 8,
            WINDOWW::_4K => 9,
            WINDOWW::_8K => 10,
            WINDOWW::_16K => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WINDOWW<'a> {
    w: &'a mut W,
}
impl<'a> _WINDOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WINDOWW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 clock cycles"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(WINDOWW::_8)
    }
    #[doc = "16 clock cycles"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(WINDOWW::_16)
    }
    #[doc = "32 clock cycles"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(WINDOWW::_32)
    }
    #[doc = "64 clock cycles"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(WINDOWW::_64)
    }
    #[doc = "128 clock cycles"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(WINDOWW::_128)
    }
    #[doc = "256 clock cycles"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(WINDOWW::_256)
    }
    #[doc = "512 clock cycles"]
    #[inline]
    pub fn _512(self) -> &'a mut W {
        self.variant(WINDOWW::_512)
    }
    #[doc = "1024 clock cycles"]
    #[inline]
    pub fn _1k(self) -> &'a mut W {
        self.variant(WINDOWW::_1K)
    }
    #[doc = "2048 clock cycles"]
    #[inline]
    pub fn _2k(self) -> &'a mut W {
        self.variant(WINDOWW::_2K)
    }
    #[doc = "4096 clock cycles"]
    #[inline]
    pub fn _4k(self) -> &'a mut W {
        self.variant(WINDOWW::_4K)
    }
    #[doc = "8192 clock cycles"]
    #[inline]
    pub fn _8k(self) -> &'a mut W {
        self.variant(WINDOWW::_8K)
    }
    #[doc = "16384 clock cycles"]
    #[inline]
    pub fn _16k(self) -> &'a mut W {
        self.variant(WINDOWW::_16K)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:3 - Time-Out Period"]
    #[inline]
    pub fn per(&self) -> PERR {
        PERR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
    #[inline]
    pub fn window(&self) -> WINDOWR {
        WINDOWR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 187 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Time-Out Period"]
    #[inline]
    pub fn per(&mut self) -> _PERW {
        _PERW { w: self }
    }
    #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
    #[inline]
    pub fn window(&mut self) -> _WINDOWW {
        _WINDOWW { w: self }
    }
}
