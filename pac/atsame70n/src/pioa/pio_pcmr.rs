#[doc = "Register `PIO_PCMR` reader"]
pub struct R(crate::R<PIO_PCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO_PCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO_PCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO_PCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO_PCMR` writer"]
pub struct W(crate::W<PIO_PCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO_PCMR_SPEC>;
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
impl From<crate::W<PIO_PCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO_PCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCEN` reader - Parallel Capture Mode Enable"]
pub struct PCEN_R(crate::FieldReader<bool, bool>);
impl PCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCEN` writer - Parallel Capture Mode Enable"]
pub struct PCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Parallel Capture Mode Data Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSIZE_A {
    #[doc = "0: The reception data in the PIO_PCRHR is a byte (8-bit)"]
    BYTE = 0,
    #[doc = "1: The reception data in the PIO_PCRHR is a half-word (16-bit)"]
    HALFWORD = 1,
    #[doc = "2: The reception data in the PIO_PCRHR is a word (32-bit)"]
    WORD = 2,
}
impl From<DSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DSIZE` reader - Parallel Capture Mode Data Size"]
pub struct DSIZE_R(crate::FieldReader<u8, DSIZE_A>);
impl DSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSIZE_A> {
        match self.bits {
            0 => Some(DSIZE_A::BYTE),
            1 => Some(DSIZE_A::HALFWORD),
            2 => Some(DSIZE_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        **self == DSIZE_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        **self == DSIZE_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        **self == DSIZE_A::WORD
    }
}
impl core::ops::Deref for DSIZE_R {
    type Target = crate::FieldReader<u8, DSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSIZE` writer - Parallel Capture Mode Data Size"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The reception data in the PIO_PCRHR is a byte (8-bit)"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DSIZE_A::BYTE)
    }
    #[doc = "The reception data in the PIO_PCRHR is a half-word (16-bit)"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut W {
        self.variant(DSIZE_A::HALFWORD)
    }
    #[doc = "The reception data in the PIO_PCRHR is a word (32-bit)"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DSIZE_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `ALWYS` reader - Parallel Capture Mode Always Sampling"]
pub struct ALWYS_R(crate::FieldReader<bool, bool>);
impl ALWYS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALWYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALWYS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALWYS` writer - Parallel Capture Mode Always Sampling"]
pub struct ALWYS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALWYS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `HALFS` reader - Parallel Capture Mode Half Sampling"]
pub struct HALFS_R(crate::FieldReader<bool, bool>);
impl HALFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALFS` writer - Parallel Capture Mode Half Sampling"]
pub struct HALFS_W<'a> {
    w: &'a mut W,
}
impl<'a> HALFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FRSTS` reader - Parallel Capture Mode First Sample"]
pub struct FRSTS_R(crate::FieldReader<bool, bool>);
impl FRSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRSTS` writer - Parallel Capture Mode First Sample"]
pub struct FRSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Parallel Capture Mode Enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Parallel Capture Mode Data Size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Parallel Capture Mode Always Sampling"]
    #[inline(always)]
    pub fn alwys(&self) -> ALWYS_R {
        ALWYS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Parallel Capture Mode Half Sampling"]
    #[inline(always)]
    pub fn halfs(&self) -> HALFS_R {
        HALFS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Parallel Capture Mode First Sample"]
    #[inline(always)]
    pub fn frsts(&self) -> FRSTS_R {
        FRSTS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parallel Capture Mode Enable"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PCEN_W {
        PCEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Parallel Capture Mode Data Size"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
    #[doc = "Bit 9 - Parallel Capture Mode Always Sampling"]
    #[inline(always)]
    pub fn alwys(&mut self) -> ALWYS_W {
        ALWYS_W { w: self }
    }
    #[doc = "Bit 10 - Parallel Capture Mode Half Sampling"]
    #[inline(always)]
    pub fn halfs(&mut self) -> HALFS_W {
        HALFS_W { w: self }
    }
    #[doc = "Bit 11 - Parallel Capture Mode First Sample"]
    #[inline(always)]
    pub fn frsts(&mut self) -> FRSTS_W {
        FRSTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel Capture Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pcmr](index.html) module"]
pub struct PIO_PCMR_SPEC;
impl crate::RegisterSpec for PIO_PCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio_pcmr::R](R) reader structure"]
impl crate::Readable for PIO_PCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio_pcmr::W](W) writer structure"]
impl crate::Writable for PIO_PCMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIO_PCMR to value 0"]
impl crate::Resettable for PIO_PCMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
