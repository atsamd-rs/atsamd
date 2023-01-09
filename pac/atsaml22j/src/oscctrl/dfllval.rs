#[doc = "Register `DFLLVAL` reader"]
pub struct R(crate::R<DFLLVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLVAL` writer"]
pub struct W(crate::W<DFLLVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLVAL_SPEC>;
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
impl From<crate::W<DFLLVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINE` reader - Fine Value"]
pub type FINE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FINE` writer - Fine Value"]
pub type FINE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFLLVAL_SPEC, u16, u16, 10, O>;
#[doc = "Field `COARSE` reader - Coarse Value"]
pub type COARSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COARSE` writer - Coarse Value"]
pub type COARSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFLLVAL_SPEC, u8, u8, 6, O>;
#[doc = "Field `DIFF` reader - Multiplication Ratio Difference"]
pub type DIFF_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Fine Value"]
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Coarse Value"]
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Multiplication Ratio Difference"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Fine Value"]
    #[inline(always)]
    #[must_use]
    pub fn fine(&mut self) -> FINE_W<0> {
        FINE_W::new(self)
    }
    #[doc = "Bits 10:15 - Coarse Value"]
    #[inline(always)]
    #[must_use]
    pub fn coarse(&mut self) -> COARSE_W<10> {
        COARSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL48M Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllval](index.html) module"]
pub struct DFLLVAL_SPEC;
impl crate::RegisterSpec for DFLLVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfllval::R](R) reader structure"]
impl crate::Readable for DFLLVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllval::W](W) writer structure"]
impl crate::Writable for DFLLVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLLVAL to value 0"]
impl crate::Resettable for DFLLVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
