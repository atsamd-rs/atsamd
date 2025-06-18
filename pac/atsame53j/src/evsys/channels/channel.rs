#[doc = "Register `CHANNEL` reader"]
pub type R = crate::R<ChannelSpec>;
#[doc = "Register `CHANNEL` writer"]
pub type W = crate::W<ChannelSpec>;
#[doc = "Field `EVGEN` reader - Event Generator Selection"]
pub type EvgenR = crate::FieldReader;
#[doc = "Field `EVGEN` writer - Event Generator Selection"]
pub type EvgenW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Path Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pathselect {
    #[doc = "0: Synchronous path"]
    Synchronous = 0,
    #[doc = "1: Resynchronized path"]
    Resynchronized = 1,
    #[doc = "2: Asynchronous path"]
    Asynchronous = 2,
}
impl From<Pathselect> for u8 {
    #[inline(always)]
    fn from(variant: Pathselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pathselect {
    type Ux = u8;
}
impl crate::IsEnum for Pathselect {}
#[doc = "Field `PATH` reader - Path Selection"]
pub type PathR = crate::FieldReader<Pathselect>;
impl PathR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pathselect> {
        match self.bits {
            0 => Some(Pathselect::Synchronous),
            1 => Some(Pathselect::Resynchronized),
            2 => Some(Pathselect::Asynchronous),
            _ => None,
        }
    }
    #[doc = "Synchronous path"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Pathselect::Synchronous
    }
    #[doc = "Resynchronized path"]
    #[inline(always)]
    pub fn is_resynchronized(&self) -> bool {
        *self == Pathselect::Resynchronized
    }
    #[doc = "Asynchronous path"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Pathselect::Asynchronous
    }
}
#[doc = "Field `PATH` writer - Path Selection"]
pub type PathW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pathselect>;
impl<'a, REG> PathW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Synchronous path"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Pathselect::Synchronous)
    }
    #[doc = "Resynchronized path"]
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut crate::W<REG> {
        self.variant(Pathselect::Resynchronized)
    }
    #[doc = "Asynchronous path"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Pathselect::Asynchronous)
    }
}
#[doc = "Edge Detection Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edgselselect {
    #[doc = "0: No event output when using the resynchronized or synchronous path"]
    NoEvtOutput = 0,
    #[doc = "1: Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    RisingEdge = 1,
    #[doc = "2: Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    FallingEdge = 2,
    #[doc = "3: Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    BothEdges = 3,
}
impl From<Edgselselect> for u8 {
    #[inline(always)]
    fn from(variant: Edgselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edgselselect {
    type Ux = u8;
}
impl crate::IsEnum for Edgselselect {}
#[doc = "Field `EDGSEL` reader - Edge Detection Selection"]
pub type EdgselR = crate::FieldReader<Edgselselect>;
impl EdgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edgselselect {
        match self.bits {
            0 => Edgselselect::NoEvtOutput,
            1 => Edgselselect::RisingEdge,
            2 => Edgselselect::FallingEdge,
            3 => Edgselselect::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "No event output when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn is_no_evt_output(&self) -> bool {
        *self == Edgselselect::NoEvtOutput
    }
    #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Edgselselect::RisingEdge
    }
    #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Edgselselect::FallingEdge
    }
    #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == Edgselselect::BothEdges
    }
}
#[doc = "Field `EDGSEL` writer - Edge Detection Selection"]
pub type EdgselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edgselselect, crate::Safe>;
impl<'a, REG> EdgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event output when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn no_evt_output(self) -> &'a mut crate::W<REG> {
        self.variant(Edgselselect::NoEvtOutput)
    }
    #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Edgselselect::RisingEdge)
    }
    #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Edgselselect::FallingEdge)
    }
    #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(Edgselselect::BothEdges)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - Generic Clock On Demand"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - Generic Clock On Demand"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Event Generator Selection"]
    #[inline(always)]
    pub fn evgen(&self) -> EvgenR {
        EvgenR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Path Selection"]
    #[inline(always)]
    pub fn path(&self) -> PathR {
        PathR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Edge Detection Selection"]
    #[inline(always)]
    pub fn edgsel(&self) -> EdgselR {
        EdgselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - Run in standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Generic Clock On Demand"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Event Generator Selection"]
    #[inline(always)]
    pub fn evgen(&mut self) -> EvgenW<ChannelSpec> {
        EvgenW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Path Selection"]
    #[inline(always)]
    pub fn path(&mut self) -> PathW<ChannelSpec> {
        PathW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Edge Detection Selection"]
    #[inline(always)]
    pub fn edgsel(&mut self) -> EdgselW<ChannelSpec> {
        EdgselW::new(self, 10)
    }
    #[doc = "Bit 14 - Run in standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<ChannelSpec> {
        RunstdbyW::new(self, 14)
    }
    #[doc = "Bit 15 - Generic Clock On Demand"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<ChannelSpec> {
        OndemandW::new(self, 15)
    }
}
#[doc = "Channel n Control\n\nYou can [`read`](crate::Reg::read) this register and get [`channel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChannelSpec;
impl crate::RegisterSpec for ChannelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel::R`](R) reader structure"]
impl crate::Readable for ChannelSpec {}
#[doc = "`write(|w| ..)` method takes [`channel::W`](W) writer structure"]
impl crate::Writable for ChannelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHANNEL to value 0x8000"]
impl crate::Resettable for ChannelSpec {
    const RESET_VALUE: u32 = 0x8000;
}
