#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ECCERR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ADDRR {
    bits: u32,
}
impl ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `TYPEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPELR {
    #[doc = "No Error Detected Since Last Read"]
    NONE,
    #[doc = "At Least One Single Error Detected Since last Read"]
    SINGLE,
    #[doc = "At Least One Dual Error Detected Since Last Read"]
    DUAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TYPELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TYPELR::NONE => 0,
            TYPELR::SINGLE => 1,
            TYPELR::DUAL => 2,
            TYPELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TYPELR {
        match value {
            0 => TYPELR::NONE,
            1 => TYPELR::SINGLE,
            2 => TYPELR::DUAL,
            i => TYPELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == TYPELR::NONE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == TYPELR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline]
    pub fn is_dual(&self) -> bool {
        *self == TYPELR::DUAL
    }
}
#[doc = "Possible values of the field `TYPEH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPEHR {
    #[doc = "No Error Detected Since Last Read"]
    NONE,
    #[doc = "At Least One Single Error Detected Since last Read"]
    SINGLE,
    #[doc = "At Least One Dual Error Detected Since Last Read"]
    DUAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TYPEHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TYPEHR::NONE => 0,
            TYPEHR::SINGLE => 1,
            TYPEHR::DUAL => 2,
            TYPEHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TYPEHR {
        match value {
            0 => TYPEHR::NONE,
            1 => TYPEHR::SINGLE,
            2 => TYPEHR::DUAL,
            i => TYPEHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == TYPEHR::NONE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == TYPEHR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline]
    pub fn is_dual(&self) -> bool {
        *self == TYPEHR::DUAL
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:23 - Error Address"]
    #[inline]
    pub fn addr(&self) -> ADDRR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ADDRR { bits }
    }
    #[doc = "Bits 28:29 - Low Double-Word Error Type"]
    #[inline]
    pub fn typel(&self) -> TYPELR {
        TYPELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - High Double-Word Error Type"]
    #[inline]
    pub fn typeh(&self) -> TYPEHR {
        TYPEHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
