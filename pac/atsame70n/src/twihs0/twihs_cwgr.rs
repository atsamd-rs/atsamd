#[doc = "Register `TWIHS_CWGR` reader"]
pub struct R(crate::R<TWIHS_CWGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWIHS_CWGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWIHS_CWGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWIHS_CWGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWIHS_CWGR` writer"]
pub struct W(crate::W<TWIHS_CWGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWIHS_CWGR_SPEC>;
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
impl From<crate::W<TWIHS_CWGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWIHS_CWGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLDIV` reader - Clock Low Divider"]
pub struct CLDIV_R(crate::FieldReader<u8, u8>);
impl CLDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLDIV` writer - Clock Low Divider"]
pub struct CLDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CHDIV` reader - Clock High Divider"]
pub struct CHDIV_R(crate::FieldReader<u8, u8>);
impl CHDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHDIV` writer - Clock High Divider"]
pub struct CHDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CKDIV` reader - Clock Divider"]
pub struct CKDIV_R(crate::FieldReader<u8, u8>);
impl CKDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKDIV` writer - Clock Divider"]
pub struct CKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `HOLD` reader - TWD Hold Time Versus TWCK Falling"]
pub struct HOLD_R(crate::FieldReader<u8, u8>);
impl HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOLD` writer - TWD Hold Time Versus TWCK Falling"]
pub struct HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> HOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock Low Divider"]
    #[inline(always)]
    pub fn cldiv(&self) -> CLDIV_R {
        CLDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock High Divider"]
    #[inline(always)]
    pub fn chdiv(&self) -> CHDIV_R {
        CHDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Clock Divider"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 24:29 - TWD Hold Time Versus TWCK Falling"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Low Divider"]
    #[inline(always)]
    pub fn cldiv(&mut self) -> CLDIV_W {
        CLDIV_W { w: self }
    }
    #[doc = "Bits 8:15 - Clock High Divider"]
    #[inline(always)]
    pub fn chdiv(&mut self) -> CHDIV_W {
        CHDIV_W { w: self }
    }
    #[doc = "Bits 16:18 - Clock Divider"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CKDIV_W {
        CKDIV_W { w: self }
    }
    #[doc = "Bits 24:29 - TWD Hold Time Versus TWCK Falling"]
    #[inline(always)]
    pub fn hold(&mut self) -> HOLD_W {
        HOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Waveform Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_cwgr](index.html) module"]
pub struct TWIHS_CWGR_SPEC;
impl crate::RegisterSpec for TWIHS_CWGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twihs_cwgr::R](R) reader structure"]
impl crate::Readable for TWIHS_CWGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twihs_cwgr::W](W) writer structure"]
impl crate::Writable for TWIHS_CWGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWIHS_CWGR to value 0"]
impl crate::Resettable for TWIHS_CWGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
