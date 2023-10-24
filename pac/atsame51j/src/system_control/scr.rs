#[doc = "Register `SCR` reader"]
pub type R = crate::R<SCR_SPEC>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCR_SPEC>;
#[doc = "Field `SLEEPONEXIT` reader - Sleep-on-exit on handler return"]
pub type SLEEPONEXIT_R = crate::BitReader<SLEEPONEXITSELECT_A>;
#[doc = "Sleep-on-exit on handler return\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEPONEXITSELECT_A {
    #[doc = "0: Do not sleep when returning to Thread mode"]
    VALUE_0 = 0,
    #[doc = "1: Enter sleep, or deep sleep, on return from an ISR"]
    VALUE_1 = 1,
}
impl From<SLEEPONEXITSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPONEXITSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEPONEXIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLEEPONEXITSELECT_A {
        match self.bits {
            false => SLEEPONEXITSELECT_A::VALUE_0,
            true => SLEEPONEXITSELECT_A::VALUE_1,
        }
    }
    #[doc = "Do not sleep when returning to Thread mode"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SLEEPONEXITSELECT_A::VALUE_0
    }
    #[doc = "Enter sleep, or deep sleep, on return from an ISR"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SLEEPONEXITSELECT_A::VALUE_1
    }
}
#[doc = "Field `SLEEPONEXIT` writer - Sleep-on-exit on handler return"]
pub type SLEEPONEXIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLEEPONEXITSELECT_A>;
impl<'a, REG, const O: u8> SLEEPONEXIT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not sleep when returning to Thread mode"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEPONEXITSELECT_A::VALUE_0)
    }
    #[doc = "Enter sleep, or deep sleep, on return from an ISR"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEPONEXITSELECT_A::VALUE_1)
    }
}
#[doc = "Field `SLEEPDEEP` reader - Deep Sleep used as low power mode"]
pub type SLEEPDEEP_R = crate::BitReader<SLEEPDEEPSELECT_A>;
#[doc = "Deep Sleep used as low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEPDEEPSELECT_A {
    #[doc = "0: Sleep"]
    VALUE_0 = 0,
    #[doc = "1: Deep sleep"]
    VALUE_1 = 1,
}
impl From<SLEEPDEEPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPDEEPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEPDEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLEEPDEEPSELECT_A {
        match self.bits {
            false => SLEEPDEEPSELECT_A::VALUE_0,
            true => SLEEPDEEPSELECT_A::VALUE_1,
        }
    }
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SLEEPDEEPSELECT_A::VALUE_0
    }
    #[doc = "Deep sleep"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SLEEPDEEPSELECT_A::VALUE_1
    }
}
#[doc = "Field `SLEEPDEEP` writer - Deep Sleep used as low power mode"]
pub type SLEEPDEEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLEEPDEEPSELECT_A>;
impl<'a, REG, const O: u8> SLEEPDEEP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEPDEEPSELECT_A::VALUE_0)
    }
    #[doc = "Deep sleep"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEPDEEPSELECT_A::VALUE_1)
    }
}
#[doc = "Field `SEVONPEND` reader - Send Event on Pending bit"]
pub type SEVONPEND_R = crate::BitReader<SEVONPENDSELECT_A>;
#[doc = "Send Event on Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEVONPENDSELECT_A {
    #[doc = "0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    VALUE_0 = 0,
    #[doc = "1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    VALUE_1 = 1,
}
impl From<SEVONPENDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SEVONPENDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SEVONPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEVONPENDSELECT_A {
        match self.bits {
            false => SEVONPENDSELECT_A::VALUE_0,
            true => SEVONPENDSELECT_A::VALUE_1,
        }
    }
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SEVONPENDSELECT_A::VALUE_0
    }
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SEVONPENDSELECT_A::VALUE_1
    }
}
#[doc = "Field `SEVONPEND` writer - Send Event on Pending bit"]
pub type SEVONPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SEVONPENDSELECT_A>;
impl<'a, REG, const O: u8> SEVONPEND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPENDSELECT_A::VALUE_0)
    }
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPENDSELECT_A::VALUE_1)
    }
}
impl R {
    #[doc = "Bit 1 - Sleep-on-exit on handler return"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Deep Sleep used as low power mode"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sleep-on-exit on handler return"]
    #[inline(always)]
    #[must_use]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W<SCR_SPEC, 1> {
        SLEEPONEXIT_W::new(self)
    }
    #[doc = "Bit 2 - Deep Sleep used as low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W<SCR_SPEC, 2> {
        SLEEPDEEP_W::new(self)
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn sevonpend(&mut self) -> SEVONPEND_W<SCR_SPEC, 4> {
        SEVONPEND_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for SCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
