#[doc = "Register `CC%s_DITH6` reader"]
pub struct R(crate::R<CC_DITH6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC_DITH6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC_DITH6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC_DITH6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC%s_DITH6` writer"]
pub struct W(crate::W<CC_DITH6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC_DITH6_SPEC>;
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
impl From<crate::W<CC_DITH6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC_DITH6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DITHER` reader - Dithering Cycle Number"]
pub type DITHER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DITHER` writer - Dithering Cycle Number"]
pub type DITHER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC_DITH6_SPEC, u8, u8, 6, O>;
#[doc = "Field `CC` reader - Channel Compare/Capture Value"]
pub type CC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CC` writer - Channel Compare/Capture Value"]
pub type CC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC_DITH6_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:5 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:23 - Channel Compare/Capture Value"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new((self.bits >> 6) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dithering Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DITHER_W<0> {
        DITHER_W::new(self)
    }
    #[doc = "Bits 6:23 - Channel Compare/Capture Value"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<6> {
        CC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare and Capture\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_dith6](index.html) module"]
pub struct CC_DITH6_SPEC;
impl crate::RegisterSpec for CC_DITH6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc_dith6::R](R) reader structure"]
impl crate::Readable for CC_DITH6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc_dith6::W](W) writer structure"]
impl crate::Writable for CC_DITH6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC%s_DITH6 to value 0"]
impl crate::Resettable for CC_DITH6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
