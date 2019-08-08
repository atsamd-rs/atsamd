#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::STATUSC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct STATE0R {
    bits: bool,
}
impl STATE0R {
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
pub struct STATE1R {
    bits: bool,
}
impl STATE1R {
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
#[doc = "Possible values of the field `WSTATE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSTATE0R {
    #[doc = "Signal is above window"]
    ABOVE,
    #[doc = "Signal is inside window"]
    INSIDE,
    #[doc = "Signal is below window"]
    BELOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WSTATE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WSTATE0R::ABOVE => 0,
            WSTATE0R::INSIDE => 1,
            WSTATE0R::BELOW => 2,
            WSTATE0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WSTATE0R {
        match value {
            0 => WSTATE0R::ABOVE,
            1 => WSTATE0R::INSIDE,
            2 => WSTATE0R::BELOW,
            i => WSTATE0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline]
    pub fn is_above(&self) -> bool {
        *self == WSTATE0R::ABOVE
    }
    #[doc = "Checks if the value of the field is `INSIDE`"]
    #[inline]
    pub fn is_inside(&self) -> bool {
        *self == WSTATE0R::INSIDE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline]
    pub fn is_below(&self) -> bool {
        *self == WSTATE0R::BELOW
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Comparator 0 Current State"]
    #[inline]
    pub fn state0(&self) -> STATE0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        STATE0R { bits }
    }
    #[doc = "Bit 1 - Comparator 1 Current State"]
    #[inline]
    pub fn state1(&self) -> STATE1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        STATE1R { bits }
    }
    #[doc = "Bits 4:5 - Window 0 Current State"]
    #[inline]
    pub fn wstate0(&self) -> WSTATE0R {
        WSTATE0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
