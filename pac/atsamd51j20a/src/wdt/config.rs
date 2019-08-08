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
    CYC8,
    #[doc = "16 clock cycles"]
    CYC16,
    #[doc = "32 clock cycles"]
    CYC32,
    #[doc = "64 clock cycles"]
    CYC64,
    #[doc = "128 clock cycles"]
    CYC128,
    #[doc = "256 clock cycles"]
    CYC256,
    #[doc = "512 clock cycles"]
    CYC512,
    #[doc = "1024 clock cycles"]
    CYC1024,
    #[doc = "2048 clock cycles"]
    CYC2048,
    #[doc = "4096 clock cycles"]
    CYC4096,
    #[doc = "8192 clock cycles"]
    CYC8192,
    #[doc = "16384 clock cycles"]
    CYC16384,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PERR::CYC8 => 0,
            PERR::CYC16 => 1,
            PERR::CYC32 => 2,
            PERR::CYC64 => 3,
            PERR::CYC128 => 4,
            PERR::CYC256 => 5,
            PERR::CYC512 => 6,
            PERR::CYC1024 => 7,
            PERR::CYC2048 => 8,
            PERR::CYC4096 => 9,
            PERR::CYC8192 => 10,
            PERR::CYC16384 => 11,
            PERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PERR {
        match value {
            0 => PERR::CYC8,
            1 => PERR::CYC16,
            2 => PERR::CYC32,
            3 => PERR::CYC64,
            4 => PERR::CYC128,
            5 => PERR::CYC256,
            6 => PERR::CYC512,
            7 => PERR::CYC1024,
            8 => PERR::CYC2048,
            9 => PERR::CYC4096,
            10 => PERR::CYC8192,
            11 => PERR::CYC16384,
            i => PERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CYC8`"]
    #[inline]
    pub fn is_cyc8(&self) -> bool {
        *self == PERR::CYC8
    }
    #[doc = "Checks if the value of the field is `CYC16`"]
    #[inline]
    pub fn is_cyc16(&self) -> bool {
        *self == PERR::CYC16
    }
    #[doc = "Checks if the value of the field is `CYC32`"]
    #[inline]
    pub fn is_cyc32(&self) -> bool {
        *self == PERR::CYC32
    }
    #[doc = "Checks if the value of the field is `CYC64`"]
    #[inline]
    pub fn is_cyc64(&self) -> bool {
        *self == PERR::CYC64
    }
    #[doc = "Checks if the value of the field is `CYC128`"]
    #[inline]
    pub fn is_cyc128(&self) -> bool {
        *self == PERR::CYC128
    }
    #[doc = "Checks if the value of the field is `CYC256`"]
    #[inline]
    pub fn is_cyc256(&self) -> bool {
        *self == PERR::CYC256
    }
    #[doc = "Checks if the value of the field is `CYC512`"]
    #[inline]
    pub fn is_cyc512(&self) -> bool {
        *self == PERR::CYC512
    }
    #[doc = "Checks if the value of the field is `CYC1024`"]
    #[inline]
    pub fn is_cyc1024(&self) -> bool {
        *self == PERR::CYC1024
    }
    #[doc = "Checks if the value of the field is `CYC2048`"]
    #[inline]
    pub fn is_cyc2048(&self) -> bool {
        *self == PERR::CYC2048
    }
    #[doc = "Checks if the value of the field is `CYC4096`"]
    #[inline]
    pub fn is_cyc4096(&self) -> bool {
        *self == PERR::CYC4096
    }
    #[doc = "Checks if the value of the field is `CYC8192`"]
    #[inline]
    pub fn is_cyc8192(&self) -> bool {
        *self == PERR::CYC8192
    }
    #[doc = "Checks if the value of the field is `CYC16384`"]
    #[inline]
    pub fn is_cyc16384(&self) -> bool {
        *self == PERR::CYC16384
    }
}
#[doc = "Possible values of the field `WINDOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINDOWR {
    #[doc = "8 clock cycles"]
    CYC8,
    #[doc = "16 clock cycles"]
    CYC16,
    #[doc = "32 clock cycles"]
    CYC32,
    #[doc = "64 clock cycles"]
    CYC64,
    #[doc = "128 clock cycles"]
    CYC128,
    #[doc = "256 clock cycles"]
    CYC256,
    #[doc = "512 clock cycles"]
    CYC512,
    #[doc = "1024 clock cycles"]
    CYC1024,
    #[doc = "2048 clock cycles"]
    CYC2048,
    #[doc = "4096 clock cycles"]
    CYC4096,
    #[doc = "8192 clock cycles"]
    CYC8192,
    #[doc = "16384 clock cycles"]
    CYC16384,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WINDOWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WINDOWR::CYC8 => 0,
            WINDOWR::CYC16 => 1,
            WINDOWR::CYC32 => 2,
            WINDOWR::CYC64 => 3,
            WINDOWR::CYC128 => 4,
            WINDOWR::CYC256 => 5,
            WINDOWR::CYC512 => 6,
            WINDOWR::CYC1024 => 7,
            WINDOWR::CYC2048 => 8,
            WINDOWR::CYC4096 => 9,
            WINDOWR::CYC8192 => 10,
            WINDOWR::CYC16384 => 11,
            WINDOWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WINDOWR {
        match value {
            0 => WINDOWR::CYC8,
            1 => WINDOWR::CYC16,
            2 => WINDOWR::CYC32,
            3 => WINDOWR::CYC64,
            4 => WINDOWR::CYC128,
            5 => WINDOWR::CYC256,
            6 => WINDOWR::CYC512,
            7 => WINDOWR::CYC1024,
            8 => WINDOWR::CYC2048,
            9 => WINDOWR::CYC4096,
            10 => WINDOWR::CYC8192,
            11 => WINDOWR::CYC16384,
            i => WINDOWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CYC8`"]
    #[inline]
    pub fn is_cyc8(&self) -> bool {
        *self == WINDOWR::CYC8
    }
    #[doc = "Checks if the value of the field is `CYC16`"]
    #[inline]
    pub fn is_cyc16(&self) -> bool {
        *self == WINDOWR::CYC16
    }
    #[doc = "Checks if the value of the field is `CYC32`"]
    #[inline]
    pub fn is_cyc32(&self) -> bool {
        *self == WINDOWR::CYC32
    }
    #[doc = "Checks if the value of the field is `CYC64`"]
    #[inline]
    pub fn is_cyc64(&self) -> bool {
        *self == WINDOWR::CYC64
    }
    #[doc = "Checks if the value of the field is `CYC128`"]
    #[inline]
    pub fn is_cyc128(&self) -> bool {
        *self == WINDOWR::CYC128
    }
    #[doc = "Checks if the value of the field is `CYC256`"]
    #[inline]
    pub fn is_cyc256(&self) -> bool {
        *self == WINDOWR::CYC256
    }
    #[doc = "Checks if the value of the field is `CYC512`"]
    #[inline]
    pub fn is_cyc512(&self) -> bool {
        *self == WINDOWR::CYC512
    }
    #[doc = "Checks if the value of the field is `CYC1024`"]
    #[inline]
    pub fn is_cyc1024(&self) -> bool {
        *self == WINDOWR::CYC1024
    }
    #[doc = "Checks if the value of the field is `CYC2048`"]
    #[inline]
    pub fn is_cyc2048(&self) -> bool {
        *self == WINDOWR::CYC2048
    }
    #[doc = "Checks if the value of the field is `CYC4096`"]
    #[inline]
    pub fn is_cyc4096(&self) -> bool {
        *self == WINDOWR::CYC4096
    }
    #[doc = "Checks if the value of the field is `CYC8192`"]
    #[inline]
    pub fn is_cyc8192(&self) -> bool {
        *self == WINDOWR::CYC8192
    }
    #[doc = "Checks if the value of the field is `CYC16384`"]
    #[inline]
    pub fn is_cyc16384(&self) -> bool {
        *self == WINDOWR::CYC16384
    }
}
#[doc = "Values that can be written to the field `PER`"]
pub enum PERW {
    #[doc = "8 clock cycles"]
    CYC8,
    #[doc = "16 clock cycles"]
    CYC16,
    #[doc = "32 clock cycles"]
    CYC32,
    #[doc = "64 clock cycles"]
    CYC64,
    #[doc = "128 clock cycles"]
    CYC128,
    #[doc = "256 clock cycles"]
    CYC256,
    #[doc = "512 clock cycles"]
    CYC512,
    #[doc = "1024 clock cycles"]
    CYC1024,
    #[doc = "2048 clock cycles"]
    CYC2048,
    #[doc = "4096 clock cycles"]
    CYC4096,
    #[doc = "8192 clock cycles"]
    CYC8192,
    #[doc = "16384 clock cycles"]
    CYC16384,
}
impl PERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERW::CYC8 => 0,
            PERW::CYC16 => 1,
            PERW::CYC32 => 2,
            PERW::CYC64 => 3,
            PERW::CYC128 => 4,
            PERW::CYC256 => 5,
            PERW::CYC512 => 6,
            PERW::CYC1024 => 7,
            PERW::CYC2048 => 8,
            PERW::CYC4096 => 9,
            PERW::CYC8192 => 10,
            PERW::CYC16384 => 11,
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
    pub fn cyc8(self) -> &'a mut W {
        self.variant(PERW::CYC8)
    }
    #[doc = "16 clock cycles"]
    #[inline]
    pub fn cyc16(self) -> &'a mut W {
        self.variant(PERW::CYC16)
    }
    #[doc = "32 clock cycles"]
    #[inline]
    pub fn cyc32(self) -> &'a mut W {
        self.variant(PERW::CYC32)
    }
    #[doc = "64 clock cycles"]
    #[inline]
    pub fn cyc64(self) -> &'a mut W {
        self.variant(PERW::CYC64)
    }
    #[doc = "128 clock cycles"]
    #[inline]
    pub fn cyc128(self) -> &'a mut W {
        self.variant(PERW::CYC128)
    }
    #[doc = "256 clock cycles"]
    #[inline]
    pub fn cyc256(self) -> &'a mut W {
        self.variant(PERW::CYC256)
    }
    #[doc = "512 clock cycles"]
    #[inline]
    pub fn cyc512(self) -> &'a mut W {
        self.variant(PERW::CYC512)
    }
    #[doc = "1024 clock cycles"]
    #[inline]
    pub fn cyc1024(self) -> &'a mut W {
        self.variant(PERW::CYC1024)
    }
    #[doc = "2048 clock cycles"]
    #[inline]
    pub fn cyc2048(self) -> &'a mut W {
        self.variant(PERW::CYC2048)
    }
    #[doc = "4096 clock cycles"]
    #[inline]
    pub fn cyc4096(self) -> &'a mut W {
        self.variant(PERW::CYC4096)
    }
    #[doc = "8192 clock cycles"]
    #[inline]
    pub fn cyc8192(self) -> &'a mut W {
        self.variant(PERW::CYC8192)
    }
    #[doc = "16384 clock cycles"]
    #[inline]
    pub fn cyc16384(self) -> &'a mut W {
        self.variant(PERW::CYC16384)
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
    CYC8,
    #[doc = "16 clock cycles"]
    CYC16,
    #[doc = "32 clock cycles"]
    CYC32,
    #[doc = "64 clock cycles"]
    CYC64,
    #[doc = "128 clock cycles"]
    CYC128,
    #[doc = "256 clock cycles"]
    CYC256,
    #[doc = "512 clock cycles"]
    CYC512,
    #[doc = "1024 clock cycles"]
    CYC1024,
    #[doc = "2048 clock cycles"]
    CYC2048,
    #[doc = "4096 clock cycles"]
    CYC4096,
    #[doc = "8192 clock cycles"]
    CYC8192,
    #[doc = "16384 clock cycles"]
    CYC16384,
}
impl WINDOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WINDOWW::CYC8 => 0,
            WINDOWW::CYC16 => 1,
            WINDOWW::CYC32 => 2,
            WINDOWW::CYC64 => 3,
            WINDOWW::CYC128 => 4,
            WINDOWW::CYC256 => 5,
            WINDOWW::CYC512 => 6,
            WINDOWW::CYC1024 => 7,
            WINDOWW::CYC2048 => 8,
            WINDOWW::CYC4096 => 9,
            WINDOWW::CYC8192 => 10,
            WINDOWW::CYC16384 => 11,
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
    pub fn cyc8(self) -> &'a mut W {
        self.variant(WINDOWW::CYC8)
    }
    #[doc = "16 clock cycles"]
    #[inline]
    pub fn cyc16(self) -> &'a mut W {
        self.variant(WINDOWW::CYC16)
    }
    #[doc = "32 clock cycles"]
    #[inline]
    pub fn cyc32(self) -> &'a mut W {
        self.variant(WINDOWW::CYC32)
    }
    #[doc = "64 clock cycles"]
    #[inline]
    pub fn cyc64(self) -> &'a mut W {
        self.variant(WINDOWW::CYC64)
    }
    #[doc = "128 clock cycles"]
    #[inline]
    pub fn cyc128(self) -> &'a mut W {
        self.variant(WINDOWW::CYC128)
    }
    #[doc = "256 clock cycles"]
    #[inline]
    pub fn cyc256(self) -> &'a mut W {
        self.variant(WINDOWW::CYC256)
    }
    #[doc = "512 clock cycles"]
    #[inline]
    pub fn cyc512(self) -> &'a mut W {
        self.variant(WINDOWW::CYC512)
    }
    #[doc = "1024 clock cycles"]
    #[inline]
    pub fn cyc1024(self) -> &'a mut W {
        self.variant(WINDOWW::CYC1024)
    }
    #[doc = "2048 clock cycles"]
    #[inline]
    pub fn cyc2048(self) -> &'a mut W {
        self.variant(WINDOWW::CYC2048)
    }
    #[doc = "4096 clock cycles"]
    #[inline]
    pub fn cyc4096(self) -> &'a mut W {
        self.variant(WINDOWW::CYC4096)
    }
    #[doc = "8192 clock cycles"]
    #[inline]
    pub fn cyc8192(self) -> &'a mut W {
        self.variant(WINDOWW::CYC8192)
    }
    #[doc = "16384 clock cycles"]
    #[inline]
    pub fn cyc16384(self) -> &'a mut W {
        self.variant(WINDOWW::CYC16384)
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
