#[doc = "Register `DESCADD` reader"]
pub struct R(crate::R<DESCADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESCADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESCADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESCADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESCADD` writer"]
pub struct W(crate::W<DESCADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESCADD_SPEC>;
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
impl From<crate::W<DESCADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESCADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DESCADD` reader - Descriptor Address Value"]
pub struct DESCADD_R(crate::FieldReader<u32, u32>);
impl DESCADD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DESCADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESCADD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESCADD` writer - Descriptor Address Value"]
pub struct DESCADD_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Descriptor Address Value"]
    #[inline(always)]
    pub fn descadd(&self) -> DESCADD_R {
        DESCADD_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor Address Value"]
    #[inline(always)]
    pub fn descadd(&mut self) -> DESCADD_W {
        DESCADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descadd](index.html) module"]
pub struct DESCADD_SPEC;
impl crate::RegisterSpec for DESCADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [descadd::R](R) reader structure"]
impl crate::Readable for DESCADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [descadd::W](W) writer structure"]
impl crate::Writable for DESCADD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DESCADD to value 0"]
impl crate::Resettable for DESCADD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
