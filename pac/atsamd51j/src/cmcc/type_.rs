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
pub struct GCLK_R(crate::FieldReader<bool, bool>);
impl GCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRP` reader - Round Robin Policy supported"]
pub struct RRP_R(crate::FieldReader<bool, bool>);
impl RRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Field `WAYNUM` reader - Number of Way"]
pub struct WAYNUM_R(crate::FieldReader<u8, WAYNUM_A>);
impl WAYNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WAYNUM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAYNUM_A> {
        match self.bits {
            0 => Some(WAYNUM_A::DMAPPED),
            1 => Some(WAYNUM_A::ARCH2WAY),
            2 => Some(WAYNUM_A::ARCH4WAY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMAPPED`"]
    #[inline(always)]
    pub fn is_dmapped(&self) -> bool {
        **self == WAYNUM_A::DMAPPED
    }
    #[doc = "Checks if the value of the field is `ARCH2WAY`"]
    #[inline(always)]
    pub fn is_arch2way(&self) -> bool {
        **self == WAYNUM_A::ARCH2WAY
    }
    #[doc = "Checks if the value of the field is `ARCH4WAY`"]
    #[inline(always)]
    pub fn is_arch4way(&self) -> bool {
        **self == WAYNUM_A::ARCH4WAY
    }
}
impl core::ops::Deref for WAYNUM_R {
    type Target = crate::FieldReader<u8, WAYNUM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKDOWN` reader - Lock Down supported"]
pub struct LCKDOWN_R(crate::FieldReader<bool, bool>);
impl LCKDOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCKDOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKDOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Field `CSIZE` reader - Cache Size"]
pub struct CSIZE_R(crate::FieldReader<u8, CSIZE_A>);
impl CSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSIZE_A> {
        match self.bits {
            0 => Some(CSIZE_A::CSIZE_1KB),
            1 => Some(CSIZE_A::CSIZE_2KB),
            2 => Some(CSIZE_A::CSIZE_4KB),
            3 => Some(CSIZE_A::CSIZE_8KB),
            4 => Some(CSIZE_A::CSIZE_16KB),
            5 => Some(CSIZE_A::CSIZE_32KB),
            6 => Some(CSIZE_A::CSIZE_64KB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CSIZE_1KB`"]
    #[inline(always)]
    pub fn is_csize_1kb(&self) -> bool {
        **self == CSIZE_A::CSIZE_1KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_2KB`"]
    #[inline(always)]
    pub fn is_csize_2kb(&self) -> bool {
        **self == CSIZE_A::CSIZE_2KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_4KB`"]
    #[inline(always)]
    pub fn is_csize_4kb(&self) -> bool {
        **self == CSIZE_A::CSIZE_4KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_8KB`"]
    #[inline(always)]
    pub fn is_csize_8kb(&self) -> bool {
        **self == CSIZE_A::CSIZE_8KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_16KB`"]
    #[inline(always)]
    pub fn is_csize_16kb(&self) -> bool {
        **self == CSIZE_A::CSIZE_16KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_32KB`"]
    #[inline(always)]
    pub fn is_csize_32kb(&self) -> bool {
        **self == CSIZE_A::CSIZE_32KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_64KB`"]
    #[inline(always)]
    pub fn is_csize_64kb(&self) -> bool {
        **self == CSIZE_A::CSIZE_64KB
    }
}
impl core::ops::Deref for CSIZE_R {
    type Target = crate::FieldReader<u8, CSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CLSIZE` reader - Cache Line Size"]
pub struct CLSIZE_R(crate::FieldReader<u8, CLSIZE_A>);
impl CLSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLSIZE_A> {
        match self.bits {
            0 => Some(CLSIZE_A::CLSIZE_4B),
            1 => Some(CLSIZE_A::CLSIZE_8B),
            2 => Some(CLSIZE_A::CLSIZE_16B),
            3 => Some(CLSIZE_A::CLSIZE_32B),
            4 => Some(CLSIZE_A::CLSIZE_64B),
            5 => Some(CLSIZE_A::CLSIZE_128B),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLSIZE_4B`"]
    #[inline(always)]
    pub fn is_clsize_4b(&self) -> bool {
        **self == CLSIZE_A::CLSIZE_4B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_8B`"]
    #[inline(always)]
    pub fn is_clsize_8b(&self) -> bool {
        **self == CLSIZE_A::CLSIZE_8B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_16B`"]
    #[inline(always)]
    pub fn is_clsize_16b(&self) -> bool {
        **self == CLSIZE_A::CLSIZE_16B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_32B`"]
    #[inline(always)]
    pub fn is_clsize_32b(&self) -> bool {
        **self == CLSIZE_A::CLSIZE_32B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_64B`"]
    #[inline(always)]
    pub fn is_clsize_64b(&self) -> bool {
        **self == CLSIZE_A::CLSIZE_64B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_128B`"]
    #[inline(always)]
    pub fn is_clsize_128b(&self) -> bool {
        **self == CLSIZE_A::CLSIZE_128B
    }
}
impl core::ops::Deref for CLSIZE_R {
    type Target = crate::FieldReader<u8, CLSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x12d2
    }
}
