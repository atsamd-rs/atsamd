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
#[doc = "Write-Back Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WRBQOS_A {
    #[doc = "0: Background (no sensitive operation)"]
    DISABLE = 0,
    #[doc = "1: Sensitive Bandwidth"]
    LOW = 1,
    #[doc = "2: Sensitive Latency"]
    MEDIUM = 2,
    #[doc = "3: Critical Latency"]
    HIGH = 3,
}
impl From<WRBQOS_A> for u8 {
    #[inline(always)]
    fn from(variant: WRBQOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WRBQOS` reader - Write-Back Quality of Service"]
pub struct WRBQOS_R(crate::FieldReader<u8, WRBQOS_A>);
impl WRBQOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WRBQOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRBQOS_A {
        match self.bits {
            0 => WRBQOS_A::DISABLE,
            1 => WRBQOS_A::LOW,
            2 => WRBQOS_A::MEDIUM,
            3 => WRBQOS_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WRBQOS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == WRBQOS_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        **self == WRBQOS_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == WRBQOS_A::HIGH
    }
}
impl core::ops::Deref for WRBQOS_R {
    type Target = crate::FieldReader<u8, WRBQOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRBQOS` writer - Write-Back Quality of Service"]
pub struct WRBQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WRBQOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRBQOS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WRBQOS_A::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WRBQOS_A::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(WRBQOS_A::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WRBQOS_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Fetch Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FQOS_A {
    #[doc = "0: Background (no sensitive operation)"]
    DISABLE = 0,
    #[doc = "1: Sensitive Bandwidth"]
    LOW = 1,
    #[doc = "2: Sensitive Latency"]
    MEDIUM = 2,
    #[doc = "3: Critical Latency"]
    HIGH = 3,
}
impl From<FQOS_A> for u8 {
    #[inline(always)]
    fn from(variant: FQOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FQOS` reader - Fetch Quality of Service"]
pub struct FQOS_R(crate::FieldReader<u8, FQOS_A>);
impl FQOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FQOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FQOS_A {
        match self.bits {
            0 => FQOS_A::DISABLE,
            1 => FQOS_A::LOW,
            2 => FQOS_A::MEDIUM,
            3 => FQOS_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FQOS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == FQOS_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        **self == FQOS_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == FQOS_A::HIGH
    }
}
impl core::ops::Deref for FQOS_R {
    type Target = crate::FieldReader<u8, FQOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FQOS` writer - Fetch Quality of Service"]
pub struct FQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> FQOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FQOS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FQOS_A::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(FQOS_A::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(FQOS_A::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(FQOS_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u8 & 0x03) << 2);
        self.w
    }
}
#[doc = "Data Transfer Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DQOS_A {
    #[doc = "0: Background (no sensitive operation)"]
    DISABLE = 0,
    #[doc = "1: Sensitive Bandwidth"]
    LOW = 1,
    #[doc = "2: Sensitive Latency"]
    MEDIUM = 2,
    #[doc = "3: Critical Latency"]
    HIGH = 3,
}
impl From<DQOS_A> for u8 {
    #[inline(always)]
    fn from(variant: DQOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DQOS` reader - Data Transfer Quality of Service"]
pub struct DQOS_R(crate::FieldReader<u8, DQOS_A>);
impl DQOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DQOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQOS_A {
        match self.bits {
            0 => DQOS_A::DISABLE,
            1 => DQOS_A::LOW,
            2 => DQOS_A::MEDIUM,
            3 => DQOS_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DQOS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == DQOS_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        **self == DQOS_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == DQOS_A::HIGH
    }
}
impl core::ops::Deref for DQOS_R {
    type Target = crate::FieldReader<u8, DQOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQOS` writer - Data Transfer Quality of Service"]
pub struct DQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> DQOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQOS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DQOS_A::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DQOS_A::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(DQOS_A::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DQOS_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Write-Back Quality of Service"]
    #[inline(always)]
    pub fn wrbqos(&self) -> WRBQOS_R {
        WRBQOS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Fetch Quality of Service"]
    #[inline(always)]
    pub fn fqos(&self) -> FQOS_R {
        FQOS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Data Transfer Quality of Service"]
    #[inline(always)]
    pub fn dqos(&self) -> DQOS_R {
        DQOS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write-Back Quality of Service"]
    #[inline(always)]
    pub fn wrbqos(&mut self) -> WRBQOS_W {
        WRBQOS_W { w: self }
    }
    #[doc = "Bits 2:3 - Fetch Quality of Service"]
    #[inline(always)]
    pub fn fqos(&mut self) -> FQOS_W {
        FQOS_W { w: self }
    }
    #[doc = "Bits 4:5 - Data Transfer Quality of Service"]
    #[inline(always)]
    pub fn dqos(&mut self) -> DQOS_W {
        DQOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QOS Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qosctrl](index.html) module"]
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
}
#[doc = "`reset()` method sets QOSCTRL to value 0x15"]
impl crate::Resettable for QOSCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x15
    }
}
