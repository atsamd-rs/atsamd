#[doc = "Reader of register BSR"]
pub type R = crate::R<u16, super::BSR>;
#[doc = "Writer for register BSR"]
pub type W = crate::W<u16, super::BSR>;
#[doc = "Register BSR `reset()`'s with value 0"]
impl crate::ResetValue for super::BSR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLOCKSIZE`"]
pub type BLOCKSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BLOCKSIZE`"]
pub struct BLOCKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u16) & 0x03ff);
        self.w
    }
}
#[doc = "SDMA Buffer Boundary\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOUNDARY_A {
    #[doc = "0: 4k bytes"]
    _4K = 0,
    #[doc = "1: 8k bytes"]
    _8K = 1,
    #[doc = "2: 16k bytes"]
    _16K = 2,
    #[doc = "3: 32k bytes"]
    _32K = 3,
    #[doc = "4: 64k bytes"]
    _64K = 4,
    #[doc = "5: 128k bytes"]
    _128K = 5,
    #[doc = "6: 256k bytes"]
    _256K = 6,
    #[doc = "7: 512k bytes"]
    _512K = 7,
}
impl From<BOUNDARY_A> for u8 {
    #[inline(always)]
    fn from(variant: BOUNDARY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BOUNDARY`"]
pub type BOUNDARY_R = crate::R<u8, BOUNDARY_A>;
impl BOUNDARY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOUNDARY_A {
        match self.bits {
            0 => BOUNDARY_A::_4K,
            1 => BOUNDARY_A::_8K,
            2 => BOUNDARY_A::_16K,
            3 => BOUNDARY_A::_32K,
            4 => BOUNDARY_A::_64K,
            5 => BOUNDARY_A::_128K,
            6 => BOUNDARY_A::_256K,
            7 => BOUNDARY_A::_512K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == BOUNDARY_A::_4K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == BOUNDARY_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == BOUNDARY_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == BOUNDARY_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == BOUNDARY_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == BOUNDARY_A::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == BOUNDARY_A::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == BOUNDARY_A::_512K
    }
}
#[doc = "Write proxy for field `BOUNDARY`"]
pub struct BOUNDARY_W<'a> {
    w: &'a mut W,
}
impl<'a> BOUNDARY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOUNDARY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4k bytes"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut W {
        self.variant(BOUNDARY_A::_4K)
    }
    #[doc = "8k bytes"]
    #[inline(always)]
    pub fn _8k(self) -> &'a mut W {
        self.variant(BOUNDARY_A::_8K)
    }
    #[doc = "16k bytes"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut W {
        self.variant(BOUNDARY_A::_16K)
    }
    #[doc = "32k bytes"]
    #[inline(always)]
    pub fn _32k(self) -> &'a mut W {
        self.variant(BOUNDARY_A::_32K)
    }
    #[doc = "64k bytes"]
    #[inline(always)]
    pub fn _64k(self) -> &'a mut W {
        self.variant(BOUNDARY_A::_64K)
    }
    #[doc = "128k bytes"]
    #[inline(always)]
    pub fn _128k(self) -> &'a mut W {
        self.variant(BOUNDARY_A::_128K)
    }
    #[doc = "256k bytes"]
    #[inline(always)]
    pub fn _256k(self) -> &'a mut W {
        self.variant(BOUNDARY_A::_256K)
    }
    #[doc = "512k bytes"]
    #[inline(always)]
    pub fn _512k(self) -> &'a mut W {
        self.variant(BOUNDARY_A::_512K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Transfer Block Size"]
    #[inline(always)]
    pub fn blocksize(&self) -> BLOCKSIZE_R {
        BLOCKSIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary"]
    #[inline(always)]
    pub fn boundary(&self) -> BOUNDARY_R {
        BOUNDARY_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transfer Block Size"]
    #[inline(always)]
    pub fn blocksize(&mut self) -> BLOCKSIZE_W {
        BLOCKSIZE_W { w: self }
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary"]
    #[inline(always)]
    pub fn boundary(&mut self) -> BOUNDARY_W {
        BOUNDARY_W { w: self }
    }
}
