#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::COMPCTRL {
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
pub struct SINGLER {
    bits: bool,
}
impl SINGLER {
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
#[doc = "Possible values of the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEEDR {
    #[doc = "Low speed"]
    LOW,
    #[doc = "High speed"]
    HIGH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPEEDR::LOW => 0,
            SPEEDR::HIGH => 1,
            SPEEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPEEDR {
        match value {
            0 => SPEEDR::LOW,
            1 => SPEEDR::HIGH,
            i => SPEEDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SPEEDR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SPEEDR::HIGH
    }
}
#[doc = "Possible values of the field `INTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSELR {
    #[doc = "Interrupt on comparator output toggle"]
    TOGGLE,
    #[doc = "Interrupt on comparator output rising"]
    RISING,
    #[doc = "Interrupt on comparator output falling"]
    FALLING,
    #[doc = "Interrupt on end of comparison (single-shot mode only)"]
    EOC,
}
impl INTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INTSELR::TOGGLE => 0,
            INTSELR::RISING => 1,
            INTSELR::FALLING => 2,
            INTSELR::EOC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INTSELR {
        match value {
            0 => INTSELR::TOGGLE,
            1 => INTSELR::RISING,
            2 => INTSELR::FALLING,
            3 => INTSELR::EOC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == INTSELR::TOGGLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == INTSELR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == INTSELR::FALLING
    }
    #[doc = "Checks if the value of the field is `EOC`"]
    #[inline]
    pub fn is_eoc(&self) -> bool {
        *self == INTSELR::EOC
    }
}
#[doc = "Possible values of the field `MUXNEG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXNEGR {
    #[doc = "I/O pin 0"]
    PIN0,
    #[doc = "I/O pin 1"]
    PIN1,
    #[doc = "I/O pin 2"]
    PIN2,
    #[doc = "I/O pin 3"]
    PIN3,
    #[doc = "Ground"]
    GND,
    #[doc = "VDD scaler"]
    VSCALE,
    #[doc = "Internal bandgap voltage"]
    BANDGAP,
    #[doc = "DAC output"]
    DAC,
}
impl MUXNEGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUXNEGR::PIN0 => 0,
            MUXNEGR::PIN1 => 1,
            MUXNEGR::PIN2 => 2,
            MUXNEGR::PIN3 => 3,
            MUXNEGR::GND => 4,
            MUXNEGR::VSCALE => 5,
            MUXNEGR::BANDGAP => 6,
            MUXNEGR::DAC => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUXNEGR {
        match value {
            0 => MUXNEGR::PIN0,
            1 => MUXNEGR::PIN1,
            2 => MUXNEGR::PIN2,
            3 => MUXNEGR::PIN3,
            4 => MUXNEGR::GND,
            5 => MUXNEGR::VSCALE,
            6 => MUXNEGR::BANDGAP,
            7 => MUXNEGR::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline]
    pub fn is_pin0(&self) -> bool {
        *self == MUXNEGR::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline]
    pub fn is_pin1(&self) -> bool {
        *self == MUXNEGR::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline]
    pub fn is_pin2(&self) -> bool {
        *self == MUXNEGR::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline]
    pub fn is_pin3(&self) -> bool {
        *self == MUXNEGR::PIN3
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline]
    pub fn is_gnd(&self) -> bool {
        *self == MUXNEGR::GND
    }
    #[doc = "Checks if the value of the field is `VSCALE`"]
    #[inline]
    pub fn is_vscale(&self) -> bool {
        *self == MUXNEGR::VSCALE
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXNEGR::BANDGAP
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == MUXNEGR::DAC
    }
}
#[doc = "Possible values of the field `MUXPOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXPOSR {
    #[doc = "I/O pin 0"]
    PIN0,
    #[doc = "I/O pin 1"]
    PIN1,
    #[doc = "I/O pin 2"]
    PIN2,
    #[doc = "I/O pin 3"]
    PIN3,
}
impl MUXPOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUXPOSR::PIN0 => 0,
            MUXPOSR::PIN1 => 1,
            MUXPOSR::PIN2 => 2,
            MUXPOSR::PIN3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUXPOSR {
        match value {
            0 => MUXPOSR::PIN0,
            1 => MUXPOSR::PIN1,
            2 => MUXPOSR::PIN2,
            3 => MUXPOSR::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline]
    pub fn is_pin0(&self) -> bool {
        *self == MUXPOSR::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline]
    pub fn is_pin1(&self) -> bool {
        *self == MUXPOSR::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline]
    pub fn is_pin2(&self) -> bool {
        *self == MUXPOSR::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline]
    pub fn is_pin3(&self) -> bool {
        *self == MUXPOSR::PIN3
    }
}
#[doc = r" Value of the field"]
pub struct SWAPR {
    bits: bool,
}
impl SWAPR {
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
#[doc = "Possible values of the field `OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTR {
    #[doc = "The output of COMPn is not routed to the COMPn I/O port"]
    OFF,
    #[doc = "The asynchronous output of COMPn is routed to the COMPn I/O port"]
    ASYNC,
    #[doc = "The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    SYNC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTR::OFF => 0,
            OUTR::ASYNC => 1,
            OUTR::SYNC => 2,
            OUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTR {
        match value {
            0 => OUTR::OFF,
            1 => OUTR::ASYNC,
            2 => OUTR::SYNC,
            i => OUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == OUTR::OFF
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline]
    pub fn is_async(&self) -> bool {
        *self == OUTR::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline]
    pub fn is_sync(&self) -> bool {
        *self == OUTR::SYNC
    }
}
#[doc = r" Value of the field"]
pub struct HYSTR {
    bits: bool,
}
impl HYSTR {
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
#[doc = "Possible values of the field `FLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLENR {
    #[doc = "No filtering"]
    OFF,
    #[doc = "3-bit majority function (2 of 3)"]
    MAJ3,
    #[doc = "5-bit majority function (3 of 5)"]
    MAJ5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLENR::OFF => 0,
            FLENR::MAJ3 => 1,
            FLENR::MAJ5 => 2,
            FLENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLENR {
        match value {
            0 => FLENR::OFF,
            1 => FLENR::MAJ3,
            2 => FLENR::MAJ5,
            i => FLENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == FLENR::OFF
    }
    #[doc = "Checks if the value of the field is `MAJ3`"]
    #[inline]
    pub fn is_maj3(&self) -> bool {
        *self == FLENR::MAJ3
    }
    #[doc = "Checks if the value of the field is `MAJ5`"]
    #[inline]
    pub fn is_maj5(&self) -> bool {
        *self == FLENR::MAJ5
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
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SINGLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SINGLEW<'a> {
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
#[doc = "Values that can be written to the field `SPEED`"]
pub enum SPEEDW {
    #[doc = "Low speed"]
    LOW,
    #[doc = "High speed"]
    HIGH,
}
impl SPEEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPEEDW::LOW => 0,
            SPEEDW::HIGH => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SPEEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPEEDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low speed"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SPEEDW::LOW)
    }
    #[doc = "High speed"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SPEEDW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTSEL`"]
pub enum INTSELW {
    #[doc = "Interrupt on comparator output toggle"]
    TOGGLE,
    #[doc = "Interrupt on comparator output rising"]
    RISING,
    #[doc = "Interrupt on comparator output falling"]
    FALLING,
    #[doc = "Interrupt on end of comparison (single-shot mode only)"]
    EOC,
}
impl INTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INTSELW::TOGGLE => 0,
            INTSELW::RISING => 1,
            INTSELW::FALLING => 2,
            INTSELW::EOC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt on comparator output toggle"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(INTSELW::TOGGLE)
    }
    #[doc = "Interrupt on comparator output rising"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTSELW::RISING)
    }
    #[doc = "Interrupt on comparator output falling"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTSELW::FALLING)
    }
    #[doc = "Interrupt on end of comparison (single-shot mode only)"]
    #[inline]
    pub fn eoc(self) -> &'a mut W {
        self.variant(INTSELW::EOC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MUXNEG`"]
pub enum MUXNEGW {
    #[doc = "I/O pin 0"]
    PIN0,
    #[doc = "I/O pin 1"]
    PIN1,
    #[doc = "I/O pin 2"]
    PIN2,
    #[doc = "I/O pin 3"]
    PIN3,
    #[doc = "Ground"]
    GND,
    #[doc = "VDD scaler"]
    VSCALE,
    #[doc = "Internal bandgap voltage"]
    BANDGAP,
    #[doc = "DAC output"]
    DAC,
}
impl MUXNEGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUXNEGW::PIN0 => 0,
            MUXNEGW::PIN1 => 1,
            MUXNEGW::PIN2 => 2,
            MUXNEGW::PIN3 => 3,
            MUXNEGW::GND => 4,
            MUXNEGW::VSCALE => 5,
            MUXNEGW::BANDGAP => 6,
            MUXNEGW::DAC => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUXNEGW<'a> {
    w: &'a mut W,
}
impl<'a> _MUXNEGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUXNEGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "I/O pin 0"]
    #[inline]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXNEGW::PIN0)
    }
    #[doc = "I/O pin 1"]
    #[inline]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXNEGW::PIN1)
    }
    #[doc = "I/O pin 2"]
    #[inline]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXNEGW::PIN2)
    }
    #[doc = "I/O pin 3"]
    #[inline]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXNEGW::PIN3)
    }
    #[doc = "Ground"]
    #[inline]
    pub fn gnd(self) -> &'a mut W {
        self.variant(MUXNEGW::GND)
    }
    #[doc = "VDD scaler"]
    #[inline]
    pub fn vscale(self) -> &'a mut W {
        self.variant(MUXNEGW::VSCALE)
    }
    #[doc = "Internal bandgap voltage"]
    #[inline]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(MUXNEGW::BANDGAP)
    }
    #[doc = "DAC output"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(MUXNEGW::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MUXPOS`"]
pub enum MUXPOSW {
    #[doc = "I/O pin 0"]
    PIN0,
    #[doc = "I/O pin 1"]
    PIN1,
    #[doc = "I/O pin 2"]
    PIN2,
    #[doc = "I/O pin 3"]
    PIN3,
}
impl MUXPOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUXPOSW::PIN0 => 0,
            MUXPOSW::PIN1 => 1,
            MUXPOSW::PIN2 => 2,
            MUXPOSW::PIN3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUXPOSW<'a> {
    w: &'a mut W,
}
impl<'a> _MUXPOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUXPOSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "I/O pin 0"]
    #[inline]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN0)
    }
    #[doc = "I/O pin 1"]
    #[inline]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN1)
    }
    #[doc = "I/O pin 2"]
    #[inline]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN2)
    }
    #[doc = "I/O pin 3"]
    #[inline]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SWAPW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OUT`"]
