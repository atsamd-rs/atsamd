#[doc = "Register `PERBUF_DITH5_MODE` reader"]
pub struct R(crate::R<PERBUF_DITH5_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERBUF_DITH5_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERBUF_DITH5_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERBUF_DITH5_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERBUF_DITH5_MODE` writer"]
pub struct W(crate::W<PERBUF_DITH5_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERBUF_DITH5_MODE_SPEC>;
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
impl From<crate::W<PERBUF_DITH5_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERBUF_DITH5_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DITHERBUF` reader - Dithering Buffer Cycle Number"]
pub struct DITHERBUF_R(crate::FieldReader<u8, u8>);
impl DITHERBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DITHERBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DITHERBUF_R {
    type Target = crate::FieldReader<u8, u8>;
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
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `PERBUF` reader - Period Buffer Value"]
pub struct PERBUF_R(crate::FieldReader<u32, u32>);
impl PERBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERBUF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERBUF` writer - Period Buffer Value"]
pub struct PERBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PERBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0007_ffff << 5)) | ((value as u32 & 0x0007_ffff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&self) -> DITHERBUF_R {
        DITHERBUF_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&self) -> PERBUF_R {
        PERBUF_R::new(((self.bits >> 5) & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&mut self) -> DITHERBUF_W {
        DITHERBUF_W { w: self }
    }
    #[doc = "Bits 5:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&mut self) -> PERBUF_W {
        PERBUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Period Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perbuf_dith5_mode](index.html) module"]
pub struct PERBUF_DITH5_MODE_SPEC;
impl crate::RegisterSpec for PERBUF_DITH5_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perbuf_dith5_mode::R](R) reader structure"]
impl crate::Readable for PERBUF_DITH5_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perbuf_dith5_mode::W](W) writer structure"]
impl crate::Writable for PERBUF_DITH5_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERBUF_DITH5_MODE to value 0xffff_ffff"]
impl crate::Resettable for PERBUF_DITH5_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
