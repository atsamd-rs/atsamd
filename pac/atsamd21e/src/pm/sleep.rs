#[doc = "Register `SLEEP` reader"]
pub struct R(crate::R<SLEEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLEEP` writer"]
pub struct W(crate::W<SLEEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLEEP_SPEC>;
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
impl From<crate::W<SLEEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLEEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDLE` reader - Idle Mode Configuration"]
pub type IDLE_R = crate::FieldReader<u8, IDLESELECT_A>;
#[doc = "Idle Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDLESELECT_A {
    #[doc = "0: The CPU clock domain is stopped"]
    CPU = 0,
    #[doc = "1: The CPU and AHB clock domains are stopped"]
    AHB = 1,
    #[doc = "2: The CPU, AHB and APB clock domains are stopped"]
    APB = 2,
}
impl From<IDLESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLESELECT_A) -> Self {
        variant as _
    }
}
impl IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLESELECT_A> {
        match self.bits {
            0 => Some(IDLESELECT_A::CPU),
            1 => Some(IDLESELECT_A::AHB),
            2 => Some(IDLESELECT_A::APB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CPU`"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        *self == IDLESELECT_A::CPU
    }
    #[doc = "Checks if the value of the field is `AHB`"]
    #[inline(always)]
    pub fn is_ahb(&self) -> bool {
        *self == IDLESELECT_A::AHB
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == IDLESELECT_A::APB
    }
}
#[doc = "Field `IDLE` writer - Idle Mode Configuration"]
pub type IDLE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SLEEP_SPEC, u8, IDLESELECT_A, 2, O>;
impl<'a, const O: u8> IDLE_W<'a, O> {
    #[doc = "The CPU clock domain is stopped"]
    #[inline(always)]
    pub fn cpu(self) -> &'a mut W {
        self.variant(IDLESELECT_A::CPU)
    }
    #[doc = "The CPU and AHB clock domains are stopped"]
    #[inline(always)]
    pub fn ahb(self) -> &'a mut W {
        self.variant(IDLESELECT_A::AHB)
    }
    #[doc = "The CPU, AHB and APB clock domains are stopped"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(IDLESELECT_A::APB)
    }
}
impl R {
    #[doc = "Bits 0:1 - Idle Mode Configuration"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Idle Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IDLE_W<0> {
        IDLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleep](index.html) module"]
pub struct SLEEP_SPEC;
impl crate::RegisterSpec for SLEEP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sleep::R](R) reader structure"]
impl crate::Readable for SLEEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sleep::W](W) writer structure"]
impl crate::Writable for SLEEP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLEEP to value 0"]
impl crate::Resettable for SLEEP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
