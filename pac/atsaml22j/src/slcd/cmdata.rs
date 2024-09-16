#[doc = "Register `CMDATA` writer"]
pub struct W(crate::W<CMDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDATA_SPEC>;
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
impl From<crate::W<CMDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDATA` writer - Segments Data"]
pub type SDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDATA_SPEC, u32, u32, 24, O>;
impl W {
    #[doc = "Bits 0:23 - Segments Data"]
    #[inline(always)]
    #[must_use]
    pub fn sdata(&mut self) -> SDATA_W<0> {
        SDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Character Mapping Segments Data\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdata](index.html) module"]
pub struct CMDATA_SPEC;
impl crate::RegisterSpec for CMDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmdata::W](W) writer structure"]
impl crate::Writable for CMDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDATA to value 0"]
impl crate::Resettable for CMDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
