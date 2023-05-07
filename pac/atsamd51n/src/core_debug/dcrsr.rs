#[doc = "Register `DCRSR` writer"]
pub struct W(crate::W<DCRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCRSR_SPEC>;
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
impl From<crate::W<DCRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGSEL` writer - "]
pub type REGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCRSR_SPEC, u8, u8, 5, O>;
#[doc = "Field `REGWnR` writer - "]
pub type REGWN_R_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCRSR_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn regsel(&mut self) -> REGSEL_W<0> {
        REGSEL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn regwn_r(&mut self) -> REGWN_R_W<16> {
        REGWN_R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Core Register Selector Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrsr](index.html) module"]
pub struct DCRSR_SPEC;
impl crate::RegisterSpec for DCRSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dcrsr::W](W) writer structure"]
impl crate::Writable for DCRSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCRSR to value 0"]
impl crate::Resettable for DCRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
