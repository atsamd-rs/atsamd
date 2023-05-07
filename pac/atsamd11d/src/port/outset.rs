#[doc = "Register `OUTSET%s` reader"]
pub struct R(crate::R<OUTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTSET%s` writer"]
pub struct W(crate::W<OUTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTSET_SPEC>;
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
impl From<crate::W<OUTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTSET` reader - Port Data Output Value Set"]
pub type OUTSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OUTSET` writer - Port Data Output Value Set"]
pub type OUTSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTSET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value Set"]
    #[inline(always)]
    pub fn outset(&self) -> OUTSET_R {
        OUTSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value Set"]
    #[inline(always)]
    #[must_use]
    pub fn outset(&mut self) -> OUTSET_W<0> {
        OUTSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Output Value Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outset](index.html) module"]
pub struct OUTSET_SPEC;
impl crate::RegisterSpec for OUTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outset::R](R) reader structure"]
impl crate::Readable for OUTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outset::W](W) writer structure"]
impl crate::Writable for OUTSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTSET%s to value 0"]
impl crate::Resettable for OUTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
