#[doc = "Register `CLEAR` writer"]
pub struct W(crate::W<CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEAR_SPEC>;
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
impl From<crate::W<CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Watchdog Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLEARSELECT_AW {
    #[doc = "165: Clear Key"]
    KEY = 165,
}
impl From<CLEARSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: CLEARSELECT_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CLEAR` writer - Watchdog Clear"]
pub type CLEAR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CLEAR_SPEC, u8, CLEARSELECT_AW, 8, O>;
impl<'a, const O: u8> CLEAR_W<'a, O> {
    #[doc = "Clear Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(CLEARSELECT_AW::KEY)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog Clear"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<0> {
        CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clear](index.html) module"]
pub struct CLEAR_SPEC;
impl crate::RegisterSpec for CLEAR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [clear::W](W) writer structure"]
impl crate::Writable for CLEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLEAR to value 0"]
impl crate::Resettable for CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
