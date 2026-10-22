#[doc = "Register `CFGA` reader"]
pub struct R(crate::R<CFGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGA` writer"]
pub struct W(crate::W<CFGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGA_SPEC>;
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
impl From<crate::W<CFGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFNUM` reader - Number of Reference Clock Cycles"]
pub type REFNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFNUM` writer - Number of Reference Clock Cycles"]
pub type REFNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CFGA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    pub fn refnum(&self) -> REFNUM_R {
        REFNUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn refnum(&mut self) -> REFNUM_W<0> {
        REFNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Config A register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfga](index.html) module"]
pub struct CFGA_SPEC;
impl crate::RegisterSpec for CFGA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cfga::R](R) reader structure"]
impl crate::Readable for CFGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfga::W](W) writer structure"]
impl crate::Writable for CFGA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGA to value 0"]
impl crate::Resettable for CFGA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
