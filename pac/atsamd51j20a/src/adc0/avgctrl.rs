#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::AVGCTRL {
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
#[doc = "Possible values of the field `SAMPLENUM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLENUMR {
    #[doc = "1 sample"]
    _1,
    #[doc = "2 samples"]
    _2,
    #[doc = "4 samples"]
    _4,
    #[doc = "8 samples"]
    _8,
    #[doc = "16 samples"]
    _16,
    #[doc = "32 samples"]
    _32,
    #[doc = "64 samples"]
    _64,
    #[doc = "128 samples"]
    _128,
    #[doc = "256 samples"]
    _256,
    #[doc = "512 samples"]
    _512,
    #[doc = "1024 samples"]
    _1024,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SAMPLENUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAMPLENUMR::_1 => 0,
            SAMPLENUMR::_2 => 0x01,
            SAMPLENUMR::_4 => 0x02,
            SAMPLENUMR::_8 => 0x03,
            SAMPLENUMR::_16 => 0x04,
            SAMPLENUMR::_32 => 0x05,
            SAMPLENUMR::_64 => 0x06,
            SAMPLENUMR::_128 => 0x07,
            SAMPLENUMR::_256 => 0x08,
            SAMPLENUMR::_512 => 0x09,
            SAMPLENUMR::_1024 => 0x0a,
            SAMPLENUMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAMPLENUMR {
        match value {
            0 => SAMPLENUMR::_1,
            1 => SAMPLENUMR::_2,
            2 => SAMPLENUMR::_4,
            3 => SAMPLENUMR::_8,
            4 => SAMPLENUMR::_16,
            5 => SAMPLENUMR::_32,
            6 => SAMPLENUMR::_64,
            7 => SAMPLENUMR::_128,
            8 => SAMPLENUMR::_256,
            9 => SAMPLENUMR::_512,
            10 => SAMPLENUMR::_1024,
            i => SAMPLENUMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SAMPLENUMR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == SAMPLENUMR::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == SAMPLENUMR::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == SAMPLENUMR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == SAMPLENUMR::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == SAMPLENUMR::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == SAMPLENUMR::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == SAMPLENUMR::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == SAMPLENUMR::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == SAMPLENUMR::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == SAMPLENUMR::_1024
    }
}
#[doc = r" Value of the field"]
pub struct ADJRESR {
    bits: u8,
}
impl ADJRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SAMPLENUM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLENUMW {
    #[doc = "1 sample"]
    _1,
    #[doc = "2 samples"]
    _2,
    #[doc = "4 samples"]
    _4,
    #[doc = "8 samples"]
    _8,
    #[doc = "16 samples"]
    _16,
    #[doc = "32 samples"]
    _32,
    #[doc = "64 samples"]
    _64,
    #[doc = "128 samples"]
    _128,
    #[doc = "256 samples"]
    _256,
    #[doc = "512 samples"]
    _512,
    #[doc = "1024 samples"]
    _1024,
}
impl SAMPLENUMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAMPLENUMW::_1 => 0,
            SAMPLENUMW::_2 => 1,
            SAMPLENUMW::_4 => 2,
            SAMPLENUMW::_8 => 3,
            SAMPLENUMW::_16 => 4,
            SAMPLENUMW::_32 => 5,
            SAMPLENUMW::_64 => 6,
            SAMPLENUMW::_128 => 7,
            SAMPLENUMW::_256 => 8,
            SAMPLENUMW::_512 => 9,
            SAMPLENUMW::_1024 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMPLENUMW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLENUMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMPLENUMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 sample"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAMPLENUMW::_1)
    }
    #[doc = "2 samples"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(SAMPLENUMW::_2)
    }
    #[doc = "4 samples"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(SAMPLENUMW::_4)
    }
    #[doc = "8 samples"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(SAMPLENUMW::_8)
    }
    #[doc = "16 samples"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(SAMPLENUMW::_16)
    }
    #[doc = "32 samples"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(SAMPLENUMW::_32)
    }
    #[doc = "64 samples"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(SAMPLENUMW::_64)
    }
    #[doc = "128 samples"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(SAMPLENUMW::_128)
    }
    #[doc = "256 samples"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(SAMPLENUMW::_256)
    }
    #[doc = "512 samples"]
    #[inline]
    pub fn _512(self) -> &'a mut W {
        self.variant(SAMPLENUMW::_512)
    }
    #[doc = "1024 samples"]
    #[inline]
    pub fn _1024(self) -> &'a mut W {
        self.variant(SAMPLENUMW::_1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 0);
        self.w.bits |= ((value as u8) & 0x0f) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADJRESW<'a> {
    w: &'a mut W,
}
impl<'a> _ADJRESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 4);
        self.w.bits |= ((value as u8) & 0x07) << 4;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - Number of Samples to be Collected"]
    #[inline]
    pub fn samplenum(&self) -> SAMPLENUMR {
        SAMPLENUMR::_from(((self.bits >> 0) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Adjusting Result / Division Coefficient"]
    #[inline]
    pub fn adjres(&self) -> ADJRESR {
        let bits = ((self.bits >> 4) & 0x07) as u8;
        ADJRESR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Number of Samples to be Collected"]
    #[inline]
    pub fn samplenum(&mut self) -> _SAMPLENUMW {
        _SAMPLENUMW { w: self }
    }
    #[doc = "Bits 4:6 - Adjusting Result / Division Coefficient"]
    #[inline]
    pub fn adjres(&mut self) -> _ADJRESW {
        _ADJRESW { w: self }
    }
}
