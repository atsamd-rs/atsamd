#[doc = "Register `OUTCLR%s` reader"]
pub struct R(crate::R<OUTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTCLR%s` writer"]
pub struct W(crate::W<OUTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTCLR_SPEC>;
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
impl From<crate::W<OUTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTCLR` reader - Port Data Output Value Clear"]
pub type OUTCLR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OUTCLR` writer - Port Data Output Value Clear"]
pub type OUTCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTCLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value Clear"]
    #[inline(always)]
    pub fn outclr(&self) -> OUTCLR_R {
        OUTCLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value Clear"]
    #[inline(always)]
    #[must_use]
    pub fn outclr(&mut self) -> OUTCLR_W<0> {
        OUTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Output Value Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outclr](index.html) module"]
pub struct OUTCLR_SPEC;
impl crate::RegisterSpec for OUTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outclr::R](R) reader structure"]
impl crate::Readable for OUTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outclr::W](W) writer structure"]
impl crate::Writable for OUTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTCLR%s to value 0"]
impl crate::Resettable for OUTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
