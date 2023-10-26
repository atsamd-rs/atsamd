#[doc = "Register `CHANNEL` reader"]
pub type R = crate::R<CHANNEL_SPEC>;
#[doc = "Register `CHANNEL` writer"]
pub type W = crate::W<CHANNEL_SPEC>;
#[doc = "Field `EVGEN` reader - Event Generator Selection"]
pub type EVGEN_R = crate::FieldReader;
#[doc = "Field `EVGEN` writer - Event Generator Selection"]
pub type EVGEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `PATH` reader - Path Selection"]
pub type PATH_R = crate::FieldReader<PATHSELECT_A>;
#[doc = "Path Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PATHSELECT_A {
    #[doc = "0: Synchronous path"]
    SYNCHRONOUS = 0,
    #[doc = "1: Resynchronized path"]
    RESYNCHRONIZED = 1,
    #[doc = "2: Asynchronous path"]
    ASYNCHRONOUS = 2,
}
impl From<PATHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PATHSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PATHSELECT_A {
    type Ux = u8;
}
impl PATH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PATHSELECT_A> {
        match self.bits {
            0 => Some(PATHSELECT_A::SYNCHRONOUS),
            1 => Some(PATHSELECT_A::RESYNCHRONIZED),
            2 => Some(PATHSELECT_A::ASYNCHRONOUS),
            _ => None,
        }
    }
    #[doc = "Synchronous path"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == PATHSELECT_A::SYNCHRONOUS
    }
    #[doc = "Resynchronized path"]
    #[inline(always)]
    pub fn is_resynchronized(&self) -> bool {
        *self == PATHSELECT_A::RESYNCHRONIZED
    }
    #[doc = "Asynchronous path"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == PATHSELECT_A::ASYNCHRONOUS
    }
}
#[doc = "Field `PATH` writer - Path Selection"]
pub type PATH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, PATHSELECT_A>;
impl<'a, REG, const O: u8> PATH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Synchronous path"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(PATHSELECT_A::SYNCHRONOUS)
    }
    #[doc = "Resynchronized path"]
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut crate::W<REG> {
        self.variant(PATHSELECT_A::RESYNCHRONIZED)
    }
    #[doc = "Asynchronous path"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(PATHSELECT_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `EDGSEL` reader - Edge Detection Selection"]
pub type EDGSEL_R = crate::FieldReader<EDGSELSELECT_A>;
#[doc = "Edge Detection Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGSELSELECT_A {
    #[doc = "0: No event output when using the resynchronized or synchronous path"]
    NO_EVT_OUTPUT = 0,
    #[doc = "1: Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    RISING_EDGE = 1,
    #[doc = "2: Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    FALLING_EDGE = 2,
    #[doc = "3: Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    BOTH_EDGES = 3,
}
impl From<EDGSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EDGSELSELECT_A {
    type Ux = u8;
}
impl EDGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDGSELSELECT_A {
        match self.bits {
            0 => EDGSELSELECT_A::NO_EVT_OUTPUT,
            1 => EDGSELSELECT_A::RISING_EDGE,
            2 => EDGSELSELECT_A::FALLING_EDGE,
            3 => EDGSELSELECT_A::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "No event output when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn is_no_evt_output(&self) -> bool {
        *self == EDGSELSELECT_A::NO_EVT_OUTPUT
    }
    #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EDGSELSELECT_A::RISING_EDGE
    }
    #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EDGSELSELECT_A::FALLING_EDGE
    }
    #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EDGSELSELECT_A::BOTH_EDGES
    }
}
#[doc = "Field `EDGSEL` writer - Edge Detection Selection"]
pub type EDGSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, EDGSELSELECT_A>;
impl<'a, REG, const O: u8> EDGSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event output when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn no_evt_output(self) -> &'a mut crate::W<REG> {
        self.variant(EDGSELSELECT_A::NO_EVT_OUTPUT)
    }
    #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EDGSELSELECT_A::RISING_EDGE)
    }
    #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EDGSELSELECT_A::FALLING_EDGE)
    }
    #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(EDGSELSELECT_A::BOTH_EDGES)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ONDEMAND` reader - Generic Clock On Demand"]
pub type ONDEMAND_R = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - Generic Clock On Demand"]
pub type ONDEMAND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Event Generator Selection"]
    #[inline(always)]
    pub fn evgen(&self) -> EVGEN_R {
        EVGEN_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Path Selection"]
    #[inline(always)]
    pub fn path(&self) -> PATH_R {
        PATH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Edge Detection Selection"]
    #[inline(always)]
    pub fn edgsel(&self) -> EDGSEL_R {
        EDGSEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - Run in standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Generic Clock On Demand"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Event Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn evgen(&mut self) -> EVGEN_W<CHANNEL_SPEC, 0> {
        EVGEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Path Selection"]
    #[inline(always)]
    #[must_use]
    pub fn path(&mut self) -> PATH_W<CHANNEL_SPEC, 8> {
        PATH_W::new(self)
    }
    #[doc = "Bits 10:11 - Edge Detection Selection"]
    #[inline(always)]
    #[must_use]
    pub fn edgsel(&mut self) -> EDGSEL_W<CHANNEL_SPEC, 10> {
        EDGSEL_W::new(self)
    }
    #[doc = "Bit 14 - Run in standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<CHANNEL_SPEC, 14> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 15 - Generic Clock On Demand"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<CHANNEL_SPEC, 15> {
        ONDEMAND_W::new(self)
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
#[doc = "Channel n Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHANNEL_SPEC;
impl crate::RegisterSpec for CHANNEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel::R`](R) reader structure"]
impl crate::Readable for CHANNEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`channel::W`](W) writer structure"]
impl crate::Writable for CHANNEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANNEL to value 0x8000"]
impl crate::Resettable for CHANNEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
