#[doc = "Register `OUTTGL` reader"]
pub struct R(crate::R<OUTTGL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTTGL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTTGL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTTGL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTTGL` writer"]
pub struct W(crate::W<OUTTGL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTTGL_SPEC>;
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
impl From<crate::W<OUTTGL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTTGL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTTGL` reader - PORT Data Output Value Toggle"]
pub struct OUTTGL_R(crate::FieldReader<u32, u32>);
impl OUTTGL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OUTTGL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTTGL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTTGL` writer - PORT Data Output Value Toggle"]
pub struct OUTTGL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PORT Data Output Value Toggle"]
    #[inline(always)]
    pub fn outtgl(&self) -> OUTTGL_R {
        OUTTGL_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PORT Data Output Value Toggle"]
    #[inline(always)]
    pub fn outtgl(&mut self) -> OUTTGL_W {
        OUTTGL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Output Value Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outtgl](index.html) module"]
pub struct OUTTGL_SPEC;
impl crate::RegisterSpec for OUTTGL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outtgl::R](R) reader structure"]
impl crate::Readable for OUTTGL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outtgl::W](W) writer structure"]
impl crate::Writable for OUTTGL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTTGL to value 0"]
impl crate::Resettable for OUTTGL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
