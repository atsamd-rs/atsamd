#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VREF {
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
pub struct TSENR {
    bits: bool,
}
impl TSENR {
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
pub struct VREFOER {
    bits: bool,
}
impl VREFOER {
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
pub struct TSSELR {
    bits: bool,
}
impl TSSELR {
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
pub struct RUNSTDBYR {
    bits: bool,
}
impl RUNSTDBYR {
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
pub struct ONDEMANDR {
    bits: bool,
}
impl ONDEMANDR {
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
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "1.0V voltage reference typical value"]
    _1V0,
    #[doc = "1.1V voltage reference typical value"]
    _1V1,
    #[doc = "1.2V voltage reference typical value"]
    _1V2,
    #[doc = "1.25V voltage reference typical value"]
    _1V25,
    #[doc = "2.0V voltage reference typical value"]
    _2V0,
    #[doc = "2.2V voltage reference typical value"]
    _2V2,
    #[doc = "2.4V voltage reference typical value"]
    _2V4,
    #[doc = "2.5V voltage reference typical value"]
    _2V5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELR::_1V0 => 0,
            SELR::_1V1 => 1,
            SELR::_1V2 => 2,
            SELR::_1V25 => 3,
            SELR::_2V0 => 4,
            SELR::_2V2 => 5,
            SELR::_2V4 => 6,
            SELR::_2V5 => 7,
            SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELR {
        match value {
            0 => SELR::_1V0,
            1 => SELR::_1V1,
            2 => SELR::_1V2,
            3 => SELR::_1V25,
            4 => SELR::_2V0,
            5 => SELR::_2V2,
            6 => SELR::_2V4,
            7 => SELR::_2V5,
            i => SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1V0`"]
    #[inline]
    pub fn is_1v0(&self) -> bool {
        *self == SELR::_1V0
    }
    #[doc = "Checks if the value of the field is `_1V1`"]
    #[inline]
    pub fn is_1v1(&self) -> bool {
        *self == SELR::_1V1
    }
    #[doc = "Checks if the value of the field is `_1V2`"]
    #[inline]
    pub fn is_1v2(&self) -> bool {
        *self == SELR::_1V2
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline]
    pub fn is_1v25(&self) -> bool {
        *self == SELR::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V0`"]
    #[inline]
    pub fn is_2v0(&self) -> bool {
        *self == SELR::_2V0
    }
    #[doc = "Checks if the value of the field is `_2V2`"]
    #[inline]
    pub fn is_2v2(&self) -> bool {
        *self == SELR::_2V2
    }
    #[doc = "Checks if the value of the field is `_2V4`"]
    #[inline]
    pub fn is_2v4(&self) -> bool {
        *self == SELR::_2V4
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline]
    pub fn is_2v5(&self) -> bool {
        *self == SELR::_2V5
    }
}
#[doc = r" Proxy"]
pub struct _TSENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSENW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VREFOEW<'a> {
    w: &'a mut W,
}
impl<'a> _VREFOEW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TSSELW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RUNSTDBYW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNSTDBYW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ONDEMANDW<'a> {
    w: &'a mut W,
}
impl<'a> _ONDEMANDW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEL`"]
pub enum SELW {
    #[doc = "1.0V voltage reference typical value"]
    _1V0,
    #[doc = "1.1V voltage reference typical value"]
    _1V1,
    #[doc = "1.2V voltage reference typical value"]
    _1V2,
    #[doc = "1.25V voltage reference typical value"]
    _1V25,
    #[doc = "2.0V voltage reference typical value"]
    _2V0,
    #[doc = "2.2V voltage reference typical value"]
    _2V2,
    #[doc = "2.4V voltage reference typical value"]
    _2V4,
    #[doc = "2.5V voltage reference typical value"]
    _2V5,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::_1V0 => 0,
            SELW::_1V1 => 1,
            SELW::_1V2 => 2,
            SELW::_1V25 => 3,
            SELW::_2V0 => 4,
            SELW::_2V2 => 5,
            SELW::_2V4 => 6,
            SELW::_2V5 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1.0V voltage reference typical value"]
    #[inline]
    pub fn _1v0(self) -> &'a mut W {
        self.variant(SELW::_1V0)
    }
    #[doc = "1.1V voltage reference typical value"]
    #[inline]
    pub fn _1v1(self) -> &'a mut W {
        self.variant(SELW::_1V1)
    }
    #[doc = "1.2V voltage reference typical value"]
    #[inline]
    pub fn _1v2(self) -> &'a mut W {
        self.variant(SELW::_1V2)
    }
    #[doc = "1.25V voltage reference typical value"]
    #[inline]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(SELW::_1V25)
    }
    #[doc = "2.0V voltage reference typical value"]
    #[inline]
    pub fn _2v0(self) -> &'a mut W {
        self.variant(SELW::_2V0)
    }
    #[doc = "2.2V voltage reference typical value"]
    #[inline]
    pub fn _2v2(self) -> &'a mut W {
        self.variant(SELW::_2V2)
    }
    #[doc = "2.4V voltage reference typical value"]
    #[inline]
    pub fn _2v4(self) -> &'a mut W {
        self.variant(SELW::_2V4)
    }
    #[doc = "2.5V voltage reference typical value"]
    #[inline]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(SELW::_2V5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline]
    pub fn tsen(&self) -> TSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSENR { bits }
    }
    #[doc = "Bit 2 - Voltage Reference Output Enable"]
    #[inline]
    pub fn vrefoe(&self) -> VREFOER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VREFOER { bits }
    }
    #[doc = "Bit 3 - Temperature Sensor Selection"]
    #[inline]
    pub fn tssel(&self) -> TSSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSSELR { bits }
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNSTDBYR { bits }
    }
    #[doc = "Bit 7 - On Demand Contrl"]
    #[inline]
    pub fn ondemand(&self) -> ONDEMANDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ONDEMANDR { bits }
    }
    #[doc = "Bits 16:19 - Voltage Reference Selection"]
    #[inline]
    pub fn sel(&self) -> SELR {
        SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline]
    pub fn tsen(&mut self) -> _TSENW {
        _TSENW { w: self }
    }
    #[doc = "Bit 2 - Voltage Reference Output Enable"]
    #[inline]
    pub fn vrefoe(&mut self) -> _VREFOEW {
        _VREFOEW { w: self }
    }
    #[doc = "Bit 3 - Temperature Sensor Selection"]
    #[inline]
    pub fn tssel(&mut self) -> _TSSELW {
        _TSSELW { w: self }
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bit 7 - On Demand Contrl"]
    #[inline]
    pub fn ondemand(&mut self) -> _ONDEMANDW {
        _ONDEMANDW { w: self }
    }
    #[doc = "Bits 16:19 - Voltage Reference Selection"]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
