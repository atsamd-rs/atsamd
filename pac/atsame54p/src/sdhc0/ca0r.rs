#[doc = "Register `CA0R` reader"]
pub struct R(crate::R<CA0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CA0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CA0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CA0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Timeout Clock Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TEOCLKF_A {
    #[doc = "0: Get information via another method"]
    OTHER = 0,
}
impl From<TEOCLKF_A> for u8 {
    #[inline(always)]
    fn from(variant: TEOCLKF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TEOCLKF` reader - Timeout Clock Frequency"]
pub struct TEOCLKF_R(crate::FieldReader<u8, TEOCLKF_A>);
impl TEOCLKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TEOCLKF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TEOCLKF_A> {
        match self.bits {
            0 => Some(TEOCLKF_A::OTHER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        **self == TEOCLKF_A::OTHER
    }
}
impl core::ops::Deref for TEOCLKF_R {
    type Target = crate::FieldReader<u8, TEOCLKF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Timeout Clock Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEOCLKU_A {
    #[doc = "0: KHz"]
    KHZ = 0,
    #[doc = "1: MHz"]
    MHZ = 1,
}
impl From<TEOCLKU_A> for bool {
    #[inline(always)]
    fn from(variant: TEOCLKU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEOCLKU` reader - Timeout Clock Unit"]
pub struct TEOCLKU_R(crate::FieldReader<bool, TEOCLKU_A>);
impl TEOCLKU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEOCLKU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEOCLKU_A {
        match self.bits {
            false => TEOCLKU_A::KHZ,
            true => TEOCLKU_A::MHZ,
        }
    }
    #[doc = "Checks if the value of the field is `KHZ`"]
    #[inline(always)]
    pub fn is_khz(&self) -> bool {
        **self == TEOCLKU_A::KHZ
    }
    #[doc = "Checks if the value of the field is `MHZ`"]
    #[inline(always)]
    pub fn is_mhz(&self) -> bool {
        **self == TEOCLKU_A::MHZ
    }
}
impl core::ops::Deref for TEOCLKU_R {
    type Target = crate::FieldReader<bool, TEOCLKU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Base Clock Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BASECLKF_A {
    #[doc = "0: Get information via another method"]
    OTHER = 0,
}
impl From<BASECLKF_A> for u8 {
    #[inline(always)]
    fn from(variant: BASECLKF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BASECLKF` reader - Base Clock Frequency"]
pub struct BASECLKF_R(crate::FieldReader<u8, BASECLKF_A>);
impl BASECLKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BASECLKF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BASECLKF_A> {
        match self.bits {
            0 => Some(BASECLKF_A::OTHER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        **self == BASECLKF_A::OTHER
    }
}
impl core::ops::Deref for BASECLKF_R {
    type Target = crate::FieldReader<u8, BASECLKF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Max Block Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAXBLKL_A {
    #[doc = "0: 512 bytes"]
    _512 = 0,
    #[doc = "1: 1024 bytes"]
    _1024 = 1,
    #[doc = "2: 2048 bytes"]
    _2048 = 2,
}
impl From<MAXBLKL_A> for u8 {
    #[inline(always)]
    fn from(variant: MAXBLKL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MAXBLKL` reader - Max Block Length"]
pub struct MAXBLKL_R(crate::FieldReader<u8, MAXBLKL_A>);
impl MAXBLKL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAXBLKL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAXBLKL_A> {
        match self.bits {
            0 => Some(MAXBLKL_A::_512),
            1 => Some(MAXBLKL_A::_1024),
            2 => Some(MAXBLKL_A::_2048),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        **self == MAXBLKL_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        **self == MAXBLKL_A::_1024
    }
    #[doc = "Checks if the value of the field is `_2048`"]
    #[inline(always)]
    pub fn is_2048(&self) -> bool {
        **self == MAXBLKL_A::_2048
    }
}
impl core::ops::Deref for MAXBLKL_R {
    type Target = crate::FieldReader<u8, MAXBLKL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "8-bit Support for Embedded Device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ED8SUP_A {
    #[doc = "0: 8-bit Bus Width not Supported"]
    NO = 0,
    #[doc = "1: 8-bit Bus Width Supported"]
    YES = 1,
}
impl From<ED8SUP_A> for bool {
    #[inline(always)]
    fn from(variant: ED8SUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ED8SUP` reader - 8-bit Support for Embedded Device"]
pub struct ED8SUP_R(crate::FieldReader<bool, ED8SUP_A>);
impl ED8SUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ED8SUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ED8SUP_A {
        match self.bits {
            false => ED8SUP_A::NO,
            true => ED8SUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == ED8SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == ED8SUP_A::YES
    }
}
impl core::ops::Deref for ED8SUP_R {
    type Target = crate::FieldReader<bool, ED8SUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ADMA2 Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMA2SUP_A {
    #[doc = "0: ADMA2 not Supported"]
    NO = 0,
    #[doc = "1: ADMA2 Supported"]
    YES = 1,
}
impl From<ADMA2SUP_A> for bool {
    #[inline(always)]
    fn from(variant: ADMA2SUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMA2SUP` reader - ADMA2 Support"]
pub struct ADMA2SUP_R(crate::FieldReader<bool, ADMA2SUP_A>);
impl ADMA2SUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADMA2SUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMA2SUP_A {
        match self.bits {
            false => ADMA2SUP_A::NO,
            true => ADMA2SUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == ADMA2SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == ADMA2SUP_A::YES
    }
}
impl core::ops::Deref for ADMA2SUP_R {
    type Target = crate::FieldReader<bool, ADMA2SUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "High Speed Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSSUP_A {
    #[doc = "0: High Speed not Supported"]
    NO = 0,
    #[doc = "1: High Speed Supported"]
    YES = 1,
}
impl From<HSSUP_A> for bool {
    #[inline(always)]
    fn from(variant: HSSUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSSUP` reader - High Speed Support"]
pub struct HSSUP_R(crate::FieldReader<bool, HSSUP_A>);
impl HSSUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSSUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSSUP_A {
        match self.bits {
            false => HSSUP_A::NO,
            true => HSSUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == HSSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == HSSUP_A::YES
    }
}
impl core::ops::Deref for HSSUP_R {
    type Target = crate::FieldReader<bool, HSSUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SDMA Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMASUP_A {
    #[doc = "0: SDMA not Supported"]
    NO = 0,
    #[doc = "1: SDMA Supported"]
    YES = 1,
}
impl From<SDMASUP_A> for bool {
    #[inline(always)]
    fn from(variant: SDMASUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMASUP` reader - SDMA Support"]
pub struct SDMASUP_R(crate::FieldReader<bool, SDMASUP_A>);
impl SDMASUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMASUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMASUP_A {
        match self.bits {
            false => SDMASUP_A::NO,
            true => SDMASUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == SDMASUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == SDMASUP_A::YES
    }
}
impl core::ops::Deref for SDMASUP_R {
    type Target = crate::FieldReader<bool, SDMASUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Suspend/Resume Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRSUP_A {
    #[doc = "0: Suspend/Resume not Supported"]
    NO = 0,
    #[doc = "1: Suspend/Resume Supported"]
    YES = 1,
}
impl From<SRSUP_A> for bool {
    #[inline(always)]
    fn from(variant: SRSUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRSUP` reader - Suspend/Resume Support"]
pub struct SRSUP_R(crate::FieldReader<bool, SRSUP_A>);
impl SRSUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRSUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRSUP_A {
        match self.bits {
            false => SRSUP_A::NO,
            true => SRSUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == SRSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == SRSUP_A::YES
    }
}
impl core::ops::Deref for SRSUP_R {
    type Target = crate::FieldReader<bool, SRSUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Voltage Support 3.3V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V33VSUP_A {
    #[doc = "0: 3.3V Not Supported"]
    NO = 0,
    #[doc = "1: 3.3V Supported"]
    YES = 1,
}
impl From<V33VSUP_A> for bool {
    #[inline(always)]
    fn from(variant: V33VSUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V33VSUP` reader - Voltage Support 3.3V"]
pub struct V33VSUP_R(crate::FieldReader<bool, V33VSUP_A>);
impl V33VSUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        V33VSUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V33VSUP_A {
        match self.bits {
            false => V33VSUP_A::NO,
            true => V33VSUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == V33VSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == V33VSUP_A::YES
    }
}
impl core::ops::Deref for V33VSUP_R {
    type Target = crate::FieldReader<bool, V33VSUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Voltage Support 3.0V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V30VSUP_A {
    #[doc = "0: 3.0V Not Supported"]
    NO = 0,
    #[doc = "1: 3.0V Supported"]
    YES = 1,
}
impl From<V30VSUP_A> for bool {
    #[inline(always)]
    fn from(variant: V30VSUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V30VSUP` reader - Voltage Support 3.0V"]
pub struct V30VSUP_R(crate::FieldReader<bool, V30VSUP_A>);
impl V30VSUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        V30VSUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V30VSUP_A {
        match self.bits {
            false => V30VSUP_A::NO,
            true => V30VSUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == V30VSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == V30VSUP_A::YES
    }
}
impl core::ops::Deref for V30VSUP_R {
    type Target = crate::FieldReader<bool, V30VSUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Voltage Support 1.8V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V18VSUP_A {
    #[doc = "0: 1.8V Not Supported"]
    NO = 0,
    #[doc = "1: 1.8V Supported"]
    YES = 1,
}
impl From<V18VSUP_A> for bool {
    #[inline(always)]
    fn from(variant: V18VSUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V18VSUP` reader - Voltage Support 1.8V"]
pub struct V18VSUP_R(crate::FieldReader<bool, V18VSUP_A>);
impl V18VSUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        V18VSUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V18VSUP_A {
        match self.bits {
            false => V18VSUP_A::NO,
            true => V18VSUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == V18VSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == V18VSUP_A::YES
    }
}
impl core::ops::Deref for V18VSUP_R {
    type Target = crate::FieldReader<bool, V18VSUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "64-Bit System Bus Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SB64SUP_A {
    #[doc = "0: 32-bit Address Descriptors and System Bus"]
    NO = 0,
    #[doc = "1: 64-bit Address Descriptors and System Bus"]
    YES = 1,
}
impl From<SB64SUP_A> for bool {
    #[inline(always)]
    fn from(variant: SB64SUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SB64SUP` reader - 64-Bit System Bus Support"]
pub struct SB64SUP_R(crate::FieldReader<bool, SB64SUP_A>);
impl SB64SUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SB64SUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SB64SUP_A {
        match self.bits {
            false => SB64SUP_A::NO,
            true => SB64SUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == SB64SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == SB64SUP_A::YES
    }
}
impl core::ops::Deref for SB64SUP_R {
    type Target = crate::FieldReader<bool, SB64SUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Asynchronous Interrupt Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASINTSUP_A {
    #[doc = "0: Asynchronous Interrupt not Supported"]
    NO = 0,
    #[doc = "1: Asynchronous Interrupt supported"]
    YES = 1,
}
impl From<ASINTSUP_A> for bool {
    #[inline(always)]
    fn from(variant: ASINTSUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASINTSUP` reader - Asynchronous Interrupt Support"]
pub struct ASINTSUP_R(crate::FieldReader<bool, ASINTSUP_A>);
impl ASINTSUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ASINTSUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASINTSUP_A {
        match self.bits {
            false => ASINTSUP_A::NO,
            true => ASINTSUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == ASINTSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == ASINTSUP_A::YES
    }
}
impl core::ops::Deref for ASINTSUP_R {
    type Target = crate::FieldReader<bool, ASINTSUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slot Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLTYPE_A {
    #[doc = "0: Removable Card Slot"]
    REMOVABLE = 0,
    #[doc = "1: Embedded Slot for One Device"]
    EMBEDDED = 1,
}
impl From<SLTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SLTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SLTYPE` reader - Slot Type"]
pub struct SLTYPE_R(crate::FieldReader<u8, SLTYPE_A>);
impl SLTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLTYPE_A> {
        match self.bits {
            0 => Some(SLTYPE_A::REMOVABLE),
            1 => Some(SLTYPE_A::EMBEDDED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REMOVABLE`"]
    #[inline(always)]
    pub fn is_removable(&self) -> bool {
        **self == SLTYPE_A::REMOVABLE
    }
    #[doc = "Checks if the value of the field is `EMBEDDED`"]
    #[inline(always)]
    pub fn is_embedded(&self) -> bool {
        **self == SLTYPE_A::EMBEDDED
    }
}
impl core::ops::Deref for SLTYPE_R {
    type Target = crate::FieldReader<u8, SLTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn teoclkf(&self) -> TEOCLKF_R {
        TEOCLKF_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Timeout Clock Unit"]
    #[inline(always)]
    pub fn teoclku(&self) -> TEOCLKU_R {
        TEOCLKU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency"]
    #[inline(always)]
    pub fn baseclkf(&self) -> BASECLKF_R {
        BASECLKF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Max Block Length"]
    #[inline(always)]
    pub fn maxblkl(&self) -> MAXBLKL_R {
        MAXBLKL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - 8-bit Support for Embedded Device"]
    #[inline(always)]
    pub fn ed8sup(&self) -> ED8SUP_R {
        ED8SUP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline(always)]
    pub fn adma2sup(&self) -> ADMA2SUP_R {
        ADMA2SUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline(always)]
    pub fn hssup(&self) -> HSSUP_R {
        HSSUP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDMA Support"]
    #[inline(always)]
    pub fn sdmasup(&self) -> SDMASUP_R {
        SDMASUP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Suspend/Resume Support"]
    #[inline(always)]
    pub fn srsup(&self) -> SRSUP_R {
        SRSUP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline(always)]
    pub fn v33vsup(&self) -> V33VSUP_R {
        V33VSUP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0V"]
    #[inline(always)]
    pub fn v30vsup(&self) -> V30VSUP_R {
        V30VSUP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8V"]
    #[inline(always)]
    pub fn v18vsup(&self) -> V18VSUP_R {
        V18VSUP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 64-Bit System Bus Support"]
    #[inline(always)]
    pub fn sb64sup(&self) -> SB64SUP_R {
        SB64SUP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn asintsup(&self) -> ASINTSUP_R {
        ASINTSUP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Slot Type"]
    #[inline(always)]
    pub fn sltype(&self) -> SLTYPE_R {
        SLTYPE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
#[doc = "Capabilities 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ca0r](index.html) module"]
pub struct CA0R_SPEC;
impl crate::RegisterSpec for CA0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ca0r::R](R) reader structure"]
impl crate::Readable for CA0R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CA0R to value 0x27e8_0080"]
impl crate::Resettable for CA0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x27e8_0080
    }
}
