#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Timer/counter Input Event0 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evact0select {
    #[doc = "0: Event action disabled"]
    Off = 0,
    #[doc = "1: Start, restart or re-trigger counter on event"]
    Retrigger = 1,
    #[doc = "2: Count on event"]
    Countev = 2,
    #[doc = "3: Start counter on event"]
    Start = 3,
    #[doc = "4: Increment counter on event"]
    Inc = 4,
    #[doc = "5: Count on active state of asynchronous event"]
    Count = 5,
    #[doc = "7: Non-recoverable fault"]
    Fault = 7,
}
impl From<Evact0select> for u8 {
    #[inline(always)]
    fn from(variant: Evact0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evact0select {
    type Ux = u8;
}
impl crate::IsEnum for Evact0select {}
#[doc = "Field `EVACT0` reader - Timer/counter Input Event0 Action"]
pub type Evact0R = crate::FieldReader<Evact0select>;
impl Evact0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Evact0select> {
        match self.bits {
            0 => Some(Evact0select::Off),
            1 => Some(Evact0select::Retrigger),
            2 => Some(Evact0select::Countev),
            3 => Some(Evact0select::Start),
            4 => Some(Evact0select::Inc),
            5 => Some(Evact0select::Count),
            7 => Some(Evact0select::Fault),
            _ => None,
        }
    }
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Evact0select::Off
    }
    #[doc = "Start, restart or re-trigger counter on event"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == Evact0select::Retrigger
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn is_countev(&self) -> bool {
        *self == Evact0select::Countev
    }
    #[doc = "Start counter on event"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Evact0select::Start
    }
    #[doc = "Increment counter on event"]
    #[inline(always)]
    pub fn is_inc(&self) -> bool {
        *self == Evact0select::Inc
    }
    #[doc = "Count on active state of asynchronous event"]
    #[inline(always)]
    pub fn is_count(&self) -> bool {
        *self == Evact0select::Count
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Evact0select::Fault
    }
}
#[doc = "Field `EVACT0` writer - Timer/counter Input Event0 Action"]
pub type Evact0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Evact0select>;
impl<'a, REG> Evact0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Off)
    }
    #[doc = "Start, restart or re-trigger counter on event"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Retrigger)
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn countev(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Countev)
    }
    #[doc = "Start counter on event"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Start)
    }
    #[doc = "Increment counter on event"]
    #[inline(always)]
    pub fn inc(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Inc)
    }
    #[doc = "Count on active state of asynchronous event"]
    #[inline(always)]
    pub fn count(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Count)
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Fault)
    }
}
#[doc = "Timer/counter Input Event1 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evact1select {
    #[doc = "0: Event action disabled"]
    Off = 0,
    #[doc = "1: Re-trigger counter on event"]
    Retrigger = 1,
    #[doc = "2: Direction control"]
    Dir = 2,
    #[doc = "3: Stop counter on event"]
    Stop = 3,
    #[doc = "4: Decrement counter on event"]
    Dec = 4,
    #[doc = "5: Period capture value in CC0 register, pulse width capture value in CC1 register"]
    Ppw = 5,
    #[doc = "6: Period capture value in CC1 register, pulse width capture value in CC0 register"]
    Pwp = 6,
    #[doc = "7: Non-recoverable fault"]
    Fault = 7,
}
impl From<Evact1select> for u8 {
    #[inline(always)]
    fn from(variant: Evact1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evact1select {
    type Ux = u8;
}
impl crate::IsEnum for Evact1select {}
#[doc = "Field `EVACT1` reader - Timer/counter Input Event1 Action"]
pub type Evact1R = crate::FieldReader<Evact1select>;
impl Evact1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evact1select {
        match self.bits {
            0 => Evact1select::Off,
            1 => Evact1select::Retrigger,
            2 => Evact1select::Dir,
            3 => Evact1select::Stop,
            4 => Evact1select::Dec,
            5 => Evact1select::Ppw,
            6 => Evact1select::Pwp,
            7 => Evact1select::Fault,
            _ => unreachable!(),
        }
    }
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Evact1select::Off
    }
    #[doc = "Re-trigger counter on event"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == Evact1select::Retrigger
    }
    #[doc = "Direction control"]
    #[inline(always)]
    pub fn is_dir(&self) -> bool {
        *self == Evact1select::Dir
    }
    #[doc = "Stop counter on event"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Evact1select::Stop
    }
    #[doc = "Decrement counter on event"]
    #[inline(always)]
    pub fn is_dec(&self) -> bool {
        *self == Evact1select::Dec
    }
    #[doc = "Period capture value in CC0 register, pulse width capture value in CC1 register"]
    #[inline(always)]
    pub fn is_ppw(&self) -> bool {
        *self == Evact1select::Ppw
    }
    #[doc = "Period capture value in CC1 register, pulse width capture value in CC0 register"]
    #[inline(always)]
    pub fn is_pwp(&self) -> bool {
        *self == Evact1select::Pwp
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Evact1select::Fault
    }
}
#[doc = "Field `EVACT1` writer - Timer/counter Input Event1 Action"]
pub type Evact1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Evact1select, crate::Safe>;
impl<'a, REG> Evact1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Evact1select::Off)
    }
    #[doc = "Re-trigger counter on event"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut crate::W<REG> {
        self.variant(Evact1select::Retrigger)
    }
    #[doc = "Direction control"]
    #[inline(always)]
    pub fn dir(self) -> &'a mut crate::W<REG> {
        self.variant(Evact1select::Dir)
    }
    #[doc = "Stop counter on event"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Evact1select::Stop)
    }
    #[doc = "Decrement counter on event"]
    #[inline(always)]
    pub fn dec(self) -> &'a mut crate::W<REG> {
        self.variant(Evact1select::Dec)
    }
    #[doc = "Period capture value in CC0 register, pulse width capture value in CC1 register"]
    #[inline(always)]
    pub fn ppw(self) -> &'a mut crate::W<REG> {
        self.variant(Evact1select::Ppw)
    }
    #[doc = "Period capture value in CC1 register, pulse width capture value in CC0 register"]
    #[inline(always)]
    pub fn pwp(self) -> &'a mut crate::W<REG> {
        self.variant(Evact1select::Pwp)
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut crate::W<REG> {
        self.variant(Evact1select::Fault)
    }
}
#[doc = "Timer/counter Output Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cntselselect {
    #[doc = "0: An interrupt/event is generated when a new counter cycle starts"]
    Start = 0,
    #[doc = "1: An interrupt/event is generated when a counter cycle ends"]
    End = 1,
    #[doc = "2: An interrupt/event is generated when a counter cycle ends, except for the first and last cycles"]
    Between = 2,
    #[doc = "3: An interrupt/event is generated when a new counter cycle starts or a counter cycle ends"]
    Boundary = 3,
}
impl From<Cntselselect> for u8 {
    #[inline(always)]
    fn from(variant: Cntselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cntselselect {
    type Ux = u8;
}
impl crate::IsEnum for Cntselselect {}
#[doc = "Field `CNTSEL` reader - Timer/counter Output Event Mode"]
pub type CntselR = crate::FieldReader<Cntselselect>;
impl CntselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntselselect {
        match self.bits {
            0 => Cntselselect::Start,
            1 => Cntselselect::End,
            2 => Cntselselect::Between,
            3 => Cntselselect::Boundary,
            _ => unreachable!(),
        }
    }
    #[doc = "An interrupt/event is generated when a new counter cycle starts"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Cntselselect::Start
    }
    #[doc = "An interrupt/event is generated when a counter cycle ends"]
    #[inline(always)]
    pub fn is_end(&self) -> bool {
        *self == Cntselselect::End
    }
    #[doc = "An interrupt/event is generated when a counter cycle ends, except for the first and last cycles"]
    #[inline(always)]
    pub fn is_between(&self) -> bool {
        *self == Cntselselect::Between
    }
    #[doc = "An interrupt/event is generated when a new counter cycle starts or a counter cycle ends"]
    #[inline(always)]
    pub fn is_boundary(&self) -> bool {
        *self == Cntselselect::Boundary
    }
}
#[doc = "Field `CNTSEL` writer - Timer/counter Output Event Mode"]
pub type CntselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cntselselect, crate::Safe>;
impl<'a, REG> CntselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "An interrupt/event is generated when a new counter cycle starts"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Cntselselect::Start)
    }
    #[doc = "An interrupt/event is generated when a counter cycle ends"]
    #[inline(always)]
    pub fn end(self) -> &'a mut crate::W<REG> {
        self.variant(Cntselselect::End)
    }
    #[doc = "An interrupt/event is generated when a counter cycle ends, except for the first and last cycles"]
    #[inline(always)]
    pub fn between(self) -> &'a mut crate::W<REG> {
        self.variant(Cntselselect::Between)
    }
    #[doc = "An interrupt/event is generated when a new counter cycle starts or a counter cycle ends"]
    #[inline(always)]
    pub fn boundary(self) -> &'a mut crate::W<REG> {
        self.variant(Cntselselect::Boundary)
    }
}
#[doc = "Field `OVFEO` reader - Overflow/Underflow Output Event Enable"]
pub type OvfeoR = crate::BitReader;
#[doc = "Field `OVFEO` writer - Overflow/Underflow Output Event Enable"]
pub type OvfeoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGEO` reader - Retrigger Output Event Enable"]
pub type TrgeoR = crate::BitReader;
#[doc = "Field `TRGEO` writer - Retrigger Output Event Enable"]
pub type TrgeoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTEO` reader - Timer/counter Output Event Enable"]
pub type CnteoR = crate::BitReader;
#[doc = "Field `CNTEO` writer - Timer/counter Output Event Enable"]
pub type CnteoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCINV0` reader - Inverted Event 0 Input Enable"]
pub type Tcinv0R = crate::BitReader;
#[doc = "Field `TCINV0` writer - Inverted Event 0 Input Enable"]
pub type Tcinv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCINV1` reader - Inverted Event 1 Input Enable"]
pub type Tcinv1R = crate::BitReader;
#[doc = "Field `TCINV1` writer - Inverted Event 1 Input Enable"]
pub type Tcinv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCEI0` reader - Timer/counter Event 0 Input Enable"]
pub type Tcei0R = crate::BitReader;
#[doc = "Field `TCEI0` writer - Timer/counter Event 0 Input Enable"]
pub type Tcei0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCEI1` reader - Timer/counter Event 1 Input Enable"]
pub type Tcei1R = crate::BitReader;
#[doc = "Field `TCEI1` writer - Timer/counter Event 1 Input Enable"]
pub type Tcei1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEI0` reader - Match or Capture Channel 0 Event Input Enable"]
pub type Mcei0R = crate::BitReader;
#[doc = "Field `MCEI0` writer - Match or Capture Channel 0 Event Input Enable"]
pub type Mcei0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEI1` reader - Match or Capture Channel 1 Event Input Enable"]
pub type Mcei1R = crate::BitReader;
#[doc = "Field `MCEI1` writer - Match or Capture Channel 1 Event Input Enable"]
pub type Mcei1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEI2` reader - Match or Capture Channel 2 Event Input Enable"]
pub type Mcei2R = crate::BitReader;
#[doc = "Field `MCEI2` writer - Match or Capture Channel 2 Event Input Enable"]
pub type Mcei2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEI3` reader - Match or Capture Channel 3 Event Input Enable"]
pub type Mcei3R = crate::BitReader;
#[doc = "Field `MCEI3` writer - Match or Capture Channel 3 Event Input Enable"]
pub type Mcei3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEO0` reader - Match or Capture Channel 0 Event Output Enable"]
pub type Mceo0R = crate::BitReader;
#[doc = "Field `MCEO0` writer - Match or Capture Channel 0 Event Output Enable"]
pub type Mceo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEO1` reader - Match or Capture Channel 1 Event Output Enable"]
pub type Mceo1R = crate::BitReader;
#[doc = "Field `MCEO1` writer - Match or Capture Channel 1 Event Output Enable"]
pub type Mceo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEO2` reader - Match or Capture Channel 2 Event Output Enable"]
pub type Mceo2R = crate::BitReader;
#[doc = "Field `MCEO2` writer - Match or Capture Channel 2 Event Output Enable"]
pub type Mceo2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEO3` reader - Match or Capture Channel 3 Event Output Enable"]
pub type Mceo3R = crate::BitReader;
#[doc = "Field `MCEO3` writer - Match or Capture Channel 3 Event Output Enable"]
pub type Mceo3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Timer/counter Input Event0 Action"]
    #[inline(always)]
    pub fn evact0(&self) -> Evact0R {
        Evact0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Timer/counter Input Event1 Action"]
    #[inline(always)]
    pub fn evact1(&self) -> Evact1R {
        Evact1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Timer/counter Output Event Mode"]
    #[inline(always)]
    pub fn cntsel(&self) -> CntselR {
        CntselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OvfeoR {
        OvfeoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Retrigger Output Event Enable"]
    #[inline(always)]
    pub fn trgeo(&self) -> TrgeoR {
        TrgeoR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer/counter Output Event Enable"]
    #[inline(always)]
    pub fn cnteo(&self) -> CnteoR {
        CnteoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Inverted Event 0 Input Enable"]
    #[inline(always)]
    pub fn tcinv0(&self) -> Tcinv0R {
        Tcinv0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Inverted Event 1 Input Enable"]
    #[inline(always)]
    pub fn tcinv1(&self) -> Tcinv1R {
        Tcinv1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timer/counter Event 0 Input Enable"]
    #[inline(always)]
    pub fn tcei0(&self) -> Tcei0R {
        Tcei0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer/counter Event 1 Input Enable"]
    #[inline(always)]
    pub fn tcei1(&self) -> Tcei1R {
        Tcei1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Match or Capture Channel 0 Event Input Enable"]
    #[inline(always)]
    pub fn mcei0(&self) -> Mcei0R {
        Mcei0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Match or Capture Channel 1 Event Input Enable"]
    #[inline(always)]
    pub fn mcei1(&self) -> Mcei1R {
        Mcei1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Match or Capture Channel 2 Event Input Enable"]
    #[inline(always)]
    pub fn mcei2(&self) -> Mcei2R {
        Mcei2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Match or Capture Channel 3 Event Input Enable"]
    #[inline(always)]
    pub fn mcei3(&self) -> Mcei3R {
        Mcei3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Match or Capture Channel 0 Event Output Enable"]
    #[inline(always)]
    pub fn mceo0(&self) -> Mceo0R {
        Mceo0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Match or Capture Channel 1 Event Output Enable"]
    #[inline(always)]
    pub fn mceo1(&self) -> Mceo1R {
        Mceo1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Match or Capture Channel 2 Event Output Enable"]
    #[inline(always)]
    pub fn mceo2(&self) -> Mceo2R {
        Mceo2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Match or Capture Channel 3 Event Output Enable"]
    #[inline(always)]
    pub fn mceo3(&self) -> Mceo3R {
        Mceo3R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer/counter Input Event0 Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact0(&mut self) -> Evact0W<EvctrlSpec> {
        Evact0W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Timer/counter Input Event1 Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact1(&mut self) -> Evact1W<EvctrlSpec> {
        Evact1W::new(self, 3)
    }
    #[doc = "Bits 6:7 - Timer/counter Output Event Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cntsel(&mut self) -> CntselW<EvctrlSpec> {
        CntselW::new(self, 6)
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfeo(&mut self) -> OvfeoW<EvctrlSpec> {
        OvfeoW::new(self, 8)
    }
    #[doc = "Bit 9 - Retrigger Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgeo(&mut self) -> TrgeoW<EvctrlSpec> {
        TrgeoW::new(self, 9)
    }
    #[doc = "Bit 10 - Timer/counter Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnteo(&mut self) -> CnteoW<EvctrlSpec> {
        CnteoW::new(self, 10)
    }
    #[doc = "Bit 12 - Inverted Event 0 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcinv0(&mut self) -> Tcinv0W<EvctrlSpec> {
        Tcinv0W::new(self, 12)
    }
    #[doc = "Bit 13 - Inverted Event 1 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcinv1(&mut self) -> Tcinv1W<EvctrlSpec> {
        Tcinv1W::new(self, 13)
    }
    #[doc = "Bit 14 - Timer/counter Event 0 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcei0(&mut self) -> Tcei0W<EvctrlSpec> {
        Tcei0W::new(self, 14)
    }
    #[doc = "Bit 15 - Timer/counter Event 1 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcei1(&mut self) -> Tcei1W<EvctrlSpec> {
        Tcei1W::new(self, 15)
    }
    #[doc = "Bit 16 - Match or Capture Channel 0 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcei0(&mut self) -> Mcei0W<EvctrlSpec> {
        Mcei0W::new(self, 16)
    }
    #[doc = "Bit 17 - Match or Capture Channel 1 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcei1(&mut self) -> Mcei1W<EvctrlSpec> {
        Mcei1W::new(self, 17)
    }
    #[doc = "Bit 18 - Match or Capture Channel 2 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcei2(&mut self) -> Mcei2W<EvctrlSpec> {
        Mcei2W::new(self, 18)
    }
    #[doc = "Bit 19 - Match or Capture Channel 3 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcei3(&mut self) -> Mcei3W<EvctrlSpec> {
        Mcei3W::new(self, 19)
    }
    #[doc = "Bit 24 - Match or Capture Channel 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo0(&mut self) -> Mceo0W<EvctrlSpec> {
        Mceo0W::new(self, 24)
    }
    #[doc = "Bit 25 - Match or Capture Channel 1 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo1(&mut self) -> Mceo1W<EvctrlSpec> {
        Mceo1W::new(self, 25)
    }
    #[doc = "Bit 26 - Match or Capture Channel 2 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo2(&mut self) -> Mceo2W<EvctrlSpec> {
        Mceo2W::new(self, 26)
    }
    #[doc = "Bit 27 - Match or Capture Channel 3 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo3(&mut self) -> Mceo3W<EvctrlSpec> {
        Mceo3W::new(self, 27)
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {
    const RESET_VALUE: u32 = 0;
}
