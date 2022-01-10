#[doc = "Register `OUT%s` reader"]
pub struct R(crate::R<OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT%s` writer"]
pub struct W(crate::W<OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_SPEC>;
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
impl From<crate::W<OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT` reader - Port Data Output Value"]
pub struct OUT_R(crate::FieldReader<u32, u32>);
impl OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT` writer - Port Data Output Value"]
pub struct OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value"]
    #[inline(always)]
    pub fn out(&mut self) -> OUT_W {
        OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Output Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out](index.html) module"]
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out::R](R) reader structure"]
impl crate::Readable for OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out::W](W) writer structure"]
impl crate::Writable for OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT%s to value 0"]
impl crate::Resettable for OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
