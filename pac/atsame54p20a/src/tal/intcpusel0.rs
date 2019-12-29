#[doc = "Reader of register INTCPUSEL0"]
pub type R = crate::R<u32, super::INTCPUSEL0>;
#[doc = "Writer for register INTCPUSEL0"]
pub type W = crate::W<u32, super::INTCPUSEL0>;
#[doc = "Register INTCPUSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCPUSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PAC`"]
pub type PAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAC`"]
pub struct PAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PAC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `PM`"]
pub type PM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PM`"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MCLK`"]
pub type MCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCLK`"]
pub struct MCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `OSCCTRL`"]
pub type OSCCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSCCTRL`"]
pub struct OSCCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCCTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `OSC32KCTRL`"]
pub type OSC32KCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC32KCTRL`"]
pub struct OSC32KCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32KCTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SUPC`"]
pub type SUPC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUPC`"]
pub struct SUPC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `WDT`"]
pub type WDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT`"]
pub struct WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `EIC`"]
pub type EIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EIC`"]
pub struct EIC_W<'a> {
    w: &'a mut W,
}
impl<'a> EIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `FREQM`"]
pub type FREQM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FREQM`"]
pub struct FREQM_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SERCOM0`"]
pub type SERCOM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERCOM0`"]
pub struct SERCOM0_W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SERCOM1`"]
pub type SERCOM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERCOM1`"]
pub struct SERCOM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `TC0`"]
pub type TC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC0`"]
pub struct TC0_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `TC1`"]
pub type TC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC1`"]
pub struct TC1_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PAC Interrupt CPU Select"]
    #[inline(always)]
    pub fn pac(&self) -> PAC_R {
        PAC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - PM Interrupt CPU Select"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MCLK Interrupt CPU Select"]
    #[inline(always)]
    pub fn mclk(&self) -> MCLK_R {
        MCLK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OSCCTRL Interrupt CPU Select"]
    #[inline(always)]
    pub fn oscctrl(&self) -> OSCCTRL_R {
        OSCCTRL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - OSC32KCTRL Interrupt CPU Select"]
    #[inline(always)]
    pub fn osc32kctrl(&self) -> OSC32KCTRL_R {
        OSC32KCTRL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SUPC Interrupt CPU Select"]
    #[inline(always)]
    pub fn supc(&self) -> SUPC_R {
        SUPC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - WDT Interrupt CPU Select"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RTC Interrupt CPU Select"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - EIC Interrupt CPU Select"]
    #[inline(always)]
    pub fn eic(&self) -> EIC_R {
        EIC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - FREQM Interrupt CPU Select"]
    #[inline(always)]
    pub fn freqm(&self) -> FREQM_R {
        FREQM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SERCOM0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom0(&self) -> SERCOM0_R {
        SERCOM0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SERCOM1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom1(&self) -> SERCOM1_R {
        SERCOM1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TC0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc0(&self) -> TC0_R {
        TC0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TC1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc1(&self) -> TC1_R {
        TC1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC Interrupt CPU Select"]
    #[inline(always)]
    pub fn pac(&mut self) -> PAC_W {
        PAC_W { w: self }
    }
    #[doc = "Bit 2 - PM Interrupt CPU Select"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 4 - MCLK Interrupt CPU Select"]
    #[inline(always)]
    pub fn mclk(&mut self) -> MCLK_W {
        MCLK_W { w: self }
    }
    #[doc = "Bit 8 - OSCCTRL Interrupt CPU Select"]
    #[inline(always)]
    pub fn oscctrl(&mut self) -> OSCCTRL_W {
        OSCCTRL_W { w: self }
    }
    #[doc = "Bit 10 - OSC32KCTRL Interrupt CPU Select"]
    #[inline(always)]
    pub fn osc32kctrl(&mut self) -> OSC32KCTRL_W {
        OSC32KCTRL_W { w: self }
    }
    #[doc = "Bit 12 - SUPC Interrupt CPU Select"]
    #[inline(always)]
    pub fn supc(&mut self) -> SUPC_W {
        SUPC_W { w: self }
    }
    #[doc = "Bit 16 - WDT Interrupt CPU Select"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W {
        WDT_W { w: self }
    }
    #[doc = "Bit 18 - RTC Interrupt CPU Select"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 20 - EIC Interrupt CPU Select"]
    #[inline(always)]
    pub fn eic(&mut self) -> EIC_W {
        EIC_W { w: self }
    }
    #[doc = "Bit 22 - FREQM Interrupt CPU Select"]
    #[inline(always)]
    pub fn freqm(&mut self) -> FREQM_W {
        FREQM_W { w: self }
    }
    #[doc = "Bit 24 - SERCOM0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom0(&mut self) -> SERCOM0_W {
        SERCOM0_W { w: self }
    }
    #[doc = "Bit 26 - SERCOM1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom1(&mut self) -> SERCOM1_W {
        SERCOM1_W { w: self }
    }
    #[doc = "Bit 28 - TC0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc0(&mut self) -> TC0_W {
        TC0_W { w: self }
    }
    #[doc = "Bit 30 - TC1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc1(&mut self) -> TC1_W {
        TC1_W { w: self }
    }
}