pub enum OUTW {
    #[doc = "The output of COMPn is not routed to the COMPn I/O port"]
    OFF,
    #[doc = "The asynchronous output of COMPn is routed to the COMPn I/O port"]
    ASYNC,
    #[doc = "The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    SYNC,
}
impl OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTW::OFF => 0,
            OUTW::ASYNC => 1,
            OUTW::SYNC => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The output of COMPn is not routed to the COMPn I/O port"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(OUTW::OFF)
    }
    #[doc = "The asynchronous output of COMPn is routed to the COMPn I/O port"]
    #[inline]
    pub fn async(self) -> &'a mut W {
        self.variant(OUTW::ASYNC)
    }
    #[doc = "The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    #[inline]
    pub fn sync(self) -> &'a mut W {
        self.variant(OUTW::SYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLEN`"]
pub enum FLENW {
    #[doc = "No filtering"]
    OFF,
    #[doc = "3-bit majority function (2 of 3)"]
    MAJ3,
    #[doc = "5-bit majority function (3 of 5)"]
    MAJ5,
}
impl FLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLENW::OFF => 0,
            FLENW::MAJ3 => 1,
            FLENW::MAJ5 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No filtering"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(FLENW::OFF)
    }
    #[doc = "3-bit majority function (2 of 3)"]
    #[inline]
    pub fn maj3(self) -> &'a mut W {
        self.variant(FLENW::MAJ3)
    }
    #[doc = "5-bit majority function (3 of 5)"]
    #[inline]
    pub fn maj5(self) -> &'a mut W {
        self.variant(FLENW::MAJ5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bit 1 - Single-Shot Mode"]
    #[inline]
    pub fn single(&self) -> SINGLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SINGLER { bits }
    }
    #[doc = "Bits 2:3 - Speed Selection"]
    #[inline]
    pub fn speed(&self) -> SPEEDR {
        SPEEDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - Interrupt Selection"]
    #[inline]
    pub fn intsel(&self) -> INTSELR {
        INTSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline]
    pub fn muxneg(&self) -> MUXNEGR {
        MUXNEGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Positive Input Mux Selection"]
    #[inline]
    pub fn muxpos(&self) -> MUXPOSR {
        MUXPOSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline]
    pub fn swap(&self) -> SWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWAPR { bits }
    }
    #[doc = "Bits 16:17 - Output"]
    #[inline]
    pub fn out(&self) -> OUTR {
        OUTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline]
    pub fn hyst(&self) -> HYSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HYSTR { bits }
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline]
    pub fn flen(&self) -> FLENR {
        FLENR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Single-Shot Mode"]
    #[inline]
    pub fn single(&mut self) -> _SINGLEW {
        _SINGLEW { w: self }
    }
    #[doc = "Bits 2:3 - Speed Selection"]
    #[inline]
    pub fn speed(&mut self) -> _SPEEDW {
        _SPEEDW { w: self }
    }
    #[doc = "Bits 5:6 - Interrupt Selection"]
    #[inline]
    pub fn intsel(&mut self) -> _INTSELW {
        _INTSELW { w: self }
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline]
    pub fn muxneg(&mut self) -> _MUXNEGW {
        _MUXNEGW { w: self }
    }
    #[doc = "Bits 12:13 - Positive Input Mux Selection"]
    #[inline]
    pub fn muxpos(&mut self) -> _MUXPOSW {
        _MUXPOSW { w: self }
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline]
    pub fn swap(&mut self) -> _SWAPW {
        _SWAPW { w: self }
    }
    #[doc = "Bits 16:17 - Output"]
    #[inline]
    pub fn out(&mut self) -> _OUTW {
        _OUTW { w: self }
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline]
    pub fn flen(&mut self) -> _FLENW {
        _FLENW { w: self }
    }
}
