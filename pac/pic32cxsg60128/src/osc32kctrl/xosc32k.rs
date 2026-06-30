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
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Oscillator Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startupselect {
    #[doc = "0: 62.6 ms"]
    Cycle2048 = 0,
    #[doc = "1: 125 ms"]
    Cycle4096 = 1,
    #[doc = "2: 500 ms"]
    Cycle16384 = 2,
    #[doc = "3: 1000 ms"]
    Cycle32768 = 3,
    #[doc = "4: 2000 ms"]
    Cycle65536 = 4,
    #[doc = "5: 4000 ms"]
    Cycle131072 = 5,
    #[doc = "6: 8000 ms"]
    Cycle262144 = 6,
}
impl From<Startupselect> for u8 {
    #[inline(always)]
    fn from(variant: Startupselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startupselect {
    type Ux = u8;
}
impl crate::IsEnum for Startupselect {}
#[doc = "Field `STARTUP` reader - Oscillator Start-Up Time"]
pub type StartupR = crate::FieldReader<Startupselect>;
impl StartupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Startupselect> {
        match self.bits {
            0 => Some(Startupselect::Cycle2048),
            1 => Some(Startupselect::Cycle4096),
            2 => Some(Startupselect::Cycle16384),
            3 => Some(Startupselect::Cycle32768),
            4 => Some(Startupselect::Cycle65536),
            5 => Some(Startupselect::Cycle131072),
            6 => Some(Startupselect::Cycle262144),
            _ => None,
        }
    }
    #[doc = "62.6 ms"]
    #[inline(always)]
    pub fn is_cycle2048(&self) -> bool {
        *self == Startupselect::Cycle2048
    }
    #[doc = "125 ms"]
    #[inline(always)]
    pub fn is_cycle4096(&self) -> bool {
        *self == Startupselect::Cycle4096
    }
    #[doc = "500 ms"]
    #[inline(always)]
    pub fn is_cycle16384(&self) -> bool {
        *self == Startupselect::Cycle16384
    }
    #[doc = "1000 ms"]
    #[inline(always)]
    pub fn is_cycle32768(&self) -> bool {
        *self == Startupselect::Cycle32768
    }
    #[doc = "2000 ms"]
    #[inline(always)]
    pub fn is_cycle65536(&self) -> bool {
        *self == Startupselect::Cycle65536
    }
    #[doc = "4000 ms"]
    #[inline(always)]
    pub fn is_cycle131072(&self) -> bool {
        *self == Startupselect::Cycle131072
    }
    #[doc = "8000 ms"]
    #[inline(always)]
    pub fn is_cycle262144(&self) -> bool {
        *self == Startupselect::Cycle262144
    }
}
#[doc = "Field `STARTUP` writer - Oscillator Start-Up Time"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 3, Startupselect>;
impl<'a, REG> StartupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "62.6 ms"]
    #[inline(always)]
    pub fn cycle2048(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle2048)
    }
    #[doc = "125 ms"]
    #[inline(always)]
    pub fn cycle4096(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle4096)
    }
    #[doc = "500 ms"]
    #[inline(always)]
    pub fn cycle16384(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle16384)
    }
    #[doc = "1000 ms"]
    #[inline(always)]
    pub fn cycle32768(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle32768)
    }
    #[doc = "2000 ms"]
    #[inline(always)]
    pub fn cycle65536(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle65536)
    }
    #[doc = "4000 ms"]
    #[inline(always)]
    pub fn cycle131072(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle131072)
    }
    #[doc = "8000 ms"]
    #[inline(always)]
    pub fn cycle262144(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle262144)
    }
}
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Control Gain Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cgmselect {
    #[doc = "1: Standard mode"]
    Xt = 1,
    #[doc = "2: High Speed mode"]
    Hs = 2,
}
impl From<Cgmselect> for u8 {
    #[inline(always)]
    fn from(variant: Cgmselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cgmselect {
    type Ux = u8;
}
impl crate::IsEnum for Cgmselect {}
#[doc = "Field `CGM` reader - Control Gain Mode"]
pub type CgmR = crate::FieldReader<Cgmselect>;
impl CgmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cgmselect> {
        match self.bits {
            1 => Some(Cgmselect::Xt),
            2 => Some(Cgmselect::Hs),
            _ => None,
        }
    }
    #[doc = "Standard mode"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == Cgmselect::Xt
    }
    #[doc = "High Speed mode"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == Cgmselect::Hs
    }
}
#[doc = "Field `CGM` writer - Control Gain Mode"]
pub type CgmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cgmselect>;
impl<'a, REG> CgmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard mode"]
    #[inline(always)]
    pub fn xt(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Xt)
    }
    #[doc = "High Speed mode"]
    #[inline(always)]
    pub fn hs(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Hs)
    }
}
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
    #[doc = "Bits 13:14 - Control Gain Mode"]
    #[inline(always)]
    pub fn cgm(&self) -> CgmR {
        CgmR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<Xosc32kSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xtalen(&mut self) -> XtalenW<Xosc32kSpec> {
        XtalenW::new(self, 2)
    }
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en32k(&mut self) -> En32kW<Xosc32kSpec> {
        En32kW::new(self, 3)
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1k(&mut self) -> En1kW<Xosc32kSpec> {
        En1kW::new(self, 4)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<Xosc32kSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> OndemandW<Xosc32kSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> StartupW<Xosc32kSpec> {
        StartupW::new(self, 8)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WrtlockW<Xosc32kSpec> {
        WrtlockW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Control Gain Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cgm(&mut self) -> CgmW<Xosc32kSpec> {
        CgmW::new(self, 13)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets XOSC32K to value 0x2080"]
impl crate::Resettable for Xosc32kSpec {
    const RESET_VALUE: u16 = 0x2080;
}
