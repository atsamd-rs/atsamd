#[doc = "Register `US_IDR_LON_MODE` writer"]
pub struct W(crate::W<US_IDR_LON_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_IDR_LON_MODE_SPEC>;
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
impl From<crate::W<US_IDR_LON_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_IDR_LON_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub struct TXEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEMPTY_W<'a> {
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
#[doc = "Field `RIIC` writer - Ring Indicator Input Change Disable"]
pub struct RIIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RIIC_W<'a> {
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
#[doc = "Field `DSRIC` writer - Data Set Ready Input Change Disable"]
pub struct DSRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRIC_W<'a> {
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
#[doc = "Field `DCDIC` writer - Data Carrier Detect Input Change Interrupt Disable"]
pub struct DCDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDIC_W<'a> {
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
#[doc = "Field `LSFE` writer - LON Short Frame Error Interrupt Disable"]
pub struct LSFE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSFE_W<'a> {
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
#[doc = "Field `LCRCE` writer - LON CRC Error Interrupt Disable"]
pub struct LCRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCRCE_W<'a> {
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
#[doc = "Field `LTXD` writer - LON Transmission Done Interrupt Disable"]
pub struct LTXD_W<'a> {
    w: &'a mut W,
}
impl<'a> LTXD_W<'a> {
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
#[doc = "Field `LCOL` writer - LON Collision Interrupt Disable"]
pub struct LCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> LCOL_W<'a> {
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
#[doc = "Field `LFET` writer - LON Frame Early Termination Interrupt Disable"]
pub struct LFET_W<'a> {
    w: &'a mut W,
}
impl<'a> LFET_W<'a> {
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
#[doc = "Field `LRXD` writer - LON Reception Done Interrupt Disable"]
pub struct LRXD_W<'a> {
    w: &'a mut W,
}
impl<'a> LRXD_W<'a> {
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
#[doc = "Field `LBLOVFE` writer - LON Backlog Overflow Error Interrupt Disable"]
pub struct LBLOVFE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBLOVFE_W<'a> {
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
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RXRDY_W {
        RXRDY_W { w: self }
    }
    #[doc = "Bit 1 - TXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W {
        TXRDY_W { w: self }
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OVRE_W {
        OVRE_W { w: self }
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TXEMPTY_W {
        TXEMPTY_W { w: self }
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Disable"]
    #[inline(always)]
    pub fn riic(&mut self) -> RIIC_W {
        RIIC_W { w: self }
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Disable"]
    #[inline(always)]
    pub fn dsric(&mut self) -> DSRIC_W {
        DSRIC_W { w: self }
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Disable"]
    #[inline(always)]
    pub fn dcdic(&mut self) -> DCDIC_W {
        DCDIC_W { w: self }
    }
    #[doc = "Bit 6 - LON Short Frame Error Interrupt Disable"]
    #[inline(always)]
    pub fn lsfe(&mut self) -> LSFE_W {
        LSFE_W { w: self }
    }
    #[doc = "Bit 7 - LON CRC Error Interrupt Disable"]
    #[inline(always)]
    pub fn lcrce(&mut self) -> LCRCE_W {
        LCRCE_W { w: self }
    }
    #[doc = "Bit 24 - LON Transmission Done Interrupt Disable"]
    #[inline(always)]
    pub fn ltxd(&mut self) -> LTXD_W {
        LTXD_W { w: self }
    }
    #[doc = "Bit 25 - LON Collision Interrupt Disable"]
    #[inline(always)]
    pub fn lcol(&mut self) -> LCOL_W {
        LCOL_W { w: self }
    }
    #[doc = "Bit 26 - LON Frame Early Termination Interrupt Disable"]
    #[inline(always)]
    pub fn lfet(&mut self) -> LFET_W {
        LFET_W { w: self }
    }
    #[doc = "Bit 27 - LON Reception Done Interrupt Disable"]
    #[inline(always)]
    pub fn lrxd(&mut self) -> LRXD_W {
        LRXD_W { w: self }
    }
    #[doc = "Bit 28 - LON Backlog Overflow Error Interrupt Disable"]
    #[inline(always)]
    pub fn lblovfe(&mut self) -> LBLOVFE_W {
        LBLOVFE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_idr_lon_mode](index.html) module"]
pub struct US_IDR_LON_MODE_SPEC;
impl crate::RegisterSpec for US_IDR_LON_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [us_idr_lon_mode::W](W) writer structure"]
impl crate::Writable for US_IDR_LON_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_IDR_LON_MODE to value 0"]
impl crate::Resettable for US_IDR_LON_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
