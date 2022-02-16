#[doc = "Register `USBHS_DEVIDR` writer"]
pub struct W(crate::W<USBHS_DEVIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVIDR_SPEC>;
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
impl From<crate::W<USBHS_DEVIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPEC` writer - Suspend Interrupt Disable"]
pub struct SUSPEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEC_W<'a> {
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
#[doc = "Field `MSOFEC` writer - Micro Start of Frame Interrupt Disable"]
pub struct MSOFEC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSOFEC_W<'a> {
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
#[doc = "Field `SOFEC` writer - Start of Frame Interrupt Disable"]
pub struct SOFEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFEC_W<'a> {
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
#[doc = "Field `EORSTEC` writer - End of Reset Interrupt Disable"]
pub struct EORSTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSTEC_W<'a> {
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
#[doc = "Field `WAKEUPEC` writer - Wake-Up Interrupt Disable"]
pub struct WAKEUPEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `EORSMEC` writer - End of Resume Interrupt Disable"]
pub struct EORSMEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSMEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `UPRSMEC` writer - Upstream Resume Interrupt Disable"]
pub struct UPRSMEC_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSMEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PEP_0` writer - Endpoint 0 Interrupt Disable"]
pub struct PEP_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEP_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `PEP_1` writer - Endpoint 1 Interrupt Disable"]
pub struct PEP_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEP_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PEP_2` writer - Endpoint 2 Interrupt Disable"]
pub struct PEP_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PEP_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PEP_3` writer - Endpoint 3 Interrupt Disable"]
pub struct PEP_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PEP_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `PEP_4` writer - Endpoint 4 Interrupt Disable"]
pub struct PEP_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PEP_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PEP_5` writer - Endpoint 5 Interrupt Disable"]
pub struct PEP_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PEP_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PEP_6` writer - Endpoint 6 Interrupt Disable"]
pub struct PEP_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PEP_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PEP_7` writer - Endpoint 7 Interrupt Disable"]
pub struct PEP_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PEP_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PEP_8` writer - Endpoint 8 Interrupt Disable"]
pub struct PEP_8_W<'a> {
    w: &'a mut W,
}
impl<'a> PEP_8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PEP_9` writer - Endpoint 9 Interrupt Disable"]
pub struct PEP_9_W<'a> {
    w: &'a mut W,
}
impl<'a> PEP_9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Disable"]
pub struct DMA_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Disable"]
pub struct DMA_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Disable"]
pub struct DMA_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Disable"]
pub struct DMA_4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Disable"]
pub struct DMA_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Disable"]
pub struct DMA_6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DMA_7` writer - DMA Channel 7 Interrupt Disable"]
pub struct DMA_7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Disable"]
    #[inline(always)]
    pub fn suspec(&mut self) -> SUSPEC_W {
        SUSPEC_W { w: self }
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Disable"]
    #[inline(always)]
    pub fn msofec(&mut self) -> MSOFEC_W {
        MSOFEC_W { w: self }
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Disable"]
    #[inline(always)]
    pub fn sofec(&mut self) -> SOFEC_W {
        SOFEC_W { w: self }
    }
    #[doc = "Bit 3 - End of Reset Interrupt Disable"]
    #[inline(always)]
    pub fn eorstec(&mut self) -> EORSTEC_W {
        EORSTEC_W { w: self }
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Disable"]
    #[inline(always)]
    pub fn wakeupec(&mut self) -> WAKEUPEC_W {
        WAKEUPEC_W { w: self }
    }
    #[doc = "Bit 5 - End of Resume Interrupt Disable"]
    #[inline(always)]
    pub fn eorsmec(&mut self) -> EORSMEC_W {
        EORSMEC_W { w: self }
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Disable"]
    #[inline(always)]
    pub fn uprsmec(&mut self) -> UPRSMEC_W {
        UPRSMEC_W { w: self }
    }
    #[doc = "Bit 12 - Endpoint 0 Interrupt Disable"]
    #[inline(always)]
    pub fn pep_0(&mut self) -> PEP_0_W {
        PEP_0_W { w: self }
    }
    #[doc = "Bit 13 - Endpoint 1 Interrupt Disable"]
    #[inline(always)]
    pub fn pep_1(&mut self) -> PEP_1_W {
        PEP_1_W { w: self }
    }
    #[doc = "Bit 14 - Endpoint 2 Interrupt Disable"]
    #[inline(always)]
    pub fn pep_2(&mut self) -> PEP_2_W {
        PEP_2_W { w: self }
    }
    #[doc = "Bit 15 - Endpoint 3 Interrupt Disable"]
    #[inline(always)]
    pub fn pep_3(&mut self) -> PEP_3_W {
        PEP_3_W { w: self }
    }
    #[doc = "Bit 16 - Endpoint 4 Interrupt Disable"]
    #[inline(always)]
    pub fn pep_4(&mut self) -> PEP_4_W {
        PEP_4_W { w: self }
    }
    #[doc = "Bit 17 - Endpoint 5 Interrupt Disable"]
    #[inline(always)]
    pub fn pep_5(&mut self) -> PEP_5_W {
        PEP_5_W { w: self }
    }
    #[doc = "Bit 18 - Endpoint 6 Interrupt Disable"]
    #[inline(always)]
    pub fn pep_6(&mut self) -> PEP_6_W {
        PEP_6_W { w: self }
    }
    #[doc = "Bit 19 - Endpoint 7 Interrupt Disable"]
    #[inline(always)]
    pub fn pep_7(&mut self) -> PEP_7_W {
        PEP_7_W { w: self }
    }
    #[doc = "Bit 20 - Endpoint 8 Interrupt Disable"]
    #[inline(always)]
    pub fn pep_8(&mut self) -> PEP_8_W {
        PEP_8_W { w: self }
    }
    #[doc = "Bit 21 - Endpoint 9 Interrupt Disable"]
    #[inline(always)]
    pub fn pep_9(&mut self) -> PEP_9_W {
        PEP_9_W { w: self }
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Disable"]
    #[inline(always)]
    pub fn dma_1(&mut self) -> DMA_1_W {
        DMA_1_W { w: self }
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Disable"]
    #[inline(always)]
    pub fn dma_2(&mut self) -> DMA_2_W {
        DMA_2_W { w: self }
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Disable"]
    #[inline(always)]
    pub fn dma_3(&mut self) -> DMA_3_W {
        DMA_3_W { w: self }
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Disable"]
    #[inline(always)]
    pub fn dma_4(&mut self) -> DMA_4_W {
        DMA_4_W { w: self }
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Disable"]
    #[inline(always)]
    pub fn dma_5(&mut self) -> DMA_5_W {
        DMA_5_W { w: self }
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Disable"]
    #[inline(always)]
    pub fn dma_6(&mut self) -> DMA_6_W {
        DMA_6_W { w: self }
    }
    #[doc = "Bit 31 - DMA Channel 7 Interrupt Disable"]
    #[inline(always)]
    pub fn dma_7(&mut self) -> DMA_7_W {
        DMA_7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Global Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devidr](index.html) module"]
pub struct USBHS_DEVIDR_SPEC;
impl crate::RegisterSpec for USBHS_DEVIDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_devidr::W](W) writer structure"]
impl crate::Writable for USBHS_DEVIDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVIDR to value 0"]
impl crate::Resettable for USBHS_DEVIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
