#[doc = "Register `DATABUF[%s]` writer"]
pub struct W(crate::W<DATABUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATABUF_SPEC>;
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
impl From<crate::W<DATABUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATABUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATABUF` writer - DAC0 Data Buffer"]
pub struct DATABUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DATABUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value as u16;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - DAC0 Data Buffer"]
    #[inline(always)]
    pub fn databuf(&mut self) -> DATABUF_W {
        DATABUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC n Data Buffer\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [databuf](index.html) module"]
pub struct DATABUF_SPEC;
impl crate::RegisterSpec for DATABUF_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [databuf::W](W) writer structure"]
impl crate::Writable for DATABUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATABUF[%s]
to value 0"]
impl crate::Resettable for DATABUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
