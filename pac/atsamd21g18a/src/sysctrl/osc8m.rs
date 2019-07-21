#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSC8M {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
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
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u32 {
        0x8707_0382
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
#[doc = "Possible values of the field `PRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCR {
    #[doc = "1"]
    _0,
    #[doc = "2"]
    _1,
    #[doc = "4"]
    _2,
    #[doc = "8"]
    _3,
}
impl PRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCR::_0 => 0,
            PRESCR::_1 => 0x01,
            PRESCR::_2 => 0x02,
            PRESCR::_3 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCR {
        match value {
            0 => PRESCR::_0,
            1 => PRESCR::_1,
            2 => PRESCR::_2,
            3 => PRESCR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PRESCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PRESCR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == PRESCR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == PRESCR::_3
    }
}
#[doc = r" Value of the field"]
pub struct CALIBR {
    bits: u16,
}
impl CALIBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `FRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRANGER {
    #[doc = "4 to 6MHz"]
    _0,
    #[doc = "6 to 8MHz"]
    _1,
    #[doc = "8 to 11MHz"]
    _2,
    #[doc = "11 to 15MHz"]
    _3,
}
impl FRANGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FRANGER::_0 => 0,
            FRANGER::_1 => 0x01,
            FRANGER::_2 => 0x02,
            FRANGER::_3 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FRANGER {
        match value {
            0 => FRANGER::_0,
            1 => FRANGER::_1,
            2 => FRANGER::_2,
            3 => FRANGER::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FRANGER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FRANGER::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == FRANGER::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == FRANGER::_3
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
        self.w.bits &= !(0x01 << 1);
        self.w.bits |= ((value as u32) & 0x01) << 1;
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
        self.w.bits &= !(0x01 << 6);
        self.w.bits |= ((value as u32) & 0x01) << 6;
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
        self.w.bits &= !(0x01 << 7);
        self.w.bits |= ((value as u32) & 0x01) << 7;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCW {
    #[doc = "1"]
    _0,
    #[doc = "2"]
    _1,
    #[doc = "4"]
    _2,
    #[doc = "8"]
    _3,
}
impl PRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCW::_0 => 0,
            PRESCW::_1 => 1,
            PRESCW::_2 => 2,
            PRESCW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRESCW::_0)
    }
    #[doc = "2"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRESCW::_1)
    }
    #[doc = "4"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(PRESCW::_2)
    }
    #[doc = "8"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRESCW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 8);
        self.w.bits |= ((value as u32) & 0x03) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CALIBW<'a> {
    w: &'a mut W,
}
impl<'a> _CALIBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0x0fff << 16);
        self.w.bits |= ((value as u32) & 0x0fff) << 16;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRANGEW {
    #[doc = "4 to 6MHz"]
    _0,
    #[doc = "6 to 8MHz"]
    _1,
    #[doc = "8 to 11MHz"]
    _2,
    #[doc = "11 to 15MHz"]
    _3,
}
impl FRANGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FRANGEW::_0 => 0,
            FRANGEW::_1 => 1,
            FRANGEW::_2 => 2,
            FRANGEW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _FRANGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRANGEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4 to 6MHz"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRANGEW::_0)
    }
    #[doc = "6 to 8MHz"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRANGEW::_1)
    }
    #[doc = "8 to 11MHz"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(FRANGEW::_2)
    }
    #[doc = "11 to 15MHz"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(FRANGEW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 30);
        self.w.bits |= ((value as u32) & 0x03) << 30;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        ENABLER { bits }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        RUNSTDBYR { bits }
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline]
    pub fn ondemand(&self) -> ONDEMANDR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        ONDEMANDR { bits }
    }
    #[doc = "Bits 8:9 - Oscillator Prescaler"]
    #[inline]
    pub fn presc(&self) -> PRESCR {
        PRESCR::_from(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:27 - Oscillator Calibration"]
    #[inline]
    pub fn calib(&self) -> CALIBR {
        let bits = ((self.bits >> 16) & 0x0fff) as u16;
        CALIBR { bits }
    }
    #[doc = "Bits 30:31 - Oscillator Frequency Range"]
    #[inline]
    pub fn frange(&self) -> FRANGER {
        FRANGER::_from(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline]
    pub fn ondemand(&mut self) -> _ONDEMANDW {
        _ONDEMANDW { w: self }
    }
    #[doc = "Bits 8:9 - Oscillator Prescaler"]
    #[inline]
    pub fn presc(&mut self) -> _PRESCW {
        _PRESCW { w: self }
    }
    #[doc = "Bits 16:27 - Oscillator Calibration"]
    #[inline]
    pub fn calib(&mut self) -> _CALIBW {
        _CALIBW { w: self }
    }
    #[doc = "Bits 30:31 - Oscillator Frequency Range"]
    #[inline]
    pub fn frange(&mut self) -> _FRANGEW {
        _FRANGEW { w: self }
    }
}
