#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrstselect {
    #[doc = "0: The peripheral is not reset"]
    Disable = 0,
    #[doc = "1: The peripheral is reset"]
    Enable = 1,
}
impl From<Swrstselect> for bool {
    #[inline(always)]
    fn from(variant: Swrstselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader<Swrstselect>;
impl SwrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrstselect {
        match self.bits {
            false => Swrstselect::Disable,
            true => Swrstselect::Enable,
        }
    }
    #[doc = "The peripheral is not reset"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Swrstselect::Disable
    }
    #[doc = "The peripheral is reset"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Swrstselect::Enable
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG, Swrstselect>;
impl<'a, REG> SwrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The peripheral is not reset"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstselect::Disable)
    }
    #[doc = "The peripheral is reset"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstselect::Enable)
    }
}
#[doc = "Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableselect {
    #[doc = "0: The peripheral is disabled"]
    Disable = 0,
    #[doc = "1: The peripheral is enabled"]
    Enable = 1,
}
impl From<Enableselect> for bool {
    #[inline(always)]
    fn from(variant: Enableselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader<Enableselect>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enableselect {
        match self.bits {
            false => Enableselect::Disable,
            true => Enableselect::Enable,
        }
    }
    #[doc = "The peripheral is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enableselect::Disable
    }
    #[doc = "The peripheral is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enableselect::Enable
    }
}
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enableselect>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The peripheral is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Disable)
    }
    #[doc = "The peripheral is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Enable)
    }
}
#[doc = "Run in Standby\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Runstdbyselect {
    #[doc = "0: Generic clock is not required in standby sleep mode"]
    Disable = 0,
    #[doc = "1: Generic clock is required in standby sleep mode"]
    Enable = 1,
}
impl From<Runstdbyselect> for bool {
    #[inline(always)]
    fn from(variant: Runstdbyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader<Runstdbyselect>;
impl RunstdbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Runstdbyselect {
        match self.bits {
            false => Runstdbyselect::Disable,
            true => Runstdbyselect::Enable,
        }
    }
    #[doc = "Generic clock is not required in standby sleep mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Runstdbyselect::Disable
    }
    #[doc = "Generic clock is required in standby sleep mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Runstdbyselect::Enable
    }
}
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG, Runstdbyselect>;
impl<'a, REG> RunstdbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generic clock is not required in standby sleep mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Runstdbyselect::Disable)
    }
    #[doc = "Generic clock is required in standby sleep mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Runstdbyselect::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<CtrlSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CtrlSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<CtrlSpec> {
        RunstdbyW::new(self, 6)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
