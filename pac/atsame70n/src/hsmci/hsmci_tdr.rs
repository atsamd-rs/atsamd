#[doc = "Register `HSMCI_TDR` writer"]
pub struct W(crate::W<HSMCI_TDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSMCI_TDR_SPEC>;
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
impl From<crate::W<HSMCI_TDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSMCI_TDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Data to Write"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to Write"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_tdr](index.html) module"]
pub struct HSMCI_TDR_SPEC;
impl crate::RegisterSpec for HSMCI_TDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hsmci_tdr::W](W) writer structure"]
impl crate::Writable for HSMCI_TDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSMCI_TDR to value 0"]
impl crate::Resettable for HSMCI_TDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
