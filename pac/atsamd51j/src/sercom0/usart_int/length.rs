#[doc = "Register `LENGTH` reader"]
pub struct R(crate::R<LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LENGTH` writer"]
pub struct W(crate::W<LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LENGTH_SPEC>;
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
impl From<crate::W<LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN` reader - Data Length"]
pub struct LEN_R(crate::FieldReader<u8, u8>);
impl LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEN` writer - Data Length"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `LENEN` reader - Data Length Enable"]
pub struct LENEN_R(crate::FieldReader<u8, u8>);
impl LENEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LENEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LENEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LENEN` writer - Data Length Enable"]
pub struct LENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LENEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u16 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Data Length Enable"]
    #[inline(always)]
    pub fn lenen(&self) -> LENEN_R {
        LENEN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Data Length Enable"]
    #[inline(always)]
    pub fn lenen(&mut self) -> LENEN_W {
        LENEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_INT Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [length](index.html) module"]
pub struct LENGTH_SPEC;
impl crate::RegisterSpec for LENGTH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [length::R](R) reader structure"]
impl crate::Readable for LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [length::W](W) writer structure"]
impl crate::Writable for LENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LENGTH to value 0"]
impl crate::Resettable for LENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
