#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PARAM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NVMPR {
    bits: u16,
}
impl NVMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `PSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSZR {
    #[doc = "8 bytes"]
    _8,
    #[doc = "16 bytes"]
    _16,
    #[doc = "32 bytes"]
    _32,
    #[doc = "64 bytes"]
    _64,
    #[doc = "128 bytes"]
    _128,
    #[doc = "256 bytes"]
    _256,
    #[doc = "512 bytes"]
    _512,
    #[doc = "1024 bytes"]
    _1024,
}
impl PSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSZR::_8 => 0,
            PSZR::_16 => 0x01,
            PSZR::_32 => 0x02,
            PSZR::_64 => 0x03,
            PSZR::_128 => 0x04,
            PSZR::_256 => 0x05,
            PSZR::_512 => 0x06,
            PSZR::_1024 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSZR {
        match value {
            0 => PSZR::_8,
            1 => PSZR::_16,
            2 => PSZR::_32,
            3 => PSZR::_64,
            4 => PSZR::_128,
            5 => PSZR::_256,
            6 => PSZR::_512,
            7 => PSZR::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == PSZR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == PSZR::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == PSZR::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == PSZR::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == PSZR::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == PSZR::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == PSZR::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == PSZR::_1024
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - NVM Pages"]
    #[inline]
    pub fn nvmp(&self) -> NVMPR {
        let bits = ((self.bits >> 0) & 0xffff) as u16;
        NVMPR { bits }
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline]
    pub fn psz(&self) -> PSZR {
        PSZR::_from(((self.bits >> 16) & 0x07) as u8)
    }
}
