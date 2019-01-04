#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::ACESR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `ACMD12NE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD12NER {
    #[doc = "Executed"]
    EXEC,
    #[doc = "Not executed"]
    NOT_EXEC,
}
impl ACMD12NER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ACMD12NER::EXEC => false,
            ACMD12NER::NOT_EXEC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMD12NER {
        match value {
            false => ACMD12NER::EXEC,
            true => ACMD12NER::NOT_EXEC,
        }
    }
    #[doc = "Checks if the value of the field is `EXEC`"]
    #[inline]
    pub fn is_exec(&self) -> bool {
        *self == ACMD12NER::EXEC
    }
    #[doc = "Checks if the value of the field is `NOT_EXEC`"]
    #[inline]
    pub fn is_not_exec(&self) -> bool {
        *self == ACMD12NER::NOT_EXEC
    }
}
#[doc = "Possible values of the field `ACMDTEO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDTEOR {
    #[doc = "No error"]
    NO,
    #[doc = "Timeout"]
    YES,
}
impl ACMDTEOR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ACMDTEOR::NO => false,
            ACMDTEOR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMDTEOR {
        match value {
            false => ACMDTEOR::NO,
            true => ACMDTEOR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == ACMDTEOR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == ACMDTEOR::YES
    }
}
#[doc = "Possible values of the field `ACMDCRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDCRCR {
    #[doc = "No error"]
    NO,
    #[doc = "CRC Error Generated"]
    YES,
}
impl ACMDCRCR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ACMDCRCR::NO => false,
            ACMDCRCR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMDCRCR {
        match value {
            false => ACMDCRCR::NO,
            true => ACMDCRCR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == ACMDCRCR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == ACMDCRCR::YES
    }
}
#[doc = "Possible values of the field `ACMDEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDENDR {
    #[doc = "No error"]
    NO,
    #[doc = "End Bit Error Generated"]
    YES,
}
impl ACMDENDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ACMDENDR::NO => false,
            ACMDENDR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMDENDR {
        match value {
            false => ACMDENDR::NO,
            true => ACMDENDR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == ACMDENDR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == ACMDENDR::YES
    }
}
#[doc = "Possible values of the field `ACMDIDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDIDXR {
    #[doc = "No error"]
    NO,
    #[doc = "Error"]
    YES,
}
impl ACMDIDXR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ACMDIDXR::NO => false,
            ACMDIDXR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMDIDXR {
        match value {
            false => ACMDIDXR::NO,
            true => ACMDIDXR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == ACMDIDXR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == ACMDIDXR::YES
    }
}
#[doc = "Possible values of the field `CMDNI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDNIR {
    #[doc = "No error"]
    OK,
    #[doc = "Not Issued"]
    NOT_ISSUED,
}
impl CMDNIR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CMDNIR::OK => false,
            CMDNIR::NOT_ISSUED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDNIR {
        match value {
            false => CMDNIR::OK,
            true => CMDNIR::NOT_ISSUED,
        }
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline]
    pub fn is_ok(&self) -> bool {
        *self == CMDNIR::OK
    }
    #[doc = "Checks if the value of the field is `NOT_ISSUED`"]
    #[inline]
    pub fn is_not_issued(&self) -> bool {
        *self == CMDNIR::NOT_ISSUED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline]
    pub fn acmd12ne(&self) -> ACMD12NER {
        ACMD12NER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error"]
    #[inline]
    pub fn acmdteo(&self) -> ACMDTEOR {
        ACMDTEOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Auto CMD CRC Error"]
    #[inline]
    pub fn acmdcrc(&self) -> ACMDCRCR {
        ACMDCRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error"]
    #[inline]
    pub fn acmdend(&self) -> ACMDENDR {
        ACMDENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Auto CMD Index Error"]
    #[inline]
    pub fn acmdidx(&self) -> ACMDIDXR {
        ACMDIDXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Command not Issued By Auto CMD12 Error"]
    #[inline]
    pub fn cmdni(&self) -> CMDNIR {
        CMDNIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
