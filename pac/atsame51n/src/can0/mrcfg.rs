#[doc = "Register `MRCFG` reader"]
pub struct R(crate::R<MRCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MRCFG` writer"]
pub struct W(crate::W<MRCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRCFG_SPEC>;
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
impl From<crate::W<MRCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QOS` reader - Quality of Service"]
pub type QOS_R = crate::FieldReader<u8, QOSSELECT_A>;
#[doc = "Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QOSSELECT_A {
    #[doc = "0: Background (no sensitive operation)"]
    DISABLE = 0,
    #[doc = "1: Sensitive Bandwidth"]
    LOW = 1,
    #[doc = "2: Sensitive Latency"]
    MEDIUM = 2,
    #[doc = "3: Critical Latency"]
    HIGH = 3,
}
impl From<QOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: QOSSELECT_A) -> Self {
        variant as _
    }
}
impl QOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QOSSELECT_A {
        match self.bits {
            0 => QOSSELECT_A::DISABLE,
            1 => QOSSELECT_A::LOW,
            2 => QOSSELECT_A::MEDIUM,
            3 => QOSSELECT_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == QOSSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == QOSSELECT_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == QOSSELECT_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == QOSSELECT_A::HIGH
    }
}
#[doc = "Field `QOS` writer - Quality of Service"]
pub type QOS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MRCFG_SPEC, u8, QOSSELECT_A, 2, O>;
impl<'a, const O: u8> QOS_W<'a, O> {
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(QOSSELECT_A::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(QOSSELECT_A::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(QOSSELECT_A::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(QOSSELECT_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Quality of Service"]
    #[inline(always)]
    pub fn qos(&self) -> QOS_R {
        QOS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn qos(&mut self) -> QOS_W<0> {
        QOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message RAM Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrcfg](index.html) module"]
pub struct MRCFG_SPEC;
impl crate::RegisterSpec for MRCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrcfg::R](R) reader structure"]
impl crate::Readable for MRCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mrcfg::W](W) writer structure"]
impl crate::Writable for MRCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MRCFG to value 0x02"]
impl crate::Resettable for MRCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
