#[doc = "Register `ISI_IER` writer"]
pub struct W(crate::W<ISI_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISI_IER_SPEC>;
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
impl From<crate::W<ISI_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISI_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS_DONE` writer - Disable Done Interrupt Enable"]
pub struct DIS_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_DONE_W<'a> {
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
#[doc = "Field `SRST` writer - Software Reset Interrupt Enable"]
pub struct SRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRST_W<'a> {
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
#[doc = "Field `VSYNC` writer - Vertical Synchronization Interrupt Enable"]
pub struct VSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_W<'a> {
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
#[doc = "Field `PXFR_DONE` writer - Preview DMA Transfer Done Interrupt Enable"]
pub struct PXFR_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> PXFR_DONE_W<'a> {
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
#[doc = "Field `CXFR_DONE` writer - Codec DMA Transfer Done Interrupt Enable"]
pub struct CXFR_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CXFR_DONE_W<'a> {
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
#[doc = "Field `P_OVR` writer - Preview Datapath Overflow Interrupt Enable"]
pub struct P_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> P_OVR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `C_OVR` writer - Codec Datapath Overflow Interrupt Enable"]
pub struct C_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> C_OVR_W<'a> {
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
#[doc = "Field `CRC_ERR` writer - Embedded Synchronization CRC Error Interrupt Enable"]
pub struct CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_ERR_W<'a> {
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
#[doc = "Field `FR_OVR` writer - Frame Rate Overflow Interrupt Enable"]
pub struct FR_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> FR_OVR_W<'a> {
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
impl W {
    #[doc = "Bit 1 - Disable Done Interrupt Enable"]
    #[inline(always)]
    pub fn dis_done(&mut self) -> DIS_DONE_W {
        DIS_DONE_W { w: self }
    }
    #[doc = "Bit 2 - Software Reset Interrupt Enable"]
    #[inline(always)]
    pub fn srst(&mut self) -> SRST_W {
        SRST_W { w: self }
    }
    #[doc = "Bit 10 - Vertical Synchronization Interrupt Enable"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VSYNC_W {
        VSYNC_W { w: self }
    }
    #[doc = "Bit 16 - Preview DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub fn pxfr_done(&mut self) -> PXFR_DONE_W {
        PXFR_DONE_W { w: self }
    }
    #[doc = "Bit 17 - Codec DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub fn cxfr_done(&mut self) -> CXFR_DONE_W {
        CXFR_DONE_W { w: self }
    }
    #[doc = "Bit 24 - Preview Datapath Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn p_ovr(&mut self) -> P_OVR_W {
        P_OVR_W { w: self }
    }
    #[doc = "Bit 25 - Codec Datapath Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn c_ovr(&mut self) -> C_OVR_W {
        C_OVR_W { w: self }
    }
    #[doc = "Bit 26 - Embedded Synchronization CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn crc_err(&mut self) -> CRC_ERR_W {
        CRC_ERR_W { w: self }
    }
    #[doc = "Bit 27 - Frame Rate Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fr_ovr(&mut self) -> FR_OVR_W {
        FR_OVR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_ier](index.html) module"]
pub struct ISI_IER_SPEC;
impl crate::RegisterSpec for ISI_IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [isi_ier::W](W) writer structure"]
impl crate::Writable for ISI_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISI_IER to value 0"]
impl crate::Resettable for ISI_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
