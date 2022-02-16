#[doc = "Register `AFEC_SEQ2R` reader"]
pub struct R(crate::R<AFEC_SEQ2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_SEQ2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_SEQ2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_SEQ2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_SEQ2R` writer"]
pub struct W(crate::W<AFEC_SEQ2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_SEQ2R_SPEC>;
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
impl From<crate::W<AFEC_SEQ2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_SEQ2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USCH8` reader - User Sequence Number 8"]
pub struct USCH8_R(crate::FieldReader<u8, u8>);
impl USCH8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH8` writer - User Sequence Number 8"]
pub struct USCH8_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `USCH9` reader - User Sequence Number 9"]
pub struct USCH9_R(crate::FieldReader<u8, u8>);
impl USCH9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH9` writer - User Sequence Number 9"]
pub struct USCH9_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `USCH10` reader - User Sequence Number 10"]
pub struct USCH10_R(crate::FieldReader<u8, u8>);
impl USCH10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH10` writer - User Sequence Number 10"]
pub struct USCH10_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `USCH11` reader - User Sequence Number 11"]
pub struct USCH11_R(crate::FieldReader<u8, u8>);
impl USCH11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH11` writer - User Sequence Number 11"]
pub struct USCH11_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - User Sequence Number 8"]
    #[inline(always)]
    pub fn usch8(&self) -> USCH8_R {
        USCH8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&self) -> USCH9_R {
        USCH9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&self) -> USCH10_R {
        USCH10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&self) -> USCH11_R {
        USCH11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Sequence Number 8"]
    #[inline(always)]
    pub fn usch8(&mut self) -> USCH8_W {
        USCH8_W { w: self }
    }
    #[doc = "Bits 4:7 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&mut self) -> USCH9_W {
        USCH9_W { w: self }
    }
    #[doc = "Bits 8:11 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&mut self) -> USCH10_W {
        USCH10_W { w: self }
    }
    #[doc = "Bits 12:15 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&mut self) -> USCH11_W {
        USCH11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Sequence 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_seq2r](index.html) module"]
pub struct AFEC_SEQ2R_SPEC;
impl crate::RegisterSpec for AFEC_SEQ2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_seq2r::R](R) reader structure"]
impl crate::Readable for AFEC_SEQ2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_seq2r::W](W) writer structure"]
impl crate::Writable for AFEC_SEQ2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_SEQ2R to value 0"]
impl crate::Resettable for AFEC_SEQ2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
