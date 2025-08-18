#[doc = "Register `PER_DITH6_MODE` reader"]
pub struct R(crate::R<PER_DITH6_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PER_DITH6_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PER_DITH6_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PER_DITH6_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PER_DITH6_MODE` writer"]
pub struct W(crate::W<PER_DITH6_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PER_DITH6_MODE_SPEC>;
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
impl From<crate::W<PER_DITH6_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PER_DITH6_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DITHER` reader - Dithering Cycle Number"]
pub struct DITHER_R(crate::FieldReader<u8, u8>);
impl DITHER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DITHER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DITHER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DITHER` writer - Dithering Cycle Number"]
pub struct DITHER_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `PER` reader - Period Value"]
pub struct PER_R(crate::FieldReader<u32, u32>);
impl PER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER` writer - Period Value"]
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 6)) | ((value as u32 & 0x0003_ffff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:23 - Period Value"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 6) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dither(&mut self) -> DITHER_W {
        DITHER_W { w: self }
    }
    #[doc = "Bits 6:23 - Period Value"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_dith6_mode](index.html) module"]
pub struct PER_DITH6_MODE_SPEC;
impl crate::RegisterSpec for PER_DITH6_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [per_dith6_mode::R](R) reader structure"]
impl crate::Readable for PER_DITH6_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [per_dith6_mode::W](W) writer structure"]
impl crate::Writable for PER_DITH6_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PER_DITH6_MODE to value 0xffff_ffff"]
impl crate::Resettable for PER_DITH6_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
