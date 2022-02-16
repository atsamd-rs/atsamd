#[doc = "Register `I2SC_THR` writer"]
pub struct W(crate::W<I2SC_THR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SC_THR_SPEC>;
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
impl From<crate::W<I2SC_THR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SC_THR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THR` writer - Transmitter Holding Register"]
pub struct THR_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmitter Holding Register"]
    #[inline(always)]
    pub fn thr(&mut self) -> THR_W {
        THR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_thr](index.html) module"]
pub struct I2SC_THR_SPEC;
impl crate::RegisterSpec for I2SC_THR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [i2sc_thr::W](W) writer structure"]
impl crate::Writable for I2SC_THR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SC_THR to value 0"]
impl crate::Resettable for I2SC_THR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
