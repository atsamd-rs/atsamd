#[doc = "Register `TA` writer"]
pub struct W(crate::W<TA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TA_SPEC>;
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
impl From<crate::W<TA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITDT` writer - Increment/Decrement"]
pub type ITDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TA_SPEC, u32, u32, 30, O>;
#[doc = "Field `ADJ` writer - Adjust 1588 Timer"]
pub type ADJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, TA_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:29 - Increment/Decrement"]
    #[inline(always)]
    #[must_use]
    pub fn itdt(&mut self) -> ITDT_W<0> {
        ITDT_W::new(self)
    }
    #[doc = "Bit 31 - Adjust 1588 Timer"]
    #[inline(always)]
    #[must_use]
    pub fn adj(&mut self) -> ADJ_W<31> {
        ADJ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Adjust Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta](index.html) module"]
pub struct TA_SPEC;
impl crate::RegisterSpec for TA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ta::W](W) writer structure"]
impl crate::Writable for TA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TA to value 0"]
impl crate::Resettable for TA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
