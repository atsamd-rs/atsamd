#[doc = "Register `WRCONFIG` writer"]
pub struct W(crate::W<WRCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRCONFIG_SPEC>;
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
impl From<crate::W<WRCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINMASK` writer - Pin Mask for Multiple Pin Configuration"]
pub struct PINMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `PMUXEN` writer - Peripheral Multiplexer Enable"]
pub struct PMUXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUXEN_W<'a> {
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
#[doc = "Field `INEN` writer - Input Enable"]
pub struct INEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN_W<'a> {
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
#[doc = "Field `PULLEN` writer - Pull Enable"]
pub struct PULLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLEN_W<'a> {
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
#[doc = "Field `DRVSTR` writer - Output Driver Strength Selection"]
pub struct DRVSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DRVSTR_W<'a> {
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
#[doc = "Field `PMUX` writer - Peripheral Multiplexing"]
pub struct PMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `WRPMUX` writer - Write PMUX"]
pub struct WRPMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPMUX_W<'a> {
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
#[doc = "Field `WRPINCFG` writer - Write PINCFG"]
pub struct WRPINCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPINCFG_W<'a> {
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
#[doc = "Field `HWSEL` writer - Half-Word Select"]
pub struct HWSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HWSEL_W<'a> {
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
    #[doc = "Bits 0:15 - Pin Mask for Multiple Pin Configuration"]
    #[inline(always)]
    pub fn pinmask(&mut self) -> PINMASK_W {
        PINMASK_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PMUXEN_W {
        PMUXEN_W { w: self }
    }
    #[doc = "Bit 17 - Input Enable"]
    #[inline(always)]
    pub fn inen(&mut self) -> INEN_W {
        INEN_W { w: self }
    }
    #[doc = "Bit 18 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&mut self) -> PULLEN_W {
        PULLEN_W { w: self }
    }
    #[doc = "Bit 22 - Output Driver Strength Selection"]
    #[inline(always)]
    pub fn drvstr(&mut self) -> DRVSTR_W {
        DRVSTR_W { w: self }
    }
    #[doc = "Bits 24:27 - Peripheral Multiplexing"]
    #[inline(always)]
    pub fn pmux(&mut self) -> PMUX_W {
        PMUX_W { w: self }
    }
    #[doc = "Bit 28 - Write PMUX"]
    #[inline(always)]
    pub fn wrpmux(&mut self) -> WRPMUX_W {
        WRPMUX_W { w: self }
    }
    #[doc = "Bit 30 - Write PINCFG"]
    #[inline(always)]
    pub fn wrpincfg(&mut self) -> WRPINCFG_W {
        WRPINCFG_W { w: self }
    }
    #[doc = "Bit 31 - Half-Word Select"]
    #[inline(always)]
    pub fn hwsel(&mut self) -> HWSEL_W {
        HWSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Configuration\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrconfig](index.html) module"]
pub struct WRCONFIG_SPEC;
impl crate::RegisterSpec for WRCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wrconfig::W](W) writer structure"]
impl crate::Writable for WRCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRCONFIG to value 0"]
impl crate::Resettable for WRCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
