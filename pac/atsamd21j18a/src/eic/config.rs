#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG {
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
#[doc = "Possible values of the field `SENSE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENSE0R {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising-edge detection"]
    RISE,
    #[doc = "Falling-edge detection"]
    FALL,
    #[doc = "Both-edges detection"]
    BOTH,
    #[doc = "High-level detection"]
    HIGH,
    #[doc = "Low-level detection"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SENSE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SENSE0R::NONE => 0,
            SENSE0R::RISE => 1,
            SENSE0R::FALL => 2,
            SENSE0R::BOTH => 3,
            SENSE0R::HIGH => 4,
            SENSE0R::LOW => 5,
            SENSE0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SENSE0R {
        match value {
            0 => SENSE0R::NONE,
            1 => SENSE0R::RISE,
            2 => SENSE0R::FALL,
            3 => SENSE0R::BOTH,
            4 => SENSE0R::HIGH,
            5 => SENSE0R::LOW,
            i => SENSE0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SENSE0R::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == SENSE0R::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == SENSE0R::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == SENSE0R::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SENSE0R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SENSE0R::LOW
    }
}
#[doc = r" Value of the field"]
pub struct FILTEN0R {
    bits: bool,
}
impl FILTEN0R {
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
#[doc = "Possible values of the field `SENSE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENSE1R {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SENSE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SENSE1R::NONE => 0,
            SENSE1R::RISE => 1,
            SENSE1R::FALL => 2,
            SENSE1R::BOTH => 3,
            SENSE1R::HIGH => 4,
            SENSE1R::LOW => 5,
            SENSE1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SENSE1R {
        match value {
            0 => SENSE1R::NONE,
            1 => SENSE1R::RISE,
            2 => SENSE1R::FALL,
            3 => SENSE1R::BOTH,
            4 => SENSE1R::HIGH,
            5 => SENSE1R::LOW,
            i => SENSE1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SENSE1R::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == SENSE1R::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == SENSE1R::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == SENSE1R::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SENSE1R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SENSE1R::LOW
    }
}
#[doc = r" Value of the field"]
pub struct FILTEN1R {
    bits: bool,
}
impl FILTEN1R {
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
#[doc = "Possible values of the field `SENSE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENSE2R {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SENSE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SENSE2R::NONE => 0,
            SENSE2R::RISE => 1,
            SENSE2R::FALL => 2,
            SENSE2R::BOTH => 3,
            SENSE2R::HIGH => 4,
            SENSE2R::LOW => 5,
            SENSE2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SENSE2R {
        match value {
            0 => SENSE2R::NONE,
            1 => SENSE2R::RISE,
            2 => SENSE2R::FALL,
            3 => SENSE2R::BOTH,
            4 => SENSE2R::HIGH,
            5 => SENSE2R::LOW,
            i => SENSE2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SENSE2R::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == SENSE2R::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == SENSE2R::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == SENSE2R::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SENSE2R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SENSE2R::LOW
    }
}
#[doc = r" Value of the field"]
pub struct FILTEN2R {
    bits: bool,
}
impl FILTEN2R {
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
#[doc = "Possible values of the field `SENSE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENSE3R {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SENSE3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SENSE3R::NONE => 0,
            SENSE3R::RISE => 1,
            SENSE3R::FALL => 2,
            SENSE3R::BOTH => 3,
            SENSE3R::HIGH => 4,
            SENSE3R::LOW => 5,
            SENSE3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SENSE3R {
        match value {
            0 => SENSE3R::NONE,
            1 => SENSE3R::RISE,
            2 => SENSE3R::FALL,
            3 => SENSE3R::BOTH,
            4 => SENSE3R::HIGH,
            5 => SENSE3R::LOW,
            i => SENSE3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SENSE3R::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == SENSE3R::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == SENSE3R::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == SENSE3R::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SENSE3R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SENSE3R::LOW
    }
}
#[doc = r" Value of the field"]
pub struct FILTEN3R {
    bits: bool,
}
impl FILTEN3R {
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
#[doc = "Possible values of the field `SENSE4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENSE4R {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SENSE4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SENSE4R::NONE => 0,
            SENSE4R::RISE => 1,
            SENSE4R::FALL => 2,
            SENSE4R::BOTH => 3,
            SENSE4R::HIGH => 4,
            SENSE4R::LOW => 5,
            SENSE4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SENSE4R {
        match value {
            0 => SENSE4R::NONE,
            1 => SENSE4R::RISE,
            2 => SENSE4R::FALL,
            3 => SENSE4R::BOTH,
            4 => SENSE4R::HIGH,
            5 => SENSE4R::LOW,
            i => SENSE4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SENSE4R::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == SENSE4R::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == SENSE4R::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == SENSE4R::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SENSE4R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SENSE4R::LOW
    }
}
#[doc = r" Value of the field"]
pub struct FILTEN4R {
    bits: bool,
}
impl FILTEN4R {
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
#[doc = "Possible values of the field `SENSE5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENSE5R {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SENSE5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SENSE5R::NONE => 0,
            SENSE5R::RISE => 1,
            SENSE5R::FALL => 2,
            SENSE5R::BOTH => 3,
            SENSE5R::HIGH => 4,
            SENSE5R::LOW => 5,
            SENSE5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SENSE5R {
        match value {
            0 => SENSE5R::NONE,
            1 => SENSE5R::RISE,
            2 => SENSE5R::FALL,
            3 => SENSE5R::BOTH,
            4 => SENSE5R::HIGH,
            5 => SENSE5R::LOW,
            i => SENSE5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SENSE5R::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == SENSE5R::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == SENSE5R::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == SENSE5R::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SENSE5R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SENSE5R::LOW
    }
}
#[doc = r" Value of the field"]
pub struct FILTEN5R {
    bits: bool,
}
impl FILTEN5R {
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
#[doc = "Possible values of the field `SENSE6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENSE6R {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SENSE6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SENSE6R::NONE => 0,
            SENSE6R::RISE => 1,
            SENSE6R::FALL => 2,
            SENSE6R::BOTH => 3,
            SENSE6R::HIGH => 4,
            SENSE6R::LOW => 5,
            SENSE6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SENSE6R {
        match value {
            0 => SENSE6R::NONE,
            1 => SENSE6R::RISE,
            2 => SENSE6R::FALL,
            3 => SENSE6R::BOTH,
            4 => SENSE6R::HIGH,
            5 => SENSE6R::LOW,
            i => SENSE6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SENSE6R::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == SENSE6R::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == SENSE6R::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == SENSE6R::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SENSE6R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SENSE6R::LOW
    }
}
#[doc = r" Value of the field"]
pub struct FILTEN6R {
    bits: bool,
}
impl FILTEN6R {
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
#[doc = "Possible values of the field `SENSE7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENSE7R {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SENSE7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SENSE7R::NONE => 0,
            SENSE7R::RISE => 1,
            SENSE7R::FALL => 2,
            SENSE7R::BOTH => 3,
            SENSE7R::HIGH => 4,
            SENSE7R::LOW => 5,
            SENSE7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SENSE7R {
        match value {
            0 => SENSE7R::NONE,
            1 => SENSE7R::RISE,
            2 => SENSE7R::FALL,
            3 => SENSE7R::BOTH,
            4 => SENSE7R::HIGH,
            5 => SENSE7R::LOW,
            i => SENSE7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SENSE7R::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == SENSE7R::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == SENSE7R::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == SENSE7R::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SENSE7R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SENSE7R::LOW
    }
}
#[doc = r" Value of the field"]
pub struct FILTEN7R {
    bits: bool,
}
impl FILTEN7R {
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
#[doc = "Values that can be written to the field `SENSE0`"]
pub enum SENSE0W {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising-edge detection"]
    RISE,
    #[doc = "Falling-edge detection"]
    FALL,
    #[doc = "Both-edges detection"]
    BOTH,
    #[doc = "High-level detection"]
    HIGH,
    #[doc = "Low-level detection"]
    LOW,
}
impl SENSE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SENSE0W::NONE => 0,
            SENSE0W::RISE => 1,
            SENSE0W::FALL => 2,
            SENSE0W::BOTH => 3,
            SENSE0W::HIGH => 4,
            SENSE0W::LOW => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SENSE0W<'a> {
    w: &'a mut W,
}
impl<'a> _SENSE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SENSE0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No detection"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE0W::NONE)
    }
    #[doc = "Rising-edge detection"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE0W::RISE)
    }
    #[doc = "Falling-edge detection"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE0W::FALL)
    }
    #[doc = "Both-edges detection"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE0W::BOTH)
    }
    #[doc = "High-level detection"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE0W::HIGH)
    }
    #[doc = "Low-level detection"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE0W::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _FILTEN0W<'a> {
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
#[doc = "Values that can be written to the field `SENSE1`"]
pub enum SENSE1W {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
}
impl SENSE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SENSE1W::NONE => 0,
            SENSE1W::RISE => 1,
            SENSE1W::FALL => 2,
            SENSE1W::BOTH => 3,
            SENSE1W::HIGH => 4,
            SENSE1W::LOW => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SENSE1W<'a> {
    w: &'a mut W,
}
impl<'a> _SENSE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SENSE1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No detection"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE1W::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE1W::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE1W::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE1W::BOTH)
    }
    #[doc = "High level detection"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE1W::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE1W::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _FILTEN1W<'a> {
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
#[doc = "Values that can be written to the field `SENSE2`"]
pub enum SENSE2W {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
}
impl SENSE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SENSE2W::NONE => 0,
            SENSE2W::RISE => 1,
            SENSE2W::FALL => 2,
            SENSE2W::BOTH => 3,
            SENSE2W::HIGH => 4,
            SENSE2W::LOW => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SENSE2W<'a> {
    w: &'a mut W,
}
impl<'a> _SENSE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SENSE2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No detection"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE2W::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE2W::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE2W::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE2W::BOTH)
    }
    #[doc = "High level detection"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE2W::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE2W::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _FILTEN2W<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SENSE3`"]
