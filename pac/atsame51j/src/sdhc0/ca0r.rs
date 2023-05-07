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
#[doc = "Field `TEOCLKF` reader - Timeout Clock Frequency"]
pub type TEOCLKF_R = crate::FieldReader<u8, TEOCLKFSELECT_A>;
#[doc = "Timeout Clock Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TEOCLKFSELECT_A {
    #[doc = "0: Get information via another method"]
    OTHER = 0,
}
impl From<TEOCLKFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TEOCLKFSELECT_A) -> Self {
        variant as _
    }
}
impl TEOCLKF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TEOCLKFSELECT_A> {
        match self.bits {
            0 => Some(TEOCLKFSELECT_A::OTHER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == TEOCLKFSELECT_A::OTHER
    }
}
#[doc = "Field `TEOCLKU` reader - Timeout Clock Unit"]
pub type TEOCLKU_R = crate::BitReader<TEOCLKUSELECT_A>;
#[doc = "Timeout Clock Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEOCLKUSELECT_A {
    #[doc = "0: KHz"]
    KHZ = 0,
    #[doc = "1: MHz"]
    MHZ = 1,
}
impl From<TEOCLKUSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TEOCLKUSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TEOCLKU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEOCLKUSELECT_A {
        match self.bits {
            false => TEOCLKUSELECT_A::KHZ,
            true => TEOCLKUSELECT_A::MHZ,
        }
    }
    #[doc = "Checks if the value of the field is `KHZ`"]
    #[inline(always)]
    pub fn is_khz(&self) -> bool {
        *self == TEOCLKUSELECT_A::KHZ
    }
    #[doc = "Checks if the value of the field is `MHZ`"]
    #[inline(always)]
    pub fn is_mhz(&self) -> bool {
        *self == TEOCLKUSELECT_A::MHZ
    }
}
#[doc = "Field `BASECLKF` reader - Base Clock Frequency"]
pub type BASECLKF_R = crate::FieldReader<u8, BASECLKFSELECT_A>;
#[doc = "Base Clock Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BASECLKFSELECT_A {
    #[doc = "0: Get information via another method"]
    OTHER = 0,
}
impl From<BASECLKFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BASECLKFSELECT_A) -> Self {
        variant as _
    }
}
impl BASECLKF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BASECLKFSELECT_A> {
        match self.bits {
            0 => Some(BASECLKFSELECT_A::OTHER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == BASECLKFSELECT_A::OTHER
    }
}
#[doc = "Field `MAXBLKL` reader - Max Block Length"]
pub type MAXBLKL_R = crate::FieldReader<u8, MAXBLKLSELECT_A>;
#[doc = "Max Block Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAXBLKLSELECT_A {
    #[doc = "0: 512 bytes"]
    _512 = 0,
    #[doc = "1: 1024 bytes"]
    _1024 = 1,
    #[doc = "2: 2048 bytes"]
    _2048 = 2,
}
impl From<MAXBLKLSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MAXBLKLSELECT_A) -> Self {
        variant as _
    }
}
impl MAXBLKL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAXBLKLSELECT_A> {
        match self.bits {
            0 => Some(MAXBLKLSELECT_A::_512),
            1 => Some(MAXBLKLSELECT_A::_1024),
            2 => Some(MAXBLKLSELECT_A::_2048),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == MAXBLKLSELECT_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == MAXBLKLSELECT_A::_1024
    }
    #[doc = "Checks if the value of the field is `_2048`"]
    #[inline(always)]
    pub fn is_2048(&self) -> bool {
        *self == MAXBLKLSELECT_A::_2048
    }
}
#[doc = "Field `ED8SUP` reader - 8-bit Support for Embedded Device"]
pub type ED8SUP_R = crate::BitReader<ED8SUPSELECT_A>;
#[doc = "8-bit Support for Embedded Device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ED8SUPSELECT_A {
    #[doc = "0: 8-bit Bus Width not Supported"]
    NO = 0,
    #[doc = "1: 8-bit Bus Width Supported"]
    YES = 1,
}
impl From<ED8SUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ED8SUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ED8SUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ED8SUPSELECT_A {
        match self.bits {
            false => ED8SUPSELECT_A::NO,
            true => ED8SUPSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ED8SUPSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ED8SUPSELECT_A::YES
    }
}
#[doc = "Field `ADMA2SUP` reader - ADMA2 Support"]
pub type ADMA2SUP_R = crate::BitReader<ADMA2SUPSELECT_A>;
#[doc = "ADMA2 Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADMA2SUPSELECT_A {
    #[doc = "0: ADMA2 not Supported"]
    NO = 0,
    #[doc = "1: ADMA2 Supported"]
    YES = 1,
}
impl From<ADMA2SUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ADMA2SUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADMA2SUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMA2SUPSELECT_A {
        match self.bits {
            false => ADMA2SUPSELECT_A::NO,
            true => ADMA2SUPSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ADMA2SUPSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ADMA2SUPSELECT_A::YES
    }
}
#[doc = "Field `HSSUP` reader - High Speed Support"]
pub type HSSUP_R = crate::BitReader<HSSUPSELECT_A>;
#[doc = "High Speed Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSSUPSELECT_A {
    #[doc = "0: High Speed not Supported"]
    NO = 0,
    #[doc = "1: High Speed Supported"]
    YES = 1,
}
impl From<HSSUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: HSSUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl HSSUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSSUPSELECT_A {
        match self.bits {
            false => HSSUPSELECT_A::NO,
            true => HSSUPSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == HSSUPSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == HSSUPSELECT_A::YES
    }
}
#[doc = "Field `SDMASUP` reader - SDMA Support"]
pub type SDMASUP_R = crate::BitReader<SDMASUPSELECT_A>;
#[doc = "SDMA Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMASUPSELECT_A {
    #[doc = "0: SDMA not Supported"]
    NO = 0,
    #[doc = "1: SDMA Supported"]
    YES = 1,
}
impl From<SDMASUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SDMASUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMASUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMASUPSELECT_A {
        match self.bits {
            false => SDMASUPSELECT_A::NO,
            true => SDMASUPSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SDMASUPSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SDMASUPSELECT_A::YES
    }
}
#[doc = "Field `SRSUP` reader - Suspend/Resume Support"]
pub type SRSUP_R = crate::BitReader<SRSUPSELECT_A>;
#[doc = "Suspend/Resume Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRSUPSELECT_A {
    #[doc = "0: Suspend/Resume not Supported"]
    NO = 0,
    #[doc = "1: Suspend/Resume Supported"]
    YES = 1,
}
impl From<SRSUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SRSUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SRSUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRSUPSELECT_A {
        match self.bits {
            false => SRSUPSELECT_A::NO,
            true => SRSUPSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SRSUPSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SRSUPSELECT_A::YES
    }
}
#[doc = "Field `V33VSUP` reader - Voltage Support 3.3V"]
pub type V33VSUP_R = crate::BitReader<V33VSUPSELECT_A>;
#[doc = "Voltage Support 3.3V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V33VSUPSELECT_A {
    #[doc = "0: 3.3V Not Supported"]
    NO = 0,
    #[doc = "1: 3.3V Supported"]
    YES = 1,
}
impl From<V33VSUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: V33VSUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl V33VSUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V33VSUPSELECT_A {
        match self.bits {
            false => V33VSUPSELECT_A::NO,
            true => V33VSUPSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == V33VSUPSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == V33VSUPSELECT_A::YES
    }
}
#[doc = "Field `V30VSUP` reader - Voltage Support 3.0V"]
pub type V30VSUP_R = crate::BitReader<V30VSUPSELECT_A>;
#[doc = "Voltage Support 3.0V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V30VSUPSELECT_A {
    #[doc = "0: 3.0V Not Supported"]
    NO = 0,
    #[doc = "1: 3.0V Supported"]
    YES = 1,
}
impl From<V30VSUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: V30VSUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl V30VSUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V30VSUPSELECT_A {
        match self.bits {
            false => V30VSUPSELECT_A::NO,
            true => V30VSUPSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == V30VSUPSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == V30VSUPSELECT_A::YES
    }
}
#[doc = "Field `V18VSUP` reader - Voltage Support 1.8V"]
pub type V18VSUP_R = crate::BitReader<V18VSUPSELECT_A>;
#[doc = "Voltage Support 1.8V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V18VSUPSELECT_A {
    #[doc = "0: 1.8V Not Supported"]
    NO = 0,
    #[doc = "1: 1.8V Supported"]
    YES = 1,
}
impl From<V18VSUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: V18VSUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl V18VSUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V18VSUPSELECT_A {
        match self.bits {
            false => V18VSUPSELECT_A::NO,
            true => V18VSUPSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == V18VSUPSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == V18VSUPSELECT_A::YES
    }
}
#[doc = "Field `SB64SUP` reader - 64-Bit System Bus Support"]
pub type SB64SUP_R = crate::BitReader<SB64SUPSELECT_A>;
#[doc = "64-Bit System Bus Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SB64SUPSELECT_A {
    #[doc = "0: 32-bit Address Descriptors and System Bus"]
    NO = 0,
    #[doc = "1: 64-bit Address Descriptors and System Bus"]
    YES = 1,
}
impl From<SB64SUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SB64SUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SB64SUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SB64SUPSELECT_A {
        match self.bits {
            false => SB64SUPSELECT_A::NO,
            true => SB64SUPSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SB64SUPSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SB64SUPSELECT_A::YES
    }
}
#[doc = "Field `ASINTSUP` reader - Asynchronous Interrupt Support"]
pub type ASINTSUP_R = crate::BitReader<ASINTSUPSELECT_A>;
#[doc = "Asynchronous Interrupt Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASINTSUPSELECT_A {
    #[doc = "0: Asynchronous Interrupt not Supported"]
    NO = 0,
    #[doc = "1: Asynchronous Interrupt supported"]
    YES = 1,
}
impl From<ASINTSUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ASINTSUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ASINTSUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASINTSUPSELECT_A {
        match self.bits {
            false => ASINTSUPSELECT_A::NO,
            true => ASINTSUPSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ASINTSUPSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ASINTSUPSELECT_A::YES
    }
}
#[doc = "Field `SLTYPE` reader - Slot Type"]
pub type SLTYPE_R = crate::FieldReader<u8, SLTYPESELECT_A>;
#[doc = "Slot Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLTYPESELECT_A {
    #[doc = "0: Removable Card Slot"]
    REMOVABLE = 0,
    #[doc = "1: Embedded Slot for One Device"]
    EMBEDDED = 1,
}
impl From<SLTYPESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SLTYPESELECT_A) -> Self {
        variant as _
    }
}
impl SLTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLTYPESELECT_A> {
        match self.bits {
            0 => Some(SLTYPESELECT_A::REMOVABLE),
            1 => Some(SLTYPESELECT_A::EMBEDDED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REMOVABLE`"]
    #[inline(always)]
    pub fn is_removable(&self) -> bool {
        *self == SLTYPESELECT_A::REMOVABLE
    }
    #[doc = "Checks if the value of the field is `EMBEDDED`"]
    #[inline(always)]
    pub fn is_embedded(&self) -> bool {
        *self == SLTYPESELECT_A::EMBEDDED
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
        TEOCLKU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency"]
    #[inline(always)]
    pub fn baseclkf(&self) -> BASECLKF_R {
        BASECLKF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Max Block Length"]
    #[inline(always)]
    pub fn maxblkl(&self) -> MAXBLKL_R {
        MAXBLKL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 8-bit Support for Embedded Device"]
    #[inline(always)]
    pub fn ed8sup(&self) -> ED8SUP_R {
        ED8SUP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline(always)]
    pub fn adma2sup(&self) -> ADMA2SUP_R {
        ADMA2SUP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline(always)]
    pub fn hssup(&self) -> HSSUP_R {
        HSSUP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMA Support"]
    #[inline(always)]
    pub fn sdmasup(&self) -> SDMASUP_R {
        SDMASUP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Suspend/Resume Support"]
    #[inline(always)]
    pub fn srsup(&self) -> SRSUP_R {
        SRSUP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline(always)]
    pub fn v33vsup(&self) -> V33VSUP_R {
        V33VSUP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0V"]
    #[inline(always)]
    pub fn v30vsup(&self) -> V30VSUP_R {
        V30VSUP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8V"]
    #[inline(always)]
    pub fn v18vsup(&self) -> V18VSUP_R {
        V18VSUP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 64-Bit System Bus Support"]
    #[inline(always)]
    pub fn sb64sup(&self) -> SB64SUP_R {
        SB64SUP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn asintsup(&self) -> ASINTSUP_R {
        ASINTSUP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Slot Type"]
    #[inline(always)]
    pub fn sltype(&self) -> SLTYPE_R {
        SLTYPE_R::new(((self.bits >> 30) & 3) as u8)
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
    const RESET_VALUE: Self::Ux = 0x27e8_0080;
}
