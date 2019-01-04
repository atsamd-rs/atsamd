#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MCCAR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `MAXCUR33V`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXCUR33VR {
    #[doc = "Get information via another method"]
    OTHER,
    #[doc = "4mA"]
    _4MA,
    #[doc = "8mA"]
    _8MA,
    #[doc = "12mA"]
    _12MA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MAXCUR33VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAXCUR33VR::OTHER => 0,
            MAXCUR33VR::_4MA => 1,
            MAXCUR33VR::_8MA => 2,
            MAXCUR33VR::_12MA => 3,
            MAXCUR33VR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAXCUR33VR {
        match value {
            0 => MAXCUR33VR::OTHER,
            1 => MAXCUR33VR::_4MA,
            2 => MAXCUR33VR::_8MA,
            3 => MAXCUR33VR::_12MA,
            i => MAXCUR33VR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline]
    pub fn is_other(&self) -> bool {
        *self == MAXCUR33VR::OTHER
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline]
    pub fn is_4ma(&self) -> bool {
        *self == MAXCUR33VR::_4MA
    }
    #[doc = "Checks if the value of the field is `_8MA`"]
    #[inline]
    pub fn is_8ma(&self) -> bool {
        *self == MAXCUR33VR::_8MA
    }
    #[doc = "Checks if the value of the field is `_12MA`"]
    #[inline]
    pub fn is_12ma(&self) -> bool {
        *self == MAXCUR33VR::_12MA
    }
}
#[doc = "Possible values of the field `MAXCUR30V`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXCUR30VR {
    #[doc = "Get information via another method"]
    OTHER,
    #[doc = "4mA"]
    _4MA,
    #[doc = "8mA"]
    _8MA,
    #[doc = "12mA"]
    _12MA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MAXCUR30VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAXCUR30VR::OTHER => 0,
            MAXCUR30VR::_4MA => 1,
            MAXCUR30VR::_8MA => 2,
            MAXCUR30VR::_12MA => 3,
            MAXCUR30VR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAXCUR30VR {
        match value {
            0 => MAXCUR30VR::OTHER,
            1 => MAXCUR30VR::_4MA,
            2 => MAXCUR30VR::_8MA,
            3 => MAXCUR30VR::_12MA,
            i => MAXCUR30VR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline]
    pub fn is_other(&self) -> bool {
        *self == MAXCUR30VR::OTHER
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline]
    pub fn is_4ma(&self) -> bool {
        *self == MAXCUR30VR::_4MA
    }
    #[doc = "Checks if the value of the field is `_8MA`"]
    #[inline]
    pub fn is_8ma(&self) -> bool {
        *self == MAXCUR30VR::_8MA
    }
    #[doc = "Checks if the value of the field is `_12MA`"]
    #[inline]
    pub fn is_12ma(&self) -> bool {
        *self == MAXCUR30VR::_12MA
    }
}
#[doc = "Possible values of the field `MAXCUR18V`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXCUR18VR {
    #[doc = "Get information via another method"]
    OTHER,
    #[doc = "4mA"]
    _4MA,
    #[doc = "8mA"]
    _8MA,
    #[doc = "12mA"]
    _12MA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MAXCUR18VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAXCUR18VR::OTHER => 0,
            MAXCUR18VR::_4MA => 1,
            MAXCUR18VR::_8MA => 2,
            MAXCUR18VR::_12MA => 3,
            MAXCUR18VR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAXCUR18VR {
        match value {
            0 => MAXCUR18VR::OTHER,
            1 => MAXCUR18VR::_4MA,
            2 => MAXCUR18VR::_8MA,
            3 => MAXCUR18VR::_12MA,
            i => MAXCUR18VR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline]
    pub fn is_other(&self) -> bool {
        *self == MAXCUR18VR::OTHER
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline]
    pub fn is_4ma(&self) -> bool {
        *self == MAXCUR18VR::_4MA
    }
    #[doc = "Checks if the value of the field is `_8MA`"]
    #[inline]
    pub fn is_8ma(&self) -> bool {
        *self == MAXCUR18VR::_8MA
    }
    #[doc = "Checks if the value of the field is `_12MA`"]
    #[inline]
    pub fn is_12ma(&self) -> bool {
        *self == MAXCUR18VR::_12MA
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline]
    pub fn maxcur33v(&self) -> MAXCUR33VR {
        MAXCUR33VR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V"]
    #[inline]
    pub fn maxcur30v(&self) -> MAXCUR30VR {
        MAXCUR30VR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V"]
    #[inline]
    pub fn maxcur18v(&self) -> MAXCUR18VR {
        MAXCUR18VR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
