#[doc = "Register `DSEQDATA` writer"]
pub struct W(crate::W<DSEQDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSEQDATA_SPEC>;
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
impl From<crate::W<DSEQDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSEQDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - DMA Sequential Data"]
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
    #[doc = "Bits 0:31 - DMA Sequential Data"]
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
#[doc = "DMA Sequencial Data\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dseqdata](index.html) module"]
pub struct DSEQDATA_SPEC;
impl crate::RegisterSpec for DSEQDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dseqdata::W](W) writer structure"]
impl crate::Writable for DSEQDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSEQDATA to value 0"]
impl crate::Resettable for DSEQDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
