#[doc = "Register `BDPR` reader"]
pub struct R(crate::R<BDPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDPR` writer"]
pub struct W(crate::W<BDPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDPR_SPEC>;
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
impl From<crate::W<BDPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFDATA` reader - Buffer Data"]
pub type BUFDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BUFDATA` writer - Buffer Data"]
pub type BUFDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdata(&self) -> BUFDATA_R {
        BUFDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    #[must_use]
    pub fn bufdata(&mut self) -> BUFDATA_W<0> {
        BUFDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer Data Port\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdpr](index.html) module"]
pub struct BDPR_SPEC;
impl crate::RegisterSpec for BDPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdpr::R](R) reader structure"]
impl crate::Readable for BDPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdpr::W](W) writer structure"]
impl crate::Writable for BDPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDPR to value 0"]
impl crate::Resettable for BDPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
