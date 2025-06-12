#[doc = "Register `ALARM%s` reader"]
pub type R = crate::R<AlarmSpec>;
#[doc = "Register `ALARM%s` writer"]
pub type W = crate::W<AlarmSpec>;
#[doc = "Field `SECOND` reader - Second"]
pub type SecondR = crate::FieldReader;
#[doc = "Field `SECOND` writer - Second"]
pub type SecondW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MINUTE` reader - Minute"]
pub type MinuteR = crate::FieldReader;
#[doc = "Field `MINUTE` writer - Minute"]
pub type MinuteW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Hour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hourselect {
    #[doc = "0: Morning hour"]
    Am = 0,
    #[doc = "16: Afternoon hour"]
    Pm = 16,
}
impl From<Hourselect> for u8 {
    #[inline(always)]
    fn from(variant: Hourselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hourselect {
    type Ux = u8;
}
impl crate::IsEnum for Hourselect {}
#[doc = "Field `HOUR` reader - Hour"]
pub type HourR = crate::FieldReader<Hourselect>;
impl HourR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hourselect> {
        match self.bits {
            0 => Some(Hourselect::Am),
            16 => Some(Hourselect::Pm),
            _ => None,
        }
    }
    #[doc = "Morning hour"]
    #[inline(always)]
    pub fn is_am(&self) -> bool {
        *self == Hourselect::Am
    }
    #[doc = "Afternoon hour"]
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        *self == Hourselect::Pm
    }
}
#[doc = "Field `HOUR` writer - Hour"]
pub type HourW<'a, REG> = crate::FieldWriter<'a, REG, 5, Hourselect>;
impl<'a, REG> HourW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Morning hour"]
    #[inline(always)]
    pub fn am(self) -> &'a mut crate::W<REG> {
        self.variant(Hourselect::Am)
    }
    #[doc = "Afternoon hour"]
    #[inline(always)]
    pub fn pm(self) -> &'a mut crate::W<REG> {
        self.variant(Hourselect::Pm)
    }
}
#[doc = "Field `DAY` reader - Day"]
pub type DayR = crate::FieldReader;
#[doc = "Field `DAY` writer - Day"]
pub type DayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MONTH` reader - Month"]
pub type MonthR = crate::FieldReader;
#[doc = "Field `MONTH` writer - Month"]
pub type MonthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YEAR` reader - Year"]
pub type YearR = crate::FieldReader;
#[doc = "Field `YEAR` writer - Year"]
pub type YearW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Second"]
    #[inline(always)]
    pub fn second(&self) -> SecondR {
        SecondR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline(always)]
    pub fn minute(&self) -> MinuteR {
        MinuteR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:16 - Hour"]
    #[inline(always)]
    pub fn hour(&self) -> HourR {
        HourR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline(always)]
    pub fn month(&self) -> MonthR {
        MonthR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Second"]
    #[inline(always)]
    pub fn second(&mut self) -> SecondW<AlarmSpec> {
        SecondW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline(always)]
    pub fn minute(&mut self) -> MinuteW<AlarmSpec> {
        MinuteW::new(self, 6)
    }
    #[doc = "Bits 12:16 - Hour"]
    #[inline(always)]
    pub fn hour(&mut self) -> HourW<AlarmSpec> {
        HourW::new(self, 12)
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline(always)]
    pub fn day(&mut self) -> DayW<AlarmSpec> {
        DayW::new(self, 17)
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline(always)]
    pub fn month(&mut self) -> MonthW<AlarmSpec> {
        MonthW::new(self, 22)
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline(always)]
    pub fn year(&mut self) -> YearW<AlarmSpec> {
        YearW::new(self, 26)
    }
}
#[doc = "MODE2 Alarm n Value\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlarmSpec;
impl crate::RegisterSpec for AlarmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm::R`](R) reader structure"]
impl crate::Readable for AlarmSpec {}
#[doc = "`write(|w| ..)` method takes [`alarm::W`](W) writer structure"]
impl crate::Writable for AlarmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALARM%s to value 0"]
impl crate::Resettable for AlarmSpec {}
