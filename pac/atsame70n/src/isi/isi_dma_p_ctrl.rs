#[doc = "Register `ISI_DMA_P_CTRL` reader"]
pub struct R(crate::R<ISI_DMA_P_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISI_DMA_P_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISI_DMA_P_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISI_DMA_P_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISI_DMA_P_CTRL` writer"]
pub struct W(crate::W<ISI_DMA_P_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISI_DMA_P_CTRL_SPEC>;
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
impl From<crate::W<ISI_DMA_P_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISI_DMA_P_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P_FETCH` reader - Descriptor Fetch Control Bit"]
pub struct P_FETCH_R(crate::FieldReader<bool, bool>);
impl P_FETCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P_FETCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_FETCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P_FETCH` writer - Descriptor Fetch Control Bit"]
pub struct P_FETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> P_FETCH_W<'a> {
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
#[doc = "Field `P_WB` reader - Descriptor Writeback Control Bit"]
pub struct P_WB_R(crate::FieldReader<bool, bool>);
impl P_WB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P_WB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_WB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P_WB` writer - Descriptor Writeback Control Bit"]
pub struct P_WB_W<'a> {
    w: &'a mut W,
}
impl<'a> P_WB_W<'a> {
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
#[doc = "Field `P_IEN` reader - Transfer Done Flag Control"]
pub struct P_IEN_R(crate::FieldReader<bool, bool>);
impl P_IEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P_IEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_IEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P_IEN` writer - Transfer Done Flag Control"]
pub struct P_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> P_IEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `P_DONE` reader - Preview Transfer Done"]
pub struct P_DONE_R(crate::FieldReader<bool, bool>);
impl P_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P_DONE` writer - Preview Transfer Done"]
pub struct P_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> P_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    pub fn p_fetch(&self) -> P_FETCH_R {
        P_FETCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    pub fn p_wb(&self) -> P_WB_R {
        P_WB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    pub fn p_ien(&self) -> P_IEN_R {
        P_IEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Preview Transfer Done"]
    #[inline(always)]
    pub fn p_done(&self) -> P_DONE_R {
        P_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    pub fn p_fetch(&mut self) -> P_FETCH_W {
        P_FETCH_W { w: self }
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    pub fn p_wb(&mut self) -> P_WB_W {
        P_WB_W { w: self }
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    pub fn p_ien(&mut self) -> P_IEN_W {
        P_IEN_W { w: self }
    }
    #[doc = "Bit 3 - Preview Transfer Done"]
    #[inline(always)]
    pub fn p_done(&mut self) -> P_DONE_W {
        P_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Preview Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_p_ctrl](index.html) module"]
pub struct ISI_DMA_P_CTRL_SPEC;
impl crate::RegisterSpec for ISI_DMA_P_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isi_dma_p_ctrl::R](R) reader structure"]
impl crate::Readable for ISI_DMA_P_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isi_dma_p_ctrl::W](W) writer structure"]
impl crate::Writable for ISI_DMA_P_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISI_DMA_P_CTRL to value 0"]
impl crate::Resettable for ISI_DMA_P_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
