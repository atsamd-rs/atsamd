#[doc = "Register `PMC_SCER` writer"]
pub struct W(crate::W<PMC_SCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_SCER_SPEC>;
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
impl From<crate::W<PMC_SCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_SCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBCLK` writer - Enable USB FS Clock"]
pub struct USBCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCLK_W<'a> {
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
#[doc = "Field `PCK0` writer - Programmable Clock 0 Output Enable"]
pub struct PCK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK0_W<'a> {
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
#[doc = "Field `PCK1` writer - Programmable Clock 1 Output Enable"]
pub struct PCK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK1_W<'a> {
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
#[doc = "Field `PCK2` writer - Programmable Clock 2 Output Enable"]
pub struct PCK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK2_W<'a> {
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
#[doc = "Field `PCK3` writer - Programmable Clock 3 Output Enable"]
pub struct PCK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK3_W<'a> {
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
#[doc = "Field `PCK4` writer - Programmable Clock 4 Output Enable"]
pub struct PCK4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK4_W<'a> {
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
#[doc = "Field `PCK5` writer - Programmable Clock 5 Output Enable"]
pub struct PCK5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK5_W<'a> {
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
#[doc = "Field `PCK6` writer - Programmable Clock 6 Output Enable"]
pub struct PCK6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK6_W<'a> {
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
#[doc = "Field `PCK7` writer - Programmable Clock 7 Output Enable"]
pub struct PCK7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK7_W<'a> {
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
impl W {
    #[doc = "Bit 5 - Enable USB FS Clock"]
    #[inline(always)]
    pub fn usbclk(&mut self) -> USBCLK_W {
        USBCLK_W { w: self }
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Enable"]
    #[inline(always)]
    pub fn pck0(&mut self) -> PCK0_W {
        PCK0_W { w: self }
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Enable"]
    #[inline(always)]
    pub fn pck1(&mut self) -> PCK1_W {
        PCK1_W { w: self }
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Enable"]
    #[inline(always)]
    pub fn pck2(&mut self) -> PCK2_W {
        PCK2_W { w: self }
    }
    #[doc = "Bit 11 - Programmable Clock 3 Output Enable"]
    #[inline(always)]
    pub fn pck3(&mut self) -> PCK3_W {
        PCK3_W { w: self }
    }
    #[doc = "Bit 12 - Programmable Clock 4 Output Enable"]
    #[inline(always)]
    pub fn pck4(&mut self) -> PCK4_W {
        PCK4_W { w: self }
    }
    #[doc = "Bit 13 - Programmable Clock 5 Output Enable"]
    #[inline(always)]
    pub fn pck5(&mut self) -> PCK5_W {
        PCK5_W { w: self }
    }
    #[doc = "Bit 14 - Programmable Clock 6 Output Enable"]
    #[inline(always)]
    pub fn pck6(&mut self) -> PCK6_W {
        PCK6_W { w: self }
    }
    #[doc = "Bit 15 - Programmable Clock 7 Output Enable"]
    #[inline(always)]
    pub fn pck7(&mut self) -> PCK7_W {
        PCK7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_scer](index.html) module"]
pub struct PMC_SCER_SPEC;
impl crate::RegisterSpec for PMC_SCER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_scer::W](W) writer structure"]
impl crate::Writable for PMC_SCER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_SCER to value 0"]
impl crate::Resettable for PMC_SCER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
