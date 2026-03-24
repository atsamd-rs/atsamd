#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Event Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evactselect {
    #[doc = "0: Event action disabled"]
    Off = 0,
    #[doc = "1: Start, restart or retrigger TC on event"]
    Retrigger = 1,
    #[doc = "2: Count on event"]
    Count = 2,
    #[doc = "3: Start TC on event"]
    Start = 3,
    #[doc = "4: Time stamp capture"]
    Stamp = 4,
    #[doc = "5: Period catured in CC0, pulse width in CC1"]
    Ppw = 5,
    #[doc = "6: Period catured in CC1, pulse width in CC0"]
    Pwp = 6,
    #[doc = "7: Pulse width capture"]
    Pw = 7,
}
impl From<Evactselect> for u8 {
    #[inline(always)]
    fn from(variant: Evactselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evactselect {
    type Ux = u8;
}
impl crate::IsEnum for Evactselect {}
#[doc = "Field `EVACT` reader - Event Action"]
pub type EvactR = crate::FieldReader<Evactselect>;
impl EvactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evactselect {
        match self.bits {
            0 => Evactselect::Off,
            1 => Evactselect::Retrigger,
            2 => Evactselect::Count,
            3 => Evactselect::Start,
            4 => Evactselect::Stamp,
            5 => Evactselect::Ppw,
            6 => Evactselect::Pwp,
            7 => Evactselect::Pw,
            _ => unreachable!(),
        }
    }
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Evactselect::Off
    }
    #[doc = "Start, restart or retrigger TC on event"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == Evactselect::Retrigger
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn is_count(&self) -> bool {
        *self == Evactselect::Count
    }
    #[doc = "Start TC on event"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Evactselect::Start
    }
    #[doc = "Time stamp capture"]
    #[inline(always)]
    pub fn is_stamp(&self) -> bool {
        *self == Evactselect::Stamp
    }
    #[doc = "Period catured in CC0, pulse width in CC1"]
    #[inline(always)]
    pub fn is_ppw(&self) -> bool {
        *self == Evactselect::Ppw
    }
    #[doc = "Period catured in CC1, pulse width in CC0"]
    #[inline(always)]
    pub fn is_pwp(&self) -> bool {
        *self == Evactselect::Pwp
    }
    #[doc = "Pulse width capture"]
    #[inline(always)]
    pub fn is_pw(&self) -> bool {
        *self == Evactselect::Pw
    }
}
#[doc = "Field `EVACT` writer - Event Action"]
pub type EvactW<'a, REG> = crate::FieldWriter<'a, REG, 3, Evactselect, crate::Safe>;
impl<'a, REG> EvactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Off)
    }
    #[doc = "Start, restart or retrigger TC on event"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Retrigger)
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn count(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Count)
    }
    #[doc = "Start TC on event"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Start)
    }
    #[doc = "Time stamp capture"]
    #[inline(always)]
    pub fn stamp(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Stamp)
    }
    #[doc = "Period catured in CC0, pulse width in CC1"]
    #[inline(always)]
    pub fn ppw(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Ppw)
    }
    #[doc = "Period catured in CC1, pulse width in CC0"]
    #[inline(always)]
    pub fn pwp(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Pwp)
    }
    #[doc = "Pulse width capture"]
    #[inline(always)]
    pub fn pw(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Pw)
    }
}
#[doc = "Field `TCINV` reader - TC Event Input Polarity"]
pub type TcinvR = crate::BitReader;
#[doc = "Field `TCINV` writer - TC Event Input Polarity"]
pub type TcinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCEI` reader - TC Event Enable"]
pub type TceiR = crate::BitReader;
#[doc = "Field `TCEI` writer - TC Event Enable"]
pub type TceiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFEO` reader - Event Output Enable"]
pub type OvfeoR = crate::BitReader;
#[doc = "Field `OVFEO` writer - Event Output Enable"]
pub type OvfeoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEO0` reader - MC Event Output Enable 0"]
pub type Mceo0R = crate::BitReader;
#[doc = "Field `MCEO0` writer - MC Event Output Enable 0"]
pub type Mceo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEO1` reader - MC Event Output Enable 1"]
pub type Mceo1R = crate::BitReader;
#[doc = "Field `MCEO1` writer - MC Event Output Enable 1"]
pub type Mceo1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Event Action"]
    #[inline(always)]
    pub fn evact(&self) -> EvactR {
        EvactR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - TC Event Input Polarity"]
    #[inline(always)]
    pub fn tcinv(&self) -> TcinvR {
        TcinvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC Event Enable"]
    #[inline(always)]
    pub fn tcei(&self) -> TceiR {
        TceiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OvfeoR {
        OvfeoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - MC Event Output Enable 0"]
    #[inline(always)]
    pub fn mceo0(&self) -> Mceo0R {
        Mceo0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MC Event Output Enable 1"]
    #[inline(always)]
    pub fn mceo1(&self) -> Mceo1R {
        Mceo1R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact(&mut self) -> EvactW<EvctrlSpec> {
        EvactW::new(self, 0)
    }
    #[doc = "Bit 4 - TC Event Input Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn tcinv(&mut self) -> TcinvW<EvctrlSpec> {
        TcinvW::new(self, 4)
    }
    #[doc = "Bit 5 - TC Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcei(&mut self) -> TceiW<EvctrlSpec> {
        TceiW::new(self, 5)
    }
    #[doc = "Bit 8 - Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfeo(&mut self) -> OvfeoW<EvctrlSpec> {
        OvfeoW::new(self, 8)
    }
    #[doc = "Bit 12 - MC Event Output Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn mceo0(&mut self) -> Mceo0W<EvctrlSpec> {
        Mceo0W::new(self, 12)
    }
    #[doc = "Bit 13 - MC Event Output Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn mceo1(&mut self) -> Mceo1W<EvctrlSpec> {
        Mceo1W::new(self, 13)
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {
    const RESET_VALUE: u16 = 0;
}
