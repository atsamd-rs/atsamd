#[doc = "Reader of register TYPE"]
pub type R = crate::R<u32, super::TYPE>;
#[doc = "Reader of field `GCLK`"]
pub type GCLK_R = crate::R<bool, bool>;
#[doc = "Reader of field `RRP`"]
pub type RRP_R = crate::R<bool, bool>;
#[doc = "Number of Way\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAYNUM_A {
    #[doc = "0: Direct Mapped Cache"]
    DMAPPED = 0,
    #[doc = "1: 2-WAY set associative"]
    ARCH2WAY = 1,
    #[doc = "2: 4-WAY set associative"]
    ARCH4WAY = 2,
}
impl From<WAYNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: WAYNUM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WAYNUM`"]
pub type WAYNUM_R = crate::R<u8, WAYNUM_A>;
impl WAYNUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WAYNUM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WAYNUM_A::DMAPPED),
            1 => Val(WAYNUM_A::ARCH2WAY),
            2 => Val(WAYNUM_A::ARCH4WAY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMAPPED`"]
    #[inline(always)]
    pub fn is_dmapped(&self) -> bool {
        *self == WAYNUM_A::DMAPPED
    }
    #[doc = "Checks if the value of the field is `ARCH2WAY`"]
    #[inline(always)]
    pub fn is_arch2way(&self) -> bool {
        *self == WAYNUM_A::ARCH2WAY
    }
    #[doc = "Checks if the value of the field is `ARCH4WAY`"]
    #[inline(always)]
    pub fn is_arch4way(&self) -> bool {
        *self == WAYNUM_A::ARCH4WAY
    }
}
#[doc = "Reader of field `LCKDOWN`"]
pub type LCKDOWN_R = crate::R<bool, bool>;
#[doc = "Cache Size\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSIZE_A {
    #[doc = "0: Cache Size is 1 KB"]
    CSIZE_1KB = 0,
    #[doc = "1: Cache Size is 2 KB"]
    CSIZE_2KB = 1,
    #[doc = "2: Cache Size is 4 KB"]
    CSIZE_4KB = 2,
    #[doc = "3: Cache Size is 8 KB"]
    CSIZE_8KB = 3,
    #[doc = "4: Cache Size is 16 KB"]
    CSIZE_16KB = 4,
    #[doc = "5: Cache Size is 32 KB"]
    CSIZE_32KB = 5,
    #[doc = "6: Cache Size is 64 KB"]
    CSIZE_64KB = 6,
}
impl From<CSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CSIZE`"]
pub type CSIZE_R = crate::R<u8, CSIZE_A>;
impl CSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CSIZE_A::CSIZE_1KB),
            1 => Val(CSIZE_A::CSIZE_2KB),
            2 => Val(CSIZE_A::CSIZE_4KB),
            3 => Val(CSIZE_A::CSIZE_8KB),
            4 => Val(CSIZE_A::CSIZE_16KB),
            5 => Val(CSIZE_A::CSIZE_32KB),
            6 => Val(CSIZE_A::CSIZE_64KB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CSIZE_1KB`"]
    #[inline(always)]
    pub fn is_csize_1kb(&self) -> bool {
        *self == CSIZE_A::CSIZE_1KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_2KB`"]
    #[inline(always)]
    pub fn is_csize_2kb(&self) -> bool {
        *self == CSIZE_A::CSIZE_2KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_4KB`"]
    #[inline(always)]
    pub fn is_csize_4kb(&self) -> bool {
        *self == CSIZE_A::CSIZE_4KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_8KB`"]
    #[inline(always)]
    pub fn is_csize_8kb(&self) -> bool {
        *self == CSIZE_A::CSIZE_8KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_16KB`"]
    #[inline(always)]
    pub fn is_csize_16kb(&self) -> bool {
        *self == CSIZE_A::CSIZE_16KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_32KB`"]
    #[inline(always)]
    pub fn is_csize_32kb(&self) -> bool {
        *self == CSIZE_A::CSIZE_32KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_64KB`"]
    #[inline(always)]
    pub fn is_csize_64kb(&self) -> bool {
        *self == CSIZE_A::CSIZE_64KB
    }
}
#[doc = "Cache Line Size\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLSIZE_A {
    #[doc = "0: Cache Line Size is 4 bytes"]
    CLSIZE_4B = 0,
    #[doc = "1: Cache Line Size is 8 bytes"]
    CLSIZE_8B = 1,
    #[doc = "2: Cache Line Size is 16 bytes"]
    CLSIZE_16B = 2,
    #[doc = "3: Cache Line Size is 32 bytes"]
    CLSIZE_32B = 3,
    #[doc = "4: Cache Line Size is 64 bytes"]
    CLSIZE_64B = 4,
    #[doc = "5: Cache Line Size is 128 bytes"]
    CLSIZE_128B = 5,
}
impl From<CLSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CLSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLSIZE`"]
pub type CLSIZE_R = crate::R<u8, CLSIZE_A>;
impl CLSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLSIZE_A::CLSIZE_4B),
            1 => Val(CLSIZE_A::CLSIZE_8B),
            2 => Val(CLSIZE_A::CLSIZE_16B),
            3 => Val(CLSIZE_A::CLSIZE_32B),
            4 => Val(CLSIZE_A::CLSIZE_64B),
            5 => Val(CLSIZE_A::CLSIZE_128B),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLSIZE_4B`"]
    #[inline(always)]
    pub fn is_clsize_4b(&self) -> bool {
        *self == CLSIZE_A::CLSIZE_4B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_8B`"]
    #[inline(always)]
    pub fn is_clsize_8b(&self) -> bool {
        *self == CLSIZE_A::CLSIZE_8B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_16B`"]
    #[inline(always)]
    pub fn is_clsize_16b(&self) -> bool {
        *self == CLSIZE_A::CLSIZE_16B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_32B`"]
    #[inline(always)]
    pub fn is_clsize_32b(&self) -> bool {
        *self == CLSIZE_A::CLSIZE_32B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_64B`"]
    #[inline(always)]
    pub fn is_clsize_64b(&self) -> bool {
        *self == CLSIZE_A::CLSIZE_64B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_128B`"]
    #[inline(always)]
    pub fn is_clsize_128b(&self) -> bool {
        *self == CLSIZE_A::CLSIZE_128B
    }
}
impl R {
    #[doc = "Bit 1 - dynamic Clock Gating supported"]
    #[inline(always)]
    pub fn gclk(&self) -> GCLK_R {
        GCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Round Robin Policy supported"]
    #[inline(always)]
    pub fn rrp(&self) -> RRP_R {
        RRP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Number of Way"]
    #[inline(always)]
    pub fn waynum(&self) -> WAYNUM_R {
        WAYNUM_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Lock Down supported"]
    #[inline(always)]
    pub fn lckdown(&self) -> LCKDOWN_R {
        LCKDOWN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Cache Size"]
    #[inline(always)]
    pub fn csize(&self) -> CSIZE_R {
        CSIZE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Cache Line Size"]
    #[inline(always)]
    pub fn clsize(&self) -> CLSIZE_R {
        CLSIZE_R::new(((self.bits >> 11) & 0x07) as u8)
    }
}
