#[doc = "Register `XOSCCTRL` reader"]
pub struct R(crate::R<XOSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSCCTRL` writer"]
pub struct W(crate::W<XOSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSCCTRL_SPEC>;
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
impl From<crate::W<XOSCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, O>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XTALEN_R = crate::BitReader<bool>;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XTALEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, O>;
#[doc = "Field `CFDEN` reader - Xosc Clock Failure Detecteor Enable"]
pub type CFDEN_R = crate::BitReader<bool>;
#[doc = "Field `CFDEN` writer - Xosc Clock Failure Detecteor Enable"]
pub type CFDEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, O>;
#[doc = "Field `SWBEN` reader - Xosc Clock Switch Enable"]
pub type SWBEN_R = crate::BitReader<bool>;
#[doc = "Field `SWBEN` writer - Xosc Clock Switch Enable"]
pub type SWBEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, O>;
#[doc = "Field `GAIN` reader - Oscillator Gain"]
pub type GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN` writer - Oscillator Gain"]
pub type GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, XOSCCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `AMPGC` reader - Automatic Amplitude Gain Control"]
pub type AMPGC_R = crate::BitReader<bool>;
#[doc = "Field `AMPGC` writer - Automatic Amplitude Gain Control"]
pub type AMPGC_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, O>;
#[doc = "Field `STARTUP` reader - Start-Up Time"]
pub type STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTUP` writer - Start-Up Time"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, XOSCCTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XTALEN_R {
        XTALEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Xosc Clock Failure Detecteor Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Xosc Clock Switch Enable"]
    #[inline(always)]
    pub fn swben(&self) -> SWBEN_R {
        SWBEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    pub fn ampgc(&self) -> AMPGC_R {
        AMPGC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xtalen(&mut self) -> XTALEN_W<2> {
        XTALEN_W::new(self)
    }
    #[doc = "Bit 3 - Xosc Clock Failure Detecteor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CFDEN_W<3> {
        CFDEN_W::new(self)
    }
    #[doc = "Bit 4 - Xosc Clock Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swben(&mut self) -> SWBEN_W<4> {
        SWBEN_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<8> {
        GAIN_W::new(self)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    #[must_use]
    pub fn ampgc(&mut self) -> AMPGC_W<11> {
        AMPGC_W::new(self)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<12> {
        STARTUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xoscctrl](index.html) module"]
pub struct XOSCCTRL_SPEC;
impl crate::RegisterSpec for XOSCCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xoscctrl::R](R) reader structure"]
impl crate::Readable for XOSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xoscctrl::W](W) writer structure"]
impl crate::Writable for XOSCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XOSCCTRL to value 0x80"]
impl crate::Resettable for XOSCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
