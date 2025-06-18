#[doc = "Register `FEREIS` writer"]
pub type W = crate::W<FereisSpec>;
#[doc = "Force Event for Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdteoselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Cmdteoselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdteoselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDTEO` writer - Force Event for Command Timeout Error"]
pub type CmdteoW<'a, REG> = crate::BitWriter<'a, REG, Cmdteoselect>;
impl<'a, REG> CmdteoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdteoselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdteoselect::Yes)
    }
}
#[doc = "Force Event for Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdcrcselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Cmdcrcselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdcrcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDCRC` writer - Force Event for Command CRC Error"]
pub type CmdcrcW<'a, REG> = crate::BitWriter<'a, REG, Cmdcrcselect>;
impl<'a, REG> CmdcrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcselect::Yes)
    }
}
#[doc = "Force Event for Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdendselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Cmdendselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdendselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDEND` writer - Force Event for Command End Bit Error"]
pub type CmdendW<'a, REG> = crate::BitWriter<'a, REG, Cmdendselect>;
impl<'a, REG> CmdendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendselect::Yes)
    }
}
#[doc = "Force Event for Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdidxselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Cmdidxselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdidxselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDIDX` writer - Force Event for Command Index Error"]
pub type CmdidxW<'a, REG> = crate::BitWriter<'a, REG, Cmdidxselect>;
impl<'a, REG> CmdidxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidxselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidxselect::Yes)
    }
}
#[doc = "Force Event for Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datteoselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Datteoselect> for bool {
    #[inline(always)]
    fn from(variant: Datteoselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATTEO` writer - Force Event for Data Timeout Error"]
pub type DatteoW<'a, REG> = crate::BitWriter<'a, REG, Datteoselect>;
impl<'a, REG> DatteoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Datteoselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Datteoselect::Yes)
    }
}
#[doc = "Force Event for Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datcrcselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Datcrcselect> for bool {
    #[inline(always)]
    fn from(variant: Datcrcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATCRC` writer - Force Event for Data CRC Error"]
pub type DatcrcW<'a, REG> = crate::BitWriter<'a, REG, Datcrcselect>;
impl<'a, REG> DatcrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Datcrcselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Datcrcselect::Yes)
    }
}
#[doc = "Force Event for Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datendselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Datendselect> for bool {
    #[inline(always)]
    fn from(variant: Datendselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATEND` writer - Force Event for Data End Bit Error"]
pub type DatendW<'a, REG> = crate::BitWriter<'a, REG, Datendselect>;
impl<'a, REG> DatendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Datendselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Datendselect::Yes)
    }
}
#[doc = "Force Event for Current Limit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Curlimselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Curlimselect> for bool {
    #[inline(always)]
    fn from(variant: Curlimselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURLIM` writer - Force Event for Current Limit Error"]
pub type CurlimW<'a, REG> = crate::BitWriter<'a, REG, Curlimselect>;
impl<'a, REG> CurlimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Curlimselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Curlimselect::Yes)
    }
}
#[doc = "Force Event for Auto CMD Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Acmdselect> for bool {
    #[inline(always)]
    fn from(variant: Acmdselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD` writer - Force Event for Auto CMD Error"]
pub type AcmdW<'a, REG> = crate::BitWriter<'a, REG, Acmdselect>;
impl<'a, REG> AcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdselect::Yes)
    }
}
#[doc = "Force Event for ADMA Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admaselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Admaselect> for bool {
    #[inline(always)]
    fn from(variant: Admaselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMA` writer - Force Event for ADMA Error"]
pub type AdmaW<'a, REG> = crate::BitWriter<'a, REG, Admaselect>;
impl<'a, REG> AdmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Admaselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Admaselect::Yes)
    }
}
#[doc = "Force Event for Boot Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootaeselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Bootaeselect> for bool {
    #[inline(always)]
    fn from(variant: Bootaeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTAE` writer - Force Event for Boot Acknowledge Error"]
pub type BootaeW<'a, REG> = crate::BitWriter<'a, REG, Bootaeselect>;
impl<'a, REG> BootaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Bootaeselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Bootaeselect::Yes)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Command Timeout Error"]
    #[inline(always)]
    pub fn cmdteo(&mut self) -> CmdteoW<FereisSpec> {
        CmdteoW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error"]
    #[inline(always)]
    pub fn cmdcrc(&mut self) -> CmdcrcW<FereisSpec> {
        CmdcrcW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error"]
    #[inline(always)]
    pub fn cmdend(&mut self) -> CmdendW<FereisSpec> {
        CmdendW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Event for Command Index Error"]
    #[inline(always)]
    pub fn cmdidx(&mut self) -> CmdidxW<FereisSpec> {
        CmdidxW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error"]
    #[inline(always)]
    pub fn datteo(&mut self) -> DatteoW<FereisSpec> {
        DatteoW::new(self, 4)
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error"]
    #[inline(always)]
    pub fn datcrc(&mut self) -> DatcrcW<FereisSpec> {
        DatcrcW::new(self, 5)
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error"]
    #[inline(always)]
    pub fn datend(&mut self) -> DatendW<FereisSpec> {
        DatendW::new(self, 6)
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error"]
    #[inline(always)]
    pub fn curlim(&mut self) -> CurlimW<FereisSpec> {
        CurlimW::new(self, 7)
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error"]
    #[inline(always)]
    pub fn acmd(&mut self) -> AcmdW<FereisSpec> {
        AcmdW::new(self, 8)
    }
    #[doc = "Bit 9 - Force Event for ADMA Error"]
    #[inline(always)]
    pub fn adma(&mut self) -> AdmaW<FereisSpec> {
        AdmaW::new(self, 9)
    }
    #[doc = "Bit 12 - Force Event for Boot Acknowledge Error"]
    #[inline(always)]
    pub fn bootae(&mut self) -> BootaeW<FereisSpec> {
        BootaeW::new(self, 12)
    }
}
#[doc = "Force Event for Error Interrupt Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fereis::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FereisSpec;
impl crate::RegisterSpec for FereisSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`fereis::W`](W) writer structure"]
impl crate::Writable for FereisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FEREIS to value 0"]
impl crate::Resettable for FereisSpec {}
