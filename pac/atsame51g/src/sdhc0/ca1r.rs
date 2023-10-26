#[doc = "Register `CA1R` reader"]
pub type R = crate::R<CA1R_SPEC>;
#[doc = "Field `SDR50SUP` reader - SDR50 Support"]
pub type SDR50SUP_R = crate::BitReader<SDR50SUPSELECT_A>;
#[doc = "SDR50 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDR50SUPSELECT_A {
    #[doc = "0: SDR50 is Not Supported"]
    NO = 0,
    #[doc = "1: SDR50 is Supported"]
    YES = 1,
}
impl From<SDR50SUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SDR50SUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SDR50SUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDR50SUPSELECT_A {
        match self.bits {
            false => SDR50SUPSELECT_A::NO,
            true => SDR50SUPSELECT_A::YES,
        }
    }
    #[doc = "SDR50 is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SDR50SUPSELECT_A::NO
    }
    #[doc = "SDR50 is Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SDR50SUPSELECT_A::YES
    }
}
#[doc = "Field `SDR104SUP` reader - SDR104 Support"]
pub type SDR104SUP_R = crate::BitReader<SDR104SUPSELECT_A>;
#[doc = "SDR104 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDR104SUPSELECT_A {
    #[doc = "0: SDR104 is Not Supported"]
    NO = 0,
    #[doc = "1: SDR104 is Supported"]
    YES = 1,
}
impl From<SDR104SUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SDR104SUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SDR104SUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDR104SUPSELECT_A {
        match self.bits {
            false => SDR104SUPSELECT_A::NO,
            true => SDR104SUPSELECT_A::YES,
        }
    }
    #[doc = "SDR104 is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SDR104SUPSELECT_A::NO
    }
    #[doc = "SDR104 is Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SDR104SUPSELECT_A::YES
    }
}
#[doc = "Field `DDR50SUP` reader - DDR50 Support"]
pub type DDR50SUP_R = crate::BitReader<DDR50SUPSELECT_A>;
#[doc = "DDR50 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDR50SUPSELECT_A {
    #[doc = "0: DDR50 is Not Supported"]
    NO = 0,
    #[doc = "1: DDR50 is Supported"]
    YES = 1,
}
impl From<DDR50SUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DDR50SUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DDR50SUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DDR50SUPSELECT_A {
        match self.bits {
            false => DDR50SUPSELECT_A::NO,
            true => DDR50SUPSELECT_A::YES,
        }
    }
    #[doc = "DDR50 is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == DDR50SUPSELECT_A::NO
    }
    #[doc = "DDR50 is Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == DDR50SUPSELECT_A::YES
    }
}
#[doc = "Field `DRVASUP` reader - Driver Type A Support"]
pub type DRVASUP_R = crate::BitReader<DRVASUPSELECT_A>;
#[doc = "Driver Type A Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRVASUPSELECT_A {
    #[doc = "0: Driver Type A is Not Supported"]
    NO = 0,
    #[doc = "1: Driver Type A is Supported"]
    YES = 1,
}
impl From<DRVASUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DRVASUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DRVASUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRVASUPSELECT_A {
        match self.bits {
            false => DRVASUPSELECT_A::NO,
            true => DRVASUPSELECT_A::YES,
        }
    }
    #[doc = "Driver Type A is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == DRVASUPSELECT_A::NO
    }
    #[doc = "Driver Type A is Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == DRVASUPSELECT_A::YES
    }
}
#[doc = "Field `DRVCSUP` reader - Driver Type C Support"]
pub type DRVCSUP_R = crate::BitReader<DRVCSUPSELECT_A>;
#[doc = "Driver Type C Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRVCSUPSELECT_A {
    #[doc = "0: Driver Type C is Not Supported"]
    NO = 0,
    #[doc = "1: Driver Type C is Supported"]
    YES = 1,
}
impl From<DRVCSUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DRVCSUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DRVCSUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRVCSUPSELECT_A {
        match self.bits {
            false => DRVCSUPSELECT_A::NO,
            true => DRVCSUPSELECT_A::YES,
        }
    }
    #[doc = "Driver Type C is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == DRVCSUPSELECT_A::NO
    }
    #[doc = "Driver Type C is Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == DRVCSUPSELECT_A::YES
    }
}
#[doc = "Field `DRVDSUP` reader - Driver Type D Support"]
pub type DRVDSUP_R = crate::BitReader<DRVDSUPSELECT_A>;
#[doc = "Driver Type D Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRVDSUPSELECT_A {
    #[doc = "0: Driver Type D is Not Supported"]
    NO = 0,
    #[doc = "1: Driver Type D is Supported"]
    YES = 1,
}
impl From<DRVDSUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DRVDSUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DRVDSUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRVDSUPSELECT_A {
        match self.bits {
            false => DRVDSUPSELECT_A::NO,
            true => DRVDSUPSELECT_A::YES,
        }
    }
    #[doc = "Driver Type D is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == DRVDSUPSELECT_A::NO
    }
    #[doc = "Driver Type D is Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == DRVDSUPSELECT_A::YES
    }
}
#[doc = "Field `TCNTRT` reader - Timer Count for Re-Tuning"]
pub type TCNTRT_R = crate::FieldReader<TCNTRTSELECT_A>;
#[doc = "Timer Count for Re-Tuning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCNTRTSELECT_A {
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
impl From<TCNTRTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TCNTRTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCNTRTSELECT_A {
    type Ux = u8;
}
impl TCNTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCNTRTSELECT_A> {
        match self.bits {
            0 => Some(TCNTRTSELECT_A::DISABLED),
            1 => Some(TCNTRTSELECT_A::_1S),
            2 => Some(TCNTRTSELECT_A::_2S),
            3 => Some(TCNTRTSELECT_A::_4S),
            4 => Some(TCNTRTSELECT_A::_8S),
            5 => Some(TCNTRTSELECT_A::_16S),
            6 => Some(TCNTRTSELECT_A::_32S),
            7 => Some(TCNTRTSELECT_A::_64S),
            8 => Some(TCNTRTSELECT_A::_128S),
            9 => Some(TCNTRTSELECT_A::_256S),
            10 => Some(TCNTRTSELECT_A::_512S),
            11 => Some(TCNTRTSELECT_A::_1024S),
            15 => Some(TCNTRTSELECT_A::OTHER),
            _ => None,
        }
    }
    #[doc = "Re-Tuning Timer disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCNTRTSELECT_A::DISABLED
    }
    #[doc = "1 second"]
    #[inline(always)]
    pub fn is_1s(&self) -> bool {
        *self == TCNTRTSELECT_A::_1S
    }
    #[doc = "2 seconds"]
    #[inline(always)]
    pub fn is_2s(&self) -> bool {
        *self == TCNTRTSELECT_A::_2S
    }
    #[doc = "4 seconds"]
    #[inline(always)]
    pub fn is_4s(&self) -> bool {
        *self == TCNTRTSELECT_A::_4S
    }
    #[doc = "8 seconds"]
    #[inline(always)]
    pub fn is_8s(&self) -> bool {
        *self == TCNTRTSELECT_A::_8S
    }
    #[doc = "16 seconds"]
    #[inline(always)]
    pub fn is_16s(&self) -> bool {
        *self == TCNTRTSELECT_A::_16S
    }
    #[doc = "32 seconds"]
    #[inline(always)]
    pub fn is_32s(&self) -> bool {
        *self == TCNTRTSELECT_A::_32S
    }
    #[doc = "64 seconds"]
    #[inline(always)]
    pub fn is_64s(&self) -> bool {
        *self == TCNTRTSELECT_A::_64S
    }
    #[doc = "128 seconds"]
    #[inline(always)]
    pub fn is_128s(&self) -> bool {
        *self == TCNTRTSELECT_A::_128S
    }
    #[doc = "256 seconds"]
    #[inline(always)]
    pub fn is_256s(&self) -> bool {
        *self == TCNTRTSELECT_A::_256S
    }
    #[doc = "512 seconds"]
    #[inline(always)]
    pub fn is_512s(&self) -> bool {
        *self == TCNTRTSELECT_A::_512S
    }
    #[doc = "1024 seconds"]
    #[inline(always)]
    pub fn is_1024s(&self) -> bool {
        *self == TCNTRTSELECT_A::_1024S
    }
    #[doc = "Get information from other source"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == TCNTRTSELECT_A::OTHER
    }
}
#[doc = "Field `TSDR50` reader - Use Tuning for SDR50"]
pub type TSDR50_R = crate::BitReader<TSDR50SELECT_A>;
#[doc = "Use Tuning for SDR50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSDR50SELECT_A {
    #[doc = "0: SDR50 does not require tuning"]
    NO = 0,
    #[doc = "1: SDR50 requires tuning"]
    YES = 1,
}
impl From<TSDR50SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TSDR50SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TSDR50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSDR50SELECT_A {
        match self.bits {
            false => TSDR50SELECT_A::NO,
            true => TSDR50SELECT_A::YES,
        }
    }
    #[doc = "SDR50 does not require tuning"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == TSDR50SELECT_A::NO
    }
    #[doc = "SDR50 requires tuning"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == TSDR50SELECT_A::YES
    }
}
#[doc = "Field `CLKMULT` reader - Clock Multiplier"]
pub type CLKMULT_R = crate::FieldReader<CLKMULTSELECT_A>;
#[doc = "Clock Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKMULTSELECT_A {
    #[doc = "0: Clock Multiplier is Not Supported"]
    NO = 0,
}
impl From<CLKMULTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKMULTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKMULTSELECT_A {
    type Ux = u8;
}
impl CLKMULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKMULTSELECT_A> {
        match self.bits {
            0 => Some(CLKMULTSELECT_A::NO),
            _ => None,
        }
    }
    #[doc = "Clock Multiplier is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CLKMULTSELECT_A::NO
    }
}
impl R {
    #[doc = "Bit 0 - SDR50 Support"]
    #[inline(always)]
    pub fn sdr50sup(&self) -> SDR50SUP_R {
        SDR50SUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support"]
    #[inline(always)]
    pub fn sdr104sup(&self) -> SDR104SUP_R {
        SDR104SUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support"]
    #[inline(always)]
    pub fn ddr50sup(&self) -> DDR50SUP_R {
        DDR50SUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Driver Type A Support"]
    #[inline(always)]
    pub fn drvasup(&self) -> DRVASUP_R {
        DRVASUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support"]
    #[inline(always)]
    pub fn drvcsup(&self) -> DRVCSUP_R {
        DRVCSUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support"]
    #[inline(always)]
    pub fn drvdsup(&self) -> DRVDSUP_R {
        DRVDSUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Timer Count for Re-Tuning"]
    #[inline(always)]
    pub fn tcntrt(&self) -> TCNTRT_R {
        TCNTRT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    pub fn tsdr50(&self) -> TSDR50_R {
        TSDR50_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline(always)]
    pub fn clkmult(&self) -> CLKMULT_R {
        CLKMULT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Capabilities 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ca1r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CA1R_SPEC;
impl crate::RegisterSpec for CA1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ca1r::R`](R) reader structure"]
impl crate::Readable for CA1R_SPEC {}
#[doc = "`reset()` method sets CA1R to value 0x70"]
impl crate::Resettable for CA1R_SPEC {
    const RESET_VALUE: Self::Ux = 0x70;
}
