#[doc = "Register `SUPC_CR` writer"]
pub struct W(crate::W<SUPC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUPC_CR_SPEC>;
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
impl From<crate::W<SUPC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUPC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Voltage Regulator Off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VROFF_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: If KEY is correct, VROFF asserts the vddcore_nreset and stops the voltage regulator."]
    STOP_VREG = 1,
}
impl From<VROFF_AW> for bool {
    #[inline(always)]
    fn from(variant: VROFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VROFF` writer - Voltage Regulator Off"]
pub struct VROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> VROFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VROFF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(VROFF_AW::NO_EFFECT)
    }
    #[doc = "If KEY is correct, VROFF asserts the vddcore_nreset and stops the voltage regulator."]
    #[inline(always)]
    pub fn stop_vreg(self) -> &'a mut W {
        self.variant(VROFF_AW::STOP_VREG)
    }
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
#[doc = "Crystal Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALSEL_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: If KEY is correct, XTALSEL switches the slow clock on the crystal oscillator output."]
    CRYSTAL_SEL = 1,
}
impl From<XTALSEL_AW> for bool {
    #[inline(always)]
    fn from(variant: XTALSEL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALSEL` writer - Crystal Oscillator Select"]
pub struct XTALSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTALSEL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(XTALSEL_AW::NO_EFFECT)
    }
    #[doc = "If KEY is correct, XTALSEL switches the slow clock on the crystal oscillator output."]
    #[inline(always)]
    pub fn crystal_sel(self) -> &'a mut W {
        self.variant(XTALSEL_AW::CRYSTAL_SEL)
    }
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
#[doc = "Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_AW {
    #[doc = "165: Writing any other value in this field aborts the write operation."]
    PASSWD = 165,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - Password"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEY_AW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bit 2 - Voltage Regulator Off"]
    #[inline(always)]
    pub fn vroff(&mut self) -> VROFF_W {
        VROFF_W { w: self }
    }
    #[doc = "Bit 3 - Crystal Oscillator Select"]
    #[inline(always)]
    pub fn xtalsel(&mut self) -> XTALSEL_W {
        XTALSEL_W { w: self }
    }
    #[doc = "Bits 24:31 - Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supc_cr](index.html) module"]
pub struct SUPC_CR_SPEC;
impl crate::RegisterSpec for SUPC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [supc_cr::W](W) writer structure"]
impl crate::Writable for SUPC_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUPC_CR to value 0"]
impl crate::Resettable for SUPC_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
