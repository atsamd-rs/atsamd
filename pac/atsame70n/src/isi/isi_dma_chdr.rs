#[doc = "Register `ISI_DMA_CHDR` writer"]
pub struct W(crate::W<ISI_DMA_CHDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISI_DMA_CHDR_SPEC>;
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
impl From<crate::W<ISI_DMA_CHDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISI_DMA_CHDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P_CH_DIS` writer - Preview Channel Disable Request"]
pub struct P_CH_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> P_CH_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `C_CH_DIS` writer - Codec Channel Disable Request"]
pub struct C_CH_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> C_CH_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Preview Channel Disable Request"]
    #[inline(always)]
    pub fn p_ch_dis(&mut self) -> P_CH_DIS_W {
        P_CH_DIS_W { w: self }
    }
    #[doc = "Bit 1 - Codec Channel Disable Request"]
    #[inline(always)]
    pub fn c_ch_dis(&mut self) -> C_CH_DIS_W {
        C_CH_DIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_chdr](index.html) module"]
pub struct ISI_DMA_CHDR_SPEC;
impl crate::RegisterSpec for ISI_DMA_CHDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [isi_dma_chdr::W](W) writer structure"]
impl crate::Writable for ISI_DMA_CHDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISI_DMA_CHDR to value 0"]
impl crate::Resettable for ISI_DMA_CHDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
