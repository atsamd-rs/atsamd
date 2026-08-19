#[doc = "Register `EISIER_EMMC_MODE` reader"]
pub type R = crate::R<EisierEmmcModeSpec>;
#[doc = "Register `EISIER_EMMC_MODE` writer"]
pub type W = crate::W<EisierEmmcModeSpec>;
#[doc = "Command Timeout Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdteoselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cmdteoselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdteoselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDTEO` reader - Command Timeout Error Signal Enable"]
pub type CmdteoR = crate::BitReader<Cmdteoselect>;
impl CmdteoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdteoselect {
        match self.bits {
            false => Cmdteoselect::Masked,
            true => Cmdteoselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cmdteoselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmdteoselect::Enabled
    }
}
#[doc = "Field `CMDTEO` writer - Command Timeout Error Signal Enable"]
pub type CmdteoW<'a, REG> = crate::BitWriter<'a, REG, Cmdteoselect>;
impl<'a, REG> CmdteoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdteoselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdteoselect::Enabled)
    }
}
#[doc = "Command CRC Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdcrcselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cmdcrcselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdcrcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDCRC` reader - Command CRC Error Signal Enable"]
pub type CmdcrcR = crate::BitReader<Cmdcrcselect>;
impl CmdcrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdcrcselect {
        match self.bits {
            false => Cmdcrcselect::Masked,
            true => Cmdcrcselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cmdcrcselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmdcrcselect::Enabled
    }
}
#[doc = "Field `CMDCRC` writer - Command CRC Error Signal Enable"]
pub type CmdcrcW<'a, REG> = crate::BitWriter<'a, REG, Cmdcrcselect>;
impl<'a, REG> CmdcrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcselect::Enabled)
    }
}
#[doc = "Command End Bit Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdendselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cmdendselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdendselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDEND` reader - Command End Bit Error Signal Enable"]
pub type CmdendR = crate::BitReader<Cmdendselect>;
impl CmdendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdendselect {
        match self.bits {
            false => Cmdendselect::Masked,
            true => Cmdendselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cmdendselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmdendselect::Enabled
    }
}
#[doc = "Field `CMDEND` writer - Command End Bit Error Signal Enable"]
pub type CmdendW<'a, REG> = crate::BitWriter<'a, REG, Cmdendselect>;
impl<'a, REG> CmdendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendselect::Enabled)
    }
}
#[doc = "Command Index Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdidxselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cmdidxselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdidxselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDIDX` reader - Command Index Error Signal Enable"]
pub type CmdidxR = crate::BitReader<Cmdidxselect>;
impl CmdidxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdidxselect {
        match self.bits {
            false => Cmdidxselect::Masked,
            true => Cmdidxselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cmdidxselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmdidxselect::Enabled
    }
}
#[doc = "Field `CMDIDX` writer - Command Index Error Signal Enable"]
pub type CmdidxW<'a, REG> = crate::BitWriter<'a, REG, Cmdidxselect>;
impl<'a, REG> CmdidxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidxselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidxselect::Enabled)
    }
}
#[doc = "Data Timeout Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datteoselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Datteoselect> for bool {
    #[inline(always)]
    fn from(variant: Datteoselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATTEO` reader - Data Timeout Error Signal Enable"]
pub type DatteoR = crate::BitReader<Datteoselect>;
impl DatteoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datteoselect {
        match self.bits {
            false => Datteoselect::Masked,
            true => Datteoselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Datteoselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Datteoselect::Enabled
    }
}
#[doc = "Field `DATTEO` writer - Data Timeout Error Signal Enable"]
pub type DatteoW<'a, REG> = crate::BitWriter<'a, REG, Datteoselect>;
impl<'a, REG> DatteoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Datteoselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Datteoselect::Enabled)
    }
}
#[doc = "Data CRC Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datcrcselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Datcrcselect> for bool {
    #[inline(always)]
    fn from(variant: Datcrcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATCRC` reader - Data CRC Error Signal Enable"]
pub type DatcrcR = crate::BitReader<Datcrcselect>;
impl DatcrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datcrcselect {
        match self.bits {
            false => Datcrcselect::Masked,
            true => Datcrcselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Datcrcselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Datcrcselect::Enabled
    }
}
#[doc = "Field `DATCRC` writer - Data CRC Error Signal Enable"]
pub type DatcrcW<'a, REG> = crate::BitWriter<'a, REG, Datcrcselect>;
impl<'a, REG> DatcrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Datcrcselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Datcrcselect::Enabled)
    }
}
#[doc = "Data End Bit Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datendselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Datendselect> for bool {
    #[inline(always)]
    fn from(variant: Datendselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATEND` reader - Data End Bit Error Signal Enable"]
pub type DatendR = crate::BitReader<Datendselect>;
impl DatendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datendselect {
        match self.bits {
            false => Datendselect::Masked,
            true => Datendselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Datendselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Datendselect::Enabled
    }
}
#[doc = "Field `DATEND` writer - Data End Bit Error Signal Enable"]
pub type DatendW<'a, REG> = crate::BitWriter<'a, REG, Datendselect>;
impl<'a, REG> DatendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Datendselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Datendselect::Enabled)
    }
}
#[doc = "Current Limit Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Curlimselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Curlimselect> for bool {
    #[inline(always)]
    fn from(variant: Curlimselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURLIM` reader - Current Limit Error Signal Enable"]
pub type CurlimR = crate::BitReader<Curlimselect>;
impl CurlimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Curlimselect {
        match self.bits {
            false => Curlimselect::Masked,
            true => Curlimselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Curlimselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Curlimselect::Enabled
    }
}
#[doc = "Field `CURLIM` writer - Current Limit Error Signal Enable"]
pub type CurlimW<'a, REG> = crate::BitWriter<'a, REG, Curlimselect>;
impl<'a, REG> CurlimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Curlimselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Curlimselect::Enabled)
    }
}
#[doc = "Auto CMD Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Acmdselect> for bool {
    #[inline(always)]
    fn from(variant: Acmdselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD` reader - Auto CMD Error Signal Enable"]
pub type AcmdR = crate::BitReader<Acmdselect>;
impl AcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmdselect {
        match self.bits {
            false => Acmdselect::Masked,
            true => Acmdselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Acmdselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Acmdselect::Enabled
    }
}
#[doc = "Field `ACMD` writer - Auto CMD Error Signal Enable"]
pub type AcmdW<'a, REG> = crate::BitWriter<'a, REG, Acmdselect>;
impl<'a, REG> AcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdselect::Enabled)
    }
}
#[doc = "ADMA Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admaselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Admaselect> for bool {
    #[inline(always)]
    fn from(variant: Admaselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMA` reader - ADMA Error Signal Enable"]
pub type AdmaR = crate::BitReader<Admaselect>;
impl AdmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admaselect {
        match self.bits {
            false => Admaselect::Masked,
            true => Admaselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Admaselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Admaselect::Enabled
    }
}
#[doc = "Field `ADMA` writer - ADMA Error Signal Enable"]
pub type AdmaW<'a, REG> = crate::BitWriter<'a, REG, Admaselect>;
impl<'a, REG> AdmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Admaselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Admaselect::Enabled)
    }
}
#[doc = "Field `BOOTAE` reader - Boot Acknowledge Error Signal Enable"]
pub type BootaeR = crate::BitReader;
#[doc = "Field `BOOTAE` writer - Boot Acknowledge Error Signal Enable"]
pub type BootaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn cmdteo(&self) -> CmdteoR {
        CmdteoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable"]
    #[inline(always)]
    pub fn cmdcrc(&self) -> CmdcrcR {
        CmdcrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn cmdend(&self) -> CmdendR {
        CmdendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CmdidxR {
        CmdidxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn datteo(&self) -> DatteoR {
        DatteoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable"]
    #[inline(always)]
    pub fn datcrc(&self) -> DatcrcR {
        DatcrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn datend(&self) -> DatendR {
        DatendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable"]
    #[inline(always)]
    pub fn curlim(&self) -> CurlimR {
        CurlimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable"]
    #[inline(always)]
    pub fn acmd(&self) -> AcmdR {
        AcmdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable"]
    #[inline(always)]
    pub fn adma(&self) -> AdmaR {
        AdmaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Boot Acknowledge Error Signal Enable"]
    #[inline(always)]
    pub fn bootae(&self) -> BootaeR {
        BootaeR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdteo(&mut self) -> CmdteoW<EisierEmmcModeSpec> {
        CmdteoW::new(self, 0)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrc(&mut self) -> CmdcrcW<EisierEmmcModeSpec> {
        CmdcrcW::new(self, 1)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdend(&mut self) -> CmdendW<EisierEmmcModeSpec> {
        CmdendW::new(self, 2)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdidx(&mut self) -> CmdidxW<EisierEmmcModeSpec> {
        CmdidxW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datteo(&mut self) -> DatteoW<EisierEmmcModeSpec> {
        DatteoW::new(self, 4)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datcrc(&mut self) -> DatcrcW<EisierEmmcModeSpec> {
        DatcrcW::new(self, 5)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datend(&mut self) -> DatendW<EisierEmmcModeSpec> {
        DatendW::new(self, 6)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn curlim(&mut self) -> CurlimW<EisierEmmcModeSpec> {
        CurlimW::new(self, 7)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmd(&mut self) -> AcmdW<EisierEmmcModeSpec> {
        AcmdW::new(self, 8)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adma(&mut self) -> AdmaW<EisierEmmcModeSpec> {
        AdmaW::new(self, 9)
    }
    #[doc = "Bit 12 - Boot Acknowledge Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bootae(&mut self) -> BootaeW<EisierEmmcModeSpec> {
        BootaeW::new(self, 12)
    }
}
#[doc = "Error Interrupt Signal Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eisier_emmc_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eisier_emmc_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EisierEmmcModeSpec;
impl crate::RegisterSpec for EisierEmmcModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eisier_emmc_mode::R`](R) reader structure"]
impl crate::Readable for EisierEmmcModeSpec {}
#[doc = "`write(|w| ..)` method takes [`eisier_emmc_mode::W`](W) writer structure"]
impl crate::Writable for EisierEmmcModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EISIER_EMMC_MODE to value 0"]
impl crate::Resettable for EisierEmmcModeSpec {
    const RESET_VALUE: u16 = 0;
}
