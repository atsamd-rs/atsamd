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
#[doc = "Sleep-on-exit on handler return\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPONEXIT_A {
    #[doc = "0: Do not sleep when returning to Thread mode"]
    VALUE_0 = 0,
    #[doc = "1: Enter sleep, or deep sleep, on return from an ISR"]
    VALUE_1 = 1,
}
impl From<SLEEPONEXIT_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPONEXIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPONEXIT` reader - Sleep-on-exit on handler return"]
pub struct SLEEPONEXIT_R(crate::FieldReader<bool, SLEEPONEXIT_A>);
impl SLEEPONEXIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPONEXIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPONEXIT_A {
        match self.bits {
            false => SLEEPONEXIT_A::VALUE_0,
            true => SLEEPONEXIT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == SLEEPONEXIT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == SLEEPONEXIT_A::VALUE_1
    }
}
impl core::ops::Deref for SLEEPONEXIT_R {
    type Target = crate::FieldReader<bool, SLEEPONEXIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPONEXIT` writer - Sleep-on-exit on handler return"]
pub struct SLEEPONEXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPONEXIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPONEXIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do not sleep when returning to Thread mode"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SLEEPONEXIT_A::VALUE_0)
    }
    #[doc = "Enter sleep, or deep sleep, on return from an ISR"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(SLEEPONEXIT_A::VALUE_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Deep Sleep used as low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPDEEP_A {
    #[doc = "0: Sleep"]
    VALUE_0 = 0,
    #[doc = "1: Deep sleep"]
    VALUE_1 = 1,
}
impl From<SLEEPDEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPDEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPDEEP` reader - Deep Sleep used as low power mode"]
pub struct SLEEPDEEP_R(crate::FieldReader<bool, SLEEPDEEP_A>);
impl SLEEPDEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPDEEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPDEEP_A {
        match self.bits {
            false => SLEEPDEEP_A::VALUE_0,
            true => SLEEPDEEP_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == SLEEPDEEP_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == SLEEPDEEP_A::VALUE_1
    }
}
impl core::ops::Deref for SLEEPDEEP_R {
    type Target = crate::FieldReader<bool, SLEEPDEEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPDEEP` writer - Deep Sleep used as low power mode"]
pub struct SLEEPDEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPDEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPDEEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SLEEPDEEP_A::VALUE_0)
    }
    #[doc = "Deep sleep"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(SLEEPDEEP_A::VALUE_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Send Event on Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEVONPEND_A {
    #[doc = "0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    VALUE_0 = 0,
    #[doc = "1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    VALUE_1 = 1,
}
impl From<SEVONPEND_A> for bool {
    #[inline(always)]
    fn from(variant: SEVONPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEVONPEND` reader - Send Event on Pending bit"]
pub struct SEVONPEND_R(crate::FieldReader<bool, SEVONPEND_A>);
impl SEVONPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEVONPEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEVONPEND_A {
        match self.bits {
            false => SEVONPEND_A::VALUE_0,
            true => SEVONPEND_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == SEVONPEND_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == SEVONPEND_A::VALUE_1
    }
}
impl core::ops::Deref for SEVONPEND_R {
    type Target = crate::FieldReader<bool, SEVONPEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEVONPEND` writer - Send Event on Pending bit"]
pub struct SEVONPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SEVONPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEVONPEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SEVONPEND_A::VALUE_0)
    }
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(SEVONPEND_A::VALUE_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Sleep-on-exit on handler return"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Deep Sleep used as low power mode"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sleep-on-exit on handler return"]
    #[inline(always)]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W {
        SLEEPONEXIT_W { w: self }
    }
    #[doc = "Bit 2 - Deep Sleep used as low power mode"]
    #[inline(always)]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W {
        SLEEPDEEP_W { w: self }
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline(always)]
    pub fn sevonpend(&mut self) -> SEVONPEND_W {
        SEVONPEND_W { w: self }
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
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
