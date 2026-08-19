#[doc = "Register `BGCR_EMMC_MODE` reader"]
pub type R = crate::R<BgcrEmmcModeSpec>;
#[doc = "Register `BGCR_EMMC_MODE` writer"]
pub type W = crate::W<BgcrEmmcModeSpec>;
#[doc = "Stop at Block Gap Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stpbgrselect {
    #[doc = "0: Transfer"]
    Transfer = 0,
    #[doc = "1: Stop"]
    Stop = 1,
}
impl From<Stpbgrselect> for bool {
    #[inline(always)]
    fn from(variant: Stpbgrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPBGR` reader - Stop at Block Gap Request"]
pub type StpbgrR = crate::BitReader<Stpbgrselect>;
impl StpbgrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stpbgrselect {
        match self.bits {
            false => Stpbgrselect::Transfer,
            true => Stpbgrselect::Stop,
        }
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == Stpbgrselect::Transfer
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Stpbgrselect::Stop
    }
}
#[doc = "Field `STPBGR` writer - Stop at Block Gap Request"]
pub type StpbgrW<'a, REG> = crate::BitWriter<'a, REG, Stpbgrselect>;
impl<'a, REG> StpbgrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Stpbgrselect::Transfer)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Stpbgrselect::Stop)
    }
}
#[doc = "Continue Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contrselect {
    #[doc = "0: Not affected"]
    GoOn = 0,
    #[doc = "1: Restart"]
    Restart = 1,
}
impl From<Contrselect> for bool {
    #[inline(always)]
    fn from(variant: Contrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONTR` reader - Continue Request"]
pub type ContrR = crate::BitReader<Contrselect>;
impl ContrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Contrselect {
        match self.bits {
            false => Contrselect::GoOn,
            true => Contrselect::Restart,
        }
    }
    #[doc = "Not affected"]
    #[inline(always)]
    pub fn is_go_on(&self) -> bool {
        *self == Contrselect::GoOn
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == Contrselect::Restart
    }
}
#[doc = "Field `CONTR` writer - Continue Request"]
pub type ContrW<'a, REG> = crate::BitWriter<'a, REG, Contrselect>;
impl<'a, REG> ContrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not affected"]
    #[inline(always)]
    pub fn go_on(self) -> &'a mut crate::W<REG> {
        self.variant(Contrselect::GoOn)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut crate::W<REG> {
        self.variant(Contrselect::Restart)
    }
}
impl R {
    #[doc = "Bit 0 - Stop at Block Gap Request"]
    #[inline(always)]
    pub fn stpbgr(&self) -> StpbgrR {
        StpbgrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn contr(&self) -> ContrR {
        ContrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop at Block Gap Request"]
    #[inline(always)]
    #[must_use]
    pub fn stpbgr(&mut self) -> StpbgrW<BgcrEmmcModeSpec> {
        StpbgrW::new(self, 0)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    #[must_use]
    pub fn contr(&mut self) -> ContrW<BgcrEmmcModeSpec> {
        ContrW::new(self, 1)
    }
}
#[doc = "Block Gap Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bgcr_emmc_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcr_emmc_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgcrEmmcModeSpec;
impl crate::RegisterSpec for BgcrEmmcModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bgcr_emmc_mode::R`](R) reader structure"]
impl crate::Readable for BgcrEmmcModeSpec {}
#[doc = "`write(|w| ..)` method takes [`bgcr_emmc_mode::W`](W) writer structure"]
impl crate::Writable for BgcrEmmcModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BGCR_EMMC_MODE to value 0"]
impl crate::Resettable for BgcrEmmcModeSpec {
    const RESET_VALUE: u8 = 0;
}
