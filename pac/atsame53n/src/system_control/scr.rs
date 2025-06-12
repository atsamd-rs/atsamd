#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Sleep-on-exit on handler return\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleeponexitselect {
    #[doc = "0: Do not sleep when returning to Thread mode"]
    Value0 = 0,
    #[doc = "1: Enter sleep, or deep sleep, on return from an ISR"]
    Value1 = 1,
}
impl From<Sleeponexitselect> for bool {
    #[inline(always)]
    fn from(variant: Sleeponexitselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPONEXIT` reader - Sleep-on-exit on handler return"]
pub type SleeponexitR = crate::BitReader<Sleeponexitselect>;
impl SleeponexitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleeponexitselect {
        match self.bits {
            false => Sleeponexitselect::Value0,
            true => Sleeponexitselect::Value1,
        }
    }
    #[doc = "Do not sleep when returning to Thread mode"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Sleeponexitselect::Value0
    }
    #[doc = "Enter sleep, or deep sleep, on return from an ISR"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Sleeponexitselect::Value1
    }
}
#[doc = "Field `SLEEPONEXIT` writer - Sleep-on-exit on handler return"]
pub type SleeponexitW<'a, REG> = crate::BitWriter<'a, REG, Sleeponexitselect>;
impl<'a, REG> SleeponexitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not sleep when returning to Thread mode"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sleeponexitselect::Value0)
    }
    #[doc = "Enter sleep, or deep sleep, on return from an ISR"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sleeponexitselect::Value1)
    }
}
#[doc = "Deep Sleep used as low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleepdeepselect {
    #[doc = "0: Sleep"]
    Value0 = 0,
    #[doc = "1: Deep sleep"]
    Value1 = 1,
}
impl From<Sleepdeepselect> for bool {
    #[inline(always)]
    fn from(variant: Sleepdeepselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPDEEP` reader - Deep Sleep used as low power mode"]
pub type SleepdeepR = crate::BitReader<Sleepdeepselect>;
impl SleepdeepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleepdeepselect {
        match self.bits {
            false => Sleepdeepselect::Value0,
            true => Sleepdeepselect::Value1,
        }
    }
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Sleepdeepselect::Value0
    }
    #[doc = "Deep sleep"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Sleepdeepselect::Value1
    }
}
#[doc = "Field `SLEEPDEEP` writer - Deep Sleep used as low power mode"]
pub type SleepdeepW<'a, REG> = crate::BitWriter<'a, REG, Sleepdeepselect>;
impl<'a, REG> SleepdeepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepdeepselect::Value0)
    }
    #[doc = "Deep sleep"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepdeepselect::Value1)
    }
}
#[doc = "Send Event on Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sevonpendselect {
    #[doc = "0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    Value0 = 0,
    #[doc = "1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    Value1 = 1,
}
impl From<Sevonpendselect> for bool {
    #[inline(always)]
    fn from(variant: Sevonpendselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEVONPEND` reader - Send Event on Pending bit"]
pub type SevonpendR = crate::BitReader<Sevonpendselect>;
impl SevonpendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sevonpendselect {
        match self.bits {
            false => Sevonpendselect::Value0,
            true => Sevonpendselect::Value1,
        }
    }
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Sevonpendselect::Value0
    }
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Sevonpendselect::Value1
    }
}
#[doc = "Field `SEVONPEND` writer - Send Event on Pending bit"]
pub type SevonpendW<'a, REG> = crate::BitWriter<'a, REG, Sevonpendselect>;
impl<'a, REG> SevonpendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonpendselect::Value0)
    }
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonpendselect::Value1)
    }
}
impl R {
    #[doc = "Bit 1 - Sleep-on-exit on handler return"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SleeponexitR {
        SleeponexitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Deep Sleep used as low power mode"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SleepdeepR {
        SleepdeepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SevonpendR {
        SevonpendR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sleep-on-exit on handler return"]
    #[inline(always)]
    pub fn sleeponexit(&mut self) -> SleeponexitW<ScrSpec> {
        SleeponexitW::new(self, 1)
    }
    #[doc = "Bit 2 - Deep Sleep used as low power mode"]
    #[inline(always)]
    pub fn sleepdeep(&mut self) -> SleepdeepW<ScrSpec> {
        SleepdeepW::new(self, 2)
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline(always)]
    pub fn sevonpend(&mut self) -> SevonpendW<ScrSpec> {
        SevonpendW::new(self, 4)
    }
}
#[doc = "System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {}
