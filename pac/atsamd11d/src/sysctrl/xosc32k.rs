#[doc = "Register `XOSC32K` reader"]
pub type R = crate::R<Xosc32kSpec>;
#[doc = "Register `XOSC32K` writer"]
pub type W = crate::W<Xosc32kSpec>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XtalenR = crate::BitReader;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XtalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN32K` reader - 32kHz Output Enable"]
pub type En32kR = crate::BitReader;
#[doc = "Field `EN32K` writer - 32kHz Output Enable"]
pub type En32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN1K` reader - 1kHz Output Enable"]
pub type En1kR = crate::BitReader;
#[doc = "Field `EN1K` writer - 1kHz Output Enable"]
pub type En1kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAMPEN` reader - Automatic Amplitude Control Enable"]
pub type AampenR = crate::BitReader;
#[doc = "Field `AAMPEN` writer - Automatic Amplitude Control Enable"]
pub type AampenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTUP` reader - Oscillator Start-Up Time"]
pub type StartupR = crate::FieldReader;
#[doc = "Field `STARTUP` writer - Oscillator Start-Up Time"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XtalenR {
        XtalenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> En32kR {
        En32kR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> En1kR {
        En1kR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Amplitude Control Enable"]
    #[inline(always)]
    pub fn aampen(&self) -> AampenR {
        AampenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<Xosc32kSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&mut self) -> XtalenW<Xosc32kSpec> {
        XtalenW::new(self, 2)
    }
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&mut self) -> En32kW<Xosc32kSpec> {
        En32kW::new(self, 3)
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&mut self) -> En1kW<Xosc32kSpec> {
        En1kW::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic Amplitude Control Enable"]
    #[inline(always)]
    pub fn aampen(&mut self) -> AampenW<Xosc32kSpec> {
        AampenW::new(self, 5)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<Xosc32kSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<Xosc32kSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> StartupW<Xosc32kSpec> {
        StartupW::new(self, 8)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WrtlockW<Xosc32kSpec> {
        WrtlockW::new(self, 12)
    }
}
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32k::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc32k::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xosc32kSpec;
impl crate::RegisterSpec for Xosc32kSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`xosc32k::R`](R) reader structure"]
impl crate::Readable for Xosc32kSpec {}
#[doc = "`write(|w| ..)` method takes [`xosc32k::W`](W) writer structure"]
impl crate::Writable for Xosc32kSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XOSC32K to value 0x80"]
impl crate::Resettable for Xosc32kSpec {
    const RESET_VALUE: u16 = 0x80;
}
