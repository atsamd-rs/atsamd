#[doc = "Register `WEXCTRL` reader"]
pub struct R(crate::R<WEXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WEXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WEXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WEXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WEXCTRL` writer"]
pub struct W(crate::W<WEXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WEXCTRL_SPEC>;
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
impl From<crate::W<WEXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WEXCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTMX` reader - Output Matrix"]
pub struct OTMX_R(crate::FieldReader<u8, u8>);
impl OTMX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTMX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTMX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTMX` writer - Output Matrix"]
pub struct OTMX_W<'a> {
    w: &'a mut W,
}
impl<'a> OTMX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DTIEN0` reader - Dead-time Insertion Generator 0 Enable"]
pub struct DTIEN0_R(crate::FieldReader<bool, bool>);
impl DTIEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTIEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTIEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTIEN0` writer - Dead-time Insertion Generator 0 Enable"]
pub struct DTIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIEN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DTIEN1` reader - Dead-time Insertion Generator 1 Enable"]
pub struct DTIEN1_R(crate::FieldReader<bool, bool>);
impl DTIEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTIEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTIEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTIEN1` writer - Dead-time Insertion Generator 1 Enable"]
pub struct DTIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIEN1_W<'a> {
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
#[doc = "Field `DTIEN2` reader - Dead-time Insertion Generator 2 Enable"]
pub struct DTIEN2_R(crate::FieldReader<bool, bool>);
impl DTIEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTIEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTIEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTIEN2` writer - Dead-time Insertion Generator 2 Enable"]
pub struct DTIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIEN2_W<'a> {
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
#[doc = "Field `DTIEN3` reader - Dead-time Insertion Generator 3 Enable"]
pub struct DTIEN3_R(crate::FieldReader<bool, bool>);
impl DTIEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTIEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTIEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTIEN3` writer - Dead-time Insertion Generator 3 Enable"]
pub struct DTIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIEN3_W<'a> {
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
#[doc = "Field `DTLS` reader - Dead-time Low Side Outputs Value"]
pub struct DTLS_R(crate::FieldReader<u8, u8>);
impl DTLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTLS` writer - Dead-time Low Side Outputs Value"]
pub struct DTLS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DTHS` reader - Dead-time High Side Outputs Value"]
pub struct DTHS_R(crate::FieldReader<u8, u8>);
impl DTHS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTHS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTHS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTHS` writer - Dead-time High Side Outputs Value"]
pub struct DTHS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Output Matrix"]
    #[inline(always)]
    pub fn otmx(&self) -> OTMX_R {
        OTMX_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - Dead-time Insertion Generator 0 Enable"]
    #[inline(always)]
    pub fn dtien0(&self) -> DTIEN0_R {
        DTIEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Dead-time Insertion Generator 1 Enable"]
    #[inline(always)]
    pub fn dtien1(&self) -> DTIEN1_R {
        DTIEN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Dead-time Insertion Generator 2 Enable"]
    #[inline(always)]
    pub fn dtien2(&self) -> DTIEN2_R {
        DTIEN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Dead-time Insertion Generator 3 Enable"]
    #[inline(always)]
    pub fn dtien3(&self) -> DTIEN3_R {
        DTIEN3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Dead-time Low Side Outputs Value"]
    #[inline(always)]
    pub fn dtls(&self) -> DTLS_R {
        DTLS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Dead-time High Side Outputs Value"]
    #[inline(always)]
    pub fn dths(&self) -> DTHS_R {
        DTHS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output Matrix"]
    #[inline(always)]
    pub fn otmx(&mut self) -> OTMX_W {
        OTMX_W { w: self }
    }
    #[doc = "Bit 8 - Dead-time Insertion Generator 0 Enable"]
    #[inline(always)]
    pub fn dtien0(&mut self) -> DTIEN0_W {
        DTIEN0_W { w: self }
    }
    #[doc = "Bit 9 - Dead-time Insertion Generator 1 Enable"]
    #[inline(always)]
    pub fn dtien1(&mut self) -> DTIEN1_W {
        DTIEN1_W { w: self }
    }
    #[doc = "Bit 10 - Dead-time Insertion Generator 2 Enable"]
    #[inline(always)]
    pub fn dtien2(&mut self) -> DTIEN2_W {
        DTIEN2_W { w: self }
    }
    #[doc = "Bit 11 - Dead-time Insertion Generator 3 Enable"]
    #[inline(always)]
    pub fn dtien3(&mut self) -> DTIEN3_W {
        DTIEN3_W { w: self }
    }
    #[doc = "Bits 16:23 - Dead-time Low Side Outputs Value"]
    #[inline(always)]
    pub fn dtls(&mut self) -> DTLS_W {
        DTLS_W { w: self }
    }
    #[doc = "Bits 24:31 - Dead-time High Side Outputs Value"]
    #[inline(always)]
    pub fn dths(&mut self) -> DTHS_W {
        DTHS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Waveform Extension Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wexctrl](index.html) module"]
pub struct WEXCTRL_SPEC;
impl crate::RegisterSpec for WEXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wexctrl::R](R) reader structure"]
impl crate::Readable for WEXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wexctrl::W](W) writer structure"]
impl crate::Writable for WEXCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WEXCTRL to value 0"]
impl crate::Resettable for WEXCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
