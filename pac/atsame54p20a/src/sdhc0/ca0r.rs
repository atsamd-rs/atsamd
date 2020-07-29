#[doc = "Reader of register CA0R"]
pub type R = crate::R<u32, super::CA0R>;
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
#[doc = "Reader of field `TEOCLKF`"]
pub type TEOCLKF_R = crate::R<u8, TEOCLKF_A>;
impl TEOCLKF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TEOCLKF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TEOCLKF_A::OTHER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == TEOCLKF_A::OTHER
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
#[doc = "Reader of field `TEOCLKU`"]
pub type TEOCLKU_R = crate::R<bool, TEOCLKU_A>;
impl TEOCLKU_R {
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
        *self == TEOCLKU_A::KHZ
    }
    #[doc = "Checks if the value of the field is `MHZ`"]
    #[inline(always)]
    pub fn is_mhz(&self) -> bool {
        *self == TEOCLKU_A::MHZ
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
#[doc = "Reader of field `BASECLKF`"]
pub type BASECLKF_R = crate::R<u8, BASECLKF_A>;
impl BASECLKF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BASECLKF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BASECLKF_A::OTHER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == BASECLKF_A::OTHER
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
#[doc = "Reader of field `MAXBLKL`"]
pub type MAXBLKL_R = crate::R<u8, MAXBLKL_A>;
impl MAXBLKL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAXBLKL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MAXBLKL_A::_512),
            1 => Val(MAXBLKL_A::_1024),
            2 => Val(MAXBLKL_A::_2048),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == MAXBLKL_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == MAXBLKL_A::_1024
    }
    #[doc = "Checks if the value of the field is `_2048`"]
    #[inline(always)]
    pub fn is_2048(&self) -> bool {
        *self == MAXBLKL_A::_2048
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
#[doc = "Reader of field `ED8SUP`"]
pub type ED8SUP_R = crate::R<bool, ED8SUP_A>;
impl ED8SUP_R {
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
        *self == ED8SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ED8SUP_A::YES
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
#[doc = "Reader of field `ADMA2SUP`"]
pub type ADMA2SUP_R = crate::R<bool, ADMA2SUP_A>;
impl ADMA2SUP_R {
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
        *self == ADMA2SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ADMA2SUP_A::YES
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
#[doc = "Reader of field `HSSUP`"]
pub type HSSUP_R = crate::R<bool, HSSUP_A>;
impl HSSUP_R {
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
        *self == HSSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == HSSUP_A::YES
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
#[doc = "Reader of field `SDMASUP`"]
pub type SDMASUP_R = crate::R<bool, SDMASUP_A>;
impl SDMASUP_R {
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
        *self == SDMASUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SDMASUP_A::YES
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
#[doc = "Reader of field `SRSUP`"]
pub type SRSUP_R = crate::R<bool, SRSUP_A>;
impl SRSUP_R {
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
        *self == SRSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SRSUP_A::YES
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
#[doc = "Reader of field `V33VSUP`"]
pub type V33VSUP_R = crate::R<bool, V33VSUP_A>;
impl V33VSUP_R {
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
        *self == V33VSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == V33VSUP_A::YES
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
#[doc = "Reader of field `V30VSUP`"]
pub type V30VSUP_R = crate::R<bool, V30VSUP_A>;
impl V30VSUP_R {
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
        *self == V30VSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == V30VSUP_A::YES
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
#[doc = "Reader of field `V18VSUP`"]
pub type V18VSUP_R = crate::R<bool, V18VSUP_A>;
impl V18VSUP_R {
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
        *self == V18VSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == V18VSUP_A::YES
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
#[doc = "Reader of field `SB64SUP`"]
pub type SB64SUP_R = crate::R<bool, SB64SUP_A>;
impl SB64SUP_R {
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
        *self == SB64SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SB64SUP_A::YES
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
#[doc = "Reader of field `ASINTSUP`"]
pub type ASINTSUP_R = crate::R<bool, ASINTSUP_A>;
impl ASINTSUP_R {
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
        *self == ASINTSUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ASINTSUP_A::YES
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
#[doc = "Reader of field `SLTYPE`"]
pub type SLTYPE_R = crate::R<u8, SLTYPE_A>;
impl SLTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SLTYPE_A::REMOVABLE),
            1 => Val(SLTYPE_A::EMBEDDED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REMOVABLE`"]
    #[inline(always)]
    pub fn is_removable(&self) -> bool {
        *self == SLTYPE_A::REMOVABLE
    }
    #[doc = "Checks if the value of the field is `EMBEDDED`"]
    #[inline(always)]
    pub fn is_embedded(&self) -> bool {
        *self == SLTYPE_A::EMBEDDED
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
