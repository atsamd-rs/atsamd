#[doc = "Register `OSC32K` reader"]
pub type R = crate::R<OSC32K_SPEC>;
#[doc = "Register `OSC32K` writer"]
pub type W = crate::W<OSC32K_SPEC>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN32K` reader - 32kHz Output Enable"]
pub type EN32K_R = crate::BitReader;
#[doc = "Field `EN32K` writer - 32kHz Output Enable"]
pub type EN32K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN1K` reader - 1kHz Output Enable"]
pub type EN1K_R = crate::BitReader;
#[doc = "Field `EN1K` writer - 1kHz Output Enable"]
pub type EN1K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STARTUP` reader - Oscillator Start-Up Time"]
pub type STARTUP_R = crate::FieldReader;
#[doc = "Field `STARTUP` writer - Oscillator Start-Up Time"]
pub type STARTUP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WRTLOCK_R = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WRTLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALIB` reader - Oscillator Calibration"]
pub type CALIB_R = crate::FieldReader;
#[doc = "Field `CALIB` writer - Oscillator Calibration"]
pub type CALIB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
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
    pub fn enable(&mut self) -> ENABLE_W<OSC32K_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - 32kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en32k(&mut self) -> EN32K_W<OSC32K_SPEC, 2> {
        EN32K_W::new(self)
    }
    #[doc = "Bit 3 - 1kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1k(&mut self) -> EN1K_W<OSC32K_SPEC, 3> {
        EN1K_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<OSC32K_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<OSC32K_SPEC, 7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<OSC32K_SPEC, 8> {
        STARTUP_W::new(self)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WRTLOCK_W<OSC32K_SPEC, 12> {
        WRTLOCK_W::new(self)
    }
    #[doc = "Bits 16:22 - Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CALIB_W<OSC32K_SPEC, 16> {
        CALIB_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "32kHz Internal Oscillator (OSC32K) Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc32k::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc32k::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSC32K_SPEC;
impl crate::RegisterSpec for OSC32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc32k::R`](R) reader structure"]
impl crate::Readable for OSC32K_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osc32k::W`](W) writer structure"]
impl crate::Writable for OSC32K_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC32K to value 0x003f_0080"]
impl crate::Resettable for OSC32K_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_0080;
}
