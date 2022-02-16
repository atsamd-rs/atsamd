#[doc = "Register `QSPI_IER` writer"]
pub struct W(crate::W<QSPI_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_IER_SPEC>;
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
impl From<crate::W<QSPI_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDRF` writer - Receive Data Register Full Interrupt Enable"]
pub struct RDRF_W<'a> {
    w: &'a mut W,
}
impl<'a> RDRF_W<'a> {
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
#[doc = "Field `TDRE` writer - Transmit Data Register Empty Interrupt Enable"]
pub struct TDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRE_W<'a> {
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
#[doc = "Field `TXEMPTY` writer - Transmission Registers Empty Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `OVRES` writer - Overrun Error Interrupt Enable"]
pub struct OVRES_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRES_W<'a> {
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
#[doc = "Field `CSR` writer - Chip Select Rise Interrupt Enable"]
pub struct CSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR_W<'a> {
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
#[doc = "Field `CSS` writer - Chip Select Status Interrupt Enable"]
pub struct CSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSS_W<'a> {
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
#[doc = "Field `INSTRE` writer - Instruction End Interrupt Enable"]
pub struct INSTRE_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTRE_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Enable"]
    #[inline(always)]
    pub fn rdrf(&mut self) -> RDRF_W {
        RDRF_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tdre(&mut self) -> TDRE_W {
        TDRE_W { w: self }
    }
    #[doc = "Bit 2 - Transmission Registers Empty Enable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TXEMPTY_W {
        TXEMPTY_W { w: self }
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovres(&mut self) -> OVRES_W {
        OVRES_W { w: self }
    }
    #[doc = "Bit 8 - Chip Select Rise Interrupt Enable"]
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W {
        CSR_W { w: self }
    }
    #[doc = "Bit 9 - Chip Select Status Interrupt Enable"]
    #[inline(always)]
    pub fn css(&mut self) -> CSS_W {
        CSS_W { w: self }
    }
    #[doc = "Bit 10 - Instruction End Interrupt Enable"]
    #[inline(always)]
    pub fn instre(&mut self) -> INSTRE_W {
        INSTRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_ier](index.html) module"]
pub struct QSPI_IER_SPEC;
impl crate::RegisterSpec for QSPI_IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [qspi_ier::W](W) writer structure"]
impl crate::Writable for QSPI_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_IER to value 0"]
impl crate::Resettable for QSPI_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
