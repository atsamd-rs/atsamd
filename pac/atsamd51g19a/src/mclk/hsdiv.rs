#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::HSDIV {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVR {
    #[doc = "Divide by 1"]
    DIV1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVR::DIV1 => 1,
            DIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVR {
        match value {
            1 => DIVR::DIV1,
            i => DIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == DIVR::DIV1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:7 - CPU Clock Division Factor"]
    #[inline]
    pub fn div(&self) -> DIVR {
        DIVR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
