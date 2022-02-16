#[doc = "Register `AFEC_SEQ1R` reader"]
pub struct R(crate::R<AFEC_SEQ1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_SEQ1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_SEQ1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_SEQ1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_SEQ1R` writer"]
pub struct W(crate::W<AFEC_SEQ1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_SEQ1R_SPEC>;
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
impl From<crate::W<AFEC_SEQ1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_SEQ1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USCH0` reader - User Sequence Number 0"]
pub struct USCH0_R(crate::FieldReader<u8, u8>);
impl USCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH0` writer - User Sequence Number 0"]
pub struct USCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `USCH1` reader - User Sequence Number 1"]
pub struct USCH1_R(crate::FieldReader<u8, u8>);
impl USCH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH1` writer - User Sequence Number 1"]
pub struct USCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `USCH2` reader - User Sequence Number 2"]
pub struct USCH2_R(crate::FieldReader<u8, u8>);
impl USCH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH2` writer - User Sequence Number 2"]
pub struct USCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `USCH3` reader - User Sequence Number 3"]
pub struct USCH3_R(crate::FieldReader<u8, u8>);
impl USCH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH3` writer - User Sequence Number 3"]
pub struct USCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `USCH4` reader - User Sequence Number 4"]
pub struct USCH4_R(crate::FieldReader<u8, u8>);
impl USCH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH4` writer - User Sequence Number 4"]
pub struct USCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `USCH5` reader - User Sequence Number 5"]
pub struct USCH5_R(crate::FieldReader<u8, u8>);
impl USCH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH5` writer - User Sequence Number 5"]
pub struct USCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `USCH6` reader - User Sequence Number 6"]
pub struct USCH6_R(crate::FieldReader<u8, u8>);
impl USCH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH6` writer - User Sequence Number 6"]
pub struct USCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `USCH7` reader - User Sequence Number 7"]
pub struct USCH7_R(crate::FieldReader<u8, u8>);
impl USCH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH7` writer - User Sequence Number 7"]
pub struct USCH7_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - User Sequence Number 0"]
    #[inline(always)]
    pub fn usch0(&self) -> USCH0_R {
        USCH0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 1"]
    #[inline(always)]
    pub fn usch1(&self) -> USCH1_R {
        USCH1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 2"]
    #[inline(always)]
    pub fn usch2(&self) -> USCH2_R {
        USCH2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 3"]
    #[inline(always)]
    pub fn usch3(&self) -> USCH3_R {
        USCH3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - User Sequence Number 4"]
    #[inline(always)]
    pub fn usch4(&self) -> USCH4_R {
        USCH4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - User Sequence Number 5"]
    #[inline(always)]
    pub fn usch5(&self) -> USCH5_R {
        USCH5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - User Sequence Number 6"]
    #[inline(always)]
    pub fn usch6(&self) -> USCH6_R {
        USCH6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - User Sequence Number 7"]
    #[inline(always)]
    pub fn usch7(&self) -> USCH7_R {
        USCH7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Sequence Number 0"]
    #[inline(always)]
    pub fn usch0(&mut self) -> USCH0_W {
        USCH0_W { w: self }
    }
    #[doc = "Bits 4:7 - User Sequence Number 1"]
    #[inline(always)]
    pub fn usch1(&mut self) -> USCH1_W {
        USCH1_W { w: self }
    }
    #[doc = "Bits 8:11 - User Sequence Number 2"]
    #[inline(always)]
    pub fn usch2(&mut self) -> USCH2_W {
        USCH2_W { w: self }
    }
    #[doc = "Bits 12:15 - User Sequence Number 3"]
    #[inline(always)]
    pub fn usch3(&mut self) -> USCH3_W {
        USCH3_W { w: self }
    }
    #[doc = "Bits 16:19 - User Sequence Number 4"]
    #[inline(always)]
    pub fn usch4(&mut self) -> USCH4_W {
        USCH4_W { w: self }
    }
    #[doc = "Bits 20:23 - User Sequence Number 5"]
    #[inline(always)]
    pub fn usch5(&mut self) -> USCH5_W {
        USCH5_W { w: self }
    }
    #[doc = "Bits 24:27 - User Sequence Number 6"]
    #[inline(always)]
    pub fn usch6(&mut self) -> USCH6_W {
        USCH6_W { w: self }
    }
    #[doc = "Bits 28:31 - User Sequence Number 7"]
    #[inline(always)]
    pub fn usch7(&mut self) -> USCH7_W {
        USCH7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Sequence 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_seq1r](index.html) module"]
pub struct AFEC_SEQ1R_SPEC;
impl crate::RegisterSpec for AFEC_SEQ1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_seq1r::R](R) reader structure"]
impl crate::Readable for AFEC_SEQ1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_seq1r::W](W) writer structure"]
impl crate::Writable for AFEC_SEQ1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_SEQ1R to value 0"]
impl crate::Resettable for AFEC_SEQ1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
