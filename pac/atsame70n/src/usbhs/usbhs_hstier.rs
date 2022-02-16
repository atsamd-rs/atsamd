#[doc = "Register `USBHS_HSTIER` writer"]
pub struct W(crate::W<USBHS_HSTIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTIER_SPEC>;
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
impl From<crate::W<USBHS_HSTIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCONNIES` writer - Device Connection Interrupt Enable"]
pub struct DCONNIES_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONNIES_W<'a> {
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
#[doc = "Field `DDISCIES` writer - Device Disconnection Interrupt Enable"]
pub struct DDISCIES_W<'a> {
    w: &'a mut W,
}
impl<'a> DDISCIES_W<'a> {
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
#[doc = "Field `RSTIES` writer - USB Reset Sent Interrupt Enable"]
pub struct RSTIES_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIES_W<'a> {
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
#[doc = "Field `RSMEDIES` writer - Downstream Resume Sent Interrupt Enable"]
pub struct RSMEDIES_W<'a> {
    w: &'a mut W,
}
impl<'a> RSMEDIES_W<'a> {
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
#[doc = "Field `RXRSMIES` writer - Upstream Resume Received Interrupt Enable"]
pub struct RXRSMIES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRSMIES_W<'a> {
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
#[doc = "Field `HSOFIES` writer - Host Start of Frame Interrupt Enable"]
pub struct HSOFIES_W<'a> {
    w: &'a mut W,
}
impl<'a> HSOFIES_W<'a> {
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
#[doc = "Field `HWUPIES` writer - Host Wake-Up Interrupt Enable"]
pub struct HWUPIES_W<'a> {
    w: &'a mut W,
}
impl<'a> HWUPIES_W<'a> {
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
#[doc = "Field `PEP_0` writer - Pipe 0 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PEP_1` writer - Pipe 1 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PEP_2` writer - Pipe 2 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `PEP_3` writer - Pipe 3 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PEP_4` writer - Pipe 4 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `PEP_5` writer - Pipe 5 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PEP_6` writer - Pipe 6 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PEP_7` writer - Pipe 7 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `PEP_8` writer - Pipe 8 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PEP_9` writer - Pipe 9 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `DMA_0` writer - DMA Channel 0 Interrupt Enable"]
pub struct DMA_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_0_W<'a> {
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
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Device Connection Interrupt Enable"]
    #[inline(always)]
    pub fn dconnies(&mut self) -> DCONNIES_W {
        DCONNIES_W { w: self }
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Enable"]
    #[inline(always)]
    pub fn ddiscies(&mut self) -> DDISCIES_W {
        DDISCIES_W { w: self }
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Enable"]
    #[inline(always)]
    pub fn rsties(&mut self) -> RSTIES_W {
        RSTIES_W { w: self }
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Enable"]
    #[inline(always)]
    pub fn rsmedies(&mut self) -> RSMEDIES_W {
        RSMEDIES_W { w: self }
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Enable"]
    #[inline(always)]
    pub fn rxrsmies(&mut self) -> RXRSMIES_W {
        RXRSMIES_W { w: self }
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn hsofies(&mut self) -> HSOFIES_W {
        HSOFIES_W { w: self }
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Enable"]
    #[inline(always)]
    pub fn hwupies(&mut self) -> HWUPIES_W {
        HWUPIES_W { w: self }
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_0(&mut self) -> PEP_0_W {
        PEP_0_W { w: self }
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_1(&mut self) -> PEP_1_W {
        PEP_1_W { w: self }
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_2(&mut self) -> PEP_2_W {
        PEP_2_W { w: self }
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_3(&mut self) -> PEP_3_W {
        PEP_3_W { w: self }
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_4(&mut self) -> PEP_4_W {
        PEP_4_W { w: self }
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_5(&mut self) -> PEP_5_W {
        PEP_5_W { w: self }
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_6(&mut self) -> PEP_6_W {
        PEP_6_W { w: self }
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_7(&mut self) -> PEP_7_W {
        PEP_7_W { w: self }
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_8(&mut self) -> PEP_8_W {
        PEP_8_W { w: self }
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_9(&mut self) -> PEP_9_W {
        PEP_9_W { w: self }
    }
    #[doc = "Bit 25 - DMA Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_0(&mut self) -> DMA_0_W {
        DMA_0_W { w: self }
    }
    #[doc = "Bit 26 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_1(&mut self) -> DMA_1_W {
        DMA_1_W { w: self }
    }
    #[doc = "Bit 27 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_2(&mut self) -> DMA_2_W {
        DMA_2_W { w: self }
    }
    #[doc = "Bit 28 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_3(&mut self) -> DMA_3_W {
        DMA_3_W { w: self }
    }
    #[doc = "Bit 29 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_4(&mut self) -> DMA_4_W {
        DMA_4_W { w: self }
    }
    #[doc = "Bit 30 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_5(&mut self) -> DMA_5_W {
        DMA_5_W { w: self }
    }
    #[doc = "Bit 31 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_6(&mut self) -> DMA_6_W {
        DMA_6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Global Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstier](index.html) module"]
pub struct USBHS_HSTIER_SPEC;
impl crate::RegisterSpec for USBHS_HSTIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstier::W](W) writer structure"]
impl crate::Writable for USBHS_HSTIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTIER to value 0"]
impl crate::Resettable for USBHS_HSTIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
