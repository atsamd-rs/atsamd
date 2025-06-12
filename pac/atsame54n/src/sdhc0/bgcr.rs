#[doc = "Register `BGCR` reader"]
pub type R = crate::R<BgcrSpec>;
#[doc = "Register `BGCR` writer"]
pub type W = crate::W<BgcrSpec>;
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
#[doc = "Read Wait Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwctrlselect {
    #[doc = "0: Disable Read Wait Control"]
    Disable = 0,
    #[doc = "1: Enable Read Wait Control"]
    Enable = 1,
}
impl From<Rwctrlselect> for bool {
    #[inline(always)]
    fn from(variant: Rwctrlselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWCTRL` reader - Read Wait Control"]
pub type RwctrlR = crate::BitReader<Rwctrlselect>;
impl RwctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwctrlselect {
        match self.bits {
            false => Rwctrlselect::Disable,
            true => Rwctrlselect::Enable,
        }
    }
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rwctrlselect::Disable
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rwctrlselect::Enable
    }
}
#[doc = "Field `RWCTRL` writer - Read Wait Control"]
pub type RwctrlW<'a, REG> = crate::BitWriter<'a, REG, Rwctrlselect>;
impl<'a, REG> RwctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rwctrlselect::Disable)
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rwctrlselect::Enable)
    }
}
#[doc = "Interrupt at Block Gap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intbgselect {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Intbgselect> for bool {
    #[inline(always)]
    fn from(variant: Intbgselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTBG` reader - Interrupt at Block Gap"]
pub type IntbgR = crate::BitReader<Intbgselect>;
impl IntbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intbgselect {
        match self.bits {
            false => Intbgselect::Disabled,
            true => Intbgselect::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Intbgselect::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Intbgselect::Enabled
    }
}
#[doc = "Field `INTBG` writer - Interrupt at Block Gap"]
pub type IntbgW<'a, REG> = crate::BitWriter<'a, REG, Intbgselect>;
impl<'a, REG> IntbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Intbgselect::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Intbgselect::Enabled)
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
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctrl(&self) -> RwctrlR {
        RwctrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt at Block Gap"]
    #[inline(always)]
    pub fn intbg(&self) -> IntbgR {
        IntbgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop at Block Gap Request"]
    #[inline(always)]
    pub fn stpbgr(&mut self) -> StpbgrW<BgcrSpec> {
        StpbgrW::new(self, 0)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn contr(&mut self) -> ContrW<BgcrSpec> {
        ContrW::new(self, 1)
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctrl(&mut self) -> RwctrlW<BgcrSpec> {
        RwctrlW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt at Block Gap"]
    #[inline(always)]
    pub fn intbg(&mut self) -> IntbgW<BgcrSpec> {
        IntbgW::new(self, 3)
    }
}
#[doc = "Block Gap Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgcrSpec;
impl crate::RegisterSpec for BgcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bgcr::R`](R) reader structure"]
impl crate::Readable for BgcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bgcr::W`](W) writer structure"]
impl crate::Writable for BgcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BGCR to value 0"]
impl crate::Resettable for BgcrSpec {}
