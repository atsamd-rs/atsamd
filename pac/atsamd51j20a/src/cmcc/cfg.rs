#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0x20"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `ICDIS`"]
pub type ICDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICDIS`"]
pub struct ICDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DCDIS`"]
pub type DCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDIS`"]
pub struct DCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Cache size configured by software\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSIZESW_A {
    #[doc = "0: the Cache Size is configured to 1KB"]
    CONF_CSIZE_1KB,
    #[doc = "1: the Cache Size is configured to 2KB"]
    CONF_CSIZE_2KB,
    #[doc = "2: the Cache Size is configured to 4KB"]
    CONF_CSIZE_4KB,
    #[doc = "3: the Cache Size is configured to 8KB"]
    CONF_CSIZE_8KB,
    #[doc = "4: the Cache Size is configured to 16KB"]
    CONF_CSIZE_16KB,
    #[doc = "5: the Cache Size is configured to 32KB"]
    CONF_CSIZE_32KB,
    #[doc = "6: the Cache Size is configured to 64KB"]
    CONF_CSIZE_64KB,
}
impl From<CSIZESW_A> for u8 {
    #[inline(always)]
    fn from(variant: CSIZESW_A) -> Self {
        match variant {
            CSIZESW_A::CONF_CSIZE_1KB => 0,
            CSIZESW_A::CONF_CSIZE_2KB => 1,
            CSIZESW_A::CONF_CSIZE_4KB => 2,
            CSIZESW_A::CONF_CSIZE_8KB => 3,
            CSIZESW_A::CONF_CSIZE_16KB => 4,
            CSIZESW_A::CONF_CSIZE_32KB => 5,
            CSIZESW_A::CONF_CSIZE_64KB => 6,
        }
    }
}
#[doc = "Reader of field `CSIZESW`"]
pub type CSIZESW_R = crate::R<u8, CSIZESW_A>;
impl CSIZESW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CSIZESW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CSIZESW_A::CONF_CSIZE_1KB),
            1 => Val(CSIZESW_A::CONF_CSIZE_2KB),
            2 => Val(CSIZESW_A::CONF_CSIZE_4KB),
            3 => Val(CSIZESW_A::CONF_CSIZE_8KB),
            4 => Val(CSIZESW_A::CONF_CSIZE_16KB),
            5 => Val(CSIZESW_A::CONF_CSIZE_32KB),
            6 => Val(CSIZESW_A::CONF_CSIZE_64KB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_1KB`"]
    #[inline(always)]
    pub fn is_conf_csize_1kb(&self) -> bool {
        *self == CSIZESW_A::CONF_CSIZE_1KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_2KB`"]
    #[inline(always)]
    pub fn is_conf_csize_2kb(&self) -> bool {
        *self == CSIZESW_A::CONF_CSIZE_2KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_4KB`"]
    #[inline(always)]
    pub fn is_conf_csize_4kb(&self) -> bool {
        *self == CSIZESW_A::CONF_CSIZE_4KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_8KB`"]
    #[inline(always)]
    pub fn is_conf_csize_8kb(&self) -> bool {
        *self == CSIZESW_A::CONF_CSIZE_8KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_16KB`"]
    #[inline(always)]
    pub fn is_conf_csize_16kb(&self) -> bool {
        *self == CSIZESW_A::CONF_CSIZE_16KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_32KB`"]
    #[inline(always)]
    pub fn is_conf_csize_32kb(&self) -> bool {
        *self == CSIZESW_A::CONF_CSIZE_32KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_64KB`"]
    #[inline(always)]
    pub fn is_conf_csize_64kb(&self) -> bool {
        *self == CSIZESW_A::CONF_CSIZE_64KB
    }
}
#[doc = "Write proxy for field `CSIZESW`"]
pub struct CSIZESW_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIZESW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSIZESW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "the Cache Size is configured to 1KB"]
    #[inline(always)]
    pub fn conf_csize_1kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_1KB)
    }
    #[doc = "the Cache Size is configured to 2KB"]
    #[inline(always)]
    pub fn conf_csize_2kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_2KB)
    }
    #[doc = "the Cache Size is configured to 4KB"]
    #[inline(always)]
    pub fn conf_csize_4kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_4KB)
    }
    #[doc = "the Cache Size is configured to 8KB"]
    #[inline(always)]
    pub fn conf_csize_8kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_8KB)
    }
    #[doc = "the Cache Size is configured to 16KB"]
    #[inline(always)]
    pub fn conf_csize_16kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_16KB)
    }
    #[doc = "the Cache Size is configured to 32KB"]
    #[inline(always)]
    pub fn conf_csize_32kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_32KB)
    }
    #[doc = "the Cache Size is configured to 64KB"]
    #[inline(always)]
    pub fn conf_csize_64kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_64KB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Instruction Cache Disable"]
    #[inline(always)]
    pub fn icdis(&self) -> ICDIS_R {
        ICDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data Cache Disable"]
    #[inline(always)]
    pub fn dcdis(&self) -> DCDIS_R {
        DCDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Cache size configured by software"]
    #[inline(always)]
    pub fn csizesw(&self) -> CSIZESW_R {
        CSIZESW_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Instruction Cache Disable"]
    #[inline(always)]
    pub fn icdis(&mut self) -> ICDIS_W {
        ICDIS_W { w: self }
    }
    #[doc = "Bit 2 - Data Cache Disable"]
    #[inline(always)]
    pub fn dcdis(&mut self) -> DCDIS_W {
        DCDIS_W { w: self }
    }
    #[doc = "Bits 4:6 - Cache size configured by software"]
    #[inline(always)]
    pub fn csizesw(&mut self) -> CSIZESW_W {
        CSIZESW_W { w: self }
    }
}
