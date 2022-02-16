#[doc = "Register `HSMCI_IER` writer"]
pub struct W(crate::W<HSMCI_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSMCI_IER_SPEC>;
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
impl From<crate::W<HSMCI_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSMCI_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDRDY` writer - Command Ready Interrupt Enable"]
pub struct CMDRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRDY_W<'a> {
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
#[doc = "Field `RXRDY` writer - Receiver Ready Interrupt Enable"]
pub struct RXRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRDY_W<'a> {
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
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Enable"]
pub struct TXRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDY_W<'a> {
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
#[doc = "Field `BLKE` writer - Data Block Ended Interrupt Enable"]
pub struct BLKE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKE_W<'a> {
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
#[doc = "Field `DTIP` writer - Data Transfer in Progress Interrupt Enable"]
pub struct DTIP_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIP_W<'a> {
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
#[doc = "Field `NOTBUSY` writer - Data Not Busy Interrupt Enable"]
pub struct NOTBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTBUSY_W<'a> {
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
#[doc = "Field `SDIOIRQA` writer - SDIO Interrupt for Slot A Interrupt Enable"]
pub struct SDIOIRQA_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOIRQA_W<'a> {
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
#[doc = "Field `SDIOWAIT` writer - SDIO Read Wait Operation Status Interrupt Enable"]
pub struct SDIOWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOWAIT_W<'a> {
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
#[doc = "Field `CSRCV` writer - Completion Signal Received Interrupt Enable"]
pub struct CSRCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRCV_W<'a> {
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
#[doc = "Field `RINDE` writer - Response Index Error Interrupt Enable"]
pub struct RINDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RINDE_W<'a> {
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
#[doc = "Field `RDIRE` writer - Response Direction Error Interrupt Enable"]
pub struct RDIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIRE_W<'a> {
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
#[doc = "Field `RCRCE` writer - Response CRC Error Interrupt Enable"]
pub struct RCRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRCE_W<'a> {
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
#[doc = "Field `RENDE` writer - Response End Bit Error Interrupt Enable"]
pub struct RENDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RENDE_W<'a> {
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
#[doc = "Field `RTOE` writer - Response Time-out Error Interrupt Enable"]
pub struct RTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTOE_W<'a> {
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
#[doc = "Field `DCRCE` writer - Data CRC Error Interrupt Enable"]
pub struct DCRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCE_W<'a> {
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
#[doc = "Field `DTOE` writer - Data Time-out Error Interrupt Enable"]
pub struct DTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CSTOE` writer - Completion Signal Timeout Error Interrupt Enable"]
pub struct CSTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `BLKOVRE` writer - DMA Block Overrun Error Interrupt Enable"]
pub struct BLKOVRE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKOVRE_W<'a> {
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
#[doc = "Field `FIFOEMPTY` writer - FIFO empty Interrupt enable"]
pub struct FIFOEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOEMPTY_W<'a> {
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
#[doc = "Field `XFRDONE` writer - Transfer Done Interrupt enable"]
pub struct XFRDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRDONE_W<'a> {
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
#[doc = "Field `ACKRCV` writer - Boot Acknowledge Interrupt Enable"]
pub struct ACKRCV_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKRCV_W<'a> {
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
#[doc = "Field `ACKRCVE` writer - Boot Acknowledge Error Interrupt Enable"]
pub struct ACKRCVE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKRCVE_W<'a> {
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
#[doc = "Field `OVRE` writer - Overrun Interrupt Enable"]
pub struct OVRE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRE_W<'a> {
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
#[doc = "Field `UNRE` writer - Underrun Interrupt Enable"]
pub struct UNRE_W<'a> {
    w: &'a mut W,
}
impl<'a> UNRE_W<'a> {
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
    #[doc = "Bit 0 - Command Ready Interrupt Enable"]
    #[inline(always)]
    pub fn cmdrdy(&mut self) -> CMDRDY_W {
        CMDRDY_W { w: self }
    }
    #[doc = "Bit 1 - Receiver Ready Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RXRDY_W {
        RXRDY_W { w: self }
    }
    #[doc = "Bit 2 - Transmit Ready Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W {
        TXRDY_W { w: self }
    }
    #[doc = "Bit 3 - Data Block Ended Interrupt Enable"]
    #[inline(always)]
    pub fn blke(&mut self) -> BLKE_W {
        BLKE_W { w: self }
    }
    #[doc = "Bit 4 - Data Transfer in Progress Interrupt Enable"]
    #[inline(always)]
    pub fn dtip(&mut self) -> DTIP_W {
        DTIP_W { w: self }
    }
    #[doc = "Bit 5 - Data Not Busy Interrupt Enable"]
    #[inline(always)]
    pub fn notbusy(&mut self) -> NOTBUSY_W {
        NOTBUSY_W { w: self }
    }
    #[doc = "Bit 8 - SDIO Interrupt for Slot A Interrupt Enable"]
    #[inline(always)]
    pub fn sdioirqa(&mut self) -> SDIOIRQA_W {
        SDIOIRQA_W { w: self }
    }
    #[doc = "Bit 12 - SDIO Read Wait Operation Status Interrupt Enable"]
    #[inline(always)]
    pub fn sdiowait(&mut self) -> SDIOWAIT_W {
        SDIOWAIT_W { w: self }
    }
    #[doc = "Bit 13 - Completion Signal Received Interrupt Enable"]
    #[inline(always)]
    pub fn csrcv(&mut self) -> CSRCV_W {
        CSRCV_W { w: self }
    }
    #[doc = "Bit 16 - Response Index Error Interrupt Enable"]
    #[inline(always)]
    pub fn rinde(&mut self) -> RINDE_W {
        RINDE_W { w: self }
    }
    #[doc = "Bit 17 - Response Direction Error Interrupt Enable"]
    #[inline(always)]
    pub fn rdire(&mut self) -> RDIRE_W {
        RDIRE_W { w: self }
    }
    #[doc = "Bit 18 - Response CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn rcrce(&mut self) -> RCRCE_W {
        RCRCE_W { w: self }
    }
    #[doc = "Bit 19 - Response End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn rende(&mut self) -> RENDE_W {
        RENDE_W { w: self }
    }
    #[doc = "Bit 20 - Response Time-out Error Interrupt Enable"]
    #[inline(always)]
    pub fn rtoe(&mut self) -> RTOE_W {
        RTOE_W { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn dcrce(&mut self) -> DCRCE_W {
        DCRCE_W { w: self }
    }
    #[doc = "Bit 22 - Data Time-out Error Interrupt Enable"]
    #[inline(always)]
    pub fn dtoe(&mut self) -> DTOE_W {
        DTOE_W { w: self }
    }
    #[doc = "Bit 23 - Completion Signal Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn cstoe(&mut self) -> CSTOE_W {
        CSTOE_W { w: self }
    }
    #[doc = "Bit 24 - DMA Block Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn blkovre(&mut self) -> BLKOVRE_W {
        BLKOVRE_W { w: self }
    }
    #[doc = "Bit 26 - FIFO empty Interrupt enable"]
    #[inline(always)]
    pub fn fifoempty(&mut self) -> FIFOEMPTY_W {
        FIFOEMPTY_W { w: self }
    }
    #[doc = "Bit 27 - Transfer Done Interrupt enable"]
    #[inline(always)]
    pub fn xfrdone(&mut self) -> XFRDONE_W {
        XFRDONE_W { w: self }
    }
    #[doc = "Bit 28 - Boot Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn ackrcv(&mut self) -> ACKRCV_W {
        ACKRCV_W { w: self }
    }
    #[doc = "Bit 29 - Boot Acknowledge Error Interrupt Enable"]
    #[inline(always)]
    pub fn ackrcve(&mut self) -> ACKRCVE_W {
        ACKRCVE_W { w: self }
    }
    #[doc = "Bit 30 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OVRE_W {
        OVRE_W { w: self }
    }
    #[doc = "Bit 31 - Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn unre(&mut self) -> UNRE_W {
        UNRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_ier](index.html) module"]
pub struct HSMCI_IER_SPEC;
impl crate::RegisterSpec for HSMCI_IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hsmci_ier::W](W) writer structure"]
impl crate::Writable for HSMCI_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSMCI_IER to value 0"]
impl crate::Resettable for HSMCI_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
