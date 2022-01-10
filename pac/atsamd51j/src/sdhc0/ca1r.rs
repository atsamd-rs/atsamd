#[doc = "Register `CA1R` reader"]
pub struct R(crate::R<CA1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CA1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CA1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CA1R_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `SDR50SUP` reader - SDR50 Support"]
pub struct SDR50SUP_R(crate::FieldReader<bool, SDR50SUP_A>);
impl SDR50SUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDR50SUP_R(crate::FieldReader::new(bits))
    }
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
        **self == SDR50SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == SDR50SUP_A::YES
    }
}
impl core::ops::Deref for SDR50SUP_R {
    type Target = crate::FieldReader<bool, SDR50SUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `SDR104SUP` reader - SDR104 Support"]
pub struct SDR104SUP_R(crate::FieldReader<bool, SDR104SUP_A>);
impl SDR104SUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDR104SUP_R(crate::FieldReader::new(bits))
    }
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
        **self == SDR104SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == SDR104SUP_A::YES
    }
}
impl core::ops::Deref for SDR104SUP_R {
    type Target = crate::FieldReader<bool, SDR104SUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `DDR50SUP` reader - DDR50 Support"]
pub struct DDR50SUP_R(crate::FieldReader<bool, DDR50SUP_A>);
impl DDR50SUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DDR50SUP_R(crate::FieldReader::new(bits))
    }
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
        **self == DDR50SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == DDR50SUP_A::YES
    }
}
impl core::ops::Deref for DDR50SUP_R {
    type Target = crate::FieldReader<bool, DDR50SUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `DRVASUP` reader - Driver Type A Support"]
pub struct DRVASUP_R(crate::FieldReader<bool, DRVASUP_A>);
impl DRVASUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRVASUP_R(crate::FieldReader::new(bits))
    }
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
        **self == DRVASUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == DRVASUP_A::YES
    }
}
impl core::ops::Deref for DRVASUP_R {
    type Target = crate::FieldReader<bool, DRVASUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `DRVCSUP` reader - Driver Type C Support"]
pub struct DRVCSUP_R(crate::FieldReader<bool, DRVCSUP_A>);
impl DRVCSUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRVCSUP_R(crate::FieldReader::new(bits))
    }
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
        **self == DRVCSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == DRVCSUP_A::YES
    }
}
impl core::ops::Deref for DRVCSUP_R {
    type Target = crate::FieldReader<bool, DRVCSUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `DRVDSUP` reader - Driver Type D Support"]
pub struct DRVDSUP_R(crate::FieldReader<bool, DRVDSUP_A>);
impl DRVDSUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRVDSUP_R(crate::FieldReader::new(bits))
    }
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
        **self == DRVDSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == DRVDSUP_A::YES
    }
}
impl core::ops::Deref for DRVDSUP_R {
    type Target = crate::FieldReader<bool, DRVDSUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `TCNTRT` reader - Timer Count for Re-Tuning"]
pub struct TCNTRT_R(crate::FieldReader<u8, TCNTRT_A>);
impl TCNTRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCNTRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCNTRT_A> {
        match self.bits {
            0 => Some(TCNTRT_A::DISABLED),
            1 => Some(TCNTRT_A::_1S),
            2 => Some(TCNTRT_A::_2S),
            3 => Some(TCNTRT_A::_4S),
            4 => Some(TCNTRT_A::_8S),
            5 => Some(TCNTRT_A::_16S),
            6 => Some(TCNTRT_A::_32S),
            7 => Some(TCNTRT_A::_64S),
            8 => Some(TCNTRT_A::_128S),
            9 => Some(TCNTRT_A::_256S),
            10 => Some(TCNTRT_A::_512S),
            11 => Some(TCNTRT_A::_1024S),
            15 => Some(TCNTRT_A::OTHER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TCNTRT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `_1S`"]
    #[inline(always)]
    pub fn is_1s(&self) -> bool {
        **self == TCNTRT_A::_1S
    }
    #[doc = "Checks if the value of the field is `_2S`"]
    #[inline(always)]
    pub fn is_2s(&self) -> bool {
        **self == TCNTRT_A::_2S
    }
    #[doc = "Checks if the value of the field is `_4S`"]
    #[inline(always)]
    pub fn is_4s(&self) -> bool {
        **self == TCNTRT_A::_4S
    }
    #[doc = "Checks if the value of the field is `_8S`"]
    #[inline(always)]
    pub fn is_8s(&self) -> bool {
        **self == TCNTRT_A::_8S
    }
    #[doc = "Checks if the value of the field is `_16S`"]
    #[inline(always)]
    pub fn is_16s(&self) -> bool {
        **self == TCNTRT_A::_16S
    }
    #[doc = "Checks if the value of the field is `_32S`"]
    #[inline(always)]
    pub fn is_32s(&self) -> bool {
        **self == TCNTRT_A::_32S
    }
    #[doc = "Checks if the value of the field is `_64S`"]
    #[inline(always)]
    pub fn is_64s(&self) -> bool {
        **self == TCNTRT_A::_64S
    }
    #[doc = "Checks if the value of the field is `_128S`"]
    #[inline(always)]
    pub fn is_128s(&self) -> bool {
        **self == TCNTRT_A::_128S
    }
    #[doc = "Checks if the value of the field is `_256S`"]
    #[inline(always)]
    pub fn is_256s(&self) -> bool {
        **self == TCNTRT_A::_256S
    }
    #[doc = "Checks if the value of the field is `_512S`"]
    #[inline(always)]
    pub fn is_512s(&self) -> bool {
        **self == TCNTRT_A::_512S
    }
    #[doc = "Checks if the value of the field is `_1024S`"]
    #[inline(always)]
    pub fn is_1024s(&self) -> bool {
        **self == TCNTRT_A::_1024S
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        **self == TCNTRT_A::OTHER
    }
}
impl core::ops::Deref for TCNTRT_R {
    type Target = crate::FieldReader<u8, TCNTRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `TSDR50` reader - Use Tuning for SDR50"]
pub struct TSDR50_R(crate::FieldReader<bool, TSDR50_A>);
impl TSDR50_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSDR50_R(crate::FieldReader::new(bits))
    }
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
        **self == TSDR50_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == TSDR50_A::YES
    }
}
impl core::ops::Deref for TSDR50_R {
    type Target = crate::FieldReader<bool, TSDR50_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CLKMULT` reader - Clock Multiplier"]
pub struct CLKMULT_R(crate::FieldReader<u8, CLKMULT_A>);
impl CLKMULT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKMULT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKMULT_A> {
        match self.bits {
            0 => Some(CLKMULT_A::NO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == CLKMULT_A::NO
    }
}
impl core::ops::Deref for CLKMULT_R {
    type Target = crate::FieldReader<u8, CLKMULT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Capabilities 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ca1r](index.html) module"]
pub struct CA1R_SPEC;
impl crate::RegisterSpec for CA1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ca1r::R](R) reader structure"]
impl crate::Readable for CA1R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CA1R to value 0x70"]
impl crate::Resettable for CA1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x70
    }
}