pub enum SENSE3W {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
}
impl SENSE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SENSE3W::NONE => 0,
            SENSE3W::RISE => 1,
            SENSE3W::FALL => 2,
            SENSE3W::BOTH => 3,
            SENSE3W::HIGH => 4,
            SENSE3W::LOW => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SENSE3W<'a> {
    w: &'a mut W,
}
impl<'a> _SENSE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SENSE3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No detection"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE3W::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE3W::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE3W::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE3W::BOTH)
    }
    #[doc = "High level detection"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE3W::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE3W::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _FILTEN3W<'a> {
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
#[doc = "Values that can be written to the field `SENSE4`"]
pub enum SENSE4W {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
}
impl SENSE4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SENSE4W::NONE => 0,
            SENSE4W::RISE => 1,
            SENSE4W::FALL => 2,
            SENSE4W::BOTH => 3,
            SENSE4W::HIGH => 4,
            SENSE4W::LOW => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SENSE4W<'a> {
    w: &'a mut W,
}
impl<'a> _SENSE4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SENSE4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No detection"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE4W::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE4W::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE4W::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE4W::BOTH)
    }
    #[doc = "High level detection"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE4W::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE4W::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _FILTEN4W<'a> {
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
#[doc = "Values that can be written to the field `SENSE5`"]
pub enum SENSE5W {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
}
impl SENSE5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SENSE5W::NONE => 0,
            SENSE5W::RISE => 1,
            SENSE5W::FALL => 2,
            SENSE5W::BOTH => 3,
            SENSE5W::HIGH => 4,
            SENSE5W::LOW => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SENSE5W<'a> {
    w: &'a mut W,
}
impl<'a> _SENSE5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SENSE5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No detection"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE5W::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE5W::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE5W::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE5W::BOTH)
    }
    #[doc = "High level detection"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE5W::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE5W::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _FILTEN5W<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SENSE6`"]
