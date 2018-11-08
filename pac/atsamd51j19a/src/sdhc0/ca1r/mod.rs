#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CA1R {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SDR50SUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR50SUPR {
    #[doc = "SDR50 is Not Supported"]
    NO,
    #[doc = "SDR50 is Supported"]
    YES,
}
impl SDR50SUPR {
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
            SDR50SUPR::NO => false,
            SDR50SUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDR50SUPR {
        match value {
            false => SDR50SUPR::NO,
            true => SDR50SUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == SDR50SUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == SDR50SUPR::YES
    }
}
#[doc = "Possible values of the field `SDR104SUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR104SUPR {
    #[doc = "SDR104 is Not Supported"]
    NO,
    #[doc = "SDR104 is Supported"]
    YES,
}
impl SDR104SUPR {
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
            SDR104SUPR::NO => false,
            SDR104SUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDR104SUPR {
        match value {
            false => SDR104SUPR::NO,
            true => SDR104SUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == SDR104SUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == SDR104SUPR::YES
    }
}
#[doc = "Possible values of the field `DDR50SUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDR50SUPR {
    #[doc = "DDR50 is Not Supported"]
    NO,
    #[doc = "DDR50 is Supported"]
    YES,
}
impl DDR50SUPR {
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
            DDR50SUPR::NO => false,
            DDR50SUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DDR50SUPR {
        match value {
            false => DDR50SUPR::NO,
            true => DDR50SUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == DDR50SUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == DDR50SUPR::YES
    }
}
#[doc = "Possible values of the field `DRVASUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRVASUPR {
    #[doc = "Driver Type A is Not Supported"]
    NO,
    #[doc = "Driver Type A is Supported"]
    YES,
}
impl DRVASUPR {
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
            DRVASUPR::NO => false,
            DRVASUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRVASUPR {
        match value {
            false => DRVASUPR::NO,
            true => DRVASUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == DRVASUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == DRVASUPR::YES
    }
}
#[doc = "Possible values of the field `DRVCSUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRVCSUPR {
    #[doc = "Driver Type C is Not Supported"]
    NO,
    #[doc = "Driver Type C is Supported"]
    YES,
}
impl DRVCSUPR {
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
            DRVCSUPR::NO => false,
            DRVCSUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRVCSUPR {
        match value {
            false => DRVCSUPR::NO,
            true => DRVCSUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == DRVCSUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == DRVCSUPR::YES
    }
}
#[doc = "Possible values of the field `DRVDSUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRVDSUPR {
    #[doc = "Driver Type D is Not Supported"]
    NO,
    #[doc = "Driver Type D is Supported"]
    YES,
}
impl DRVDSUPR {
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
            DRVDSUPR::NO => false,
            DRVDSUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRVDSUPR {
        match value {
            false => DRVDSUPR::NO,
            true => DRVDSUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == DRVDSUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == DRVDSUPR::YES
    }
}
#[doc = "Possible values of the field `TCNTRT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCNTRTR {
    #[doc = "Re-Tuning Timer disabled"]
    DISABLED,
    #[doc = "1 second"]
    _1S,
    #[doc = "2 seconds"]
    _2S,
    #[doc = "4 seconds"]
    _4S,
    #[doc = "8 seconds"]
    _8S,
    #[doc = "16 seconds"]
    _16S,
    #[doc = "32 seconds"]
    _32S,
    #[doc = "64 seconds"]
    _64S,
    #[doc = "128 seconds"]
    _128S,
    #[doc = "256 seconds"]
    _256S,
    #[doc = "512 seconds"]
    _512S,
    #[doc = "1024 seconds"]
    _1024S,
    #[doc = "Get information from other source"]
    OTHER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TCNTRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCNTRTR::DISABLED => 0,
            TCNTRTR::_1S => 1,
            TCNTRTR::_2S => 2,
            TCNTRTR::_4S => 3,
            TCNTRTR::_8S => 4,
            TCNTRTR::_16S => 5,
            TCNTRTR::_32S => 6,
            TCNTRTR::_64S => 7,
            TCNTRTR::_128S => 8,
            TCNTRTR::_256S => 9,
            TCNTRTR::_512S => 10,
            TCNTRTR::_1024S => 11,
            TCNTRTR::OTHER => 15,
            TCNTRTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCNTRTR {
        match value {
            0 => TCNTRTR::DISABLED,
            1 => TCNTRTR::_1S,
            2 => TCNTRTR::_2S,
            3 => TCNTRTR::_4S,
            4 => TCNTRTR::_8S,
            5 => TCNTRTR::_16S,
            6 => TCNTRTR::_32S,
            7 => TCNTRTR::_64S,
            8 => TCNTRTR::_128S,
            9 => TCNTRTR::_256S,
            10 => TCNTRTR::_512S,
            11 => TCNTRTR::_1024S,
            15 => TCNTRTR::OTHER,
            i => TCNTRTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TCNTRTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `_1S`"]
    #[inline]
    pub fn is_1s(&self) -> bool {
        *self == TCNTRTR::_1S
    }
    #[doc = "Checks if the value of the field is `_2S`"]
    #[inline]
    pub fn is_2s(&self) -> bool {
        *self == TCNTRTR::_2S
    }
    #[doc = "Checks if the value of the field is `_4S`"]
    #[inline]
    pub fn is_4s(&self) -> bool {
        *self == TCNTRTR::_4S
    }
    #[doc = "Checks if the value of the field is `_8S`"]
    #[inline]
    pub fn is_8s(&self) -> bool {
        *self == TCNTRTR::_8S
    }
    #[doc = "Checks if the value of the field is `_16S`"]
    #[inline]
    pub fn is_16s(&self) -> bool {
        *self == TCNTRTR::_16S
    }
    #[doc = "Checks if the value of the field is `_32S`"]
    #[inline]
    pub fn is_32s(&self) -> bool {
        *self == TCNTRTR::_32S
    }
    #[doc = "Checks if the value of the field is `_64S`"]
    #[inline]
    pub fn is_64s(&self) -> bool {
        *self == TCNTRTR::_64S
    }
    #[doc = "Checks if the value of the field is `_128S`"]
    #[inline]
    pub fn is_128s(&self) -> bool {
        *self == TCNTRTR::_128S
    }
    #[doc = "Checks if the value of the field is `_256S`"]
    #[inline]
    pub fn is_256s(&self) -> bool {
        *self == TCNTRTR::_256S
    }
    #[doc = "Checks if the value of the field is `_512S`"]
    #[inline]
    pub fn is_512s(&self) -> bool {
        *self == TCNTRTR::_512S
    }
    #[doc = "Checks if the value of the field is `_1024S`"]
    #[inline]
    pub fn is_1024s(&self) -> bool {
        *self == TCNTRTR::_1024S
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline]
    pub fn is_other(&self) -> bool {
        *self == TCNTRTR::OTHER
    }
}
#[doc = "Possible values of the field `TSDR50`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSDR50R {
    #[doc = "SDR50 does not require tuning"]
    NO,
    #[doc = "SDR50 requires tuning"]
    YES,
}
impl TSDR50R {
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
            TSDR50R::NO => false,
            TSDR50R::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSDR50R {
        match value {
            false => TSDR50R::NO,
            true => TSDR50R::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == TSDR50R::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == TSDR50R::YES
    }
}
#[doc = "Possible values of the field `CLKMULT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKMULTR {
    #[doc = "Clock Multiplier is Not Supported"]
    NO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKMULTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKMULTR::NO => 0,
            CLKMULTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKMULTR {
        match value {
            0 => CLKMULTR::NO,
            i => CLKMULTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CLKMULTR::NO
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SDR50 Support"]
    #[inline]
    pub fn sdr50sup(&self) -> SDR50SUPR {
        SDR50SUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - SDR104 Support"]
    #[inline]
    pub fn sdr104sup(&self) -> SDR104SUPR {
        SDR104SUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - DDR50 Support"]
    #[inline]
    pub fn ddr50sup(&self) -> DDR50SUPR {
        DDR50SUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Driver Type A Support"]
    #[inline]
    pub fn drvasup(&self) -> DRVASUPR {
        DRVASUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Driver Type C Support"]
    #[inline]
    pub fn drvcsup(&self) -> DRVCSUPR {
        DRVCSUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Driver Type D Support"]
    #[inline]
    pub fn drvdsup(&self) -> DRVDSUPR {
        DRVDSUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Timer Count for Re-Tuning"]
    #[inline]
    pub fn tcntrt(&self) -> TCNTRTR {
        TCNTRTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline]
    pub fn tsdr50(&self) -> TSDR50R {
        TSDR50R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline]
    pub fn clkmult(&self) -> CLKMULTR {
        CLKMULTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
