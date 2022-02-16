#[doc = "Register `SSC_RC1R` reader"]
pub struct R(crate::R<SSC_RC1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSC_RC1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSC_RC1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSC_RC1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSC_RC1R` writer"]
pub struct W(crate::W<SSC_RC1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSC_RC1R_SPEC>;
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
impl From<crate::W<SSC_RC1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSC_RC1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CP1` reader - Receive Compare Data 1"]
pub struct CP1_R(crate::FieldReader<u16, u16>);
impl CP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CP1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CP1` writer - Receive Compare Data 1"]
pub struct CP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Compare Data 1"]
    #[inline(always)]
    pub fn cp1(&self) -> CP1_R {
        CP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Compare Data 1"]
    #[inline(always)]
    pub fn cp1(&mut self) -> CP1_W {
        CP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Compare 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_rc1r](index.html) module"]
pub struct SSC_RC1R_SPEC;
impl crate::RegisterSpec for SSC_RC1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssc_rc1r::R](R) reader structure"]
impl crate::Readable for SSC_RC1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssc_rc1r::W](W) writer structure"]
impl crate::Writable for SSC_RC1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSC_RC1R to value 0"]
impl crate::Resettable for SSC_RC1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
