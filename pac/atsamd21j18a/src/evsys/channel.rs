#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHANNEL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct CHANNELR {
    bits: u8,
}
impl CHANNELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SWEVTR {
    bits: bool,
}
impl SWEVTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct EVGENR {
    bits: u8,
}
impl EVGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PATH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PATHR {
    #[doc = "Synchronous path"]
    SYNCHRONOUS,
    #[doc = "Resynchronized path"]
    RESYNCHRONIZED,
    #[doc = "Asynchronous path"]
    ASYNCHRONOUS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PATHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PATHR::SYNCHRONOUS => 0,
            PATHR::RESYNCHRONIZED => 1,
            PATHR::ASYNCHRONOUS => 2,
            PATHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PATHR {
        match value {
            0 => PATHR::SYNCHRONOUS,
            1 => PATHR::RESYNCHRONIZED,
            2 => PATHR::ASYNCHRONOUS,
            i => PATHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline]
    pub fn is_synchronous(&self) -> bool {
        *self == PATHR::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `RESYNCHRONIZED`"]
    #[inline]
    pub fn is_resynchronized(&self) -> bool {
        *self == PATHR::RESYNCHRONIZED
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline]
    pub fn is_asynchronous(&self) -> bool {
        *self == PATHR::ASYNCHRONOUS
    }
}
#[doc = "Possible values of the field `EDGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGSELR {
    #[doc = "No event output when using the resynchronized or synchronous path"]
    NO_EVT_OUTPUT,
    #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    RISING_EDGE,
    #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    FALLING_EDGE,
    #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    BOTH_EDGES,
}
impl EDGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGSELR::NO_EVT_OUTPUT => 0,
            EDGSELR::RISING_EDGE => 1,
            EDGSELR::FALLING_EDGE => 2,
            EDGSELR::BOTH_EDGES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGSELR {
        match value {
            0 => EDGSELR::NO_EVT_OUTPUT,
            1 => EDGSELR::RISING_EDGE,
            2 => EDGSELR::FALLING_EDGE,
            3 => EDGSELR::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVT_OUTPUT`"]
    #[inline]
    pub fn is_no_evt_output(&self) -> bool {
        *self == EDGSELR::NO_EVT_OUTPUT
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == EDGSELR::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == EDGSELR::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline]
    pub fn is_both_edges(&self) -> bool {
        *self == EDGSELR::BOTH_EDGES
    }
}
#[doc = r" Proxy"]
pub struct _CHANNELW<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWEVTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EVGENW<'a> {
    w: &'a mut W,
}
impl<'a> _EVGENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PATH`"]
pub enum PATHW {
    #[doc = "Synchronous path"]
    SYNCHRONOUS,
    #[doc = "Resynchronized path"]
    RESYNCHRONIZED,
    #[doc = "Asynchronous path"]
    ASYNCHRONOUS,
}
impl PATHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PATHW::SYNCHRONOUS => 0,
            PATHW::RESYNCHRONIZED => 1,
            PATHW::ASYNCHRONOUS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PATHW<'a> {
    w: &'a mut W,
}
impl<'a> _PATHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PATHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Synchronous path"]
    #[inline]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(PATHW::SYNCHRONOUS)
    }
    #[doc = "Resynchronized path"]
    #[inline]
    pub fn resynchronized(self) -> &'a mut W {
        self.variant(PATHW::RESYNCHRONIZED)
    }
    #[doc = "Asynchronous path"]
    #[inline]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(PATHW::ASYNCHRONOUS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGSEL`"]
pub enum EDGSELW {
    #[doc = "No event output when using the resynchronized or synchronous path"]
    NO_EVT_OUTPUT,
    #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    RISING_EDGE,
    #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    FALLING_EDGE,
    #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    BOTH_EDGES,
}
impl EDGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGSELW::NO_EVT_OUTPUT => 0,
            EDGSELW::RISING_EDGE => 1,
            EDGSELW::FALLING_EDGE => 2,
            EDGSELW::BOTH_EDGES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No event output when using the resynchronized or synchronous path"]
    #[inline]
    pub fn no_evt_output(self) -> &'a mut W {
        self.variant(EDGSELW::NO_EVT_OUTPUT)
    }
    #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EDGSELW::RISING_EDGE)
    }
    #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EDGSELW::FALLING_EDGE)
    }
    #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EDGSELW::BOTH_EDGES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline]
    pub fn channel(&self) -> CHANNELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHANNELR { bits }
    }
    #[doc = "Bit 8 - Software Event"]
    #[inline]
    pub fn swevt(&self) -> SWEVTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWEVTR { bits }
    }
    #[doc = "Bits 16:22 - Event Generator Selection"]
    #[inline]
    pub fn evgen(&self) -> EVGENR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EVGENR { bits }
    }
    #[doc = "Bits 24:25 - Path Selection"]
    #[inline]
    pub fn path(&self) -> PATHR {
        PATHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Edge Detection Selection"]
    #[inline]
    pub fn edgsel(&self) -> EDGSELR {
        EDGSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline]
    pub fn channel(&mut self) -> _CHANNELW {
        _CHANNELW { w: self }
    }
    #[doc = "Bit 8 - Software Event"]
    #[inline]
    pub fn swevt(&mut self) -> _SWEVTW {
        _SWEVTW { w: self }
    }
    #[doc = "Bits 16:22 - Event Generator Selection"]
    #[inline]
    pub fn evgen(&mut self) -> _EVGENW {
        _EVGENW { w: self }
    }
    #[doc = "Bits 24:25 - Path Selection"]
    #[inline]
    pub fn path(&mut self) -> _PATHW {
        _PATHW { w: self }
    }
    #[doc = "Bits 26:27 - Edge Detection Selection"]
    #[inline]
    pub fn edgsel(&mut self) -> _EDGSELW {
        _EDGSELW { w: self }
    }
}
