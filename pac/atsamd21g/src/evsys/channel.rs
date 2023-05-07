#[doc = "Register `CHANNEL` reader"]
pub struct R(crate::R<CHANNEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL` writer"]
pub struct W(crate::W<CHANNEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_SPEC>;
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
impl From<crate::W<CHANNEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANNEL` reader - Channel Selection"]
pub type CHANNEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHANNEL` writer - Channel Selection"]
pub type CHANNEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHANNEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SWEVT` reader - Software Event"]
pub type SWEVT_R = crate::BitReader<bool>;
#[doc = "Field `SWEVT` writer - Software Event"]
pub type SWEVT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_SPEC, bool, O>;
#[doc = "Field `EVGEN` reader - Event Generator Selection"]
pub type EVGEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVGEN` writer - Event Generator Selection"]
pub type EVGEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHANNEL_SPEC, u8, u8, 7, O>;
#[doc = "Field `PATH` reader - Path Selection"]
pub type PATH_R = crate::FieldReader<u8, PATHSELECT_A>;
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
impl PATH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PATHSELECT_A> {
        match self.bits {
            0 => Some(PATHSELECT_A::SYNCHRONOUS),
            1 => Some(PATHSELECT_A::RESYNCHRONIZED),
            2 => Some(PATHSELECT_A::ASYNCHRONOUS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == PATHSELECT_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `RESYNCHRONIZED`"]
    #[inline(always)]
    pub fn is_resynchronized(&self) -> bool {
        *self == PATHSELECT_A::RESYNCHRONIZED
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == PATHSELECT_A::ASYNCHRONOUS
    }
}
#[doc = "Field `PATH` writer - Path Selection"]
pub type PATH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_SPEC, u8, PATHSELECT_A, 2, O>;
impl<'a, const O: u8> PATH_W<'a, O> {
    #[doc = "Synchronous path"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(PATHSELECT_A::SYNCHRONOUS)
    }
    #[doc = "Resynchronized path"]
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut W {
        self.variant(PATHSELECT_A::RESYNCHRONIZED)
    }
    #[doc = "Asynchronous path"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(PATHSELECT_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `EDGSEL` reader - Edge Detection Selection"]
pub type EDGSEL_R = crate::FieldReader<u8, EDGSELSELECT_A>;
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
impl EDGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGSELSELECT_A {
        match self.bits {
            0 => EDGSELSELECT_A::NO_EVT_OUTPUT,
            1 => EDGSELSELECT_A::RISING_EDGE,
            2 => EDGSELSELECT_A::FALLING_EDGE,
            3 => EDGSELSELECT_A::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVT_OUTPUT`"]
    #[inline(always)]
    pub fn is_no_evt_output(&self) -> bool {
        *self == EDGSELSELECT_A::NO_EVT_OUTPUT
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EDGSELSELECT_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EDGSELSELECT_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EDGSELSELECT_A::BOTH_EDGES
    }
}
#[doc = "Field `EDGSEL` writer - Edge Detection Selection"]
pub type EDGSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CHANNEL_SPEC, u8, EDGSELSELECT_A, 2, O>;
impl<'a, const O: u8> EDGSEL_W<'a, O> {
    #[doc = "No event output when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn no_evt_output(self) -> &'a mut W {
        self.variant(EDGSELSELECT_A::NO_EVT_OUTPUT)
    }
    #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EDGSELSELECT_A::RISING_EDGE)
    }
    #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EDGSELSELECT_A::FALLING_EDGE)
    }
    #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
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
    pub fn channel(&mut self) -> CHANNEL_W<0> {
        CHANNEL_W::new(self)
    }
    #[doc = "Bit 8 - Software Event"]
    #[inline(always)]
    #[must_use]
    pub fn swevt(&mut self) -> SWEVT_W<8> {
        SWEVT_W::new(self)
    }
    #[doc = "Bits 16:22 - Event Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn evgen(&mut self) -> EVGEN_W<16> {
        EVGEN_W::new(self)
    }
    #[doc = "Bits 24:25 - Path Selection"]
    #[inline(always)]
    #[must_use]
    pub fn path(&mut self) -> PATH_W<24> {
        PATH_W::new(self)
    }
    #[doc = "Bits 26:27 - Edge Detection Selection"]
    #[inline(always)]
    #[must_use]
    pub fn edgsel(&mut self) -> EDGSEL_W<26> {
        EDGSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel](index.html) module"]
pub struct CHANNEL_SPEC;
impl crate::RegisterSpec for CHANNEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel::R](R) reader structure"]
impl crate::Readable for CHANNEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel::W](W) writer structure"]
impl crate::Writable for CHANNEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANNEL to value 0"]
impl crate::Resettable for CHANNEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
