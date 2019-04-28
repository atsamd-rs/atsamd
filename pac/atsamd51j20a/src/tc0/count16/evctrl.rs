#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::EVCTRL {
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
#[doc = "Possible values of the field `EVACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVACTR {
    #[doc = "Event action disabled"]
    OFF,
    #[doc = "Start, restart or retrigger TC on event"]
    RETRIGGER,
    #[doc = "Count on event"]
    COUNT,
    #[doc = "Start TC on event"]
    START,
    #[doc = "Time stamp capture"]
    STAMP,
    #[doc = "Period catured in CC0, pulse width in CC1"]
    PPW,
    #[doc = "Period catured in CC1, pulse width in CC0"]
    PWP,
    #[doc = "Pulse width capture"]
    PW,
}
impl EVACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVACTR::OFF => 0,
            EVACTR::RETRIGGER => 1,
            EVACTR::COUNT => 2,
            EVACTR::START => 3,
            EVACTR::STAMP => 4,
            EVACTR::PPW => 5,
            EVACTR::PWP => 6,
            EVACTR::PW => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVACTR {
        match value {
            0 => EVACTR::OFF,
            1 => EVACTR::RETRIGGER,
            2 => EVACTR::COUNT,
            3 => EVACTR::START,
            4 => EVACTR::STAMP,
            5 => EVACTR::PPW,
            6 => EVACTR::PWP,
            7 => EVACTR::PW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == EVACTR::OFF
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline]
    pub fn is_retrigger(&self) -> bool {
        *self == EVACTR::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `COUNT`"]
    #[inline]
    pub fn is_count(&self) -> bool {
        *self == EVACTR::COUNT
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == EVACTR::START
    }
    #[doc = "Checks if the value of the field is `STAMP`"]
    #[inline]
    pub fn is_stamp(&self) -> bool {
        *self == EVACTR::STAMP
    }
    #[doc = "Checks if the value of the field is `PPW`"]
    #[inline]
    pub fn is_ppw(&self) -> bool {
        *self == EVACTR::PPW
    }
    #[doc = "Checks if the value of the field is `PWP`"]
    #[inline]
    pub fn is_pwp(&self) -> bool {
        *self == EVACTR::PWP
    }
    #[doc = "Checks if the value of the field is `PW`"]
    #[inline]
    pub fn is_pw(&self) -> bool {
        *self == EVACTR::PW
    }
}
#[doc = r" Value of the field"]
pub struct TCINVR {
    bits: bool,
}
impl TCINVR {
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
pub struct TCEIR {
    bits: bool,
}
impl TCEIR {
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
pub struct OVFEOR {
    bits: bool,
}
impl OVFEOR {
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
pub struct MCEO0R {
    bits: bool,
}
impl MCEO0R {
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
pub struct MCEO1R {
    bits: bool,
}
impl MCEO1R {
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
#[doc = "Values that can be written to the field `EVACT`"]
pub enum EVACTW {
    #[doc = "Event action disabled"]
    OFF,
    #[doc = "Start, restart or retrigger TC on event"]
    RETRIGGER,
    #[doc = "Count on event"]
    COUNT,
    #[doc = "Start TC on event"]
    START,
    #[doc = "Time stamp capture"]
    STAMP,
    #[doc = "Period catured in CC0, pulse width in CC1"]
    PPW,
    #[doc = "Period catured in CC1, pulse width in CC0"]
    PWP,
    #[doc = "Pulse width capture"]
    PW,
}
impl EVACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EVACTW::OFF => 0,
            EVACTW::RETRIGGER => 1,
            EVACTW::COUNT => 2,
            EVACTW::START => 3,
            EVACTW::STAMP => 4,
            EVACTW::PPW => 5,
            EVACTW::PWP => 6,
            EVACTW::PW => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVACTW<'a> {
    w: &'a mut W,
}
impl<'a> _EVACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Event action disabled"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(EVACTW::OFF)
    }
    #[doc = "Start, restart or retrigger TC on event"]
    #[inline]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(EVACTW::RETRIGGER)
    }
    #[doc = "Count on event"]
    #[inline]
    pub fn count(self) -> &'a mut W {
        self.variant(EVACTW::COUNT)
    }
    #[doc = "Start TC on event"]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(EVACTW::START)
    }
    #[doc = "Time stamp capture"]
    #[inline]
    pub fn stamp(self) -> &'a mut W {
        self.variant(EVACTW::STAMP)
    }
    #[doc = "Period catured in CC0, pulse width in CC1"]
    #[inline]
    pub fn ppw(self) -> &'a mut W {
        self.variant(EVACTW::PPW)
    }
    #[doc = "Period catured in CC1, pulse width in CC0"]
    #[inline]
    pub fn pwp(self) -> &'a mut W {
        self.variant(EVACTW::PWP)
    }
    #[doc = "Pulse width capture"]
    #[inline]
    pub fn pw(self) -> &'a mut W {
        self.variant(EVACTW::PW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TCINVW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCEIW<'a> {
    w: &'a mut W,
}
impl<'a> _TCEIW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OVFEOW<'a> {
    w: &'a mut W,
}
impl<'a> _OVFEOW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCEO0W<'a> {
    w: &'a mut W,
}
impl<'a> _MCEO0W<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCEO1W<'a> {
    w: &'a mut W,
}
impl<'a> _MCEO1W<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:2 - Event Action"]
    #[inline]
    pub fn evact(&self) -> EVACTR {
        EVACTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 4 - TC Event Input Polarity"]
    #[inline]
    pub fn tcinv(&self) -> TCINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TCINVR { bits }
    }
    #[doc = "Bit 5 - TC Event Enable"]
    #[inline]
    pub fn tcei(&self) -> TCEIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TCEIR { bits }
    }
    #[doc = "Bit 8 - Event Output Enable"]
    #[inline]
    pub fn ovfeo(&self) -> OVFEOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        OVFEOR { bits }
    }
    #[doc = "Bit 12 - MC Event Output Enable 0"]
    #[inline]
    pub fn mceo0(&self) -> MCEO0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        MCEO0R { bits }
    }
    #[doc = "Bit 13 - MC Event Output Enable 1"]
    #[inline]
    pub fn mceo1(&self) -> MCEO1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        MCEO1R { bits }
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Event Action"]
    #[inline]
    pub fn evact(&mut self) -> _EVACTW {
        _EVACTW { w: self }
    }
    #[doc = "Bit 4 - TC Event Input Polarity"]
    #[inline]
    pub fn tcinv(&mut self) -> _TCINVW {
        _TCINVW { w: self }
    }
    #[doc = "Bit 5 - TC Event Enable"]
    #[inline]
    pub fn tcei(&mut self) -> _TCEIW {
        _TCEIW { w: self }
    }
    #[doc = "Bit 8 - Event Output Enable"]
    #[inline]
    pub fn ovfeo(&mut self) -> _OVFEOW {
        _OVFEOW { w: self }
    }
    #[doc = "Bit 12 - MC Event Output Enable 0"]
    #[inline]
    pub fn mceo0(&mut self) -> _MCEO0W {
        _MCEO0W { w: self }
    }
    #[doc = "Bit 13 - MC Event Output Enable 1"]
    #[inline]
    pub fn mceo1(&mut self) -> _MCEO1W {
        _MCEO1W { w: self }
    }
}
