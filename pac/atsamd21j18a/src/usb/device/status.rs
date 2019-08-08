#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEEDR {
    #[doc = "Full-speed mode"]
    FS,
    #[doc = "High-speed mode"]
    HS,
    #[doc = "Low-speed mode"]
    LS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPEEDR::FS => 0,
            SPEEDR::HS => 1,
            SPEEDR::LS => 2,
            SPEEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPEEDR {
        match value {
            0 => SPEEDR::FS,
            1 => SPEEDR::HS,
            2 => SPEEDR::LS,
            i => SPEEDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline]
    pub fn is_fs(&self) -> bool {
        *self == SPEEDR::FS
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline]
    pub fn is_hs(&self) -> bool {
        *self == SPEEDR::HS
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline]
    pub fn is_ls(&self) -> bool {
        *self == SPEEDR::LS
    }
}
#[doc = "Possible values of the field `LINESTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINESTATER {
    #[doc = "SE0/RESET"]
    _0,
    #[doc = "FS-J or LS-K State"]
    _1,
    #[doc = "FS-K or LS-J State"]
    _2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LINESTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LINESTATER::_0 => 0,
            LINESTATER::_1 => 1,
            LINESTATER::_2 => 2,
            LINESTATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LINESTATER {
        match value {
            0 => LINESTATER::_0,
            1 => LINESTATER::_1,
            2 => LINESTATER::_2,
            i => LINESTATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LINESTATER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LINESTATER::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == LINESTATER::_2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 2:3 - Speed Status"]
    #[inline]
    pub fn speed(&self) -> SPEEDR {
        SPEEDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 6:7 - USB Line State Status"]
    #[inline]
    pub fn linestate(&self) -> LINESTATER {
        LINESTATER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
