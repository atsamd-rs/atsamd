#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `CMDINHC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDINHCR {
    #[doc = "Can issue command using only CMD line"]
    CAN,
    #[doc = "Cannot issue command"]
    CANNOT,
}
impl CMDINHCR {
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
            CMDINHCR::CAN => false,
            CMDINHCR::CANNOT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDINHCR {
        match value {
            false => CMDINHCR::CAN,
            true => CMDINHCR::CANNOT,
        }
    }
    #[doc = "Checks if the value of the field is `CAN`"]
    #[inline]
    pub fn is_can(&self) -> bool {
        *self == CMDINHCR::CAN
    }
    #[doc = "Checks if the value of the field is `CANNOT`"]
    #[inline]
    pub fn is_cannot(&self) -> bool {
        *self == CMDINHCR::CANNOT
    }
}
#[doc = "Possible values of the field `CMDINHD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDINHDR {
    #[doc = "Can issue command which uses the DAT line"]
    CAN,
    #[doc = "Cannot issue command which uses the DAT line"]
    CANNOT,
}
impl CMDINHDR {
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
            CMDINHDR::CAN => false,
            CMDINHDR::CANNOT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDINHDR {
        match value {
            false => CMDINHDR::CAN,
            true => CMDINHDR::CANNOT,
        }
    }
    #[doc = "Checks if the value of the field is `CAN`"]
    #[inline]
    pub fn is_can(&self) -> bool {
        *self == CMDINHDR::CAN
    }
    #[doc = "Checks if the value of the field is `CANNOT`"]
    #[inline]
    pub fn is_cannot(&self) -> bool {
        *self == CMDINHDR::CANNOT
    }
}
#[doc = "Possible values of the field `DLACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLACTR {
    #[doc = "DAT Line Inactive"]
    INACTIVE,
    #[doc = "DAT Line Active"]
    ACTIVE,
}
impl DLACTR {
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
            DLACTR::INACTIVE => false,
            DLACTR::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DLACTR {
        match value {
            false => DLACTR::INACTIVE,
            true => DLACTR::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == DLACTR::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == DLACTR::ACTIVE
    }
}
#[doc = "Possible values of the field `RTREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTREQR {
    #[doc = "Fixed or well-tuned sampling clock"]
    OK,
    #[doc = "Sampling clock needs re-tuning"]
    REQUIRED,
}
impl RTREQR {
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
            RTREQR::OK => false,
            RTREQR::REQUIRED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTREQR {
        match value {
            false => RTREQR::OK,
            true => RTREQR::REQUIRED,
        }
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline]
    pub fn is_ok(&self) -> bool {
        *self == RTREQR::OK
    }
    #[doc = "Checks if the value of the field is `REQUIRED`"]
    #[inline]
    pub fn is_required(&self) -> bool {
        *self == RTREQR::REQUIRED
    }
}
#[doc = "Possible values of the field `WTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTACTR {
    #[doc = "No valid data"]
    NO,
    #[doc = "Transferring data"]
    YES,
}
impl WTACTR {
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
            WTACTR::NO => false,
            WTACTR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WTACTR {
        match value {
            false => WTACTR::NO,
            true => WTACTR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == WTACTR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == WTACTR::YES
    }
}
#[doc = "Possible values of the field `RTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTACTR {
    #[doc = "No valid data"]
    NO,
    #[doc = "Transferring data"]
    YES,
}
impl RTACTR {
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
            RTACTR::NO => false,
            RTACTR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTACTR {
        match value {
            false => RTACTR::NO,
            true => RTACTR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == RTACTR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == RTACTR::YES
    }
}
#[doc = "Possible values of the field `BUFWREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFWRENR {
    #[doc = "Write disable"]
    DISABLE,
    #[doc = "Write enable"]
    ENABLE,
}
impl BUFWRENR {
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
            BUFWRENR::DISABLE => false,
            BUFWRENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFWRENR {
        match value {
            false => BUFWRENR::DISABLE,
            true => BUFWRENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BUFWRENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BUFWRENR::ENABLE
    }
}
#[doc = "Possible values of the field `BUFRDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFRDENR {
    #[doc = "Read disable"]
    DISABLE,
    #[doc = "Read enable"]
    ENABLE,
}
impl BUFRDENR {
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
            BUFRDENR::DISABLE => false,
            BUFRDENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFRDENR {
        match value {
            false => BUFRDENR::DISABLE,
            true => BUFRDENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BUFRDENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BUFRDENR::ENABLE
    }
}
#[doc = "Possible values of the field `CARDINS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARDINSR {
    #[doc = "Reset or Debouncing or No Card"]
    NO,
    #[doc = "Card inserted"]
    YES,
}
impl CARDINSR {
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
            CARDINSR::NO => false,
            CARDINSR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARDINSR {
        match value {
            false => CARDINSR::NO,
            true => CARDINSR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CARDINSR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == CARDINSR::YES
    }
}
#[doc = "Possible values of the field `CARDSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARDSSR {
    #[doc = "Reset or Debouncing"]
    NO,
    #[doc = "No Card or Insered"]
    YES,
}
impl CARDSSR {
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
            CARDSSR::NO => false,
            CARDSSR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARDSSR {
        match value {
            false => CARDSSR::NO,
            true => CARDSSR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CARDSSR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == CARDSSR::YES
    }
}
#[doc = "Possible values of the field `CARDDPL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARDDPLR {
    #[doc = "No card present (SDCD#=1)"]
    NO,
    #[doc = "Card present (SDCD#=0)"]
    YES,
}
impl CARDDPLR {
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
            CARDDPLR::NO => false,
            CARDDPLR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARDDPLR {
        match value {
            false => CARDDPLR::NO,
            true => CARDDPLR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CARDDPLR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == CARDDPLR::YES
    }
}
#[doc = "Possible values of the field `WRPPL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPPLR {
    #[doc = "Write protected (SDWP#=0)"]
    PROTECTED,
    #[doc = "Write enabled (SDWP#=1)"]
    ENABLED,
}
impl WRPPLR {
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
            WRPPLR::PROTECTED => false,
            WRPPLR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRPPLR {
        match value {
            false => WRPPLR::PROTECTED,
            true => WRPPLR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline]
    pub fn is_protected(&self) -> bool {
        *self == WRPPLR::PROTECTED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WRPPLR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct DATLLR {
    bits: u8,
}
impl DATLLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMDLLR {
    bits: bool,
}
impl CMDLLR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline]
    pub fn cmdinhc(&self) -> CMDINHCR {
        CMDINHCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline]
    pub fn cmdinhd(&self) -> CMDINHDR {
        CMDINHDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - DAT Line Active"]
    #[inline]
    pub fn dlact(&self) -> DLACTR {
        DLACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Re-Tuning Request"]
    #[inline]
    pub fn rtreq(&self) -> RTREQR {
        RTREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline]
    pub fn wtact(&self) -> WTACTR {
        WTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline]
    pub fn rtact(&self) -> RTACTR {
        RTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline]
    pub fn bufwren(&self) -> BUFWRENR {
        BUFWRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline]
    pub fn bufrden(&self) -> BUFRDENR {
        BUFRDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline]
    pub fn cardins(&self) -> CARDINSR {
        CARDINSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Card State Stable"]
    #[inline]
    pub fn cardss(&self) -> CARDSSR {
        CARDSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline]
    pub fn carddpl(&self) -> CARDDPLR {
        CARDDPLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Write Protect Pin Level"]
    #[inline]
    pub fn wrppl(&self) -> WRPPLR {
        WRPPLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 20:23 - DAT[3:0] Line Level"]
    #[inline]
    pub fn datll(&self) -> DATLLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATLLR { bits }
    }
    #[doc = "Bit 24 - CMD Line Level"]
    #[inline]
    pub fn cmdll(&self) -> CMDLLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMDLLR { bits }
    }
}
