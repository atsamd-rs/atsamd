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
#[doc = "Possible values of the field `EWOFFSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWOFFSETR {
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
impl EWOFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EWOFFSETR::CYC8 => 0,
            EWOFFSETR::CYC16 => 1,
            EWOFFSETR::CYC32 => 2,
            EWOFFSETR::CYC64 => 3,
            EWOFFSETR::CYC128 => 4,
            EWOFFSETR::CYC256 => 5,
            EWOFFSETR::CYC512 => 6,
            EWOFFSETR::CYC1024 => 7,
            EWOFFSETR::CYC2048 => 8,
            EWOFFSETR::CYC4096 => 9,
            EWOFFSETR::CYC8192 => 10,
            EWOFFSETR::CYC16384 => 11,
            EWOFFSETR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EWOFFSETR {
        match value {
            0 => EWOFFSETR::CYC8,
            1 => EWOFFSETR::CYC16,
            2 => EWOFFSETR::CYC32,
            3 => EWOFFSETR::CYC64,
            4 => EWOFFSETR::CYC128,
            5 => EWOFFSETR::CYC256,
            6 => EWOFFSETR::CYC512,
            7 => EWOFFSETR::CYC1024,
            8 => EWOFFSETR::CYC2048,
            9 => EWOFFSETR::CYC4096,
            10 => EWOFFSETR::CYC8192,
            11 => EWOFFSETR::CYC16384,
            i => EWOFFSETR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CYC8`"]
    #[inline]
    pub fn is_cyc8(&self) -> bool {
        *self == EWOFFSETR::CYC8
    }
    #[doc = "Checks if the value of the field is `CYC16`"]
    #[inline]
    pub fn is_cyc16(&self) -> bool {
        *self == EWOFFSETR::CYC16
    }
    #[doc = "Checks if the value of the field is `CYC32`"]
    #[inline]
    pub fn is_cyc32(&self) -> bool {
        *self == EWOFFSETR::CYC32
    }
    #[doc = "Checks if the value of the field is `CYC64`"]
    #[inline]
    pub fn is_cyc64(&self) -> bool {
        *self == EWOFFSETR::CYC64
    }
    #[doc = "Checks if the value of the field is `CYC128`"]
    #[inline]
    pub fn is_cyc128(&self) -> bool {
        *self == EWOFFSETR::CYC128
    }
    #[doc = "Checks if the value of the field is `CYC256`"]
    #[inline]
    pub fn is_cyc256(&self) -> bool {
        *self == EWOFFSETR::CYC256
    }
    #[doc = "Checks if the value of the field is `CYC512`"]
    #[inline]
    pub fn is_cyc512(&self) -> bool {
        *self == EWOFFSETR::CYC512
    }
    #[doc = "Checks if the value of the field is `CYC1024`"]
    #[inline]
    pub fn is_cyc1024(&self) -> bool {
        *self == EWOFFSETR::CYC1024
    }
    #[doc = "Checks if the value of the field is `CYC2048`"]
    #[inline]
    pub fn is_cyc2048(&self) -> bool {
        *self == EWOFFSETR::CYC2048
    }
    #[doc = "Checks if the value of the field is `CYC4096`"]
    #[inline]
    pub fn is_cyc4096(&self) -> bool {
        *self == EWOFFSETR::CYC4096
    }
    #[doc = "Checks if the value of the field is `CYC8192`"]
    #[inline]
    pub fn is_cyc8192(&self) -> bool {
        *self == EWOFFSETR::CYC8192
    }
    #[doc = "Checks if the value of the field is `CYC16384`"]
    #[inline]
    pub fn is_cyc16384(&self) -> bool {
        *self == EWOFFSETR::CYC16384
    }
}
#[doc = "Values that can be written to the field `EWOFFSET`"]
pub enum EWOFFSETW {
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
impl EWOFFSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EWOFFSETW::CYC8 => 0,
            EWOFFSETW::CYC16 => 1,
            EWOFFSETW::CYC32 => 2,
            EWOFFSETW::CYC64 => 3,
            EWOFFSETW::CYC128 => 4,
            EWOFFSETW::CYC256 => 5,
            EWOFFSETW::CYC512 => 6,
            EWOFFSETW::CYC1024 => 7,
            EWOFFSETW::CYC2048 => 8,
            EWOFFSETW::CYC4096 => 9,
            EWOFFSETW::CYC8192 => 10,
            EWOFFSETW::CYC16384 => 11,
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
    pub fn cyc8(self) -> &'a mut W {
        self.variant(EWOFFSETW::CYC8)
    }
    #[doc = "16 clock cycles"]
    #[inline]
    pub fn cyc16(self) -> &'a mut W {
        self.variant(EWOFFSETW::CYC16)
    }
    #[doc = "32 clock cycles"]
    #[inline]
    pub fn cyc32(self) -> &'a mut W {
        self.variant(EWOFFSETW::CYC32)
    }
    #[doc = "64 clock cycles"]
    #[inline]
    pub fn cyc64(self) -> &'a mut W {
        self.variant(EWOFFSETW::CYC64)
    }
    #[doc = "128 clock cycles"]
    #[inline]
    pub fn cyc128(self) -> &'a mut W {
        self.variant(EWOFFSETW::CYC128)
    }
    #[doc = "256 clock cycles"]
    #[inline]
    pub fn cyc256(self) -> &'a mut W {
        self.variant(EWOFFSETW::CYC256)
    }
    #[doc = "512 clock cycles"]
    #[inline]
    pub fn cyc512(self) -> &'a mut W {
        self.variant(EWOFFSETW::CYC512)
    }
    #[doc = "1024 clock cycles"]
    #[inline]
    pub fn cyc1024(self) -> &'a mut W {
        self.variant(EWOFFSETW::CYC1024)
    }
    #[doc = "2048 clock cycles"]
    #[inline]
    pub fn cyc2048(self) -> &'a mut W {
        self.variant(EWOFFSETW::CYC2048)
    }
    #[doc = "4096 clock cycles"]
    #[inline]
    pub fn cyc4096(self) -> &'a mut W {
        self.variant(EWOFFSETW::CYC4096)
    }
    #[doc = "8192 clock cycles"]
    #[inline]
    pub fn cyc8192(self) -> &'a mut W {
        self.variant(EWOFFSETW::CYC8192)
    }
    #[doc = "16384 clock cycles"]
    #[inline]
    pub fn cyc16384(self) -> &'a mut W {
        self.variant(EWOFFSETW::CYC16384)
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
    #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
    #[inline]
    pub fn ewoffset(&self) -> EWOFFSETR {
        EWOFFSETR::_from({
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
        W { bits: 11 }
    }
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
