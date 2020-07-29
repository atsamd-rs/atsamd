#[doc = "Reader of register AVGCTRL"]
pub type R = crate::R<u8, super::AVGCTRL>;
#[doc = "Writer for register AVGCTRL"]
pub type W = crate::W<u8, super::AVGCTRL>;
#[doc = "Register AVGCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::AVGCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Number of Samples to be Collected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAMPLENUM_A {
    #[doc = "0: 1 sample"]
    _1 = 0,
    #[doc = "1: 2 samples"]
    _2 = 1,
    #[doc = "2: 4 samples"]
    _4 = 2,
    #[doc = "3: 8 samples"]
    _8 = 3,
    #[doc = "4: 16 samples"]
    _16 = 4,
    #[doc = "5: 32 samples"]
    _32 = 5,
    #[doc = "6: 64 samples"]
    _64 = 6,
    #[doc = "7: 128 samples"]
    _128 = 7,
    #[doc = "8: 256 samples"]
    _256 = 8,
    #[doc = "9: 512 samples"]
    _512 = 9,
    #[doc = "10: 1024 samples"]
    _1024 = 10,
}
impl From<SAMPLENUM_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPLENUM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAMPLENUM`"]
pub type SAMPLENUM_R = crate::R<u8, SAMPLENUM_A>;
impl SAMPLENUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAMPLENUM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAMPLENUM_A::_1),
            1 => Val(SAMPLENUM_A::_2),
            2 => Val(SAMPLENUM_A::_4),
            3 => Val(SAMPLENUM_A::_8),
            4 => Val(SAMPLENUM_A::_16),
            5 => Val(SAMPLENUM_A::_32),
            6 => Val(SAMPLENUM_A::_64),
            7 => Val(SAMPLENUM_A::_128),
            8 => Val(SAMPLENUM_A::_256),
            9 => Val(SAMPLENUM_A::_512),
            10 => Val(SAMPLENUM_A::_1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAMPLENUM_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == SAMPLENUM_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == SAMPLENUM_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SAMPLENUM_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == SAMPLENUM_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == SAMPLENUM_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == SAMPLENUM_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == SAMPLENUM_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SAMPLENUM_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == SAMPLENUM_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == SAMPLENUM_A::_1024
    }
}
#[doc = "Write proxy for field `SAMPLENUM`"]
pub struct SAMPLENUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLENUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLENUM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 sample"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAMPLENUM_A::_1)
    }
    #[doc = "2 samples"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SAMPLENUM_A::_2)
    }
    #[doc = "4 samples"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(SAMPLENUM_A::_4)
    }
    #[doc = "8 samples"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SAMPLENUM_A::_8)
    }
    #[doc = "16 samples"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(SAMPLENUM_A::_16)
    }
    #[doc = "32 samples"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(SAMPLENUM_A::_32)
    }
    #[doc = "64 samples"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(SAMPLENUM_A::_64)
    }
    #[doc = "128 samples"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(SAMPLENUM_A::_128)
    }
    #[doc = "256 samples"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(SAMPLENUM_A::_256)
    }
    #[doc = "512 samples"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(SAMPLENUM_A::_512)
    }
    #[doc = "1024 samples"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(SAMPLENUM_A::_1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ADJRES`"]
pub type ADJRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADJRES`"]
pub struct ADJRES_W<'a> {
    w: &'a mut W,
}
impl<'a> ADJRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u8) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of Samples to be Collected"]
    #[inline(always)]
    pub fn samplenum(&self) -> SAMPLENUM_R {
        SAMPLENUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Adjusting Result / Division Coefficient"]
    #[inline(always)]
    pub fn adjres(&self) -> ADJRES_R {
        ADJRES_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of Samples to be Collected"]
    #[inline(always)]
    pub fn samplenum(&mut self) -> SAMPLENUM_W {
        SAMPLENUM_W { w: self }
    }
    #[doc = "Bits 4:6 - Adjusting Result / Division Coefficient"]
    #[inline(always)]
    pub fn adjres(&mut self) -> ADJRES_W {
        ADJRES_W { w: self }
    }
}
