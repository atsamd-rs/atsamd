#[doc = "Register `PSR` reader"]
pub type R = crate::R<PsrSpec>;
#[doc = "Command Inhibit (CMD)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdinhcselect {
    #[doc = "0: Can issue command using only CMD line"]
    Can = 0,
    #[doc = "1: Cannot issue command"]
    Cannot = 1,
}
impl From<Cmdinhcselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdinhcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDINHC` reader - Command Inhibit (CMD)"]
pub type CmdinhcR = crate::BitReader<Cmdinhcselect>;
impl CmdinhcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdinhcselect {
        match self.bits {
            false => Cmdinhcselect::Can,
            true => Cmdinhcselect::Cannot,
        }
    }
    #[doc = "Can issue command using only CMD line"]
    #[inline(always)]
    pub fn is_can(&self) -> bool {
        *self == Cmdinhcselect::Can
    }
    #[doc = "Cannot issue command"]
    #[inline(always)]
    pub fn is_cannot(&self) -> bool {
        *self == Cmdinhcselect::Cannot
    }
}
#[doc = "Command Inhibit (DAT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdinhdselect {
    #[doc = "0: Can issue command which uses the DAT line"]
    Can = 0,
    #[doc = "1: Cannot issue command which uses the DAT line"]
    Cannot = 1,
}
impl From<Cmdinhdselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdinhdselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDINHD` reader - Command Inhibit (DAT)"]
pub type CmdinhdR = crate::BitReader<Cmdinhdselect>;
impl CmdinhdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdinhdselect {
        match self.bits {
            false => Cmdinhdselect::Can,
            true => Cmdinhdselect::Cannot,
        }
    }
    #[doc = "Can issue command which uses the DAT line"]
    #[inline(always)]
    pub fn is_can(&self) -> bool {
        *self == Cmdinhdselect::Can
    }
    #[doc = "Cannot issue command which uses the DAT line"]
    #[inline(always)]
    pub fn is_cannot(&self) -> bool {
        *self == Cmdinhdselect::Cannot
    }
}
#[doc = "DAT Line Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlactselect {
    #[doc = "0: DAT Line Inactive"]
    Inactive = 0,
    #[doc = "1: DAT Line Active"]
    Active = 1,
}
impl From<Dlactselect> for bool {
    #[inline(always)]
    fn from(variant: Dlactselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLACT` reader - DAT Line Active"]
pub type DlactR = crate::BitReader<Dlactselect>;
impl DlactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlactselect {
        match self.bits {
            false => Dlactselect::Inactive,
            true => Dlactselect::Active,
        }
    }
    #[doc = "DAT Line Inactive"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Dlactselect::Inactive
    }
    #[doc = "DAT Line Active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Dlactselect::Active
    }
}
#[doc = "Re-Tuning Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtreqselect {
    #[doc = "0: Fixed or well-tuned sampling clock"]
    Ok = 0,
    #[doc = "1: Sampling clock needs re-tuning"]
    Required = 1,
}
impl From<Rtreqselect> for bool {
    #[inline(always)]
    fn from(variant: Rtreqselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTREQ` reader - Re-Tuning Request"]
pub type RtreqR = crate::BitReader<Rtreqselect>;
impl RtreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtreqselect {
        match self.bits {
            false => Rtreqselect::Ok,
            true => Rtreqselect::Required,
        }
    }
    #[doc = "Fixed or well-tuned sampling clock"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == Rtreqselect::Ok
    }
    #[doc = "Sampling clock needs re-tuning"]
    #[inline(always)]
    pub fn is_required(&self) -> bool {
        *self == Rtreqselect::Required
    }
}
#[doc = "Write Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wtactselect {
    #[doc = "0: No valid data"]
    No = 0,
    #[doc = "1: Transferring data"]
    Yes = 1,
}
impl From<Wtactselect> for bool {
    #[inline(always)]
    fn from(variant: Wtactselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTACT` reader - Write Transfer Active"]
pub type WtactR = crate::BitReader<Wtactselect>;
impl WtactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wtactselect {
        match self.bits {
            false => Wtactselect::No,
            true => Wtactselect::Yes,
        }
    }
    #[doc = "No valid data"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Wtactselect::No
    }
    #[doc = "Transferring data"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Wtactselect::Yes
    }
}
#[doc = "Read Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtactselect {
    #[doc = "0: No valid data"]
    No = 0,
    #[doc = "1: Transferring data"]
    Yes = 1,
}
impl From<Rtactselect> for bool {
    #[inline(always)]
    fn from(variant: Rtactselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTACT` reader - Read Transfer Active"]
pub type RtactR = crate::BitReader<Rtactselect>;
impl RtactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtactselect {
        match self.bits {
            false => Rtactselect::No,
            true => Rtactselect::Yes,
        }
    }
    #[doc = "No valid data"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Rtactselect::No
    }
    #[doc = "Transferring data"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Rtactselect::Yes
    }
}
#[doc = "Buffer Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufwrenselect {
    #[doc = "0: Write disable"]
    Disable = 0,
    #[doc = "1: Write enable"]
    Enable = 1,
}
impl From<Bufwrenselect> for bool {
    #[inline(always)]
    fn from(variant: Bufwrenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFWREN` reader - Buffer Write Enable"]
pub type BufwrenR = crate::BitReader<Bufwrenselect>;
impl BufwrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufwrenselect {
        match self.bits {
            false => Bufwrenselect::Disable,
            true => Bufwrenselect::Enable,
        }
    }
    #[doc = "Write disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bufwrenselect::Disable
    }
    #[doc = "Write enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bufwrenselect::Enable
    }
}
#[doc = "Buffer Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufrdenselect {
    #[doc = "0: Read disable"]
    Disable = 0,
    #[doc = "1: Read enable"]
    Enable = 1,
}
impl From<Bufrdenselect> for bool {
    #[inline(always)]
    fn from(variant: Bufrdenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFRDEN` reader - Buffer Read Enable"]
pub type BufrdenR = crate::BitReader<Bufrdenselect>;
impl BufrdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufrdenselect {
        match self.bits {
            false => Bufrdenselect::Disable,
            true => Bufrdenselect::Enable,
        }
    }
    #[doc = "Read disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bufrdenselect::Disable
    }
    #[doc = "Read enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bufrdenselect::Enable
    }
}
#[doc = "Card Inserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinsselect {
    #[doc = "0: Reset or Debouncing or No Card"]
    No = 0,
    #[doc = "1: Card inserted"]
    Yes = 1,
}
impl From<Cardinsselect> for bool {
    #[inline(always)]
    fn from(variant: Cardinsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDINS` reader - Card Inserted"]
pub type CardinsR = crate::BitReader<Cardinsselect>;
impl CardinsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardinsselect {
        match self.bits {
            false => Cardinsselect::No,
            true => Cardinsselect::Yes,
        }
    }
    #[doc = "Reset or Debouncing or No Card"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cardinsselect::No
    }
    #[doc = "Card inserted"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cardinsselect::Yes
    }
}
#[doc = "Card State Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardssselect {
    #[doc = "0: Reset or Debouncing"]
    No = 0,
    #[doc = "1: No Card or Insered"]
    Yes = 1,
}
impl From<Cardssselect> for bool {
    #[inline(always)]
    fn from(variant: Cardssselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDSS` reader - Card State Stable"]
pub type CardssR = crate::BitReader<Cardssselect>;
impl CardssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardssselect {
        match self.bits {
            false => Cardssselect::No,
            true => Cardssselect::Yes,
        }
    }
    #[doc = "Reset or Debouncing"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cardssselect::No
    }
    #[doc = "No Card or Insered"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cardssselect::Yes
    }
}
#[doc = "Card Detect Pin Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carddplselect {
    #[doc = "0: No card present (SDCD#=1)"]
    No = 0,
    #[doc = "1: Card present (SDCD#=0)"]
    Yes = 1,
}
impl From<Carddplselect> for bool {
    #[inline(always)]
    fn from(variant: Carddplselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDDPL` reader - Card Detect Pin Level"]
pub type CarddplR = crate::BitReader<Carddplselect>;
impl CarddplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Carddplselect {
        match self.bits {
            false => Carddplselect::No,
            true => Carddplselect::Yes,
        }
    }
    #[doc = "No card present (SDCD#=1)"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Carddplselect::No
    }
    #[doc = "Card present (SDCD#=0)"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Carddplselect::Yes
    }
}
#[doc = "Write Protect Pin Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrpplselect {
    #[doc = "0: Write protected (SDWP#=0)"]
    Protected = 0,
    #[doc = "1: Write enabled (SDWP#=1)"]
    Enabled = 1,
}
impl From<Wrpplselect> for bool {
    #[inline(always)]
    fn from(variant: Wrpplselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPPL` reader - Write Protect Pin Level"]
pub type WrpplR = crate::BitReader<Wrpplselect>;
impl WrpplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrpplselect {
        match self.bits {
            false => Wrpplselect::Protected,
            true => Wrpplselect::Enabled,
        }
    }
    #[doc = "Write protected (SDWP#=0)"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == Wrpplselect::Protected
    }
    #[doc = "Write enabled (SDWP#=1)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wrpplselect::Enabled
    }
}
#[doc = "Field `DATLL` reader - DAT\\[3:0\\] Line Level"]
pub type DatllR = crate::FieldReader;
#[doc = "Field `CMDLL` reader - CMD Line Level"]
pub type CmdllR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn cmdinhc(&self) -> CmdinhcR {
        CmdinhcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline(always)]
    pub fn cmdinhd(&self) -> CmdinhdR {
        CmdinhdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAT Line Active"]
    #[inline(always)]
    pub fn dlact(&self) -> DlactR {
        DlactR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Re-Tuning Request"]
    #[inline(always)]
    pub fn rtreq(&self) -> RtreqR {
        RtreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline(always)]
    pub fn wtact(&self) -> WtactR {
        WtactR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline(always)]
    pub fn rtact(&self) -> RtactR {
        RtactR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline(always)]
    pub fn bufwren(&self) -> BufwrenR {
        BufwrenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline(always)]
    pub fn bufrden(&self) -> BufrdenR {
        BufrdenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline(always)]
    pub fn cardins(&self) -> CardinsR {
        CardinsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Card State Stable"]
    #[inline(always)]
    pub fn cardss(&self) -> CardssR {
        CardssR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline(always)]
    pub fn carddpl(&self) -> CarddplR {
        CarddplR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write Protect Pin Level"]
    #[inline(always)]
    pub fn wrppl(&self) -> WrpplR {
        WrpplR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - DAT\\[3:0\\] Line Level"]
    #[inline(always)]
    pub fn datll(&self) -> DatllR {
        DatllR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - CMD Line Level"]
    #[inline(always)]
    pub fn cmdll(&self) -> CmdllR {
        CmdllR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Present State\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PsrSpec {}
#[doc = "`reset()` method sets PSR to value 0x00f8_0000"]
impl crate::Resettable for PsrSpec {
    const RESET_VALUE: u32 = 0x00f8_0000;
}
