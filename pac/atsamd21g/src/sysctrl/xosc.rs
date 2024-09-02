#[doc = "Register `XOSC` reader"]
pub type R = crate::R<XoscSpec>;
#[doc = "Register `XOSC` writer"]
pub type W = crate::W<XoscSpec>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XtalenR = crate::BitReader;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XtalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Oscillator Gain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gainselect {
    #[doc = "0: 2MHz"]
    _0 = 0,
    #[doc = "1: 4MHz"]
    _1 = 1,
    #[doc = "2: 8MHz"]
    _2 = 2,
    #[doc = "3: 16MHz"]
    _3 = 3,
    #[doc = "4: 30MHz"]
    _4 = 4,
}
impl From<Gainselect> for u8 {
    #[inline(always)]
    fn from(variant: Gainselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gainselect {
    type Ux = u8;
}
impl crate::IsEnum for Gainselect {}
#[doc = "Field `GAIN` reader - Oscillator Gain"]
pub type GainR = crate::FieldReader<Gainselect>;
impl GainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gainselect> {
        match self.bits {
            0 => Some(Gainselect::_0),
            1 => Some(Gainselect::_1),
            2 => Some(Gainselect::_2),
            3 => Some(Gainselect::_3),
            4 => Some(Gainselect::_4),
            _ => None,
        }
    }
    #[doc = "2MHz"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gainselect::_0
    }
    #[doc = "4MHz"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gainselect::_1
    }
    #[doc = "8MHz"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Gainselect::_2
    }
    #[doc = "16MHz"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == Gainselect::_3
    }
    #[doc = "30MHz"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Gainselect::_4
    }
}
#[doc = "Field `GAIN` writer - Oscillator Gain"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 3, Gainselect>;
impl<'a, REG> GainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::_0)
    }
    #[doc = "4MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::_1)
    }
    #[doc = "8MHz"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::_2)
    }
    #[doc = "16MHz"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::_3)
    }
    #[doc = "30MHz"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::_4)
    }
}
#[doc = "Field `AMPGC` reader - Automatic Amplitude Gain Control"]
pub type AmpgcR = crate::BitReader;
#[doc = "Field `AMPGC` writer - Automatic Amplitude Gain Control"]
pub type AmpgcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTUP` reader - Start-Up Time"]
pub type StartupR = crate::FieldReader;
#[doc = "Field `STARTUP` writer - Start-Up Time"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    pub fn ampgc(&self) -> AmpgcR {
        AmpgcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<XoscSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xtalen(&mut self) -> XtalenW<XoscSpec> {
        XtalenW::new(self, 2)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<XoscSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> OndemandW<XoscSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GainW<XoscSpec> {
        GainW::new(self, 8)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    #[must_use]
    pub fn ampgc(&mut self) -> AmpgcW<XoscSpec> {
        AmpgcW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> StartupW<XoscSpec> {
        StartupW::new(self, 12)
    }
}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XoscSpec;
impl crate::RegisterSpec for XoscSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`xosc::R`](R) reader structure"]
impl crate::Readable for XoscSpec {}
#[doc = "`write(|w| ..)` method takes [`xosc::W`](W) writer structure"]
impl crate::Writable for XoscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets XOSC to value 0x80"]
impl crate::Resettable for XoscSpec {
    const RESET_VALUE: u16 = 0x80;
}
