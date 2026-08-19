#[doc = "Register `CA1R` reader"]
pub type R = crate::R<Ca1rSpec>;
#[doc = "SDR50 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdr50supselect {
    #[doc = "0: SDR50 is Not Supported"]
    No = 0,
    #[doc = "1: SDR50 is Supported"]
    Yes = 1,
}
impl From<Sdr50supselect> for bool {
    #[inline(always)]
    fn from(variant: Sdr50supselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR50SUP` reader - SDR50 Support"]
pub type Sdr50supR = crate::BitReader<Sdr50supselect>;
impl Sdr50supR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr50supselect {
        match self.bits {
            false => Sdr50supselect::No,
            true => Sdr50supselect::Yes,
        }
    }
    #[doc = "SDR50 is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Sdr50supselect::No
    }
    #[doc = "SDR50 is Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Sdr50supselect::Yes
    }
}
#[doc = "SDR104 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdr104supselect {
    #[doc = "0: SDR104 is Not Supported"]
    No = 0,
    #[doc = "1: SDR104 is Supported"]
    Yes = 1,
}
impl From<Sdr104supselect> for bool {
    #[inline(always)]
    fn from(variant: Sdr104supselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR104SUP` reader - SDR104 Support"]
pub type Sdr104supR = crate::BitReader<Sdr104supselect>;
impl Sdr104supR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr104supselect {
        match self.bits {
            false => Sdr104supselect::No,
            true => Sdr104supselect::Yes,
        }
    }
    #[doc = "SDR104 is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Sdr104supselect::No
    }
    #[doc = "SDR104 is Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Sdr104supselect::Yes
    }
}
#[doc = "DDR50 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddr50supselect {
    #[doc = "0: DDR50 is Not Supported"]
    No = 0,
    #[doc = "1: DDR50 is Supported"]
    Yes = 1,
}
impl From<Ddr50supselect> for bool {
    #[inline(always)]
    fn from(variant: Ddr50supselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR50SUP` reader - DDR50 Support"]
pub type Ddr50supR = crate::BitReader<Ddr50supselect>;
impl Ddr50supR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddr50supselect {
        match self.bits {
            false => Ddr50supselect::No,
            true => Ddr50supselect::Yes,
        }
    }
    #[doc = "DDR50 is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Ddr50supselect::No
    }
    #[doc = "DDR50 is Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Ddr50supselect::Yes
    }
}
#[doc = "Driver Type A Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drvasupselect {
    #[doc = "0: Driver Type A is Not Supported"]
    No = 0,
    #[doc = "1: Driver Type A is Supported"]
    Yes = 1,
}
impl From<Drvasupselect> for bool {
    #[inline(always)]
    fn from(variant: Drvasupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRVASUP` reader - Driver Type A Support"]
pub type DrvasupR = crate::BitReader<Drvasupselect>;
impl DrvasupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drvasupselect {
        match self.bits {
            false => Drvasupselect::No,
            true => Drvasupselect::Yes,
        }
    }
    #[doc = "Driver Type A is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Drvasupselect::No
    }
    #[doc = "Driver Type A is Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Drvasupselect::Yes
    }
}
#[doc = "Driver Type C Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drvcsupselect {
    #[doc = "0: Driver Type C is Not Supported"]
    No = 0,
    #[doc = "1: Driver Type C is Supported"]
    Yes = 1,
}
impl From<Drvcsupselect> for bool {
    #[inline(always)]
    fn from(variant: Drvcsupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRVCSUP` reader - Driver Type C Support"]
pub type DrvcsupR = crate::BitReader<Drvcsupselect>;
impl DrvcsupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drvcsupselect {
        match self.bits {
            false => Drvcsupselect::No,
            true => Drvcsupselect::Yes,
        }
    }
    #[doc = "Driver Type C is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Drvcsupselect::No
    }
    #[doc = "Driver Type C is Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Drvcsupselect::Yes
    }
}
#[doc = "Driver Type D Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drvdsupselect {
    #[doc = "0: Driver Type D is Not Supported"]
    No = 0,
    #[doc = "1: Driver Type D is Supported"]
    Yes = 1,
}
impl From<Drvdsupselect> for bool {
    #[inline(always)]
    fn from(variant: Drvdsupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRVDSUP` reader - Driver Type D Support"]
pub type DrvdsupR = crate::BitReader<Drvdsupselect>;
impl DrvdsupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drvdsupselect {
        match self.bits {
            false => Drvdsupselect::No,
            true => Drvdsupselect::Yes,
        }
    }
    #[doc = "Driver Type D is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Drvdsupselect::No
    }
    #[doc = "Driver Type D is Supported"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Drvdsupselect::Yes
    }
}
#[doc = "Timer Count for Re-Tuning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcntrtselect {
    #[doc = "0: Re-Tuning Timer disabled"]
    Disabled = 0,
    #[doc = "1: 1 second"]
    _1s = 1,
    #[doc = "2: 2 seconds"]
    _2s = 2,
    #[doc = "3: 4 seconds"]
    _4s = 3,
    #[doc = "4: 8 seconds"]
    _8s = 4,
    #[doc = "5: 16 seconds"]
    _16s = 5,
    #[doc = "6: 32 seconds"]
    _32s = 6,
    #[doc = "7: 64 seconds"]
    _64s = 7,
    #[doc = "8: 128 seconds"]
    _128s = 8,
    #[doc = "9: 256 seconds"]
    _256s = 9,
    #[doc = "10: 512 seconds"]
    _512s = 10,
    #[doc = "11: 1024 seconds"]
    _1024s = 11,
    #[doc = "15: Get information from other source"]
    Other = 15,
}
impl From<Tcntrtselect> for u8 {
    #[inline(always)]
    fn from(variant: Tcntrtselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcntrtselect {
    type Ux = u8;
}
impl crate::IsEnum for Tcntrtselect {}
#[doc = "Field `TCNTRT` reader - Timer Count for Re-Tuning"]
pub type TcntrtR = crate::FieldReader<Tcntrtselect>;
impl TcntrtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tcntrtselect> {
        match self.bits {
            0 => Some(Tcntrtselect::Disabled),
            1 => Some(Tcntrtselect::_1s),
            2 => Some(Tcntrtselect::_2s),
            3 => Some(Tcntrtselect::_4s),
            4 => Some(Tcntrtselect::_8s),
            5 => Some(Tcntrtselect::_16s),
            6 => Some(Tcntrtselect::_32s),
            7 => Some(Tcntrtselect::_64s),
            8 => Some(Tcntrtselect::_128s),
            9 => Some(Tcntrtselect::_256s),
            10 => Some(Tcntrtselect::_512s),
            11 => Some(Tcntrtselect::_1024s),
            15 => Some(Tcntrtselect::Other),
            _ => None,
        }
    }
    #[doc = "Re-Tuning Timer disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tcntrtselect::Disabled
    }
    #[doc = "1 second"]
    #[inline(always)]
    pub fn is_1s(&self) -> bool {
        *self == Tcntrtselect::_1s
    }
    #[doc = "2 seconds"]
    #[inline(always)]
    pub fn is_2s(&self) -> bool {
        *self == Tcntrtselect::_2s
    }
    #[doc = "4 seconds"]
    #[inline(always)]
    pub fn is_4s(&self) -> bool {
        *self == Tcntrtselect::_4s
    }
    #[doc = "8 seconds"]
    #[inline(always)]
    pub fn is_8s(&self) -> bool {
        *self == Tcntrtselect::_8s
    }
    #[doc = "16 seconds"]
    #[inline(always)]
    pub fn is_16s(&self) -> bool {
        *self == Tcntrtselect::_16s
    }
    #[doc = "32 seconds"]
    #[inline(always)]
    pub fn is_32s(&self) -> bool {
        *self == Tcntrtselect::_32s
    }
    #[doc = "64 seconds"]
    #[inline(always)]
    pub fn is_64s(&self) -> bool {
        *self == Tcntrtselect::_64s
    }
    #[doc = "128 seconds"]
    #[inline(always)]
    pub fn is_128s(&self) -> bool {
        *self == Tcntrtselect::_128s
    }
    #[doc = "256 seconds"]
    #[inline(always)]
    pub fn is_256s(&self) -> bool {
        *self == Tcntrtselect::_256s
    }
    #[doc = "512 seconds"]
    #[inline(always)]
    pub fn is_512s(&self) -> bool {
        *self == Tcntrtselect::_512s
    }
    #[doc = "1024 seconds"]
    #[inline(always)]
    pub fn is_1024s(&self) -> bool {
        *self == Tcntrtselect::_1024s
    }
    #[doc = "Get information from other source"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == Tcntrtselect::Other
    }
}
#[doc = "Use Tuning for SDR50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsdr50select {
    #[doc = "0: SDR50 does not require tuning"]
    No = 0,
    #[doc = "1: SDR50 requires tuning"]
    Yes = 1,
}
impl From<Tsdr50select> for bool {
    #[inline(always)]
    fn from(variant: Tsdr50select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSDR50` reader - Use Tuning for SDR50"]
pub type Tsdr50R = crate::BitReader<Tsdr50select>;
impl Tsdr50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsdr50select {
        match self.bits {
            false => Tsdr50select::No,
            true => Tsdr50select::Yes,
        }
    }
    #[doc = "SDR50 does not require tuning"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Tsdr50select::No
    }
    #[doc = "SDR50 requires tuning"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Tsdr50select::Yes
    }
}
#[doc = "Clock Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkmultselect {
    #[doc = "0: Clock Multiplier is Not Supported"]
    No = 0,
}
impl From<Clkmultselect> for u8 {
    #[inline(always)]
    fn from(variant: Clkmultselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkmultselect {
    type Ux = u8;
}
impl crate::IsEnum for Clkmultselect {}
#[doc = "Field `CLKMULT` reader - Clock Multiplier"]
pub type ClkmultR = crate::FieldReader<Clkmultselect>;
impl ClkmultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkmultselect> {
        match self.bits {
            0 => Some(Clkmultselect::No),
            _ => None,
        }
    }
    #[doc = "Clock Multiplier is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Clkmultselect::No
    }
}
impl R {
    #[doc = "Bit 0 - SDR50 Support"]
    #[inline(always)]
    pub fn sdr50sup(&self) -> Sdr50supR {
        Sdr50supR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support"]
    #[inline(always)]
    pub fn sdr104sup(&self) -> Sdr104supR {
        Sdr104supR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support"]
    #[inline(always)]
    pub fn ddr50sup(&self) -> Ddr50supR {
        Ddr50supR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Driver Type A Support"]
    #[inline(always)]
    pub fn drvasup(&self) -> DrvasupR {
        DrvasupR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support"]
    #[inline(always)]
    pub fn drvcsup(&self) -> DrvcsupR {
        DrvcsupR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support"]
    #[inline(always)]
    pub fn drvdsup(&self) -> DrvdsupR {
        DrvdsupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Timer Count for Re-Tuning"]
    #[inline(always)]
    pub fn tcntrt(&self) -> TcntrtR {
        TcntrtR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    pub fn tsdr50(&self) -> Tsdr50R {
        Tsdr50R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline(always)]
    pub fn clkmult(&self) -> ClkmultR {
        ClkmultR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Capabilities 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ca1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ca1rSpec;
impl crate::RegisterSpec for Ca1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ca1r::R`](R) reader structure"]
impl crate::Readable for Ca1rSpec {}
#[doc = "`reset()` method sets CA1R to value 0x70"]
impl crate::Resettable for Ca1rSpec {
    const RESET_VALUE: u32 = 0x70;
}
