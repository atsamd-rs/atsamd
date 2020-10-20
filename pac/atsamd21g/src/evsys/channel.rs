#[doc = "Reader of register CHANNEL"]
pub type R = crate::R<u32, super::CHANNEL>;
#[doc = "Writer for register CHANNEL"]
pub type W = crate::W<u32, super::CHANNEL>;
#[doc = "Register CHANNEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CHANNEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHANNEL`"]
pub type CHANNEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHANNEL`"]
pub struct CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `SWEVT`"]
pub type SWEVT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEVT`"]
pub struct SWEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEVT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `EVGEN`"]
pub type EVGEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EVGEN`"]
pub struct EVGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVGEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Path Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PATH_A {
    #[doc = "0: Synchronous path"]
    SYNCHRONOUS = 0,
    #[doc = "1: Resynchronized path"]
    RESYNCHRONIZED = 1,
    #[doc = "2: Asynchronous path"]
    ASYNCHRONOUS = 2,
}
impl From<PATH_A> for u8 {
    #[inline(always)]
    fn from(variant: PATH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PATH`"]
pub type PATH_R = crate::R<u8, PATH_A>;
impl PATH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PATH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PATH_A::SYNCHRONOUS),
            1 => Val(PATH_A::RESYNCHRONIZED),
            2 => Val(PATH_A::ASYNCHRONOUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == PATH_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `RESYNCHRONIZED`"]
    #[inline(always)]
    pub fn is_resynchronized(&self) -> bool {
        *self == PATH_A::RESYNCHRONIZED
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == PATH_A::ASYNCHRONOUS
    }
}
#[doc = "Write proxy for field `PATH`"]
pub struct PATH_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PATH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Synchronous path"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(PATH_A::SYNCHRONOUS)
    }
    #[doc = "Resynchronized path"]
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut W {
        self.variant(PATH_A::RESYNCHRONIZED)
    }
    #[doc = "Asynchronous path"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(PATH_A::ASYNCHRONOUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Edge Detection Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGSEL_A {
    #[doc = "0: No event output when using the resynchronized or synchronous path"]
    NO_EVT_OUTPUT = 0,
    #[doc = "1: Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    RISING_EDGE = 1,
    #[doc = "2: Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    FALLING_EDGE = 2,
    #[doc = "3: Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    BOTH_EDGES = 3,
}
impl From<EDGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EDGSEL`"]
pub type EDGSEL_R = crate::R<u8, EDGSEL_A>;
impl EDGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGSEL_A {
        match self.bits {
            0 => EDGSEL_A::NO_EVT_OUTPUT,
            1 => EDGSEL_A::RISING_EDGE,
            2 => EDGSEL_A::FALLING_EDGE,
            3 => EDGSEL_A::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVT_OUTPUT`"]
    #[inline(always)]
    pub fn is_no_evt_output(&self) -> bool {
        *self == EDGSEL_A::NO_EVT_OUTPUT
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EDGSEL_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EDGSEL_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EDGSEL_A::BOTH_EDGES
    }
}
#[doc = "Write proxy for field `EDGSEL`"]
pub struct EDGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No event output when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn no_evt_output(self) -> &'a mut W {
        self.variant(EDGSEL_A::NO_EVT_OUTPUT)
    }
    #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EDGSEL_A::RISING_EDGE)
    }
    #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EDGSEL_A::FALLING_EDGE)
    }
    #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EDGSEL_A::BOTH_EDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
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
        SWEVT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Event Generator Selection"]
    #[inline(always)]
    pub fn evgen(&self) -> EVGEN_R {
        EVGEN_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:25 - Path Selection"]
    #[inline(always)]
    pub fn path(&self) -> PATH_R {
        PATH_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Edge Detection Selection"]
    #[inline(always)]
    pub fn edgsel(&self) -> EDGSEL_R {
        EDGSEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    pub fn channel(&mut self) -> CHANNEL_W {
        CHANNEL_W { w: self }
    }
    #[doc = "Bit 8 - Software Event"]
    #[inline(always)]
    pub fn swevt(&mut self) -> SWEVT_W {
        SWEVT_W { w: self }
    }
    #[doc = "Bits 16:22 - Event Generator Selection"]
    #[inline(always)]
    pub fn evgen(&mut self) -> EVGEN_W {
        EVGEN_W { w: self }
    }
    #[doc = "Bits 24:25 - Path Selection"]
    #[inline(always)]
    pub fn path(&mut self) -> PATH_W {
        PATH_W { w: self }
    }
    #[doc = "Bits 26:27 - Edge Detection Selection"]
    #[inline(always)]
    pub fn edgsel(&mut self) -> EDGSEL_W {
        EDGSEL_W { w: self }
    }
}
