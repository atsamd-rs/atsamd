#[doc = "Register `TI` reader"]
pub struct R(crate::R<TI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TI` writer"]
pub struct W(crate::W<TI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TI_SPEC>;
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
impl From<crate::W<TI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNS` reader - Count Nanoseconds"]
pub type CNS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNS` writer - Count Nanoseconds"]
pub type CNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TI_SPEC, u8, u8, 8, O>;
#[doc = "Field `ACNS` reader - Alternative Count Nanoseconds"]
pub type ACNS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACNS` writer - Alternative Count Nanoseconds"]
pub type ACNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TI_SPEC, u8, u8, 8, O>;
#[doc = "Field `NIT` reader - Number of Increments"]
pub type NIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NIT` writer - Number of Increments"]
pub type NIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TI_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Count Nanoseconds"]
    #[inline(always)]
    pub fn cns(&self) -> CNS_R {
        CNS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Alternative Count Nanoseconds"]
    #[inline(always)]
    pub fn acns(&self) -> ACNS_R {
        ACNS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of Increments"]
    #[inline(always)]
    pub fn nit(&self) -> NIT_R {
        NIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Count Nanoseconds"]
    #[inline(always)]
    #[must_use]
    pub fn cns(&mut self) -> CNS_W<0> {
        CNS_W::new(self)
    }
    #[doc = "Bits 8:15 - Alternative Count Nanoseconds"]
    #[inline(always)]
    #[must_use]
    pub fn acns(&mut self) -> ACNS_W<8> {
        ACNS_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of Increments"]
    #[inline(always)]
    #[must_use]
    pub fn nit(&mut self) -> NIT_W<16> {
        NIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Increment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ti](index.html) module"]
pub struct TI_SPEC;
impl crate::RegisterSpec for TI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ti::R](R) reader structure"]
impl crate::Readable for TI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ti::W](W) writer structure"]
impl crate::Writable for TI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TI to value 0"]
impl crate::Resettable for TI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
