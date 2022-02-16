#[doc = "Register `PMC_IDR` writer"]
pub struct W(crate::W<PMC_IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_IDR_SPEC>;
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
impl From<crate::W<PMC_IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOSCXTS` writer - Main Crystal Oscillator Status Interrupt Disable"]
pub struct MOSCXTS_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCXTS_W<'a> {
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
#[doc = "Field `LOCKA` writer - PLLA Lock Interrupt Disable"]
pub struct LOCKA_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKA_W<'a> {
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
#[doc = "Field `MCKRDY` writer - Master Clock Ready Interrupt Disable"]
pub struct MCKRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKRDY_W<'a> {
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
#[doc = "Field `LOCKU` writer - UTMI PLL Lock Interrupt Disable"]
pub struct LOCKU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKU_W<'a> {
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
#[doc = "Field `PCKRDY0` writer - Programmable Clock Ready 0 Interrupt Disable"]
pub struct PCKRDY0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCKRDY0_W<'a> {
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
#[doc = "Field `PCKRDY1` writer - Programmable Clock Ready 1 Interrupt Disable"]
pub struct PCKRDY1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCKRDY1_W<'a> {
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
#[doc = "Field `PCKRDY2` writer - Programmable Clock Ready 2 Interrupt Disable"]
pub struct PCKRDY2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCKRDY2_W<'a> {
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
#[doc = "Field `PCKRDY3` writer - Programmable Clock Ready 3 Interrupt Disable"]
pub struct PCKRDY3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCKRDY3_W<'a> {
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
#[doc = "Field `PCKRDY4` writer - Programmable Clock Ready 4 Interrupt Disable"]
pub struct PCKRDY4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCKRDY4_W<'a> {
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
#[doc = "Field `PCKRDY5` writer - Programmable Clock Ready 5 Interrupt Disable"]
pub struct PCKRDY5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCKRDY5_W<'a> {
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
#[doc = "Field `PCKRDY6` writer - Programmable Clock Ready 6 Interrupt Disable"]
pub struct PCKRDY6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCKRDY6_W<'a> {
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
#[doc = "Field `PCKRDY7` writer - Programmable Clock Ready 7 Interrupt Disable"]
pub struct PCKRDY7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCKRDY7_W<'a> {
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
#[doc = "Field `MOSCSELS` writer - Main Clock Source Oscillator Selection Status Interrupt Disable"]
pub struct MOSCSELS_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCSELS_W<'a> {
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
#[doc = "Field `MOSCRCS` writer - Main RC Status Interrupt Disable"]
pub struct MOSCRCS_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCRCS_W<'a> {
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
#[doc = "Field `CFDEV` writer - Clock Failure Detector Event Interrupt Disable"]
pub struct CFDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEV_W<'a> {
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
#[doc = "Field `XT32KERR` writer - 32.768 kHz Crystal Oscillator Error Interrupt Disable"]
pub struct XT32KERR_W<'a> {
    w: &'a mut W,
}
impl<'a> XT32KERR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Disable"]
    #[inline(always)]
    pub fn moscxts(&mut self) -> MOSCXTS_W {
        MOSCXTS_W { w: self }
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Disable"]
    #[inline(always)]
    pub fn locka(&mut self) -> LOCKA_W {
        LOCKA_W { w: self }
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Disable"]
    #[inline(always)]
    pub fn mckrdy(&mut self) -> MCKRDY_W {
        MCKRDY_W { w: self }
    }
    #[doc = "Bit 6 - UTMI PLL Lock Interrupt Disable"]
    #[inline(always)]
    pub fn locku(&mut self) -> LOCKU_W {
        LOCKU_W { w: self }
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Disable"]
    #[inline(always)]
    pub fn pckrdy0(&mut self) -> PCKRDY0_W {
        PCKRDY0_W { w: self }
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Disable"]
    #[inline(always)]
    pub fn pckrdy1(&mut self) -> PCKRDY1_W {
        PCKRDY1_W { w: self }
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Disable"]
    #[inline(always)]
    pub fn pckrdy2(&mut self) -> PCKRDY2_W {
        PCKRDY2_W { w: self }
    }
    #[doc = "Bit 11 - Programmable Clock Ready 3 Interrupt Disable"]
    #[inline(always)]
    pub fn pckrdy3(&mut self) -> PCKRDY3_W {
        PCKRDY3_W { w: self }
    }
    #[doc = "Bit 12 - Programmable Clock Ready 4 Interrupt Disable"]
    #[inline(always)]
    pub fn pckrdy4(&mut self) -> PCKRDY4_W {
        PCKRDY4_W { w: self }
    }
    #[doc = "Bit 13 - Programmable Clock Ready 5 Interrupt Disable"]
    #[inline(always)]
    pub fn pckrdy5(&mut self) -> PCKRDY5_W {
        PCKRDY5_W { w: self }
    }
    #[doc = "Bit 14 - Programmable Clock Ready 6 Interrupt Disable"]
    #[inline(always)]
    pub fn pckrdy6(&mut self) -> PCKRDY6_W {
        PCKRDY6_W { w: self }
    }
    #[doc = "Bit 15 - Programmable Clock Ready 7 Interrupt Disable"]
    #[inline(always)]
    pub fn pckrdy7(&mut self) -> PCKRDY7_W {
        PCKRDY7_W { w: self }
    }
    #[doc = "Bit 16 - Main Clock Source Oscillator Selection Status Interrupt Disable"]
    #[inline(always)]
    pub fn moscsels(&mut self) -> MOSCSELS_W {
        MOSCSELS_W { w: self }
    }
    #[doc = "Bit 17 - Main RC Status Interrupt Disable"]
    #[inline(always)]
    pub fn moscrcs(&mut self) -> MOSCRCS_W {
        MOSCRCS_W { w: self }
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Disable"]
    #[inline(always)]
    pub fn cfdev(&mut self) -> CFDEV_W {
        CFDEV_W { w: self }
    }
    #[doc = "Bit 21 - 32.768 kHz Crystal Oscillator Error Interrupt Disable"]
    #[inline(always)]
    pub fn xt32kerr(&mut self) -> XT32KERR_W {
        XT32KERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_idr](index.html) module"]
pub struct PMC_IDR_SPEC;
impl crate::RegisterSpec for PMC_IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_idr::W](W) writer structure"]
impl crate::Writable for PMC_IDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_IDR to value 0"]
impl crate::Resettable for PMC_IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
