#[doc = "Register `FERACES` writer"]
pub type W = crate::W<FeracesSpec>;
#[doc = "Force Event for Auto CMD12 Not Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmd12neselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Acmd12neselect> for bool {
    #[inline(always)]
    fn from(variant: Acmd12neselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD12NE` writer - Force Event for Auto CMD12 Not Executed"]
pub type Acmd12neW<'a, REG> = crate::BitWriter<'a, REG, Acmd12neselect>;
impl<'a, REG> Acmd12neW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Acmd12neselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Acmd12neselect::Yes)
    }
}
#[doc = "Force Event for Auto CMD Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdteoselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Acmdteoselect> for bool {
    #[inline(always)]
    fn from(variant: Acmdteoselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDTEO` writer - Force Event for Auto CMD Timeout Error"]
pub type AcmdteoW<'a, REG> = crate::BitWriter<'a, REG, Acmdteoselect>;
impl<'a, REG> AcmdteoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdteoselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdteoselect::Yes)
    }
}
#[doc = "Force Event for Auto CMD CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdcrcselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Acmdcrcselect> for bool {
    #[inline(always)]
    fn from(variant: Acmdcrcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDCRC` writer - Force Event for Auto CMD CRC Error"]
pub type AcmdcrcW<'a, REG> = crate::BitWriter<'a, REG, Acmdcrcselect>;
impl<'a, REG> AcmdcrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdcrcselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdcrcselect::Yes)
    }
}
#[doc = "Force Event for Auto CMD End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdendselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Acmdendselect> for bool {
    #[inline(always)]
    fn from(variant: Acmdendselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDEND` writer - Force Event for Auto CMD End Bit Error"]
pub type AcmdendW<'a, REG> = crate::BitWriter<'a, REG, Acmdendselect>;
impl<'a, REG> AcmdendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdendselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdendselect::Yes)
    }
}
#[doc = "Force Event for Auto CMD Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdidxselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Acmdidxselect> for bool {
    #[inline(always)]
    fn from(variant: Acmdidxselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDIDX` writer - Force Event for Auto CMD Index Error"]
pub type AcmdidxW<'a, REG> = crate::BitWriter<'a, REG, Acmdidxselect>;
impl<'a, REG> AcmdidxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdidxselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdidxselect::Yes)
    }
}
#[doc = "Force Event for Command Not Issued By Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdniselect {
    #[doc = "0: No Interrupt"]
    No = 0,
    #[doc = "1: Interrupt is generated"]
    Yes = 1,
}
impl From<Cmdniselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdniselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDNI` writer - Force Event for Command Not Issued By Auto CMD12 Error"]
pub type CmdniW<'a, REG> = crate::BitWriter<'a, REG, Cmdniselect>;
impl<'a, REG> CmdniW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdniselect::No)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdniselect::Yes)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn acmd12ne(&mut self) -> Acmd12neW<FeracesSpec> {
        Acmd12neW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn acmdteo(&mut self) -> AcmdteoW<FeracesSpec> {
        AcmdteoW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    pub fn acmdcrc(&mut self) -> AcmdcrcW<FeracesSpec> {
        AcmdcrcW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn acmdend(&mut self) -> AcmdendW<FeracesSpec> {
        AcmdendW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    pub fn acmdidx(&mut self) -> AcmdidxW<FeracesSpec> {
        AcmdidxW::new(self, 4)
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cmdni(&mut self) -> CmdniW<FeracesSpec> {
        CmdniW::new(self, 7)
    }
}
#[doc = "Force Event for Auto CMD Error Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feraces::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeracesSpec;
impl crate::RegisterSpec for FeracesSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`feraces::W`](W) writer structure"]
impl crate::Writable for FeracesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FERACES to value 0"]
impl crate::Resettable for FeracesSpec {}
