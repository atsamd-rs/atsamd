#[doc = "Register `US_ICDIFF` reader"]
pub struct R(crate::R<US_ICDIFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_ICDIFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_ICDIFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_ICDIFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_ICDIFF` writer"]
pub struct W(crate::W<US_ICDIFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_ICDIFF_SPEC>;
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
impl From<crate::W<US_ICDIFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_ICDIFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICDIFF` reader - IC Differentiator Number"]
pub struct ICDIFF_R(crate::FieldReader<u8, u8>);
impl ICDIFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ICDIFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICDIFF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICDIFF` writer - IC Differentiator Number"]
pub struct ICDIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ICDIFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - IC Differentiator Number"]
    #[inline(always)]
    pub fn icdiff(&self) -> ICDIFF_R {
        ICDIFF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IC Differentiator Number"]
    #[inline(always)]
    pub fn icdiff(&mut self) -> ICDIFF_W {
        ICDIFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IC DIFF Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_icdiff](index.html) module"]
pub struct US_ICDIFF_SPEC;
impl crate::RegisterSpec for US_ICDIFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_icdiff::R](R) reader structure"]
impl crate::Readable for US_ICDIFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_icdiff::W](W) writer structure"]
impl crate::Writable for US_ICDIFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_ICDIFF to value 0"]
impl crate::Resettable for US_ICDIFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
