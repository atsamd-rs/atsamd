#[doc = "Register `CCBUF_DITH4_MODE[%s]` reader"]
pub struct R(crate::R<CCBUF_DITH4_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCBUF_DITH4_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCBUF_DITH4_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCBUF_DITH4_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCBUF_DITH4_MODE[%s]` writer"]
pub struct W(crate::W<CCBUF_DITH4_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCBUF_DITH4_MODE_SPEC>;
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
impl From<crate::W<CCBUF_DITH4_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCBUF_DITH4_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCBUF` reader - Channel Compare/Capture Buffer Value"]
pub struct CCBUF_R(crate::FieldReader<u8, u8>);
impl CCBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBUF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBUF` writer - Channel Compare/Capture Buffer Value"]
pub struct CCBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `DITHERBUF` reader - Dithering Buffer Cycle Number"]
pub struct DITHERBUF_R(crate::FieldReader<u32, u32>);
impl DITHERBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DITHERBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DITHERBUF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DITHERBUF` writer - Dithering Buffer Cycle Number"]
pub struct DITHERBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHERBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 4)) | ((value as u32 & 0x000f_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&self) -> CCBUF_R {
        CCBUF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&self) -> DITHERBUF_R {
        DITHERBUF_R::new(((self.bits >> 4) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&mut self) -> CCBUF_W {
        CCBUF_W { w: self }
    }
    #[doc = "Bits 4:23 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&mut self) -> DITHERBUF_W {
        DITHERBUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccbuf_dith4_mode](index.html) module"]
pub struct CCBUF_DITH4_MODE_SPEC;
impl crate::RegisterSpec for CCBUF_DITH4_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccbuf_dith4_mode::R](R) reader structure"]
impl crate::Readable for CCBUF_DITH4_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccbuf_dith4_mode::W](W) writer structure"]
impl crate::Writable for CCBUF_DITH4_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCBUF_DITH4_MODE[%s]
to value 0"]
impl crate::Resettable for CCBUF_DITH4_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
