#[doc = "Reader of register MRCFG"]
pub type R = crate::R<u32, super::MRCFG>;
#[doc = "Writer for register MRCFG"]
pub type W = crate::W<u32, super::MRCFG>;
#[doc = "Register MRCFG `reset()`'s with value 0x02"]
impl crate::ResetValue for super::MRCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
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
#[doc = "Reader of field `QOS`"]
pub type QOS_R = crate::R<u8, QOS_A>;
impl QOS_R {
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
        *self == QOS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == QOS_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == QOS_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == QOS_A::HIGH
    }
}
#[doc = "Write proxy for field `QOS`"]
pub struct QOS_W<'a> {
    w: &'a mut W,
}
impl<'a> QOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QOS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
}
