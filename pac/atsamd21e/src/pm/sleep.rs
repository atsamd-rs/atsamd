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
#[doc = "Idle Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDLE_A {
    #[doc = "0: The CPU clock domain is stopped"]
    CPU = 0,
    #[doc = "1: The CPU and AHB clock domains are stopped"]
    AHB = 1,
    #[doc = "2: The CPU, AHB and APB clock domains are stopped"]
    APB = 2,
}
impl From<IDLE_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDLE` reader - Idle Mode Configuration"]
pub struct IDLE_R(crate::FieldReader<u8, IDLE_A>);
impl IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IDLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLE_A> {
        match self.bits {
            0 => Some(IDLE_A::CPU),
            1 => Some(IDLE_A::AHB),
            2 => Some(IDLE_A::APB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CPU`"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        **self == IDLE_A::CPU
    }
    #[doc = "Checks if the value of the field is `AHB`"]
    #[inline(always)]
    pub fn is_ahb(&self) -> bool {
        **self == IDLE_A::AHB
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        **self == IDLE_A::APB
    }
}
impl core::ops::Deref for IDLE_R {
    type Target = crate::FieldReader<u8, IDLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE` writer - Idle Mode Configuration"]
pub struct IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The CPU clock domain is stopped"]
    #[inline(always)]
    pub fn cpu(self) -> &'a mut W {
        self.variant(IDLE_A::CPU)
    }
    #[doc = "The CPU and AHB clock domains are stopped"]
    #[inline(always)]
    pub fn ahb(self) -> &'a mut W {
        self.variant(IDLE_A::AHB)
    }
    #[doc = "The CPU, AHB and APB clock domains are stopped"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(IDLE_A::APB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Idle Mode Configuration"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Idle Mode Configuration"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W {
        IDLE_W { w: self }
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
}
#[doc = "`reset()` method sets SLEEP to value 0"]
impl crate::Resettable for SLEEP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
