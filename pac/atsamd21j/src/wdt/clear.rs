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
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLEAR_AW {
    #[doc = "165: Clear Key"]
    KEY = 165,
}
impl From<CLEAR_AW> for u8 {
    #[inline(always)]
    fn from(variant: CLEAR_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CLEAR` writer - Watchdog Clear"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLEAR_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clear Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(CLEAR_AW::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog Clear"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
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
}
#[doc = "`reset()` method sets CLEAR to value 0"]
impl crate::Resettable for CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
