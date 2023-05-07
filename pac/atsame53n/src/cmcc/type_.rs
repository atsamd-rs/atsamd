#[doc = "Register `TYPE` reader"]
pub struct R(crate::R<TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GCLK` reader - dynamic Clock Gating supported"]
pub type GCLK_R = crate::BitReader<bool>;
#[doc = "Field `RRP` reader - Round Robin Policy supported"]
pub type RRP_R = crate::BitReader<bool>;
#[doc = "Field `WAYNUM` reader - Number of Way"]
pub type WAYNUM_R = crate::FieldReader<u8, WAYNUMSELECT_A>;
#[doc = "Number of Way\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAYNUMSELECT_A {
    #[doc = "0: Direct Mapped Cache"]
    DMAPPED = 0,
    #[doc = "1: 2-WAY set associative"]
    ARCH2WAY = 1,
    #[doc = "2: 4-WAY set associative"]
    ARCH4WAY = 2,
}
impl From<WAYNUMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WAYNUMSELECT_A) -> Self {
        variant as _
    }
}
impl WAYNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAYNUMSELECT_A> {
        match self.bits {
            0 => Some(WAYNUMSELECT_A::DMAPPED),
            1 => Some(WAYNUMSELECT_A::ARCH2WAY),
            2 => Some(WAYNUMSELECT_A::ARCH4WAY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMAPPED`"]
    #[inline(always)]
    pub fn is_dmapped(&self) -> bool {
        *self == WAYNUMSELECT_A::DMAPPED
    }
    #[doc = "Checks if the value of the field is `ARCH2WAY`"]
    #[inline(always)]
    pub fn is_arch2way(&self) -> bool {
        *self == WAYNUMSELECT_A::ARCH2WAY
    }
    #[doc = "Checks if the value of the field is `ARCH4WAY`"]
    #[inline(always)]
    pub fn is_arch4way(&self) -> bool {
        *self == WAYNUMSELECT_A::ARCH4WAY
    }
}
#[doc = "Field `LCKDOWN` reader - Lock Down supported"]
pub type LCKDOWN_R = crate::BitReader<bool>;
#[doc = "Field `CSIZE` reader - Cache Size"]
pub type CSIZE_R = crate::FieldReader<u8, CSIZESELECT_A>;
#[doc = "Cache Size\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSIZESELECT_A {
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
impl From<CSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSIZESELECT_A) -> Self {
        variant as _
    }
}
impl CSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSIZESELECT_A> {
        match self.bits {
            0 => Some(CSIZESELECT_A::CSIZE_1KB),
            1 => Some(CSIZESELECT_A::CSIZE_2KB),
            2 => Some(CSIZESELECT_A::CSIZE_4KB),
            3 => Some(CSIZESELECT_A::CSIZE_8KB),
            4 => Some(CSIZESELECT_A::CSIZE_16KB),
            5 => Some(CSIZESELECT_A::CSIZE_32KB),
            6 => Some(CSIZESELECT_A::CSIZE_64KB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CSIZE_1KB`"]
    #[inline(always)]
    pub fn is_csize_1kb(&self) -> bool {
        *self == CSIZESELECT_A::CSIZE_1KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_2KB`"]
    #[inline(always)]
    pub fn is_csize_2kb(&self) -> bool {
        *self == CSIZESELECT_A::CSIZE_2KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_4KB`"]
    #[inline(always)]
    pub fn is_csize_4kb(&self) -> bool {
        *self == CSIZESELECT_A::CSIZE_4KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_8KB`"]
    #[inline(always)]
    pub fn is_csize_8kb(&self) -> bool {
        *self == CSIZESELECT_A::CSIZE_8KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_16KB`"]
    #[inline(always)]
    pub fn is_csize_16kb(&self) -> bool {
        *self == CSIZESELECT_A::CSIZE_16KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_32KB`"]
    #[inline(always)]
    pub fn is_csize_32kb(&self) -> bool {
        *self == CSIZESELECT_A::CSIZE_32KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_64KB`"]
    #[inline(always)]
    pub fn is_csize_64kb(&self) -> bool {
        *self == CSIZESELECT_A::CSIZE_64KB
    }
}
#[doc = "Field `CLSIZE` reader - Cache Line Size"]
pub type CLSIZE_R = crate::FieldReader<u8, CLSIZESELECT_A>;
#[doc = "Cache Line Size\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLSIZESELECT_A {
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
impl From<CLSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CLSIZESELECT_A) -> Self {
        variant as _
    }
}
impl CLSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLSIZESELECT_A> {
        match self.bits {
            0 => Some(CLSIZESELECT_A::CLSIZE_4B),
            1 => Some(CLSIZESELECT_A::CLSIZE_8B),
            2 => Some(CLSIZESELECT_A::CLSIZE_16B),
            3 => Some(CLSIZESELECT_A::CLSIZE_32B),
            4 => Some(CLSIZESELECT_A::CLSIZE_64B),
            5 => Some(CLSIZESELECT_A::CLSIZE_128B),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLSIZE_4B`"]
    #[inline(always)]
    pub fn is_clsize_4b(&self) -> bool {
        *self == CLSIZESELECT_A::CLSIZE_4B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_8B`"]
    #[inline(always)]
    pub fn is_clsize_8b(&self) -> bool {
        *self == CLSIZESELECT_A::CLSIZE_8B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_16B`"]
    #[inline(always)]
    pub fn is_clsize_16b(&self) -> bool {
        *self == CLSIZESELECT_A::CLSIZE_16B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_32B`"]
    #[inline(always)]
    pub fn is_clsize_32b(&self) -> bool {
        *self == CLSIZESELECT_A::CLSIZE_32B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_64B`"]
    #[inline(always)]
    pub fn is_clsize_64b(&self) -> bool {
        *self == CLSIZESELECT_A::CLSIZE_64B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_128B`"]
    #[inline(always)]
    pub fn is_clsize_128b(&self) -> bool {
        *self == CLSIZESELECT_A::CLSIZE_128B
    }
}
impl R {
    #[doc = "Bit 1 - dynamic Clock Gating supported"]
    #[inline(always)]
    pub fn gclk(&self) -> GCLK_R {
        GCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Round Robin Policy supported"]
    #[inline(always)]
    pub fn rrp(&self) -> RRP_R {
        RRP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Number of Way"]
    #[inline(always)]
    pub fn waynum(&self) -> WAYNUM_R {
        WAYNUM_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Lock Down supported"]
    #[inline(always)]
    pub fn lckdown(&self) -> LCKDOWN_R {
        LCKDOWN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Cache Size"]
    #[inline(always)]
    pub fn csize(&self) -> CSIZE_R {
        CSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Cache Line Size"]
    #[inline(always)]
    pub fn clsize(&self) -> CLSIZE_R {
        CLSIZE_R::new(((self.bits >> 11) & 7) as u8)
    }
}
#[doc = "Cache Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type_](index.html) module"]
pub struct TYPE_SPEC;
impl crate::RegisterSpec for TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [type_::R](R) reader structure"]
impl crate::Readable for TYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TYPE to value 0x12d2"]
impl crate::Resettable for TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0x12d2;
}
