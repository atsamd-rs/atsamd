#[doc = "Register `QSPI_TDR` writer"]
pub struct W(crate::W<QSPI_TDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_TDR_SPEC>;
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
impl From<crate::W<QSPI_TDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_TDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TD` writer - Transmit Data"]
pub struct TD_W<'a> {
    w: &'a mut W,
}
impl<'a> TD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn td(&mut self) -> TD_W {
        TD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_tdr](index.html) module"]
pub struct QSPI_TDR_SPEC;
impl crate::RegisterSpec for QSPI_TDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [qspi_tdr::W](W) writer structure"]
impl crate::Writable for QSPI_TDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_TDR to value 0"]
impl crate::Resettable for QSPI_TDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
