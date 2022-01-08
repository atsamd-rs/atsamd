#[doc = "Register `TXDATA` writer"]
pub struct W(crate::W<TXDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDATA_SPEC>;
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
impl From<crate::W<TXDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Transmit Data"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
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
#[doc = "Transmit Data\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdata](index.html) module"]
pub struct TXDATA_SPEC;
impl crate::RegisterSpec for TXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txdata::W](W) writer structure"]
impl crate::Writable for TXDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TXDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
