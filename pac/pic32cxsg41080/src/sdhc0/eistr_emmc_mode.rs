#[doc = "Register `EISTR_EMMC_MODE` reader"]
pub type R = crate::R<EistrEmmcModeSpec>;
#[doc = "Register `EISTR_EMMC_MODE` writer"]
pub type W = crate::W<EistrEmmcModeSpec>;
#[doc = "Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdteoselect {
    #[doc = "0: No Error"]
    No = 0,
    #[doc = "1: Timeout"]
    Yes = 1,
}
impl From<Cmdteoselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdteoselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDTEO` reader - Command Timeout Error"]
pub type CmdteoR = crate::BitReader<Cmdteoselect>;
impl CmdteoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdteoselect {
        match self.bits {
            false => Cmdteoselect::No,
            true => Cmdteoselect::Yes,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cmdteoselect::No
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cmdteoselect::Yes
    }
}
#[doc = "Field `CMDTEO` writer - Command Timeout Error"]
pub type CmdteoW<'a, REG> = crate::BitWriter<'a, REG, Cmdteoselect>;
impl<'a, REG> CmdteoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdteoselect::No)
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdteoselect::Yes)
    }
}
#[doc = "Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdcrcselect {
    #[doc = "0: No Error"]
    No = 0,
    #[doc = "1: CRC Error Generated"]
    Yes = 1,
}
impl From<Cmdcrcselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdcrcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDCRC` reader - Command CRC Error"]
pub type CmdcrcR = crate::BitReader<Cmdcrcselect>;
impl CmdcrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdcrcselect {
        match self.bits {
            false => Cmdcrcselect::No,
            true => Cmdcrcselect::Yes,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cmdcrcselect::No
    }
    #[doc = "CRC Error Generated"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cmdcrcselect::Yes
    }
}
#[doc = "Field `CMDCRC` writer - Command CRC Error"]
pub type CmdcrcW<'a, REG> = crate::BitWriter<'a, REG, Cmdcrcselect>;
impl<'a, REG> CmdcrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcselect::No)
    }
    #[doc = "CRC Error Generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcselect::Yes)
    }
}
#[doc = "Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdendselect {
    #[doc = "0: No error"]
    No = 0,
    #[doc = "1: End Bit Error Generated"]
    Yes = 1,
}
impl From<Cmdendselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdendselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDEND` reader - Command End Bit Error"]
pub type CmdendR = crate::BitReader<Cmdendselect>;
impl CmdendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdendselect {
        match self.bits {
            false => Cmdendselect::No,
            true => Cmdendselect::Yes,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cmdendselect::No
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cmdendselect::Yes
    }
}
#[doc = "Field `CMDEND` writer - Command End Bit Error"]
pub type CmdendW<'a, REG> = crate::BitWriter<'a, REG, Cmdendselect>;
impl<'a, REG> CmdendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendselect::No)
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendselect::Yes)
    }
}
#[doc = "Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdidxselect {
    #[doc = "0: No Error"]
    No = 0,
    #[doc = "1: Error"]
    Yes = 1,
}
impl From<Cmdidxselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdidxselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDIDX` reader - Command Index Error"]
pub type CmdidxR = crate::BitReader<Cmdidxselect>;
impl CmdidxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdidxselect {
        match self.bits {
            false => Cmdidxselect::No,
            true => Cmdidxselect::Yes,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cmdidxselect::No
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cmdidxselect::Yes
    }
}
#[doc = "Field `CMDIDX` writer - Command Index Error"]
pub type CmdidxW<'a, REG> = crate::BitWriter<'a, REG, Cmdidxselect>;
impl<'a, REG> CmdidxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidxselect::No)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidxselect::Yes)
    }
}
#[doc = "Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datteoselect {
    #[doc = "0: No Error"]
    No = 0,
    #[doc = "1: Timeout"]
    Yes = 1,
}
impl From<Datteoselect> for bool {
    #[inline(always)]
    fn from(variant: Datteoselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATTEO` reader - Data Timeout Error"]
pub type DatteoR = crate::BitReader<Datteoselect>;
impl DatteoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datteoselect {
        match self.bits {
            false => Datteoselect::No,
            true => Datteoselect::Yes,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Datteoselect::No
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Datteoselect::Yes
    }
}
#[doc = "Field `DATTEO` writer - Data Timeout Error"]
pub type DatteoW<'a, REG> = crate::BitWriter<'a, REG, Datteoselect>;
impl<'a, REG> DatteoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Datteoselect::No)
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Datteoselect::Yes)
    }
}
#[doc = "Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datcrcselect {
    #[doc = "0: No Error"]
    No = 0,
    #[doc = "1: Error"]
    Yes = 1,
}
impl From<Datcrcselect> for bool {
    #[inline(always)]
    fn from(variant: Datcrcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATCRC` reader - Data CRC Error"]
pub type DatcrcR = crate::BitReader<Datcrcselect>;
impl DatcrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datcrcselect {
        match self.bits {
            false => Datcrcselect::No,
            true => Datcrcselect::Yes,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Datcrcselect::No
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Datcrcselect::Yes
    }
}
#[doc = "Field `DATCRC` writer - Data CRC Error"]
pub type DatcrcW<'a, REG> = crate::BitWriter<'a, REG, Datcrcselect>;
impl<'a, REG> DatcrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Datcrcselect::No)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Datcrcselect::Yes)
    }
}
#[doc = "Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datendselect {
    #[doc = "0: No Error"]
    No = 0,
    #[doc = "1: Error"]
    Yes = 1,
}
impl From<Datendselect> for bool {
    #[inline(always)]
    fn from(variant: Datendselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATEND` reader - Data End Bit Error"]
pub type DatendR = crate::BitReader<Datendselect>;
impl DatendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datendselect {
        match self.bits {
            false => Datendselect::No,
            true => Datendselect::Yes,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Datendselect::No
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Datendselect::Yes
    }
}
#[doc = "Field `DATEND` writer - Data End Bit Error"]
pub type DatendW<'a, REG> = crate::BitWriter<'a, REG, Datendselect>;
impl<'a, REG> DatendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Datendselect::No)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Datendselect::Yes)
    }
}
#[doc = "Current Limit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Curlimselect {
    #[doc = "0: No Error"]
    No = 0,
    #[doc = "1: Power Fail"]
    Yes = 1,
}
impl From<Curlimselect> for bool {
    #[inline(always)]
    fn from(variant: Curlimselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURLIM` reader - Current Limit Error"]
pub type CurlimR = crate::BitReader<Curlimselect>;
impl CurlimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Curlimselect {
        match self.bits {
            false => Curlimselect::No,
            true => Curlimselect::Yes,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Curlimselect::No
    }
    #[doc = "Power Fail"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Curlimselect::Yes
    }
}
#[doc = "Field `CURLIM` writer - Current Limit Error"]
pub type CurlimW<'a, REG> = crate::BitWriter<'a, REG, Curlimselect>;
impl<'a, REG> CurlimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Curlimselect::No)
    }
    #[doc = "Power Fail"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Curlimselect::Yes)
    }
}
#[doc = "Auto CMD Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdselect {
    #[doc = "0: No Error"]
    No = 0,
    #[doc = "1: Error"]
    Yes = 1,
}
impl From<Acmdselect> for bool {
    #[inline(always)]
    fn from(variant: Acmdselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD` reader - Auto CMD Error"]
pub type AcmdR = crate::BitReader<Acmdselect>;
impl AcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmdselect {
        match self.bits {
            false => Acmdselect::No,
            true => Acmdselect::Yes,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Acmdselect::No
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Acmdselect::Yes
    }
}
#[doc = "Field `ACMD` writer - Auto CMD Error"]
pub type AcmdW<'a, REG> = crate::BitWriter<'a, REG, Acmdselect>;
impl<'a, REG> AcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdselect::No)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdselect::Yes)
    }
}
#[doc = "ADMA Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admaselect {
    #[doc = "0: No Error"]
    No = 0,
    #[doc = "1: Error"]
    Yes = 1,
}
impl From<Admaselect> for bool {
    #[inline(always)]
    fn from(variant: Admaselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMA` reader - ADMA Error"]
pub type AdmaR = crate::BitReader<Admaselect>;
impl AdmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admaselect {
        match self.bits {
            false => Admaselect::No,
            true => Admaselect::Yes,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Admaselect::No
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Admaselect::Yes
    }
}
#[doc = "Field `ADMA` writer - ADMA Error"]
pub type AdmaW<'a, REG> = crate::BitWriter<'a, REG, Admaselect>;
impl<'a, REG> AdmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Admaselect::No)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Admaselect::Yes)
    }
}
#[doc = "Boot Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootaeselect {
    #[doc = "0: FIFO contains at least one byte"]
    Fifonotempty = 0,
    #[doc = "1: FIFO is empty"]
    Fifoempty = 1,
}
impl From<Bootaeselect> for bool {
    #[inline(always)]
    fn from(variant: Bootaeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTAE` reader - Boot Acknowledge Error"]
pub type BootaeR = crate::BitReader<Bootaeselect>;
impl BootaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootaeselect {
        match self.bits {
            false => Bootaeselect::Fifonotempty,
            true => Bootaeselect::Fifoempty,
        }
    }
    #[doc = "FIFO contains at least one byte"]
    #[inline(always)]
    pub fn is_fifonotempty(&self) -> bool {
        *self == Bootaeselect::Fifonotempty
    }
    #[doc = "FIFO is empty"]
    #[inline(always)]
    pub fn is_fifoempty(&self) -> bool {
        *self == Bootaeselect::Fifoempty
    }
}
#[doc = "Field `BOOTAE` writer - Boot Acknowledge Error"]
pub type BootaeW<'a, REG> = crate::BitWriter<'a, REG, Bootaeselect>;
impl<'a, REG> BootaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO contains at least one byte"]
    #[inline(always)]
    pub fn fifonotempty(self) -> &'a mut crate::W<REG> {
        self.variant(Bootaeselect::Fifonotempty)
    }
    #[doc = "FIFO is empty"]
    #[inline(always)]
    pub fn fifoempty(self) -> &'a mut crate::W<REG> {
        self.variant(Bootaeselect::Fifoempty)
    }
}
impl R {
    #[doc = "Bit 0 - Command Timeout Error"]
    #[inline(always)]
    pub fn cmdteo(&self) -> CmdteoR {
        CmdteoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error"]
    #[inline(always)]
    pub fn cmdcrc(&self) -> CmdcrcR {
        CmdcrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error"]
    #[inline(always)]
    pub fn cmdend(&self) -> CmdendR {
        CmdendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CmdidxR {
        CmdidxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error"]
    #[inline(always)]
    pub fn datteo(&self) -> DatteoR {
        DatteoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error"]
    #[inline(always)]
    pub fn datcrc(&self) -> DatcrcR {
        DatcrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error"]
    #[inline(always)]
    pub fn datend(&self) -> DatendR {
        DatendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error"]
    #[inline(always)]
    pub fn curlim(&self) -> CurlimR {
        CurlimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error"]
    #[inline(always)]
    pub fn acmd(&self) -> AcmdR {
        AcmdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADMA Error"]
    #[inline(always)]
    pub fn adma(&self) -> AdmaR {
        AdmaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Boot Acknowledge Error"]
    #[inline(always)]
    pub fn bootae(&self) -> BootaeR {
        BootaeR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdteo(&mut self) -> CmdteoW<EistrEmmcModeSpec> {
        CmdteoW::new(self, 0)
    }
    #[doc = "Bit 1 - Command CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrc(&mut self) -> CmdcrcW<EistrEmmcModeSpec> {
        CmdcrcW::new(self, 1)
    }
    #[doc = "Bit 2 - Command End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdend(&mut self) -> CmdendW<EistrEmmcModeSpec> {
        CmdendW::new(self, 2)
    }
    #[doc = "Bit 3 - Command Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdidx(&mut self) -> CmdidxW<EistrEmmcModeSpec> {
        CmdidxW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn datteo(&mut self) -> DatteoW<EistrEmmcModeSpec> {
        DatteoW::new(self, 4)
    }
    #[doc = "Bit 5 - Data CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn datcrc(&mut self) -> DatcrcW<EistrEmmcModeSpec> {
        DatcrcW::new(self, 5)
    }
    #[doc = "Bit 6 - Data End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn datend(&mut self) -> DatendW<EistrEmmcModeSpec> {
        DatendW::new(self, 6)
    }
    #[doc = "Bit 7 - Current Limit Error"]
    #[inline(always)]
    #[must_use]
    pub fn curlim(&mut self) -> CurlimW<EistrEmmcModeSpec> {
        CurlimW::new(self, 7)
    }
    #[doc = "Bit 8 - Auto CMD Error"]
    #[inline(always)]
    #[must_use]
    pub fn acmd(&mut self) -> AcmdW<EistrEmmcModeSpec> {
        AcmdW::new(self, 8)
    }
    #[doc = "Bit 9 - ADMA Error"]
    #[inline(always)]
    #[must_use]
    pub fn adma(&mut self) -> AdmaW<EistrEmmcModeSpec> {
        AdmaW::new(self, 9)
    }
    #[doc = "Bit 12 - Boot Acknowledge Error"]
    #[inline(always)]
    #[must_use]
    pub fn bootae(&mut self) -> BootaeW<EistrEmmcModeSpec> {
        BootaeW::new(self, 12)
    }
}
#[doc = "Error Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`eistr_emmc_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eistr_emmc_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EistrEmmcModeSpec;
impl crate::RegisterSpec for EistrEmmcModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eistr_emmc_mode::R`](R) reader structure"]
impl crate::Readable for EistrEmmcModeSpec {}
#[doc = "`write(|w| ..)` method takes [`eistr_emmc_mode::W`](W) writer structure"]
impl crate::Writable for EistrEmmcModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EISTR_EMMC_MODE to value 0"]
impl crate::Resettable for EistrEmmcModeSpec {
    const RESET_VALUE: u16 = 0;
}
