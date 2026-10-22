#[doc = "Register `BAUD` reader"]
pub struct R(crate::R<BAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD` writer"]
pub struct W(crate::W<BAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD_SPEC>;
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
impl From<crate::W<BAUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUD` reader - Baud Rate Value"]
pub struct BAUD_R(crate::FieldReader<u8, u8>);
impl BAUD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BAUD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAUD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAUD` writer - Baud Rate Value"]
pub struct BAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `BAUDLOW` reader - Baud Rate Value Low"]
pub struct BAUDLOW_R(crate::FieldReader<u8, u8>);
impl BAUDLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BAUDLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAUDLOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAUDLOW` writer - Baud Rate Value Low"]
pub struct BAUDLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUDLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `HSBAUD` reader - High Speed Baud Rate Value"]
pub struct HSBAUD_R(crate::FieldReader<u8, u8>);
impl HSBAUD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSBAUD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSBAUD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSBAUD` writer - High Speed Baud Rate Value"]
pub struct HSBAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> HSBAUD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `HSBAUDLOW` reader - High Speed Baud Rate Value Low"]
pub struct HSBAUDLOW_R(crate::FieldReader<u8, u8>);
impl HSBAUDLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSBAUDLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSBAUDLOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSBAUDLOW` writer - High Speed Baud Rate Value Low"]
pub struct HSBAUDLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSBAUDLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Baud Rate Value Low"]
    #[inline(always)]
    pub fn baudlow(&self) -> BAUDLOW_R {
        BAUDLOW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - High Speed Baud Rate Value"]
    #[inline(always)]
    pub fn hsbaud(&self) -> HSBAUD_R {
        HSBAUD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - High Speed Baud Rate Value Low"]
    #[inline(always)]
    pub fn hsbaudlow(&self) -> HSBAUDLOW_R {
        HSBAUDLOW_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&mut self) -> BAUD_W {
        BAUD_W { w: self }
    }
    #[doc = "Bits 8:15 - Baud Rate Value Low"]
    #[inline(always)]
    pub fn baudlow(&mut self) -> BAUDLOW_W {
        BAUDLOW_W { w: self }
    }
    #[doc = "Bits 16:23 - High Speed Baud Rate Value"]
    #[inline(always)]
    pub fn hsbaud(&mut self) -> HSBAUD_W {
        HSBAUD_W { w: self }
    }
    #[doc = "Bits 24:31 - High Speed Baud Rate Value Low"]
    #[inline(always)]
    pub fn hsbaudlow(&mut self) -> HSBAUDLOW_W {
        HSBAUDLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CM Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud](index.html) module"]
pub struct BAUD_SPEC;
impl crate::RegisterSpec for BAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baud::R](R) reader structure"]
impl crate::Readable for BAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud::W](W) writer structure"]
impl crate::Writable for BAUD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUD to value 0"]
impl crate::Resettable for BAUD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
