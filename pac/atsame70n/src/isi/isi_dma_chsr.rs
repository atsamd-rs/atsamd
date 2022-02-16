#[doc = "Register `ISI_DMA_CHSR` reader"]
pub struct R(crate::R<ISI_DMA_CHSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISI_DMA_CHSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISI_DMA_CHSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISI_DMA_CHSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P_CH_S` reader - Preview DMA Channel Status"]
pub struct P_CH_S_R(crate::FieldReader<bool, bool>);
impl P_CH_S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P_CH_S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_CH_S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_CH_S` reader - Code DMA Channel Status"]
pub struct C_CH_S_R(crate::FieldReader<bool, bool>);
impl C_CH_S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C_CH_S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_CH_S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Preview DMA Channel Status"]
    #[inline(always)]
    pub fn p_ch_s(&self) -> P_CH_S_R {
        P_CH_S_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Code DMA Channel Status"]
    #[inline(always)]
    pub fn c_ch_s(&self) -> C_CH_S_R {
        C_CH_S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "DMA Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_chsr](index.html) module"]
pub struct ISI_DMA_CHSR_SPEC;
impl crate::RegisterSpec for ISI_DMA_CHSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isi_dma_chsr::R](R) reader structure"]
impl crate::Readable for ISI_DMA_CHSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISI_DMA_CHSR to value 0"]
impl crate::Resettable for ISI_DMA_CHSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
