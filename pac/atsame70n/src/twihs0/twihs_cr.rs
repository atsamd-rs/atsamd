#[doc = "Register `TWIHS_CR` writer"]
pub struct W(crate::W<TWIHS_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWIHS_CR_SPEC>;
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
impl From<crate::W<TWIHS_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWIHS_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` writer - Send a START Condition"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Field `STOP` writer - Send a STOP Condition"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
#[doc = "Field `MSEN` writer - TWIHS Master Mode Enabled"]
pub struct MSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEN_W<'a> {
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
#[doc = "Field `MSDIS` writer - TWIHS Master Mode Disabled"]
pub struct MSDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSDIS_W<'a> {
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
#[doc = "Field `SVEN` writer - TWIHS Slave Mode Enabled"]
pub struct SVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SVEN_W<'a> {
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
#[doc = "Field `SVDIS` writer - TWIHS Slave Mode Disabled"]
pub struct SVDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SVDIS_W<'a> {
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
#[doc = "Field `QUICK` writer - SMBus Quick Command"]
pub struct QUICK_W<'a> {
    w: &'a mut W,
}
impl<'a> QUICK_W<'a> {
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
#[doc = "Field `SWRST` writer - Software Reset"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `HSEN` writer - TWIHS High-Speed Mode Enabled"]
pub struct HSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEN_W<'a> {
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
#[doc = "Field `HSDIS` writer - TWIHS High-Speed Mode Disabled"]
pub struct HSDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDIS_W<'a> {
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
#[doc = "Field `SMBEN` writer - SMBus Mode Enabled"]
pub struct SMBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBEN_W<'a> {
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
#[doc = "Field `SMBDIS` writer - SMBus Mode Disabled"]
pub struct SMBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBDIS_W<'a> {
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
#[doc = "Field `PECEN` writer - Packet Error Checking Enable"]
pub struct PECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PECEN_W<'a> {
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
#[doc = "Field `PECDIS` writer - Packet Error Checking Disable"]
pub struct PECDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PECDIS_W<'a> {
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
#[doc = "Field `PECRQ` writer - PEC Request"]
pub struct PECRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PECRQ_W<'a> {
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
#[doc = "Field `CLEAR` writer - Bus CLEAR Command"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
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
#[doc = "Field `ACMEN` writer - Alternative Command Mode Enable"]
pub struct ACMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMEN_W<'a> {
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
#[doc = "Field `ACMDIS` writer - Alternative Command Mode Disable"]
pub struct ACMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMDIS_W<'a> {
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
#[doc = "Field `THRCLR` writer - Transmit Holding Register Clear"]
pub struct THRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> THRCLR_W<'a> {
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
#[doc = "Field `LOCKCLR` writer - Lock Clear"]
pub struct LOCKCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKCLR_W<'a> {
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
#[doc = "Field `FIFOEN` writer - FIFO Enable"]
pub struct FIFOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOEN_W<'a> {
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
#[doc = "Field `FIFODIS` writer - FIFO Disable"]
pub struct FIFODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFODIS_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Send a START Condition"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - Send a STOP Condition"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 2 - TWIHS Master Mode Enabled"]
    #[inline(always)]
    pub fn msen(&mut self) -> MSEN_W {
        MSEN_W { w: self }
    }
    #[doc = "Bit 3 - TWIHS Master Mode Disabled"]
    #[inline(always)]
    pub fn msdis(&mut self) -> MSDIS_W {
        MSDIS_W { w: self }
    }
    #[doc = "Bit 4 - TWIHS Slave Mode Enabled"]
    #[inline(always)]
    pub fn sven(&mut self) -> SVEN_W {
        SVEN_W { w: self }
    }
    #[doc = "Bit 5 - TWIHS Slave Mode Disabled"]
    #[inline(always)]
    pub fn svdis(&mut self) -> SVDIS_W {
        SVDIS_W { w: self }
    }
    #[doc = "Bit 6 - SMBus Quick Command"]
    #[inline(always)]
    pub fn quick(&mut self) -> QUICK_W {
        QUICK_W { w: self }
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 8 - TWIHS High-Speed Mode Enabled"]
    #[inline(always)]
    pub fn hsen(&mut self) -> HSEN_W {
        HSEN_W { w: self }
    }
    #[doc = "Bit 9 - TWIHS High-Speed Mode Disabled"]
    #[inline(always)]
    pub fn hsdis(&mut self) -> HSDIS_W {
        HSDIS_W { w: self }
    }
    #[doc = "Bit 10 - SMBus Mode Enabled"]
    #[inline(always)]
    pub fn smben(&mut self) -> SMBEN_W {
        SMBEN_W { w: self }
    }
    #[doc = "Bit 11 - SMBus Mode Disabled"]
    #[inline(always)]
    pub fn smbdis(&mut self) -> SMBDIS_W {
        SMBDIS_W { w: self }
    }
    #[doc = "Bit 12 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W {
        PECEN_W { w: self }
    }
    #[doc = "Bit 13 - Packet Error Checking Disable"]
    #[inline(always)]
    pub fn pecdis(&mut self) -> PECDIS_W {
        PECDIS_W { w: self }
    }
    #[doc = "Bit 14 - PEC Request"]
    #[inline(always)]
    pub fn pecrq(&mut self) -> PECRQ_W {
        PECRQ_W { w: self }
    }
    #[doc = "Bit 15 - Bus CLEAR Command"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
    #[doc = "Bit 16 - Alternative Command Mode Enable"]
    #[inline(always)]
    pub fn acmen(&mut self) -> ACMEN_W {
        ACMEN_W { w: self }
    }
    #[doc = "Bit 17 - Alternative Command Mode Disable"]
    #[inline(always)]
    pub fn acmdis(&mut self) -> ACMDIS_W {
        ACMDIS_W { w: self }
    }
    #[doc = "Bit 24 - Transmit Holding Register Clear"]
    #[inline(always)]
    pub fn thrclr(&mut self) -> THRCLR_W {
        THRCLR_W { w: self }
    }
    #[doc = "Bit 26 - Lock Clear"]
    #[inline(always)]
    pub fn lockclr(&mut self) -> LOCKCLR_W {
        LOCKCLR_W { w: self }
    }
    #[doc = "Bit 28 - FIFO Enable"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W {
        FIFOEN_W { w: self }
    }
    #[doc = "Bit 29 - FIFO Disable"]
    #[inline(always)]
    pub fn fifodis(&mut self) -> FIFODIS_W {
        FIFODIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_cr](index.html) module"]
pub struct TWIHS_CR_SPEC;
impl crate::RegisterSpec for TWIHS_CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [twihs_cr::W](W) writer structure"]
impl crate::Writable for TWIHS_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWIHS_CR to value 0"]
impl crate::Resettable for TWIHS_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
