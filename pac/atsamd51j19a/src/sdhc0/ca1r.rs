#[doc = "Reader of register CA1R"]
pub type R = crate::R<u32, super::CA1R>;
#[doc = "SDR50 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR50SUP_A {
    #[doc = "0: SDR50 is Not Supported"]
    NO = 0,
    #[doc = "1: SDR50 is Supported"]
    YES = 1,
}
impl From<SDR50SUP_A> for bool {
    #[inline(always)]
    fn from(variant: SDR50SUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDR50SUP`"]
pub type SDR50SUP_R = crate::R<bool, SDR50SUP_A>;
impl SDR50SUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDR50SUP_A {
        match self.bits {
            false => SDR50SUP_A::NO,
            true => SDR50SUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SDR50SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SDR50SUP_A::YES
    }
}
#[doc = "SDR104 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR104SUP_A {
    #[doc = "0: SDR104 is Not Supported"]
    NO = 0,
    #[doc = "1: SDR104 is Supported"]
    YES = 1,
}
impl From<SDR104SUP_A> for bool {
    #[inline(always)]
    fn from(variant: SDR104SUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDR104SUP`"]
pub type SDR104SUP_R = crate::R<bool, SDR104SUP_A>;
impl SDR104SUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDR104SUP_A {
        match self.bits {
            false => SDR104SUP_A::NO,
            true => SDR104SUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SDR104SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SDR104SUP_A::YES
    }
}
#[doc = "DDR50 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDR50SUP_A {
    #[doc = "0: DDR50 is Not Supported"]
    NO = 0,
    #[doc = "1: DDR50 is Supported"]
    YES = 1,
}
impl From<DDR50SUP_A> for bool {
    #[inline(always)]
    fn from(variant: DDR50SUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDR50SUP`"]
pub type DDR50SUP_R = crate::R<bool, DDR50SUP_A>;
impl DDR50SUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDR50SUP_A {
        match self.bits {
            false => DDR50SUP_A::NO,
            true => DDR50SUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == DDR50SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == DDR50SUP_A::YES
    }
}
#[doc = "Driver Type A Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRVASUP_A {
    #[doc = "0: Driver Type A is Not Supported"]
    NO = 0,
    #[doc = "1: Driver Type A is Supported"]
    YES = 1,
}
impl From<DRVASUP_A> for bool {
    #[inline(always)]
    fn from(variant: DRVASUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRVASUP`"]
pub type DRVASUP_R = crate::R<bool, DRVASUP_A>;
impl DRVASUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRVASUP_A {
        match self.bits {
            false => DRVASUP_A::NO,
            true => DRVASUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == DRVASUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == DRVASUP_A::YES
    }
}
#[doc = "Driver Type C Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRVCSUP_A {
    #[doc = "0: Driver Type C is Not Supported"]
    NO = 0,
    #[doc = "1: Driver Type C is Supported"]
    YES = 1,
}
impl From<DRVCSUP_A> for bool {
    #[inline(always)]
    fn from(variant: DRVCSUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRVCSUP`"]
pub type DRVCSUP_R = crate::R<bool, DRVCSUP_A>;
impl DRVCSUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRVCSUP_A {
        match self.bits {
            false => DRVCSUP_A::NO,
            true => DRVCSUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == DRVCSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == DRVCSUP_A::YES
    }
}
#[doc = "Driver Type D Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRVDSUP_A {
    #[doc = "0: Driver Type D is Not Supported"]
    NO = 0,
    #[doc = "1: Driver Type D is Supported"]
    YES = 1,
}
impl From<DRVDSUP_A> for bool {
    #[inline(always)]
    fn from(variant: DRVDSUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRVDSUP`"]
pub type DRVDSUP_R = crate::R<bool, DRVDSUP_A>;
impl DRVDSUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRVDSUP_A {
        match self.bits {
            false => DRVDSUP_A::NO,
            true => DRVDSUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == DRVDSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == DRVDSUP_A::YES
    }
}
#[doc = "Timer Count for Re-Tuning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCNTRT_A {
    #[doc = "0: Re-Tuning Timer disabled"]
    DISABLED = 0,
    #[doc = "1: 1 second"]
    _1S = 1,
    #[doc = "2: 2 seconds"]
    _2S = 2,
    #[doc = "3: 4 seconds"]
    _4S = 3,
    #[doc = "4: 8 seconds"]
    _8S = 4,
    #[doc = "5: 16 seconds"]
    _16S = 5,
    #[doc = "6: 32 seconds"]
    _32S = 6,
    #[doc = "7: 64 seconds"]
    _64S = 7,
    #[doc = "8: 128 seconds"]
    _128S = 8,
    #[doc = "9: 256 seconds"]
    _256S = 9,
    #[doc = "10: 512 seconds"]
    _512S = 10,
    #[doc = "11: 1024 seconds"]
    _1024S = 11,
    #[doc = "15: Get information from other source"]
    OTHER = 15,
}
impl From<TCNTRT_A> for u8 {
    #[inline(always)]
    fn from(variant: TCNTRT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TCNTRT`"]
pub type TCNTRT_R = crate::R<u8, TCNTRT_A>;
impl TCNTRT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TCNTRT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TCNTRT_A::DISABLED),
            1 => Val(TCNTRT_A::_1S),
            2 => Val(TCNTRT_A::_2S),
            3 => Val(TCNTRT_A::_4S),
            4 => Val(TCNTRT_A::_8S),
            5 => Val(TCNTRT_A::_16S),
            6 => Val(TCNTRT_A::_32S),
            7 => Val(TCNTRT_A::_64S),
            8 => Val(TCNTRT_A::_128S),
            9 => Val(TCNTRT_A::_256S),
            10 => Val(TCNTRT_A::_512S),
            11 => Val(TCNTRT_A::_1024S),
            15 => Val(TCNTRT_A::OTHER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCNTRT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `_1S`"]
    #[inline(always)]
    pub fn is_1s(&self) -> bool {
        *self == TCNTRT_A::_1S
    }
    #[doc = "Checks if the value of the field is `_2S`"]
    #[inline(always)]
    pub fn is_2s(&self) -> bool {
        *self == TCNTRT_A::_2S
    }
    #[doc = "Checks if the value of the field is `_4S`"]
    #[inline(always)]
    pub fn is_4s(&self) -> bool {
        *self == TCNTRT_A::_4S
    }
    #[doc = "Checks if the value of the field is `_8S`"]
    #[inline(always)]
    pub fn is_8s(&self) -> bool {
        *self == TCNTRT_A::_8S
    }
    #[doc = "Checks if the value of the field is `_16S`"]
    #[inline(always)]
    pub fn is_16s(&self) -> bool {
        *self == TCNTRT_A::_16S
    }
    #[doc = "Checks if the value of the field is `_32S`"]
    #[inline(always)]
    pub fn is_32s(&self) -> bool {
        *self == TCNTRT_A::_32S
    }
    #[doc = "Checks if the value of the field is `_64S`"]
    #[inline(always)]
    pub fn is_64s(&self) -> bool {
        *self == TCNTRT_A::_64S
    }
    #[doc = "Checks if the value of the field is `_128S`"]
    #[inline(always)]
    pub fn is_128s(&self) -> bool {
        *self == TCNTRT_A::_128S
    }
    #[doc = "Checks if the value of the field is `_256S`"]
    #[inline(always)]
    pub fn is_256s(&self) -> bool {
        *self == TCNTRT_A::_256S
    }
    #[doc = "Checks if the value of the field is `_512S`"]
    #[inline(always)]
    pub fn is_512s(&self) -> bool {
        *self == TCNTRT_A::_512S
    }
    #[doc = "Checks if the value of the field is `_1024S`"]
    #[inline(always)]
    pub fn is_1024s(&self) -> bool {
        *self == TCNTRT_A::_1024S
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == TCNTRT_A::OTHER
    }
}
#[doc = "Use Tuning for SDR50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSDR50_A {
    #[doc = "0: SDR50 does not require tuning"]
    NO = 0,
    #[doc = "1: SDR50 requires tuning"]
    YES = 1,
}
impl From<TSDR50_A> for bool {
    #[inline(always)]
    fn from(variant: TSDR50_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSDR50`"]
pub type TSDR50_R = crate::R<bool, TSDR50_A>;
impl TSDR50_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSDR50_A {
        match self.bits {
            false => TSDR50_A::NO,
            true => TSDR50_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == TSDR50_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == TSDR50_A::YES
    }
}
#[doc = "Clock Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKMULT_A {
    #[doc = "0: Clock Multiplier is Not Supported"]
    NO = 0,
}
impl From<CLKMULT_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKMULT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKMULT`"]
pub type CLKMULT_R = crate::R<u8, CLKMULT_A>;
impl CLKMULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKMULT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKMULT_A::NO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CLKMULT_A::NO
    }
}
impl R {
    #[doc = "Bit 0 - SDR50 Support"]
    #[inline(always)]
    pub fn sdr50sup(&self) -> SDR50SUP_R {
        SDR50SUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support"]
    #[inline(always)]
    pub fn sdr104sup(&self) -> SDR104SUP_R {
        SDR104SUP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support"]
    #[inline(always)]
    pub fn ddr50sup(&self) -> DDR50SUP_R {
        DDR50SUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Driver Type A Support"]
    #[inline(always)]
    pub fn drvasup(&self) -> DRVASUP_R {
        DRVASUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support"]
    #[inline(always)]
    pub fn drvcsup(&self) -> DRVCSUP_R {
        DRVCSUP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support"]
    #[inline(always)]
    pub fn drvdsup(&self) -> DRVDSUP_R {
        DRVDSUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Timer Count for Re-Tuning"]
    #[inline(always)]
    pub fn tcntrt(&self) -> TCNTRT_R {
        TCNTRT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    pub fn tsdr50(&self) -> TSDR50_R {
        TSDR50_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline(always)]
    pub fn clkmult(&self) -> CLKMULT_R {
        CLKMULT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
