#[doc = "Register `USBHS_DEVIFR` writer"]
pub struct W(crate::W<USBHS_DEVIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVIFR_SPEC>;
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
impl From<crate::W<USBHS_DEVIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPS` writer - Suspend Interrupt Set"]
pub struct SUSPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPS_W<'a> {
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
#[doc = "Field `MSOFS` writer - Micro Start of Frame Interrupt Set"]
pub struct MSOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSOFS_W<'a> {
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
#[doc = "Field `SOFS` writer - Start of Frame Interrupt Set"]
pub struct SOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFS_W<'a> {
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
#[doc = "Field `EORSTS` writer - End of Reset Interrupt Set"]
pub struct EORSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSTS_W<'a> {
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
#[doc = "Field `WAKEUPS` writer - Wake-Up Interrupt Set"]
pub struct WAKEUPS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPS_W<'a> {
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
#[doc = "Field `EORSMS` writer - End of Resume Interrupt Set"]
pub struct EORSMS_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSMS_W<'a> {
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
#[doc = "Field `UPRSMS` writer - Upstream Resume Interrupt Set"]
pub struct UPRSMS_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSMS_W<'a> {
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
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Set"]
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
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Set"]
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
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Set"]
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
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Set"]
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
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Set"]
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
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Set"]
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
#[doc = "Field `DMA_7` writer - DMA Channel 7 Interrupt Set"]
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
    #[doc = "Bit 0 - Suspend Interrupt Set"]
    #[inline(always)]
    pub fn susps(&mut self) -> SUSPS_W {
        SUSPS_W { w: self }
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Set"]
    #[inline(always)]
    pub fn msofs(&mut self) -> MSOFS_W {
        MSOFS_W { w: self }
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Set"]
    #[inline(always)]
    pub fn sofs(&mut self) -> SOFS_W {
        SOFS_W { w: self }
    }
    #[doc = "Bit 3 - End of Reset Interrupt Set"]
    #[inline(always)]
    pub fn eorsts(&mut self) -> EORSTS_W {
        EORSTS_W { w: self }
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Set"]
    #[inline(always)]
    pub fn wakeups(&mut self) -> WAKEUPS_W {
        WAKEUPS_W { w: self }
    }
    #[doc = "Bit 5 - End of Resume Interrupt Set"]
    #[inline(always)]
    pub fn eorsms(&mut self) -> EORSMS_W {
        EORSMS_W { w: self }
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Set"]
    #[inline(always)]
    pub fn uprsms(&mut self) -> UPRSMS_W {
        UPRSMS_W { w: self }
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Set"]
    #[inline(always)]
    pub fn dma_1(&mut self) -> DMA_1_W {
        DMA_1_W { w: self }
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Set"]
    #[inline(always)]
    pub fn dma_2(&mut self) -> DMA_2_W {
        DMA_2_W { w: self }
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Set"]
    #[inline(always)]
    pub fn dma_3(&mut self) -> DMA_3_W {
        DMA_3_W { w: self }
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Set"]
    #[inline(always)]
    pub fn dma_4(&mut self) -> DMA_4_W {
        DMA_4_W { w: self }
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Set"]
    #[inline(always)]
    pub fn dma_5(&mut self) -> DMA_5_W {
        DMA_5_W { w: self }
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Set"]
    #[inline(always)]
    pub fn dma_6(&mut self) -> DMA_6_W {
        DMA_6_W { w: self }
    }
    #[doc = "Bit 31 - DMA Channel 7 Interrupt Set"]
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
#[doc = "Device Global Interrupt Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devifr](index.html) module"]
pub struct USBHS_DEVIFR_SPEC;
impl crate::RegisterSpec for USBHS_DEVIFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_devifr::W](W) writer structure"]
impl crate::Writable for USBHS_DEVIFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVIFR to value 0"]
impl crate::Resettable for USBHS_DEVIFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
