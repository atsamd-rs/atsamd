#[doc = "Register `TWIHS_MMR` reader"]
pub struct R(crate::R<TWIHS_MMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWIHS_MMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWIHS_MMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWIHS_MMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWIHS_MMR` writer"]
pub struct W(crate::W<TWIHS_MMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWIHS_MMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TWIHS_MMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWIHS_MMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Internal Device Address Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IADRSZ_A {
    #[doc = "0: No internal device address"]
    NONE = 0,
    #[doc = "1: One-byte internal device address"]
    _1_BYTE = 1,
    #[doc = "2: Two-byte internal device address"]
    _2_BYTE = 2,
    #[doc = "3: Three-byte internal device address"]
    _3_BYTE = 3,
}
impl From<IADRSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: IADRSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IADRSZ` reader - Internal Device Address Size"]
pub struct IADRSZ_R(crate::FieldReader<u8, IADRSZ_A>);
impl IADRSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IADRSZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IADRSZ_A {
        match self.bits {
            0 => IADRSZ_A::NONE,
            1 => IADRSZ_A::_1_BYTE,
            2 => IADRSZ_A::_2_BYTE,
            3 => IADRSZ_A::_3_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == IADRSZ_A::NONE
    }
    #[doc = "Checks if the value of the field is `_1_BYTE`"]
    #[inline(always)]
    pub fn is_1_byte(&self) -> bool {
        **self == IADRSZ_A::_1_BYTE
    }
    #[doc = "Checks if the value of the field is `_2_BYTE`"]
    #[inline(always)]
    pub fn is_2_byte(&self) -> bool {
        **self == IADRSZ_A::_2_BYTE
    }
    #[doc = "Checks if the value of the field is `_3_BYTE`"]
    #[inline(always)]
    pub fn is_3_byte(&self) -> bool {
        **self == IADRSZ_A::_3_BYTE
    }
}
impl core::ops::Deref for IADRSZ_R {
    type Target = crate::FieldReader<u8, IADRSZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IADRSZ` writer - Internal Device Address Size"]
pub struct IADRSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IADRSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IADRSZ_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No internal device address"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(IADRSZ_A::NONE)
    }
    #[doc = "One-byte internal device address"]
    #[inline(always)]
    pub fn _1_byte(self) -> &'a mut W {
        self.variant(IADRSZ_A::_1_BYTE)
    }
    #[doc = "Two-byte internal device address"]
    #[inline(always)]
    pub fn _2_byte(self) -> &'a mut W {
        self.variant(IADRSZ_A::_2_BYTE)
    }
    #[doc = "Three-byte internal device address"]
    #[inline(always)]
    pub fn _3_byte(self) -> &'a mut W {
        self.variant(IADRSZ_A::_3_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `MREAD` reader - Master Read Direction"]
pub struct MREAD_R(crate::FieldReader<bool, bool>);
impl MREAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MREAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MREAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MREAD` writer - Master Read Direction"]
pub struct MREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> MREAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DADR` reader - Device Address"]
pub struct DADR_R(crate::FieldReader<u8, u8>);
impl DADR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DADR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DADR` writer - Device Address"]
pub struct DADR_W<'a> {
    w: &'a mut W,
}
impl<'a> DADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    pub fn iadrsz(&self) -> IADRSZ_R {
        IADRSZ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    pub fn mread(&self) -> MREAD_R {
        MREAD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    pub fn dadr(&self) -> DADR_R {
        DADR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    pub fn iadrsz(&mut self) -> IADRSZ_W {
        IADRSZ_W { w: self }
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    pub fn mread(&mut self) -> MREAD_W {
        MREAD_W { w: self }
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    pub fn dadr(&mut self) -> DADR_W {
        DADR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_mmr](index.html) module"]
pub struct TWIHS_MMR_SPEC;
impl crate::RegisterSpec for TWIHS_MMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twihs_mmr::R](R) reader structure"]
impl crate::Readable for TWIHS_MMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twihs_mmr::W](W) writer structure"]
impl crate::Writable for TWIHS_MMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWIHS_MMR to value 0"]
impl crate::Resettable for TWIHS_MMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
