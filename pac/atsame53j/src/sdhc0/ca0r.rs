#[doc = "Register `CA0R` reader"]
pub type R = crate::R<Ca0rSpec>;
#[doc = "Timeout Clock Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Teoclkfselect {
    #[doc = "0: Get information via another method"]
    Other = 0,
}
impl From<Teoclkfselect> for u8 {
    #[inline(always)]
    fn from(variant: Teoclkfselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Teoclkfselect {
    type Ux = u8;
}
impl crate::IsEnum for Teoclkfselect {}
#[doc = "Field `TEOCLKF` reader - Timeout Clock Frequency"]
pub type TeoclkfR = crate::FieldReader<Teoclkfselect>;
impl TeoclkfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Teoclkfselect> {
        match self.bits {
            0 => Some(Teoclkfselect::Other),
            _ => None,
        }
    }
    #[doc = "Get information via another method"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == Teoclkfselect::Other
    }
}
#[doc = "Timeout Clock Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teoclkuselect {
    #[doc = "0: KHz"]
    Khz = 0,
    #[doc = "1: MHz"]
    Mhz = 1,
}
impl From<Teoclkuselect> for bool {
    #[inline(always)]
    fn from(variant: Teoclkuselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEOCLKU` reader - Timeout Clock Unit"]
pub type TeoclkuR = crate::BitReader<Teoclkuselect>;
impl TeoclkuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teoclkuselect {
        match self.bits {
            false => Teoclkuselect::Khz,
            true => Teoclkuselect::Mhz,
        }
    }
    #[doc = "KHz"]
    #[inline(always)]
    pub fn is_khz(&self) -> bool {
        *self == Teoclkuselect::Khz
    }
    #[doc = "MHz"]
    #[inline(always)]
    pub fn is_mhz(&self) -> bool {
        *self == Teoclkuselect::Mhz
    }
}
#[doc = "Base Clock Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Baseclkfselect {
    #[doc = "0: Get information via another method"]
    Other = 0,
}
impl From<Baseclkfselect> for u8 {
    #[inline(always)]
    fn from(variant: Baseclkfselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Baseclkfselect {
    type Ux = u8;
}
impl crate::IsEnum for Baseclkfselect {}
#[doc = "Field `BASECLKF` reader - Base Clock Frequency"]
pub type BaseclkfR = crate::FieldReader<Baseclkfselect>;
impl BaseclkfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Baseclkfselect> {
        match self.bits {
            0 => Some(Baseclkfselect::Other),
            _ => None,
        }
    }
    #[doc = "Get information via another method"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == Baseclkfselect::Other
    }
}
#[doc = "Max Block Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maxblklselect {
    #[doc = "0: 512 bytes"]
    _512 = 0,
    #[doc = "1: 1024 bytes"]
    _1024 = 1,
    #[doc = "2: 2048 bytes"]
    _2048 = 2,
}
impl From<Maxblklselect> for u8 {
    #[inline(always)]
    fn from(variant: Maxblklselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maxblklselect {
    type Ux = u8;
}
impl crate::IsEnum for Maxblklselect {}
#[doc = "Field `MAXBLKL` reader - Max Block Length"]
pub type MaxblklR = crate::FieldReader<Maxblklselect>;
impl MaxblklR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maxblklselect> {
        match self.bits {
            0 => Some(Maxblklselect::_512),
            1 => Some(Maxblklselect::_1024),
            2 => Some(Maxblklselect::_2048),
            _ => None,
        }
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Maxblklselect::_512
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Maxblklselect::_1024
    }
    #[doc = "2048 bytes"]
    #[inline(always)]
    pub fn is_2048(&self) -> bool {
        *self == Maxblklselect::_2048
    }
}
#[doc = "8-bit Support for Embedded Device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ed8supselect {
    #[doc = "0: 8-bit Bus Width not Supported"]
    No = 0,
    #[doc = "1: 8-bit Bus Width Supported"]
    Yes = 1,
}
impl From<Ed8supselect> for bool {
    #[inline(always)]
    fn from(variant: Ed8supselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ED8SUP` reader - 8-bit Support for Embedded Device"]
pub type Ed8supR = crate::BitReader<Ed8supselect>;
impl Ed8supR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ed8supselect {
        match self.bits {
            false => Ed8supselect::No,
            true => Ed8supselect::Yes,
        }
    }
    #[doc = "8-bit Bus Width not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Ed8supselect::No
    }
    #[doc = "8-bit Bus Width Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Ed8supselect::Yes
    }
}
#[doc = "ADMA2 Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adma2supselect {
    #[doc = "0: ADMA2 not Supported"]
    No = 0,
    #[doc = "1: ADMA2 Supported"]
    Yes = 1,
}
impl From<Adma2supselect> for bool {
    #[inline(always)]
    fn from(variant: Adma2supselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMA2SUP` reader - ADMA2 Support"]
pub type Adma2supR = crate::BitReader<Adma2supselect>;
impl Adma2supR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adma2supselect {
        match self.bits {
            false => Adma2supselect::No,
            true => Adma2supselect::Yes,
        }
    }
    #[doc = "ADMA2 not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Adma2supselect::No
    }
    #[doc = "ADMA2 Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Adma2supselect::Yes
    }
}
#[doc = "High Speed Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hssupselect {
    #[doc = "0: High Speed not Supported"]
    No = 0,
    #[doc = "1: High Speed Supported"]
    Yes = 1,
}
impl From<Hssupselect> for bool {
    #[inline(always)]
    fn from(variant: Hssupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSSUP` reader - High Speed Support"]
pub type HssupR = crate::BitReader<Hssupselect>;
impl HssupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hssupselect {
        match self.bits {
            false => Hssupselect::No,
            true => Hssupselect::Yes,
        }
    }
    #[doc = "High Speed not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Hssupselect::No
    }
    #[doc = "High Speed Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Hssupselect::Yes
    }
}
#[doc = "SDMA Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdmasupselect {
    #[doc = "0: SDMA not Supported"]
    No = 0,
    #[doc = "1: SDMA Supported"]
    Yes = 1,
}
impl From<Sdmasupselect> for bool {
    #[inline(always)]
    fn from(variant: Sdmasupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMASUP` reader - SDMA Support"]
pub type SdmasupR = crate::BitReader<Sdmasupselect>;
impl SdmasupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdmasupselect {
        match self.bits {
            false => Sdmasupselect::No,
            true => Sdmasupselect::Yes,
        }
    }
    #[doc = "SDMA not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Sdmasupselect::No
    }
    #[doc = "SDMA Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Sdmasupselect::Yes
    }
}
#[doc = "Suspend/Resume Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srsupselect {
    #[doc = "0: Suspend/Resume not Supported"]
    No = 0,
    #[doc = "1: Suspend/Resume Supported"]
    Yes = 1,
}
impl From<Srsupselect> for bool {
    #[inline(always)]
    fn from(variant: Srsupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRSUP` reader - Suspend/Resume Support"]
pub type SrsupR = crate::BitReader<Srsupselect>;
impl SrsupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srsupselect {
        match self.bits {
            false => Srsupselect::No,
            true => Srsupselect::Yes,
        }
    }
    #[doc = "Suspend/Resume not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Srsupselect::No
    }
    #[doc = "Suspend/Resume Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Srsupselect::Yes
    }
}
#[doc = "Voltage Support 3.3V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V33vsupselect {
    #[doc = "0: 3.3V Not Supported"]
    No = 0,
    #[doc = "1: 3.3V Supported"]
    Yes = 1,
}
impl From<V33vsupselect> for bool {
    #[inline(always)]
    fn from(variant: V33vsupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V33VSUP` reader - Voltage Support 3.3V"]
pub type V33vsupR = crate::BitReader<V33vsupselect>;
impl V33vsupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> V33vsupselect {
        match self.bits {
            false => V33vsupselect::No,
            true => V33vsupselect::Yes,
        }
    }
    #[doc = "3.3V Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == V33vsupselect::No
    }
    #[doc = "3.3V Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == V33vsupselect::Yes
    }
}
#[doc = "Voltage Support 3.0V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V30vsupselect {
    #[doc = "0: 3.0V Not Supported"]
    No = 0,
    #[doc = "1: 3.0V Supported"]
    Yes = 1,
}
impl From<V30vsupselect> for bool {
    #[inline(always)]
    fn from(variant: V30vsupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V30VSUP` reader - Voltage Support 3.0V"]
pub type V30vsupR = crate::BitReader<V30vsupselect>;
impl V30vsupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> V30vsupselect {
        match self.bits {
            false => V30vsupselect::No,
            true => V30vsupselect::Yes,
        }
    }
    #[doc = "3.0V Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == V30vsupselect::No
    }
    #[doc = "3.0V Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == V30vsupselect::Yes
    }
}
#[doc = "Voltage Support 1.8V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V18vsupselect {
    #[doc = "0: 1.8V Not Supported"]
    No = 0,
    #[doc = "1: 1.8V Supported"]
    Yes = 1,
}
impl From<V18vsupselect> for bool {
    #[inline(always)]
    fn from(variant: V18vsupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V18VSUP` reader - Voltage Support 1.8V"]
pub type V18vsupR = crate::BitReader<V18vsupselect>;
impl V18vsupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> V18vsupselect {
        match self.bits {
            false => V18vsupselect::No,
            true => V18vsupselect::Yes,
        }
    }
    #[doc = "1.8V Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == V18vsupselect::No
    }
    #[doc = "1.8V Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == V18vsupselect::Yes
    }
}
#[doc = "64-Bit System Bus Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sb64supselect {
    #[doc = "0: 32-bit Address Descriptors and System Bus"]
    No = 0,
    #[doc = "1: 64-bit Address Descriptors and System Bus"]
    Yes = 1,
}
impl From<Sb64supselect> for bool {
    #[inline(always)]
    fn from(variant: Sb64supselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SB64SUP` reader - 64-Bit System Bus Support"]
pub type Sb64supR = crate::BitReader<Sb64supselect>;
impl Sb64supR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sb64supselect {
        match self.bits {
            false => Sb64supselect::No,
            true => Sb64supselect::Yes,
        }
    }
    #[doc = "32-bit Address Descriptors and System Bus"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Sb64supselect::No
    }
    #[doc = "64-bit Address Descriptors and System Bus"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Sb64supselect::Yes
    }
}
#[doc = "Asynchronous Interrupt Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asintsupselect {
    #[doc = "0: Asynchronous Interrupt not Supported"]
    No = 0,
    #[doc = "1: Asynchronous Interrupt supported"]
    Yes = 1,
}
impl From<Asintsupselect> for bool {
    #[inline(always)]
    fn from(variant: Asintsupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASINTSUP` reader - Asynchronous Interrupt Support"]
pub type AsintsupR = crate::BitReader<Asintsupselect>;
impl AsintsupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asintsupselect {
        match self.bits {
            false => Asintsupselect::No,
            true => Asintsupselect::Yes,
        }
    }
    #[doc = "Asynchronous Interrupt not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Asintsupselect::No
    }
    #[doc = "Asynchronous Interrupt supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Asintsupselect::Yes
    }
}
#[doc = "Slot Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sltypeselect {
    #[doc = "0: Removable Card Slot"]
    Removable = 0,
    #[doc = "1: Embedded Slot for One Device"]
    Embedded = 1,
}
impl From<Sltypeselect> for u8 {
    #[inline(always)]
    fn from(variant: Sltypeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sltypeselect {
    type Ux = u8;
}
impl crate::IsEnum for Sltypeselect {}
#[doc = "Field `SLTYPE` reader - Slot Type"]
pub type SltypeR = crate::FieldReader<Sltypeselect>;
impl SltypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sltypeselect> {
        match self.bits {
            0 => Some(Sltypeselect::Removable),
            1 => Some(Sltypeselect::Embedded),
            _ => None,
        }
    }
    #[doc = "Removable Card Slot"]
    #[inline(always)]
    pub fn is_removable(&self) -> bool {
        *self == Sltypeselect::Removable
    }
    #[doc = "Embedded Slot for One Device"]
    #[inline(always)]
    pub fn is_embedded(&self) -> bool {
        *self == Sltypeselect::Embedded
    }
}
impl R {
    #[doc = "Bits 0:5 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn teoclkf(&self) -> TeoclkfR {
        TeoclkfR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Timeout Clock Unit"]
    #[inline(always)]
    pub fn teoclku(&self) -> TeoclkuR {
        TeoclkuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency"]
    #[inline(always)]
    pub fn baseclkf(&self) -> BaseclkfR {
        BaseclkfR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Max Block Length"]
    #[inline(always)]
    pub fn maxblkl(&self) -> MaxblklR {
        MaxblklR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 8-bit Support for Embedded Device"]
    #[inline(always)]
    pub fn ed8sup(&self) -> Ed8supR {
        Ed8supR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline(always)]
    pub fn adma2sup(&self) -> Adma2supR {
        Adma2supR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline(always)]
    pub fn hssup(&self) -> HssupR {
        HssupR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMA Support"]
    #[inline(always)]
    pub fn sdmasup(&self) -> SdmasupR {
        SdmasupR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Suspend/Resume Support"]
    #[inline(always)]
    pub fn srsup(&self) -> SrsupR {
        SrsupR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline(always)]
    pub fn v33vsup(&self) -> V33vsupR {
        V33vsupR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0V"]
    #[inline(always)]
    pub fn v30vsup(&self) -> V30vsupR {
        V30vsupR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8V"]
    #[inline(always)]
    pub fn v18vsup(&self) -> V18vsupR {
        V18vsupR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 64-Bit System Bus Support"]
    #[inline(always)]
    pub fn sb64sup(&self) -> Sb64supR {
        Sb64supR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn asintsup(&self) -> AsintsupR {
        AsintsupR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Slot Type"]
    #[inline(always)]
    pub fn sltype(&self) -> SltypeR {
        SltypeR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Capabilities 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ca0r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ca0rSpec;
impl crate::RegisterSpec for Ca0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ca0r::R`](R) reader structure"]
impl crate::Readable for Ca0rSpec {}
#[doc = "`reset()` method sets CA0R to value 0x27e8_0080"]
impl crate::Resettable for Ca0rSpec {
    const RESET_VALUE: u32 = 0x27e8_0080;
}
