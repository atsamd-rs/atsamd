#[doc = "Reader of register QOSCTRL"]
pub type R = crate::R<u8, super::QOSCTRL>;
#[doc = "Writer for register QOSCTRL"]
pub type W = crate::W<u8, super::QOSCTRL>;
#[doc = "Register QOSCTRL `reset()`'s with value 0x15"]
impl crate::ResetValue for super::QOSCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x15
    }
}
#[doc = "Possible values of the field `WRBQOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRBQOS_A {
    #[doc = "Background (no sensitive operation)"]
    DISABLE,
    #[doc = "Sensitive Bandwidth"]
    LOW,
    #[doc = "Sensitive Latency"]
    MEDIUM,
    #[doc = "Critical Latency"]
    HIGH,
}
impl crate::ToBits<u8> for WRBQOS_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            WRBQOS_A::DISABLE => 0,
            WRBQOS_A::LOW => 1,
            WRBQOS_A::MEDIUM => 2,
            WRBQOS_A::HIGH => 3,
        }
    }
}
#[doc = "Reader of field `WRBQOS`"]
pub type WRBQOS_R = crate::R<u8, WRBQOS_A>;
impl WRBQOS_R {
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
        *self == WRBQOS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WRBQOS_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == WRBQOS_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WRBQOS_A::HIGH
    }
}
#[doc = "Write proxy for field `WRBQOS`"]
pub struct WRBQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WRBQOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRBQOS_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `FQOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FQOS_A {
    #[doc = "Background (no sensitive operation)"]
    DISABLE,
    #[doc = "Sensitive Bandwidth"]
    LOW,
    #[doc = "Sensitive Latency"]
    MEDIUM,
    #[doc = "Critical Latency"]
    HIGH,
}
impl crate::ToBits<u8> for FQOS_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FQOS_A::DISABLE => 0,
            FQOS_A::LOW => 1,
            FQOS_A::MEDIUM => 2,
            FQOS_A::HIGH => 3,
        }
    }
}
#[doc = "Reader of field `FQOS`"]
pub type FQOS_R = crate::R<u8, FQOS_A>;
impl FQOS_R {
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
        *self == FQOS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == FQOS_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == FQOS_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == FQOS_A::HIGH
    }
}
#[doc = "Write proxy for field `FQOS`"]
pub struct FQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> FQOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FQOS_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u8) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `DQOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQOS_A {
    #[doc = "Background (no sensitive operation)"]
    DISABLE,
    #[doc = "Sensitive Bandwidth"]
    LOW,
    #[doc = "Sensitive Latency"]
    MEDIUM,
    #[doc = "Critical Latency"]
    HIGH,
}
impl crate::ToBits<u8> for DQOS_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DQOS_A::DISABLE => 0,
            DQOS_A::LOW => 1,
            DQOS_A::MEDIUM => 2,
            DQOS_A::HIGH => 3,
        }
    }
}
#[doc = "Reader of field `DQOS`"]
pub type DQOS_R = crate::R<u8, DQOS_A>;
impl DQOS_R {
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
        *self == DQOS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DQOS_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == DQOS_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DQOS_A::HIGH
    }
}
#[doc = "Write proxy for field `DQOS`"]
pub struct DQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> DQOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQOS_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
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
}
