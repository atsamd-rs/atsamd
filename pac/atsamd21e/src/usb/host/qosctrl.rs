#[doc = "Register `QOSCTRL` reader"]
pub struct R(crate::R<QOSCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QOSCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QOSCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QOSCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QOSCTRL` writer"]
pub struct W(crate::W<QOSCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QOSCTRL_SPEC>;
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
impl From<crate::W<QOSCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QOSCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CQOS` reader - Configuration Quality of Service"]
pub type CQOS_R = crate::FieldReader<u8, CQOSSELECT_A>;
#[doc = "Configuration Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CQOSSELECT_A {
    #[doc = "0: Background (no sensitive operation)"]
    DISABLE = 0,
    #[doc = "1: Sensitive Bandwidth"]
    LOW = 1,
    #[doc = "2: Sensitive Latency"]
    MEDIUM = 2,
    #[doc = "3: Critical Latency"]
    HIGH = 3,
}
impl From<CQOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CQOSSELECT_A) -> Self {
        variant as _
    }
}
impl CQOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CQOSSELECT_A {
        match self.bits {
            0 => CQOSSELECT_A::DISABLE,
            1 => CQOSSELECT_A::LOW,
            2 => CQOSSELECT_A::MEDIUM,
            3 => CQOSSELECT_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CQOSSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CQOSSELECT_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == CQOSSELECT_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CQOSSELECT_A::HIGH
    }
}
#[doc = "Field `CQOS` writer - Configuration Quality of Service"]
pub type CQOS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, QOSCTRL_SPEC, u8, CQOSSELECT_A, 2, O>;
impl<'a, const O: u8> CQOS_W<'a, O> {
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CQOSSELECT_A::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CQOSSELECT_A::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(CQOSSELECT_A::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CQOSSELECT_A::HIGH)
    }
}
#[doc = "Field `DQOS` reader - Data Quality of Service"]
pub type DQOS_R = crate::FieldReader<u8, DQOSSELECT_A>;
#[doc = "Data Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DQOSSELECT_A {
    #[doc = "0: Background (no sensitive operation)"]
    DISABLE = 0,
    #[doc = "1: Sensitive Bandwidth"]
    LOW = 1,
    #[doc = "2: Sensitive Latency"]
    MEDIUM = 2,
    #[doc = "3: Critical Latency"]
    HIGH = 3,
}
impl From<DQOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DQOSSELECT_A) -> Self {
        variant as _
    }
}
impl DQOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQOSSELECT_A {
        match self.bits {
            0 => DQOSSELECT_A::DISABLE,
            1 => DQOSSELECT_A::LOW,
            2 => DQOSSELECT_A::MEDIUM,
            3 => DQOSSELECT_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DQOSSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DQOSSELECT_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == DQOSSELECT_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DQOSSELECT_A::HIGH
    }
}
#[doc = "Field `DQOS` writer - Data Quality of Service"]
pub type DQOS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, QOSCTRL_SPEC, u8, DQOSSELECT_A, 2, O>;
impl<'a, const O: u8> DQOS_W<'a, O> {
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DQOSSELECT_A::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DQOSSELECT_A::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(DQOSSELECT_A::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DQOSSELECT_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline(always)]
    pub fn cqos(&self) -> CQOS_R {
        CQOS_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline(always)]
    pub fn dqos(&self) -> DQOS_R {
        DQOS_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn cqos(&mut self) -> CQOS_W<0> {
        CQOS_W::new(self)
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn dqos(&mut self) -> DQOS_W<2> {
        DQOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Quality Of Service\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qosctrl](index.html) module"]
pub struct QOSCTRL_SPEC;
impl crate::RegisterSpec for QOSCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [qosctrl::R](R) reader structure"]
impl crate::Readable for QOSCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qosctrl::W](W) writer structure"]
impl crate::Writable for QOSCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QOSCTRL to value 0x05"]
impl crate::Resettable for QOSCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