pub enum SENSE6W {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
}
impl SENSE6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SENSE6W::NONE => 0,
            SENSE6W::RISE => 1,
            SENSE6W::FALL => 2,
            SENSE6W::BOTH => 3,
            SENSE6W::HIGH => 4,
            SENSE6W::LOW => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SENSE6W<'a> {
    w: &'a mut W,
}
impl<'a> _SENSE6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SENSE6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No detection"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE6W::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE6W::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE6W::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE6W::BOTH)
    }
    #[doc = "High level detection"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE6W::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE6W::LOW)
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
#[doc = r" Proxy"]
pub struct _FILTEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _FILTEN6W<'a> {
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
#[doc = "Values that can be written to the field `SENSE7`"]
pub enum SENSE7W {
    #[doc = "No detection"]
    NONE,
    #[doc = "Rising edge detection"]
    RISE,
    #[doc = "Falling edge detection"]
    FALL,
    #[doc = "Both edges detection"]
    BOTH,
    #[doc = "High level detection"]
    HIGH,
    #[doc = "Low level detection"]
    LOW,
}
impl SENSE7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SENSE7W::NONE => 0,
            SENSE7W::RISE => 1,
            SENSE7W::FALL => 2,
            SENSE7W::BOTH => 3,
            SENSE7W::HIGH => 4,
            SENSE7W::LOW => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SENSE7W<'a> {
    w: &'a mut W,
}
impl<'a> _SENSE7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SENSE7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No detection"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE7W::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE7W::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE7W::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE7W::BOTH)
    }
    #[doc = "High level detection"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE7W::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE7W::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _FILTEN7W<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:2 - Input Sense 0 Configuration"]
    #[inline]
    pub fn sense0(&self) -> SENSE0R {
        SENSE0R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Filter 0 Enable"]
    #[inline]
    pub fn filten0(&self) -> FILTEN0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTEN0R { bits }
    }
    #[doc = "Bits 4:6 - Input Sense 1 Configuration"]
    #[inline]
    pub fn sense1(&self) -> SENSE1R {
        SENSE1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Filter 1 Enable"]
    #[inline]
    pub fn filten1(&self) -> FILTEN1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTEN1R { bits }
    }
    #[doc = "Bits 8:10 - Input Sense 2 Configuration"]
    #[inline]
    pub fn sense2(&self) -> SENSE2R {
        SENSE2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Filter 2 Enable"]
    #[inline]
    pub fn filten2(&self) -> FILTEN2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTEN2R { bits }
    }
    #[doc = "Bits 12:14 - Input Sense 3 Configuration"]
    #[inline]
    pub fn sense3(&self) -> SENSE3R {
        SENSE3R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Filter 3 Enable"]
    #[inline]
    pub fn filten3(&self) -> FILTEN3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTEN3R { bits }
    }
    #[doc = "Bits 16:18 - Input Sense 4 Configuration"]
    #[inline]
    pub fn sense4(&self) -> SENSE4R {
        SENSE4R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Filter 4 Enable"]
    #[inline]
    pub fn filten4(&self) -> FILTEN4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTEN4R { bits }
    }
    #[doc = "Bits 20:22 - Input Sense 5 Configuration"]
    #[inline]
    pub fn sense5(&self) -> SENSE5R {
        SENSE5R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - Filter 5 Enable"]
    #[inline]
    pub fn filten5(&self) -> FILTEN5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTEN5R { bits }
    }
    #[doc = "Bits 24:26 - Input Sense 6 Configuration"]
    #[inline]
    pub fn sense6(&self) -> SENSE6R {
        SENSE6R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - Filter 6 Enable"]
    #[inline]
    pub fn filten6(&self) -> FILTEN6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTEN6R { bits }
    }
    #[doc = "Bits 28:30 - Input Sense 7 Configuration"]
    #[inline]
    pub fn sense7(&self) -> SENSE7R {
        SENSE7R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Filter 7 Enable"]
    #[inline]
    pub fn filten7(&self) -> FILTEN7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTEN7R { bits }
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
    #[doc = "Bits 0:2 - Input Sense 0 Configuration"]
    #[inline]
    pub fn sense0(&mut self) -> _SENSE0W {
        _SENSE0W { w: self }
    }
    #[doc = "Bit 3 - Filter 0 Enable"]
    #[inline]
    pub fn filten0(&mut self) -> _FILTEN0W {
        _FILTEN0W { w: self }
    }
    #[doc = "Bits 4:6 - Input Sense 1 Configuration"]
    #[inline]
    pub fn sense1(&mut self) -> _SENSE1W {
        _SENSE1W { w: self }
    }
    #[doc = "Bit 7 - Filter 1 Enable"]
    #[inline]
    pub fn filten1(&mut self) -> _FILTEN1W {
        _FILTEN1W { w: self }
    }
    #[doc = "Bits 8:10 - Input Sense 2 Configuration"]
    #[inline]
    pub fn sense2(&mut self) -> _SENSE2W {
        _SENSE2W { w: self }
    }
    #[doc = "Bit 11 - Filter 2 Enable"]
    #[inline]
    pub fn filten2(&mut self) -> _FILTEN2W {
        _FILTEN2W { w: self }
    }
    #[doc = "Bits 12:14 - Input Sense 3 Configuration"]
    #[inline]
    pub fn sense3(&mut self) -> _SENSE3W {
        _SENSE3W { w: self }
    }
    #[doc = "Bit 15 - Filter 3 Enable"]
    #[inline]
    pub fn filten3(&mut self) -> _FILTEN3W {
        _FILTEN3W { w: self }
    }
    #[doc = "Bits 16:18 - Input Sense 4 Configuration"]
    #[inline]
    pub fn sense4(&mut self) -> _SENSE4W {
        _SENSE4W { w: self }
    }
    #[doc = "Bit 19 - Filter 4 Enable"]
    #[inline]
    pub fn filten4(&mut self) -> _FILTEN4W {
        _FILTEN4W { w: self }
    }
    #[doc = "Bits 20:22 - Input Sense 5 Configuration"]
    #[inline]
    pub fn sense5(&mut self) -> _SENSE5W {
        _SENSE5W { w: self }
    }
    #[doc = "Bit 23 - Filter 5 Enable"]
    #[inline]
    pub fn filten5(&mut self) -> _FILTEN5W {
        _FILTEN5W { w: self }
    }
    #[doc = "Bits 24:26 - Input Sense 6 Configuration"]
    #[inline]
    pub fn sense6(&mut self) -> _SENSE6W {
        _SENSE6W { w: self }
    }
    #[doc = "Bit 27 - Filter 6 Enable"]
    #[inline]
    pub fn filten6(&mut self) -> _FILTEN6W {
        _FILTEN6W { w: self }
    }
    #[doc = "Bits 28:30 - Input Sense 7 Configuration"]
    #[inline]
    pub fn sense7(&mut self) -> _SENSE7W {
        _SENSE7W { w: self }
    }
    #[doc = "Bit 31 - Filter 7 Enable"]
    #[inline]
    pub fn filten7(&mut self) -> _FILTEN7W {
        _FILTEN7W { w: self }
    }
}
