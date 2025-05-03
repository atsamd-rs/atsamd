#[doc = "Register `CRCDATAIN` reader"]
pub struct R(crate::R<CRCDATAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCDATAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCDATAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCDATAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCDATAIN` writer"]
pub struct W(crate::W<CRCDATAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCDATAIN_SPEC>;
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
impl From<crate::W<CRCDATAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCDATAIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCDATAIN` reader - CRC Data Input"]
pub type CRCDATAIN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRCDATAIN` writer - CRC Data Input"]
pub type CRCDATAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CRCDATAIN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CRC Data Input"]
    #[inline(always)]
    pub fn crcdatain(&self) -> CRCDATAIN_R {
        CRCDATAIN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Data Input"]
    #[inline(always)]
    #[must_use]
    pub fn crcdatain(&mut self) -> CRCDATAIN_W<0> {
        CRCDATAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Data Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdatain](index.html) module"]
pub struct CRCDATAIN_SPEC;
impl crate::RegisterSpec for CRCDATAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcdatain::R](R) reader structure"]
impl crate::Readable for CRCDATAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcdatain::W](W) writer structure"]
impl crate::Writable for CRCDATAIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDATAIN to value 0"]
impl crate::Resettable for CRCDATAIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
