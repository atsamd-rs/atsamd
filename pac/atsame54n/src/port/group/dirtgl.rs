#[doc = "Register `DIRTGL` reader"]
pub struct R(crate::R<DIRTGL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRTGL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRTGL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRTGL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIRTGL` writer"]
pub struct W(crate::W<DIRTGL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRTGL_SPEC>;
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
impl From<crate::W<DIRTGL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRTGL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRTGL` reader - Port Data Direction Toggle"]
pub struct DIRTGL_R(crate::FieldReader<u32, u32>);
impl DIRTGL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DIRTGL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRTGL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRTGL` writer - Port Data Direction Toggle"]
pub struct DIRTGL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Direction Toggle"]
    #[inline(always)]
    pub fn dirtgl(&self) -> DIRTGL_R {
        DIRTGL_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction Toggle"]
    #[inline(always)]
    pub fn dirtgl(&mut self) -> DIRTGL_W {
        DIRTGL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Direction Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirtgl](index.html) module"]
pub struct DIRTGL_SPEC;
impl crate::RegisterSpec for DIRTGL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dirtgl::R](R) reader structure"]
impl crate::Readable for DIRTGL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dirtgl::W](W) writer structure"]
impl crate::Writable for DIRTGL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIRTGL to value 0"]
impl crate::Resettable for DIRTGL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
