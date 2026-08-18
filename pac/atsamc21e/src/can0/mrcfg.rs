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
#[doc = "Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QOS_A {
    #[doc = "0: Background (no sensitive operation)"]
    DISABLE = 0,
    #[doc = "1: Sensitive Bandwidth"]
    LOW = 1,
    #[doc = "2: Sensitive Latency"]
    MEDIUM = 2,
    #[doc = "3: Critical Latency"]
    HIGH = 3,
}
impl From<QOS_A> for u8 {
    #[inline(always)]
    fn from(variant: QOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `QOS` reader - Quality of Service"]
pub struct QOS_R(crate::FieldReader<u8, QOS_A>);
impl QOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QOS_A {
        match self.bits {
            0 => QOS_A::DISABLE,
            1 => QOS_A::LOW,
            2 => QOS_A::MEDIUM,
            3 => QOS_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == QOS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == QOS_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        **self == QOS_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == QOS_A::HIGH
    }
}
impl core::ops::Deref for QOS_R {
    type Target = crate::FieldReader<u8, QOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QOS` writer - Quality of Service"]
pub struct QOS_W<'a> {
    w: &'a mut W,
}
impl<'a> QOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QOS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(QOS_A::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(QOS_A::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(QOS_A::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(QOS_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Quality of Service"]
    #[inline(always)]
    pub fn qos(&self) -> QOS_R {
        QOS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Quality of Service"]
    #[inline(always)]
    pub fn qos(&mut self) -> QOS_W {
        QOS_W { w: self }
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
}
#[doc = "`reset()` method sets MRCFG to value 0x02"]
impl crate::Resettable for MRCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
