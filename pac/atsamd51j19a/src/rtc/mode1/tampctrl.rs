#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TAMPCTRL {
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
#[doc = "Possible values of the field `IN0ACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0ACTR {
    #[doc = "Off (Disabled)"]
    OFF,
    #[doc = "Wake without timestamp"]
    WAKE,
    #[doc = "Capture timestamp"]
    CAPTURE,
    #[doc = "Compare IN0 to OUT"]
    ACTL,
}
impl IN0ACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IN0ACTR::OFF => 0,
            IN0ACTR::WAKE => 1,
            IN0ACTR::CAPTURE => 2,
            IN0ACTR::ACTL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IN0ACTR {
        match value {
            0 => IN0ACTR::OFF,
            1 => IN0ACTR::WAKE,
            2 => IN0ACTR::CAPTURE,
            3 => IN0ACTR::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == IN0ACTR::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline]
    pub fn is_wake(&self) -> bool {
        *self == IN0ACTR::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline]
    pub fn is_capture(&self) -> bool {
        *self == IN0ACTR::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline]
    pub fn is_actl(&self) -> bool {
        *self == IN0ACTR::ACTL
    }
}
#[doc = "Possible values of the field `IN1ACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1ACTR {
    #[doc = "Off (Disabled)"]
    OFF,
    #[doc = "Wake without timestamp"]
    WAKE,
    #[doc = "Capture timestamp"]
    CAPTURE,
    #[doc = "Compare IN1 to OUT"]
    ACTL,
}
impl IN1ACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IN1ACTR::OFF => 0,
            IN1ACTR::WAKE => 1,
            IN1ACTR::CAPTURE => 2,
            IN1ACTR::ACTL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IN1ACTR {
        match value {
            0 => IN1ACTR::OFF,
            1 => IN1ACTR::WAKE,
            2 => IN1ACTR::CAPTURE,
            3 => IN1ACTR::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == IN1ACTR::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline]
    pub fn is_wake(&self) -> bool {
        *self == IN1ACTR::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline]
    pub fn is_capture(&self) -> bool {
        *self == IN1ACTR::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline]
    pub fn is_actl(&self) -> bool {
        *self == IN1ACTR::ACTL
    }
}
#[doc = "Possible values of the field `IN2ACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2ACTR {
    #[doc = "Off (Disabled)"]
    OFF,
    #[doc = "Wake without timestamp"]
    WAKE,
    #[doc = "Capture timestamp"]
    CAPTURE,
    #[doc = "Compare IN2 to OUT"]
    ACTL,
}
impl IN2ACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IN2ACTR::OFF => 0,
            IN2ACTR::WAKE => 1,
            IN2ACTR::CAPTURE => 2,
            IN2ACTR::ACTL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IN2ACTR {
        match value {
            0 => IN2ACTR::OFF,
            1 => IN2ACTR::WAKE,
            2 => IN2ACTR::CAPTURE,
            3 => IN2ACTR::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == IN2ACTR::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline]
    pub fn is_wake(&self) -> bool {
        *self == IN2ACTR::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline]
    pub fn is_capture(&self) -> bool {
        *self == IN2ACTR::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline]
    pub fn is_actl(&self) -> bool {
        *self == IN2ACTR::ACTL
    }
}
#[doc = "Possible values of the field `IN3ACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3ACTR {
    #[doc = "Off (Disabled)"]
    OFF,
    #[doc = "Wake without timestamp"]
    WAKE,
    #[doc = "Capture timestamp"]
    CAPTURE,
    #[doc = "Compare IN3 to OUT"]
    ACTL,
}
impl IN3ACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IN3ACTR::OFF => 0,
            IN3ACTR::WAKE => 1,
            IN3ACTR::CAPTURE => 2,
            IN3ACTR::ACTL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IN3ACTR {
        match value {
            0 => IN3ACTR::OFF,
            1 => IN3ACTR::WAKE,
            2 => IN3ACTR::CAPTURE,
            3 => IN3ACTR::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == IN3ACTR::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline]
    pub fn is_wake(&self) -> bool {
        *self == IN3ACTR::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline]
    pub fn is_capture(&self) -> bool {
        *self == IN3ACTR::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline]
    pub fn is_actl(&self) -> bool {
        *self == IN3ACTR::ACTL
    }
}
#[doc = "Possible values of the field `IN4ACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN4ACTR {
    #[doc = "Off (Disabled)"]
    OFF,
    #[doc = "Wake without timestamp"]
    WAKE,
    #[doc = "Capture timestamp"]
    CAPTURE,
    #[doc = "Compare IN4 to OUT"]
    ACTL,
}
impl IN4ACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IN4ACTR::OFF => 0,
            IN4ACTR::WAKE => 1,
            IN4ACTR::CAPTURE => 2,
            IN4ACTR::ACTL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IN4ACTR {
        match value {
            0 => IN4ACTR::OFF,
            1 => IN4ACTR::WAKE,
            2 => IN4ACTR::CAPTURE,
            3 => IN4ACTR::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == IN4ACTR::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline]
    pub fn is_wake(&self) -> bool {
        *self == IN4ACTR::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline]
    pub fn is_capture(&self) -> bool {
        *self == IN4ACTR::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline]
    pub fn is_actl(&self) -> bool {
        *self == IN4ACTR::ACTL
    }
}
#[doc = r" Value of the field"]
pub struct TAMLVL0R {
    bits: bool,
}
impl TAMLVL0R {
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
pub struct TAMLVL1R {
    bits: bool,
}
impl TAMLVL1R {
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
pub struct TAMLVL2R {
    bits: bool,
}
impl TAMLVL2R {
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
pub struct TAMLVL3R {
    bits: bool,
}
impl TAMLVL3R {
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
pub struct TAMLVL4R {
    bits: bool,
}
impl TAMLVL4R {
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
pub struct DEBNC0R {
    bits: bool,
}
impl DEBNC0R {
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
pub struct DEBNC1R {
    bits: bool,
}
impl DEBNC1R {
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
pub struct DEBNC2R {
    bits: bool,
}
impl DEBNC2R {
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
pub struct DEBNC3R {
    bits: bool,
}
impl DEBNC3R {
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
pub struct DEBNC4R {
    bits: bool,
}
impl DEBNC4R {
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
#[doc = "Values that can be written to the field `IN0ACT`"]
pub enum IN0ACTW {
    #[doc = "Off (Disabled)"]
    OFF,
    #[doc = "Wake without timestamp"]
    WAKE,
    #[doc = "Capture timestamp"]
    CAPTURE,
    #[doc = "Compare IN0 to OUT"]
    ACTL,
}
impl IN0ACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IN0ACTW::OFF => 0,
            IN0ACTW::WAKE => 1,
            IN0ACTW::CAPTURE => 2,
            IN0ACTW::ACTL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN0ACTW<'a> {
    w: &'a mut W,
}
impl<'a> _IN0ACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN0ACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(IN0ACTW::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN0ACTW::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN0ACTW::CAPTURE)
    }
    #[doc = "Compare IN0 to OUT"]
    #[inline]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN0ACTW::ACTL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IN1ACT`"]
pub enum IN1ACTW {
    #[doc = "Off (Disabled)"]
    OFF,
    #[doc = "Wake without timestamp"]
    WAKE,
    #[doc = "Capture timestamp"]
    CAPTURE,
    #[doc = "Compare IN1 to OUT"]
    ACTL,
}
impl IN1ACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IN1ACTW::OFF => 0,
            IN1ACTW::WAKE => 1,
            IN1ACTW::CAPTURE => 2,
            IN1ACTW::ACTL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN1ACTW<'a> {
    w: &'a mut W,
}
impl<'a> _IN1ACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN1ACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(IN1ACTW::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN1ACTW::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN1ACTW::CAPTURE)
    }
    #[doc = "Compare IN1 to OUT"]
    #[inline]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN1ACTW::ACTL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IN2ACT`"]
pub enum IN2ACTW {
    #[doc = "Off (Disabled)"]
    OFF,
    #[doc = "Wake without timestamp"]
    WAKE,
    #[doc = "Capture timestamp"]
    CAPTURE,
    #[doc = "Compare IN2 to OUT"]
    ACTL,
}
impl IN2ACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IN2ACTW::OFF => 0,
            IN2ACTW::WAKE => 1,
            IN2ACTW::CAPTURE => 2,
            IN2ACTW::ACTL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN2ACTW<'a> {
    w: &'a mut W,
}
impl<'a> _IN2ACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN2ACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(IN2ACTW::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN2ACTW::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN2ACTW::CAPTURE)
    }
    #[doc = "Compare IN2 to OUT"]
    #[inline]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN2ACTW::ACTL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IN3ACT`"]
pub enum IN3ACTW {
    #[doc = "Off (Disabled)"]
    OFF,
    #[doc = "Wake without timestamp"]
    WAKE,
    #[doc = "Capture timestamp"]
    CAPTURE,
    #[doc = "Compare IN3 to OUT"]
    ACTL,
}
impl IN3ACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IN3ACTW::OFF => 0,
            IN3ACTW::WAKE => 1,
            IN3ACTW::CAPTURE => 2,
            IN3ACTW::ACTL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN3ACTW<'a> {
    w: &'a mut W,
}
impl<'a> _IN3ACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN3ACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(IN3ACTW::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN3ACTW::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN3ACTW::CAPTURE)
    }
    #[doc = "Compare IN3 to OUT"]
    #[inline]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN3ACTW::ACTL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IN4ACT`"]
pub enum IN4ACTW {
    #[doc = "Off (Disabled)"]
    OFF,
    #[doc = "Wake without timestamp"]
    WAKE,
    #[doc = "Capture timestamp"]
    CAPTURE,
    #[doc = "Compare IN4 to OUT"]
    ACTL,
}
impl IN4ACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IN4ACTW::OFF => 0,
            IN4ACTW::WAKE => 1,
            IN4ACTW::CAPTURE => 2,
            IN4ACTW::ACTL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN4ACTW<'a> {
    w: &'a mut W,
}
impl<'a> _IN4ACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN4ACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(IN4ACTW::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN4ACTW::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN4ACTW::CAPTURE)
    }
    #[doc = "Compare IN4 to OUT"]
    #[inline]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN4ACTW::ACTL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAMLVL0W<'a> {
    w: &'a mut W,
}
impl<'a> _TAMLVL0W<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAMLVL1W<'a> {
    w: &'a mut W,
}
impl<'a> _TAMLVL1W<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAMLVL2W<'a> {
    w: &'a mut W,
}
impl<'a> _TAMLVL2W<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAMLVL3W<'a> {
    w: &'a mut W,
}
impl<'a> _TAMLVL3W<'a> {
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
#[doc = r" Proxy"]
pub struct _TAMLVL4W<'a> {
    w: &'a mut W,
}
impl<'a> _TAMLVL4W<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBNC0W<'a> {
    w: &'a mut W,
}
impl<'a> _DEBNC0W<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBNC1W<'a> {
    w: &'a mut W,
}
impl<'a> _DEBNC1W<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBNC2W<'a> {
    w: &'a mut W,
}
impl<'a> _DEBNC2W<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBNC3W<'a> {
    w: &'a mut W,
}
impl<'a> _DEBNC3W<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBNC4W<'a> {
    w: &'a mut W,
}
impl<'a> _DEBNC4W<'a> {
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:1 - Tamper Input 0 Action"]
    #[inline]
    pub fn in0act(&self) -> IN0ACTR {
        IN0ACTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Tamper Input 1 Action"]
    #[inline]
    pub fn in1act(&self) -> IN1ACTR {
        IN1ACTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Tamper Input 2 Action"]
    #[inline]
    pub fn in2act(&self) -> IN2ACTR {
        IN2ACTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Tamper Input 3 Action"]
    #[inline]
    pub fn in3act(&self) -> IN3ACTR {
        IN3ACTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Tamper Input 4 Action"]
    #[inline]
    pub fn in4act(&self) -> IN4ACTR {
        IN4ACTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Tamper Level Select 0"]
    #[inline]
    pub fn tamlvl0(&self) -> TAMLVL0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAMLVL0R { bits }
    }
    #[doc = "Bit 17 - Tamper Level Select 1"]
    #[inline]
    pub fn tamlvl1(&self) -> TAMLVL1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAMLVL1R { bits }
    }
    #[doc = "Bit 18 - Tamper Level Select 2"]
    #[inline]
    pub fn tamlvl2(&self) -> TAMLVL2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAMLVL2R { bits }
    }
    #[doc = "Bit 19 - Tamper Level Select 3"]
    #[inline]
    pub fn tamlvl3(&self) -> TAMLVL3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAMLVL3R { bits }
    }
    #[doc = "Bit 20 - Tamper Level Select 4"]
    #[inline]
    pub fn tamlvl4(&self) -> TAMLVL4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAMLVL4R { bits }
    }
    #[doc = "Bit 24 - Debouncer Enable 0"]
    #[inline]
    pub fn debnc0(&self) -> DEBNC0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBNC0R { bits }
    }
    #[doc = "Bit 25 - Debouncer Enable 1"]
    #[inline]
    pub fn debnc1(&self) -> DEBNC1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBNC1R { bits }
    }
    #[doc = "Bit 26 - Debouncer Enable 2"]
    #[inline]
    pub fn debnc2(&self) -> DEBNC2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBNC2R { bits }
    }
    #[doc = "Bit 27 - Debouncer Enable 3"]
    #[inline]
    pub fn debnc3(&self) -> DEBNC3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBNC3R { bits }
    }
    #[doc = "Bit 28 - Debouncer Enable 4"]
    #[inline]
    pub fn debnc4(&self) -> DEBNC4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBNC4R { bits }
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
    #[doc = "Bits 0:1 - Tamper Input 0 Action"]
    #[inline]
    pub fn in0act(&mut self) -> _IN0ACTW {
        _IN0ACTW { w: self }
    }
    #[doc = "Bits 2:3 - Tamper Input 1 Action"]
    #[inline]
    pub fn in1act(&mut self) -> _IN1ACTW {
        _IN1ACTW { w: self }
    }
    #[doc = "Bits 4:5 - Tamper Input 2 Action"]
    #[inline]
    pub fn in2act(&mut self) -> _IN2ACTW {
        _IN2ACTW { w: self }
    }
    #[doc = "Bits 6:7 - Tamper Input 3 Action"]
    #[inline]
    pub fn in3act(&mut self) -> _IN3ACTW {
        _IN3ACTW { w: self }
    }
    #[doc = "Bits 8:9 - Tamper Input 4 Action"]
    #[inline]
    pub fn in4act(&mut self) -> _IN4ACTW {
        _IN4ACTW { w: self }
    }
    #[doc = "Bit 16 - Tamper Level Select 0"]
    #[inline]
    pub fn tamlvl0(&mut self) -> _TAMLVL0W {
        _TAMLVL0W { w: self }
    }
    #[doc = "Bit 17 - Tamper Level Select 1"]
    #[inline]
    pub fn tamlvl1(&mut self) -> _TAMLVL1W {
        _TAMLVL1W { w: self }
    }
    #[doc = "Bit 18 - Tamper Level Select 2"]
    #[inline]
    pub fn tamlvl2(&mut self) -> _TAMLVL2W {
        _TAMLVL2W { w: self }
    }
    #[doc = "Bit 19 - Tamper Level Select 3"]
    #[inline]
    pub fn tamlvl3(&mut self) -> _TAMLVL3W {
        _TAMLVL3W { w: self }
    }
    #[doc = "Bit 20 - Tamper Level Select 4"]
    #[inline]
    pub fn tamlvl4(&mut self) -> _TAMLVL4W {
        _TAMLVL4W { w: self }
    }
    #[doc = "Bit 24 - Debouncer Enable 0"]
    #[inline]
    pub fn debnc0(&mut self) -> _DEBNC0W {
        _DEBNC0W { w: self }
    }
    #[doc = "Bit 25 - Debouncer Enable 1"]
    #[inline]
    pub fn debnc1(&mut self) -> _DEBNC1W {
        _DEBNC1W { w: self }
    }
    #[doc = "Bit 26 - Debouncer Enable 2"]
    #[inline]
    pub fn debnc2(&mut self) -> _DEBNC2W {
        _DEBNC2W { w: self }
    }
    #[doc = "Bit 27 - Debouncer Enable 3"]
    #[inline]
    pub fn debnc3(&mut self) -> _DEBNC3W {
        _DEBNC3W { w: self }
    }
    #[doc = "Bit 28 - Debouncer Enable 4"]
    #[inline]
    pub fn debnc4(&mut self) -> _DEBNC4W {
        _DEBNC4W { w: self }
    }
}
