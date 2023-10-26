#[doc = "Register `CHANNEL` reader"]
pub type R = crate::R<CHANNEL_SPEC>;
#[doc = "Register `CHANNEL` writer"]
pub type W = crate::W<CHANNEL_SPEC>;
#[doc = "Field `CHANNEL` reader - Channel Selection"]
pub type CHANNEL_R = crate::FieldReader;
#[doc = "Field `CHANNEL` writer - Channel Selection"]
pub type CHANNEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SWEVT` reader - Software Event"]
pub type SWEVT_R = crate::BitReader;
#[doc = "Field `SWEVT` writer - Software Event"]
pub type SWEVT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
impl R {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Software Event"]
    #[inline(always)]
    pub fn swevt(&self) -> SWEVT_R {
        SWEVT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Event Generator Selection"]
    #[inline(always)]
    pub fn evgen(&self) -> EVGEN_R {
        EVGEN_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:25 - Path Selection"]
    #[inline(always)]
    pub fn path(&self) -> PATH_R {
        PATH_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Edge Detection Selection"]
    #[inline(always)]
    pub fn edgsel(&self) -> EDGSEL_R {
        EDGSEL_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self) -> CHANNEL_W<CHANNEL_SPEC, 0> {
        CHANNEL_W::new(self)
    }
    #[doc = "Bit 8 - Software Event"]
    #[inline(always)]
    #[must_use]
    pub fn swevt(&mut self) -> SWEVT_W<CHANNEL_SPEC, 8> {
        SWEVT_W::new(self)
    }
    #[doc = "Bits 16:22 - Event Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn evgen(&mut self) -> EVGEN_W<CHANNEL_SPEC, 16> {
        EVGEN_W::new(self)
    }
    #[doc = "Bits 24:25 - Path Selection"]
    #[inline(always)]
    #[must_use]
    pub fn path(&mut self) -> PATH_W<CHANNEL_SPEC, 24> {
        PATH_W::new(self)
    }
    #[doc = "Bits 26:27 - Edge Detection Selection"]
    #[inline(always)]
    #[must_use]
    pub fn edgsel(&mut self) -> EDGSEL_W<CHANNEL_SPEC, 26> {
        EDGSEL_W::new(self)
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
#[doc = "Channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CHANNEL to value 0"]
impl crate::Resettable for CHANNEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
