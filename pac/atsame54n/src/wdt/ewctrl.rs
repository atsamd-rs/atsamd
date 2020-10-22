#[doc = "Reader of register EWCTRL"]
pub type R = crate::R<u8, super::EWCTRL>;
#[doc = "Writer for register EWCTRL"]
pub type W = crate::W<u8, super::EWCTRL>;
#[doc = "Register EWCTRL `reset()`'s with value 0x0b"]
impl crate::ResetValue for super::EWCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0b
    }
}
#[doc = "Early Warning Interrupt Time Offset\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EWOFFSET_A {
    #[doc = "0: 8 clock cycles"]
    CYC8 = 0,
    #[doc = "1: 16 clock cycles"]
    CYC16 = 1,
    #[doc = "2: 32 clock cycles"]
    CYC32 = 2,
    #[doc = "3: 64 clock cycles"]
    CYC64 = 3,
    #[doc = "4: 128 clock cycles"]
    CYC128 = 4,
    #[doc = "5: 256 clock cycles"]
    CYC256 = 5,
    #[doc = "6: 512 clock cycles"]
    CYC512 = 6,
    #[doc = "7: 1024 clock cycles"]
    CYC1024 = 7,
    #[doc = "8: 2048 clock cycles"]
    CYC2048 = 8,
    #[doc = "9: 4096 clock cycles"]
    CYC4096 = 9,
    #[doc = "10: 8192 clock cycles"]
    CYC8192 = 10,
    #[doc = "11: 16384 clock cycles"]
    CYC16384 = 11,
}
impl From<EWOFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: EWOFFSET_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EWOFFSET`"]
pub type EWOFFSET_R = crate::R<u8, EWOFFSET_A>;
impl EWOFFSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EWOFFSET_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EWOFFSET_A::CYC8),
            1 => Val(EWOFFSET_A::CYC16),
            2 => Val(EWOFFSET_A::CYC32),
            3 => Val(EWOFFSET_A::CYC64),
            4 => Val(EWOFFSET_A::CYC128),
            5 => Val(EWOFFSET_A::CYC256),
            6 => Val(EWOFFSET_A::CYC512),
            7 => Val(EWOFFSET_A::CYC1024),
            8 => Val(EWOFFSET_A::CYC2048),
            9 => Val(EWOFFSET_A::CYC4096),
            10 => Val(EWOFFSET_A::CYC8192),
            11 => Val(EWOFFSET_A::CYC16384),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CYC8`"]
    #[inline(always)]
    pub fn is_cyc8(&self) -> bool {
        *self == EWOFFSET_A::CYC8
    }
    #[doc = "Checks if the value of the field is `CYC16`"]
    #[inline(always)]
    pub fn is_cyc16(&self) -> bool {
        *self == EWOFFSET_A::CYC16
    }
    #[doc = "Checks if the value of the field is `CYC32`"]
    #[inline(always)]
    pub fn is_cyc32(&self) -> bool {
        *self == EWOFFSET_A::CYC32
    }
    #[doc = "Checks if the value of the field is `CYC64`"]
    #[inline(always)]
    pub fn is_cyc64(&self) -> bool {
        *self == EWOFFSET_A::CYC64
    }
    #[doc = "Checks if the value of the field is `CYC128`"]
    #[inline(always)]
    pub fn is_cyc128(&self) -> bool {
        *self == EWOFFSET_A::CYC128
    }
    #[doc = "Checks if the value of the field is `CYC256`"]
    #[inline(always)]
    pub fn is_cyc256(&self) -> bool {
        *self == EWOFFSET_A::CYC256
    }
    #[doc = "Checks if the value of the field is `CYC512`"]
    #[inline(always)]
    pub fn is_cyc512(&self) -> bool {
        *self == EWOFFSET_A::CYC512
    }
    #[doc = "Checks if the value of the field is `CYC1024`"]
    #[inline(always)]
    pub fn is_cyc1024(&self) -> bool {
        *self == EWOFFSET_A::CYC1024
    }
    #[doc = "Checks if the value of the field is `CYC2048`"]
    #[inline(always)]
    pub fn is_cyc2048(&self) -> bool {
        *self == EWOFFSET_A::CYC2048
    }
    #[doc = "Checks if the value of the field is `CYC4096`"]
    #[inline(always)]
    pub fn is_cyc4096(&self) -> bool {
        *self == EWOFFSET_A::CYC4096
    }
    #[doc = "Checks if the value of the field is `CYC8192`"]
    #[inline(always)]
    pub fn is_cyc8192(&self) -> bool {
        *self == EWOFFSET_A::CYC8192
    }
    #[doc = "Checks if the value of the field is `CYC16384`"]
    #[inline(always)]
    pub fn is_cyc16384(&self) -> bool {
        *self == EWOFFSET_A::CYC16384
    }
}
#[doc = "Write proxy for field `EWOFFSET`"]
pub struct EWOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> EWOFFSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWOFFSET_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn cyc8(self) -> &'a mut W {
        self.variant(EWOFFSET_A::CYC8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn cyc16(self) -> &'a mut W {
        self.variant(EWOFFSET_A::CYC16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn cyc32(self) -> &'a mut W {
        self.variant(EWOFFSET_A::CYC32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn cyc64(self) -> &'a mut W {
        self.variant(EWOFFSET_A::CYC64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn cyc128(self) -> &'a mut W {
        self.variant(EWOFFSET_A::CYC128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn cyc256(self) -> &'a mut W {
        self.variant(EWOFFSET_A::CYC256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn cyc512(self) -> &'a mut W {
        self.variant(EWOFFSET_A::CYC512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn cyc1024(self) -> &'a mut W {
        self.variant(EWOFFSET_A::CYC1024)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn cyc2048(self) -> &'a mut W {
        self.variant(EWOFFSET_A::CYC2048)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn cyc4096(self) -> &'a mut W {
        self.variant(EWOFFSET_A::CYC4096)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn cyc8192(self) -> &'a mut W {
        self.variant(EWOFFSET_A::CYC8192)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn cyc16384(self) -> &'a mut W {
        self.variant(EWOFFSET_A::CYC16384)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
    #[inline(always)]
    pub fn ewoffset(&self) -> EWOFFSET_R {
        EWOFFSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
    #[inline(always)]
    pub fn ewoffset(&mut self) -> EWOFFSET_W {
        EWOFFSET_W { w: self }
    }
}
