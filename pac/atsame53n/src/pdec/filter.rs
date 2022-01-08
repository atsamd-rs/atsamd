#[doc = "Register `FILTER` reader"]
pub struct R(crate::R<FILTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTER` writer"]
pub struct W(crate::W<FILTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTER_SPEC>;
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
impl From<crate::W<FILTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER` reader - Filter Value"]
pub struct FILTER_R(crate::FieldReader<u8, u8>);
impl FILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER` writer - Filter Value"]
pub struct FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Filter Value"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter Value"]
    #[inline(always)]
    pub fn filter(&mut self) -> FILTER_W {
        FILTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter](index.html) module"]
pub struct FILTER_SPEC;
impl crate::RegisterSpec for FILTER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [filter::R](R) reader structure"]
impl crate::Readable for FILTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filter::W](W) writer structure"]
impl crate::Writable for FILTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILTER to value 0"]
impl crate::Resettable for FILTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
