#[doc = "Register `US_LONPR` reader"]
pub struct R(crate::R<US_LONPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_LONPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_LONPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_LONPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_LONPR` writer"]
pub struct W(crate::W<US_LONPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_LONPR_SPEC>;
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
impl From<crate::W<US_LONPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_LONPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LONPL` reader - LON Preamble Length"]
pub struct LONPL_R(crate::FieldReader<u16, u16>);
impl LONPL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LONPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LONPL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LONPL` writer - LON Preamble Length"]
pub struct LONPL_W<'a> {
    w: &'a mut W,
}
impl<'a> LONPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - LON Preamble Length"]
    #[inline(always)]
    pub fn lonpl(&self) -> LONPL_R {
        LONPL_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - LON Preamble Length"]
    #[inline(always)]
    pub fn lonpl(&mut self) -> LONPL_W {
        LONPL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LON Preamble Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_lonpr](index.html) module"]
pub struct US_LONPR_SPEC;
impl crate::RegisterSpec for US_LONPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_lonpr::R](R) reader structure"]
impl crate::Readable for US_LONPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_lonpr::W](W) writer structure"]
impl crate::Writable for US_LONPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_LONPR to value 0"]
impl crate::Resettable for US_LONPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
