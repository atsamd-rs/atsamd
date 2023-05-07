#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> SLEEPONEXITSELECT_A {
        match self.bits {
            false => SLEEPONEXITSELECT_A::VALUE_0,
            true => SLEEPONEXITSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SLEEPONEXITSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SLEEPONEXITSELECT_A::VALUE_1
    }
}
#[doc = "Field `SLEEPONEXIT` writer - Sleep-on-exit on handler return"]
pub type SLEEPONEXIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCR_SPEC, SLEEPONEXITSELECT_A, O>;
impl<'a, const O: u8> SLEEPONEXIT_W<'a, O> {
    #[doc = "Do not sleep when returning to Thread mode"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SLEEPONEXITSELECT_A::VALUE_0)
    }
    #[doc = "Enter sleep, or deep sleep, on return from an ISR"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
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
    pub fn variant(&self) -> SLEEPDEEPSELECT_A {
        match self.bits {
            false => SLEEPDEEPSELECT_A::VALUE_0,
            true => SLEEPDEEPSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SLEEPDEEPSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SLEEPDEEPSELECT_A::VALUE_1
    }
}
#[doc = "Field `SLEEPDEEP` writer - Deep Sleep used as low power mode"]
pub type SLEEPDEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, SLEEPDEEPSELECT_A, O>;
impl<'a, const O: u8> SLEEPDEEP_W<'a, O> {
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SLEEPDEEPSELECT_A::VALUE_0)
    }
    #[doc = "Deep sleep"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
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
    pub fn variant(&self) -> SEVONPENDSELECT_A {
        match self.bits {
            false => SEVONPENDSELECT_A::VALUE_0,
            true => SEVONPENDSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SEVONPENDSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SEVONPENDSELECT_A::VALUE_1
    }
}
#[doc = "Field `SEVONPEND` writer - Send Event on Pending bit"]
pub type SEVONPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, SEVONPENDSELECT_A, O>;
impl<'a, const O: u8> SEVONPEND_W<'a, O> {
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SEVONPENDSELECT_A::VALUE_0)
    }
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
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
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W<1> {
        SLEEPONEXIT_W::new(self)
    }
    #[doc = "Bit 2 - Deep Sleep used as low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W<2> {
        SLEEPDEEP_W::new(self)
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn sevonpend(&mut self) -> SEVONPEND_W<4> {
        SEVONPEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
