#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CA0R {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TEOCLKF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEOCLKFR {
    #[doc = "Get information via another method"]
    OTHER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TEOCLKFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TEOCLKFR::OTHER => 0,
            TEOCLKFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TEOCLKFR {
        match value {
            0 => TEOCLKFR::OTHER,
            i => TEOCLKFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline]
    pub fn is_other(&self) -> bool {
        *self == TEOCLKFR::OTHER
    }
}
#[doc = "Possible values of the field `TEOCLKU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEOCLKUR {
    #[doc = "kHz"]
    KHZ,
    #[doc = "MHz"]
    MHZ,
}
impl TEOCLKUR {
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
            TEOCLKUR::KHZ => false,
            TEOCLKUR::MHZ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEOCLKUR {
        match value {
            false => TEOCLKUR::KHZ,
            true => TEOCLKUR::MHZ,
        }
    }
    #[doc = "Checks if the value of the field is `KHZ`"]
    #[inline]
    pub fn is_khz(&self) -> bool {
        *self == TEOCLKUR::KHZ
    }
    #[doc = "Checks if the value of the field is `MHZ`"]
    #[inline]
    pub fn is_mhz(&self) -> bool {
        *self == TEOCLKUR::MHZ
    }
}
#[doc = "Possible values of the field `BASECLKF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BASECLKFR {
    #[doc = "Get information via another method"]
    OTHER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BASECLKFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BASECLKFR::OTHER => 0,
            BASECLKFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BASECLKFR {
        match value {
            0 => BASECLKFR::OTHER,
            i => BASECLKFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline]
    pub fn is_other(&self) -> bool {
        *self == BASECLKFR::OTHER
    }
}
#[doc = "Possible values of the field `MAXBLKL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXBLKLR {
    #[doc = "512 bytes"]
    _512,
    #[doc = "1024 bytes"]
    _1024,
    #[doc = "2048 bytes"]
    _2048,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MAXBLKLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAXBLKLR::_512 => 0,
            MAXBLKLR::_1024 => 1,
            MAXBLKLR::_2048 => 2,
            MAXBLKLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAXBLKLR {
        match value {
            0 => MAXBLKLR::_512,
            1 => MAXBLKLR::_1024,
            2 => MAXBLKLR::_2048,
            i => MAXBLKLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == MAXBLKLR::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == MAXBLKLR::_1024
    }
    #[doc = "Checks if the value of the field is `_2048`"]
    #[inline]
    pub fn is_2048(&self) -> bool {
        *self == MAXBLKLR::_2048
    }
}
#[doc = "Possible values of the field `ED8SUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ED8SUPR {
    #[doc = "8-bit Bus Width not Supported"]
    NO,
    #[doc = "8-bit Bus Width Supported"]
    YES,
}
impl ED8SUPR {
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
            ED8SUPR::NO => false,
            ED8SUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ED8SUPR {
        match value {
            false => ED8SUPR::NO,
            true => ED8SUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == ED8SUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == ED8SUPR::YES
    }
}
#[doc = "Possible values of the field `ADMA2SUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMA2SUPR {
    #[doc = "ADMA2 not Supported"]
    NO,
    #[doc = "ADMA2 Supported"]
    YES,
}
impl ADMA2SUPR {
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
            ADMA2SUPR::NO => false,
            ADMA2SUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADMA2SUPR {
        match value {
            false => ADMA2SUPR::NO,
            true => ADMA2SUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == ADMA2SUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == ADMA2SUPR::YES
    }
}
#[doc = "Possible values of the field `HSSUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSSUPR {
    #[doc = "High Speed not Supported"]
    NO,
    #[doc = "High Speed Supported"]
    YES,
}
impl HSSUPR {
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
            HSSUPR::NO => false,
            HSSUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSSUPR {
        match value {
            false => HSSUPR::NO,
            true => HSSUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == HSSUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == HSSUPR::YES
    }
}
#[doc = "Possible values of the field `SDMASUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMASUPR {
    #[doc = "SDMA not Supported"]
    NO,
    #[doc = "SDMA Supported"]
    YES,
}
impl SDMASUPR {
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
            SDMASUPR::NO => false,
            SDMASUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDMASUPR {
        match value {
            false => SDMASUPR::NO,
            true => SDMASUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == SDMASUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == SDMASUPR::YES
    }
}
#[doc = "Possible values of the field `SRSUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRSUPR {
    #[doc = "Suspend/Resume not Supported"]
    NO,
    #[doc = "Suspend/Resume Supported"]
    YES,
}
impl SRSUPR {
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
            SRSUPR::NO => false,
            SRSUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRSUPR {
        match value {
            false => SRSUPR::NO,
            true => SRSUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == SRSUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == SRSUPR::YES
    }
}
#[doc = "Possible values of the field `V33VSUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V33VSUPR {
    #[doc = "3.3V Not Supported"]
    NO,
    #[doc = "3.3V Supported"]
    YES,
}
impl V33VSUPR {
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
            V33VSUPR::NO => false,
            V33VSUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> V33VSUPR {
        match value {
            false => V33VSUPR::NO,
            true => V33VSUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == V33VSUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == V33VSUPR::YES
    }
}
#[doc = "Possible values of the field `V30VSUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V30VSUPR {
    #[doc = "3.0V Not Supported"]
    NO,
    #[doc = "3.0V Supported"]
    YES,
}
impl V30VSUPR {
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
            V30VSUPR::NO => false,
            V30VSUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> V30VSUPR {
        match value {
            false => V30VSUPR::NO,
            true => V30VSUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == V30VSUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == V30VSUPR::YES
    }
}
#[doc = "Possible values of the field `V18VSUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V18VSUPR {
    #[doc = "1.8V Not Supported"]
    NO,
    #[doc = "1.8V Supported"]
    YES,
}
impl V18VSUPR {
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
            V18VSUPR::NO => false,
            V18VSUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> V18VSUPR {
        match value {
            false => V18VSUPR::NO,
            true => V18VSUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == V18VSUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == V18VSUPR::YES
    }
}
#[doc = "Possible values of the field `SB64SUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SB64SUPR {
    #[doc = "32-bit Address Descriptors and System Bus"]
    NO,
    #[doc = "64-bit Address Descriptors and System Bus"]
    YES,
}
impl SB64SUPR {
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
            SB64SUPR::NO => false,
            SB64SUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SB64SUPR {
        match value {
            false => SB64SUPR::NO,
            true => SB64SUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == SB64SUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == SB64SUPR::YES
    }
}
#[doc = "Possible values of the field `ASINTSUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASINTSUPR {
    #[doc = "Asynchronous Interrupt not Supported"]
    NO,
    #[doc = "Asynchronous Interrupt supported"]
    YES,
}
impl ASINTSUPR {
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
            ASINTSUPR::NO => false,
            ASINTSUPR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASINTSUPR {
        match value {
            false => ASINTSUPR::NO,
            true => ASINTSUPR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == ASINTSUPR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == ASINTSUPR::YES
    }
}
#[doc = "Possible values of the field `SLTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLTYPER {
    #[doc = "Removable Card Slot"]
    REMOVABLE,
    #[doc = "Embedded Slot for One Device"]
    EMBEDDED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLTYPER::REMOVABLE => 0,
            SLTYPER::EMBEDDED => 1,
            SLTYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLTYPER {
        match value {
            0 => SLTYPER::REMOVABLE,
            1 => SLTYPER::EMBEDDED,
            i => SLTYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REMOVABLE`"]
    #[inline]
    pub fn is_removable(&self) -> bool {
        *self == SLTYPER::REMOVABLE
    }
    #[doc = "Checks if the value of the field is `EMBEDDED`"]
    #[inline]
    pub fn is_embedded(&self) -> bool {
        *self == SLTYPER::EMBEDDED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Timeout Clock Frequency"]
    #[inline]
    pub fn teoclkf(&self) -> TEOCLKFR {
        TEOCLKFR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Timeout Clock Unit"]
    #[inline]
    pub fn teoclku(&self) -> TEOCLKUR {
        TEOCLKUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:15 - Base Clock Frequency"]
    #[inline]
    pub fn baseclkf(&self) -> BASECLKFR {
        BASECLKFR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Max Block Length"]
    #[inline]
    pub fn maxblkl(&self) -> MAXBLKLR {
        MAXBLKLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - 8-bit Support for Embedded Device"]
    #[inline]
    pub fn ed8sup(&self) -> ED8SUPR {
        ED8SUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline]
    pub fn adma2sup(&self) -> ADMA2SUPR {
        ADMA2SUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline]
    pub fn hssup(&self) -> HSSUPR {
        HSSUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - SDMA Support"]
    #[inline]
    pub fn sdmasup(&self) -> SDMASUPR {
        SDMASUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Suspend/Resume Support"]
    #[inline]
    pub fn srsup(&self) -> SRSUPR {
        SRSUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline]
    pub fn v33vsup(&self) -> V33VSUPR {
        V33VSUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Voltage Support 3.0V"]
    #[inline]
    pub fn v30vsup(&self) -> V30VSUPR {
        V30VSUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Voltage Support 1.8V"]
    #[inline]
    pub fn v18vsup(&self) -> V18VSUPR {
        V18VSUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - 64-Bit System Bus Support"]
    #[inline]
    pub fn sb64sup(&self) -> SB64SUPR {
        SB64SUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support"]
    #[inline]
    pub fn asintsup(&self) -> ASINTSUPR {
        ASINTSUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 30:31 - Slot Type"]
    #[inline]
    pub fn sltype(&self) -> SLTYPER {
        SLTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
