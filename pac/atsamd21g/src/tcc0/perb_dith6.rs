#[doc = "Register `PERB_DITH6` reader"]
pub struct R(crate::R<PERB_DITH6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERB_DITH6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERB_DITH6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERB_DITH6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERB_DITH6` writer"]
pub struct W(crate::W<PERB_DITH6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERB_DITH6_SPEC>;
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
impl From<crate::W<PERB_DITH6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERB_DITH6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DITHERCYB` reader - Dithering Buffer Cycle Number"]
pub struct DITHERCYB_R(crate::FieldReader<u8, u8>);
impl DITHERCYB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DITHERCYB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DITHERCYB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DITHERCYB` writer - Dithering Buffer Cycle Number"]
pub struct DITHERCYB_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHERCYB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `PERB` reader - Period Buffer Value"]
pub struct PERB_R(crate::FieldReader<u32, u32>);
impl PERB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERB` writer - Period Buffer Value"]
pub struct PERB_W<'a> {
    w: &'a mut W,
}
impl<'a> PERB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 6)) | ((value as u32 & 0x0003_ffff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&self) -> DITHERCYB_R {
        DITHERCYB_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new(((self.bits >> 6) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&mut self) -> DITHERCYB_W {
        DITHERCYB_W { w: self }
    }
    #[doc = "Bits 6:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&mut self) -> PERB_W {
        PERB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Period Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perb_dith6](index.html) module"]
pub struct PERB_DITH6_SPEC;
impl crate::RegisterSpec for PERB_DITH6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perb_dith6::R](R) reader structure"]
impl crate::Readable for PERB_DITH6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perb_dith6::W](W) writer structure"]
impl crate::Writable for PERB_DITH6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERB_DITH6 to value 0xffff_ffff"]
impl crate::Resettable for PERB_DITH6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
