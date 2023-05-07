#[doc = "Register `OSC32K` reader"]
pub struct R(crate::R<OSC32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC32K` writer"]
pub struct W(crate::W<OSC32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC32K_SPEC>;
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
impl From<crate::W<OSC32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC32K_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC32K_SPEC, bool, O>;
#[doc = "Field `EN32K` reader - 32kHz Output Enable"]
pub type EN32K_R = crate::BitReader<bool>;
#[doc = "Field `EN32K` writer - 32kHz Output Enable"]
pub type EN32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC32K_SPEC, bool, O>;
#[doc = "Field `EN1K` reader - 1kHz Output Enable"]
pub type EN1K_R = crate::BitReader<bool>;
#[doc = "Field `EN1K` writer - 1kHz Output Enable"]
pub type EN1K_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC32K_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC32K_SPEC, bool, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC32K_SPEC, bool, O>;
#[doc = "Field `STARTUP` reader - Oscillator Start-Up Time"]
pub type STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTUP` writer - Oscillator Start-Up Time"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSC32K_SPEC, u8, u8, 3, O>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WRTLOCK_R = crate::BitReader<bool>;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WRTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC32K_SPEC, bool, O>;
#[doc = "Field `CALIB` reader - Oscillator Calibration"]
pub type CALIB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALIB` writer - Oscillator Calibration"]
pub type CALIB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSC32K_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - 32kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en32k(&mut self) -> EN32K_W<2> {
        EN32K_W::new(self)
    }
    #[doc = "Bit 3 - 1kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1k(&mut self) -> EN1K_W<3> {
        EN1K_W::new(self)
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
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<8> {
        STARTUP_W::new(self)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WRTLOCK_W<12> {
        WRTLOCK_W::new(self)
    }
    #[doc = "Bits 16:22 - Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CALIB_W<16> {
        CALIB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32kHz Internal Oscillator (OSC32K) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc32k](index.html) module"]
pub struct OSC32K_SPEC;
impl crate::RegisterSpec for OSC32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc32k::R](R) reader structure"]
impl crate::Readable for OSC32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc32k::W](W) writer structure"]
impl crate::Writable for OSC32K_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC32K to value 0x003f_0080"]
impl crate::Resettable for OSC32K_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_0080;
}
