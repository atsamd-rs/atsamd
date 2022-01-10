#[doc = "Register `TN` reader"]
pub struct R(crate::R<TN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TN` writer"]
pub struct W(crate::W<TN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TN_SPEC>;
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
impl From<crate::W<TN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TNS` reader - Timer Count in Nanoseconds"]
pub struct TNS_R(crate::FieldReader<u32, u32>);
impl TNS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TNS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TNS` writer - Timer Count in Nanoseconds"]
pub struct TNS_W<'a> {
    w: &'a mut W,
}
impl<'a> TNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Timer Count in Nanoseconds"]
    #[inline(always)]
    pub fn tns(&self) -> TNS_R {
        TNS_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer Count in Nanoseconds"]
    #[inline(always)]
    pub fn tns(&mut self) -> TNS_W {
        TNS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tn](index.html) module"]
pub struct TN_SPEC;
impl crate::RegisterSpec for TN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tn::R](R) reader structure"]
impl crate::Readable for TN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tn::W](W) writer structure"]
impl crate::Writable for TN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TN to value 0"]
impl crate::Resettable for TN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
