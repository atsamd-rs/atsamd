#[doc = "Register `XOSC32K` reader"]
pub struct R(crate::R<XOSC32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSC32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSC32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSC32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSC32K` writer"]
pub struct W(crate::W<XOSC32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSC32K_SPEC>;
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
impl From<crate::W<XOSC32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSC32K_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XTALEN_R = crate::BitReader<bool>;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XTALEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `EN32K` reader - 32kHz Output Enable"]
pub type EN32K_R = crate::BitReader<bool>;
#[doc = "Field `EN32K` writer - 32kHz Output Enable"]
pub type EN32K_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `EN1K` reader - 1kHz Output Enable"]
pub type EN1K_R = crate::BitReader<bool>;
#[doc = "Field `EN1K` writer - 1kHz Output Enable"]
pub type EN1K_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `AAMPEN` reader - Automatic Amplitude Control Enable"]
pub type AAMPEN_R = crate::BitReader<bool>;
#[doc = "Field `AAMPEN` writer - Automatic Amplitude Control Enable"]
pub type AAMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `STARTUP` reader - Oscillator Start-Up Time"]
pub type STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTUP` writer - Oscillator Start-Up Time"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, XOSC32K_SPEC, u8, u8, 3, O>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WRTLOCK_R = crate::BitReader<bool>;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WRTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
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
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Amplitude Control Enable"]
    #[inline(always)]
    pub fn aampen(&self) -> AAMPEN_R {
        AAMPEN_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en32k(&mut self) -> EN32K_W<3> {
        EN32K_W::new(self)
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1k(&mut self) -> EN1K_W<4> {
        EN1K_W::new(self)
    }
    #[doc = "Bit 5 - Automatic Amplitude Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aampen(&mut self) -> AAMPEN_W<5> {
        AAMPEN_W::new(self)
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc32k](index.html) module"]
pub struct XOSC32K_SPEC;
impl crate::RegisterSpec for XOSC32K_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xosc32k::R](R) reader structure"]
impl crate::Readable for XOSC32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xosc32k::W](W) writer structure"]
impl crate::Writable for XOSC32K_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XOSC32K to value 0x80"]
impl crate::Resettable for XOSC32K_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
