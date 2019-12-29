#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WBDIS`"]
pub type WBDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WBDIS`"]
pub struct WBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WBDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `EOMDIS`"]
pub type EOMDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOMDIS`"]
pub struct EOMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOMDIS_W<'a> {
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
#[doc = "Reader of field `SLBDIS`"]
pub type SLBDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLBDIS`"]
pub struct SLBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLBDIS_W<'a> {
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
#[doc = "Reader of field `BBC`"]
pub type BBC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BBC`"]
pub struct BBC_W<'a> {
    w: &'a mut W,
}
impl<'a> BBC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `ASCD`"]
pub type ASCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASCD`"]
pub struct ASCD_W<'a> {
    w: &'a mut W,
}
impl<'a> ASCD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DUALBUFF`"]
pub type DUALBUFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUALBUFF`"]
pub struct DUALBUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALBUFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `UIHASH`"]
pub type UIHASH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UIHASH`"]
pub struct UIHASH_W<'a> {
    w: &'a mut W,
}
impl<'a> UIHASH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "User SHA Algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UALGO_A {
    #[doc = "0: SHA1 Algorithm"]
    SHA1 = 0,
    #[doc = "1: SHA256 Algorithm"]
    SHA256 = 1,
    #[doc = "4: SHA224 Algorithm"]
    SHA224 = 4,
}
impl From<UALGO_A> for u8 {
    #[inline(always)]
    fn from(variant: UALGO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UALGO`"]
pub type UALGO_R = crate::R<u8, UALGO_A>;
impl UALGO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UALGO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UALGO_A::SHA1),
            1 => Val(UALGO_A::SHA256),
            4 => Val(UALGO_A::SHA224),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == UALGO_A::SHA1
    }
    #[doc = "Checks if the value of the field is `SHA256`"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == UALGO_A::SHA256
    }
    #[doc = "Checks if the value of the field is `SHA224`"]
    #[inline(always)]
    pub fn is_sha224(&self) -> bool {
        *self == UALGO_A::SHA224
    }
}
#[doc = "Write proxy for field `UALGO`"]
pub struct UALGO_W<'a> {
    w: &'a mut W,
}
impl<'a> UALGO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UALGO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SHA1 Algorithm"]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut W {
        self.variant(UALGO_A::SHA1)
    }
    #[doc = "SHA256 Algorithm"]
    #[inline(always)]
    pub fn sha256(self) -> &'a mut W {
        self.variant(UALGO_A::SHA256)
    }
    #[doc = "SHA224 Algorithm"]
    #[inline(always)]
    pub fn sha224(self) -> &'a mut W {
        self.variant(UALGO_A::SHA224)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `HAPROT`"]
pub type HAPROT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HAPROT`"]
pub struct HAPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> HAPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DAPROT`"]
pub type DAPROT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAPROT`"]
pub struct DAPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write Back Disable"]
    #[inline(always)]
    pub fn wbdis(&self) -> WBDIS_R {
        WBDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Monitoring Disable"]
    #[inline(always)]
    pub fn eomdis(&self) -> EOMDIS_R {
        EOMDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Secondary List Branching Disable"]
    #[inline(always)]
    pub fn slbdis(&self) -> SLBDIS_R {
        SLBDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Bus Burden Control"]
    #[inline(always)]
    pub fn bbc(&self) -> BBC_R {
        BBC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Automatic Switch To Compare Digest"]
    #[inline(always)]
    pub fn ascd(&self) -> ASCD_R {
        ASCD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&self) -> DUALBUFF_R {
        DUALBUFF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - User Initial Hash Value"]
    #[inline(always)]
    pub fn uihash(&self) -> UIHASH_R {
        UIHASH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - User SHA Algorithm"]
    #[inline(always)]
    pub fn ualgo(&self) -> UALGO_R {
        UALGO_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:21 - Region Hash Area Protection"]
    #[inline(always)]
    pub fn haprot(&self) -> HAPROT_R {
        HAPROT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Region Descriptor Area Protection"]
    #[inline(always)]
    pub fn daprot(&self) -> DAPROT_R {
        DAPROT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write Back Disable"]
    #[inline(always)]
    pub fn wbdis(&mut self) -> WBDIS_W {
        WBDIS_W { w: self }
    }
    #[doc = "Bit 1 - End of Monitoring Disable"]
    #[inline(always)]
    pub fn eomdis(&mut self) -> EOMDIS_W {
        EOMDIS_W { w: self }
    }
    #[doc = "Bit 2 - Secondary List Branching Disable"]
    #[inline(always)]
    pub fn slbdis(&mut self) -> SLBDIS_W {
        SLBDIS_W { w: self }
    }
    #[doc = "Bits 4:7 - Bus Burden Control"]
    #[inline(always)]
    pub fn bbc(&mut self) -> BBC_W {
        BBC_W { w: self }
    }
    #[doc = "Bit 8 - Automatic Switch To Compare Digest"]
    #[inline(always)]
    pub fn ascd(&mut self) -> ASCD_W {
        ASCD_W { w: self }
    }
    #[doc = "Bit 9 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&mut self) -> DUALBUFF_W {
        DUALBUFF_W { w: self }
    }
    #[doc = "Bit 12 - User Initial Hash Value"]
    #[inline(always)]
    pub fn uihash(&mut self) -> UIHASH_W {
        UIHASH_W { w: self }
    }
    #[doc = "Bits 13:15 - User SHA Algorithm"]
    #[inline(always)]
    pub fn ualgo(&mut self) -> UALGO_W {
        UALGO_W { w: self }
    }
    #[doc = "Bits 16:21 - Region Hash Area Protection"]
    #[inline(always)]
    pub fn haprot(&mut self) -> HAPROT_W {
        HAPROT_W { w: self }
    }
    #[doc = "Bits 24:29 - Region Descriptor Area Protection"]
    #[inline(always)]
    pub fn daprot(&mut self) -> DAPROT_W {
        DAPROT_W { w: self }
    }
}
