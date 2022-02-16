#[doc = "Register `TWIHS_THR` writer"]
pub struct W(crate::W<TWIHS_THR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWIHS_THR_SPEC>;
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
impl From<crate::W<TWIHS_THR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWIHS_THR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` writer - Master or Slave Transmit Holding Data"]
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Master or Slave Transmit Holding Data"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_thr](index.html) module"]
pub struct TWIHS_THR_SPEC;
impl crate::RegisterSpec for TWIHS_THR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [twihs_thr::W](W) writer structure"]
impl crate::Writable for TWIHS_THR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWIHS_THR to value 0"]
impl crate::Resettable for TWIHS_THR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
