#[doc = "Register `AFEC_CGR` reader"]
pub struct R(crate::R<AFEC_CGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_CGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_CGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_CGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_CGR` writer"]
pub struct W(crate::W<AFEC_CGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_CGR_SPEC>;
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
impl From<crate::W<AFEC_CGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_CGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAIN0` reader - Gain for Channel 0"]
pub struct GAIN0_R(crate::FieldReader<u8, u8>);
impl GAIN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN0` writer - Gain for Channel 0"]
pub struct GAIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `GAIN1` reader - Gain for Channel 1"]
pub struct GAIN1_R(crate::FieldReader<u8, u8>);
impl GAIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN1` writer - Gain for Channel 1"]
pub struct GAIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `GAIN2` reader - Gain for Channel 2"]
pub struct GAIN2_R(crate::FieldReader<u8, u8>);
impl GAIN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN2` writer - Gain for Channel 2"]
pub struct GAIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `GAIN3` reader - Gain for Channel 3"]
pub struct GAIN3_R(crate::FieldReader<u8, u8>);
impl GAIN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN3` writer - Gain for Channel 3"]
pub struct GAIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `GAIN4` reader - Gain for Channel 4"]
pub struct GAIN4_R(crate::FieldReader<u8, u8>);
impl GAIN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN4` writer - Gain for Channel 4"]
pub struct GAIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `GAIN5` reader - Gain for Channel 5"]
pub struct GAIN5_R(crate::FieldReader<u8, u8>);
impl GAIN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN5` writer - Gain for Channel 5"]
pub struct GAIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `GAIN6` reader - Gain for Channel 6"]
pub struct GAIN6_R(crate::FieldReader<u8, u8>);
impl GAIN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN6` writer - Gain for Channel 6"]
pub struct GAIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `GAIN7` reader - Gain for Channel 7"]
pub struct GAIN7_R(crate::FieldReader<u8, u8>);
impl GAIN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN7` writer - Gain for Channel 7"]
pub struct GAIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `GAIN8` reader - Gain for Channel 8"]
pub struct GAIN8_R(crate::FieldReader<u8, u8>);
impl GAIN8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN8` writer - Gain for Channel 8"]
pub struct GAIN8_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `GAIN9` reader - Gain for Channel 9"]
pub struct GAIN9_R(crate::FieldReader<u8, u8>);
impl GAIN9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN9` writer - Gain for Channel 9"]
pub struct GAIN9_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `GAIN10` reader - Gain for Channel 10"]
pub struct GAIN10_R(crate::FieldReader<u8, u8>);
impl GAIN10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN10` writer - Gain for Channel 10"]
pub struct GAIN10_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `GAIN11` reader - Gain for Channel 11"]
pub struct GAIN11_R(crate::FieldReader<u8, u8>);
impl GAIN11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN11` writer - Gain for Channel 11"]
pub struct GAIN11_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Gain for Channel 0"]
    #[inline(always)]
    pub fn gain0(&self) -> GAIN0_R {
        GAIN0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Gain for Channel 1"]
    #[inline(always)]
    pub fn gain1(&self) -> GAIN1_R {
        GAIN1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Gain for Channel 2"]
    #[inline(always)]
    pub fn gain2(&self) -> GAIN2_R {
        GAIN2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Gain for Channel 3"]
    #[inline(always)]
    pub fn gain3(&self) -> GAIN3_R {
        GAIN3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Gain for Channel 4"]
    #[inline(always)]
    pub fn gain4(&self) -> GAIN4_R {
        GAIN4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Gain for Channel 5"]
    #[inline(always)]
    pub fn gain5(&self) -> GAIN5_R {
        GAIN5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Gain for Channel 6"]
    #[inline(always)]
    pub fn gain6(&self) -> GAIN6_R {
        GAIN6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Gain for Channel 7"]
    #[inline(always)]
    pub fn gain7(&self) -> GAIN7_R {
        GAIN7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Gain for Channel 8"]
    #[inline(always)]
    pub fn gain8(&self) -> GAIN8_R {
        GAIN8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Gain for Channel 9"]
    #[inline(always)]
    pub fn gain9(&self) -> GAIN9_R {
        GAIN9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Gain for Channel 10"]
    #[inline(always)]
    pub fn gain10(&self) -> GAIN10_R {
        GAIN10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Gain for Channel 11"]
    #[inline(always)]
    pub fn gain11(&self) -> GAIN11_R {
        GAIN11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Gain for Channel 0"]
    #[inline(always)]
    pub fn gain0(&mut self) -> GAIN0_W {
        GAIN0_W { w: self }
    }
    #[doc = "Bits 2:3 - Gain for Channel 1"]
    #[inline(always)]
    pub fn gain1(&mut self) -> GAIN1_W {
        GAIN1_W { w: self }
    }
    #[doc = "Bits 4:5 - Gain for Channel 2"]
    #[inline(always)]
    pub fn gain2(&mut self) -> GAIN2_W {
        GAIN2_W { w: self }
    }
    #[doc = "Bits 6:7 - Gain for Channel 3"]
    #[inline(always)]
    pub fn gain3(&mut self) -> GAIN3_W {
        GAIN3_W { w: self }
    }
    #[doc = "Bits 8:9 - Gain for Channel 4"]
    #[inline(always)]
    pub fn gain4(&mut self) -> GAIN4_W {
        GAIN4_W { w: self }
    }
    #[doc = "Bits 10:11 - Gain for Channel 5"]
    #[inline(always)]
    pub fn gain5(&mut self) -> GAIN5_W {
        GAIN5_W { w: self }
    }
    #[doc = "Bits 12:13 - Gain for Channel 6"]
    #[inline(always)]
    pub fn gain6(&mut self) -> GAIN6_W {
        GAIN6_W { w: self }
    }
    #[doc = "Bits 14:15 - Gain for Channel 7"]
    #[inline(always)]
    pub fn gain7(&mut self) -> GAIN7_W {
        GAIN7_W { w: self }
    }
    #[doc = "Bits 16:17 - Gain for Channel 8"]
    #[inline(always)]
    pub fn gain8(&mut self) -> GAIN8_W {
        GAIN8_W { w: self }
    }
    #[doc = "Bits 18:19 - Gain for Channel 9"]
    #[inline(always)]
    pub fn gain9(&mut self) -> GAIN9_W {
        GAIN9_W { w: self }
    }
    #[doc = "Bits 20:21 - Gain for Channel 10"]
    #[inline(always)]
    pub fn gain10(&mut self) -> GAIN10_W {
        GAIN10_W { w: self }
    }
    #[doc = "Bits 22:23 - Gain for Channel 11"]
    #[inline(always)]
    pub fn gain11(&mut self) -> GAIN11_W {
        GAIN11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cgr](index.html) module"]
pub struct AFEC_CGR_SPEC;
impl crate::RegisterSpec for AFEC_CGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_cgr::R](R) reader structure"]
impl crate::Readable for AFEC_CGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_cgr::W](W) writer structure"]
impl crate::Writable for AFEC_CGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_CGR to value 0"]
impl crate::Resettable for AFEC_CGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
