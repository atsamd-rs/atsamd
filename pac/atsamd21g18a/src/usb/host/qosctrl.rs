#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::QOSCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u8 {
        0x05
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `CQOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CQOSR {
    #[doc = "Background (no sensitive operation)"]
    DISABLE,
    #[doc = "Sensitive Bandwidth"]
    LOW,
    #[doc = "Sensitive Latency"]
    MEDIUM,
    #[doc = "Critical Latency"]
    HIGH,
}
impl CQOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CQOSR::DISABLE => 0,
            CQOSR::LOW => 0x01,
            CQOSR::MEDIUM => 0x02,
            CQOSR::HIGH => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CQOSR {
        match value {
            0 => CQOSR::DISABLE,
            1 => CQOSR::LOW,
            2 => CQOSR::MEDIUM,
            3 => CQOSR::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CQOSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CQOSR::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline]
    pub fn is_medium(&self) -> bool {
        *self == CQOSR::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CQOSR::HIGH
    }
}
#[doc = "Possible values of the field `DQOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQOSR {
    #[doc = "Background (no sensitive operation)"]
    DISABLE,
    #[doc = "Sensitive Bandwidth"]
    LOW,
    #[doc = "Sensitive Latency"]
    MEDIUM,
    #[doc = "Critical Latency"]
    HIGH,
}
impl DQOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DQOSR::DISABLE => 0,
            DQOSR::LOW => 0x01,
            DQOSR::MEDIUM => 0x02,
            DQOSR::HIGH => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DQOSR {
        match value {
            0 => DQOSR::DISABLE,
            1 => DQOSR::LOW,
            2 => DQOSR::MEDIUM,
            3 => DQOSR::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DQOSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DQOSR::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline]
    pub fn is_medium(&self) -> bool {
        *self == DQOSR::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DQOSR::HIGH
    }
}
#[doc = "Values that can be written to the field `CQOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CQOSW {
    #[doc = "Background (no sensitive operation)"]
    DISABLE,
    #[doc = "Sensitive Bandwidth"]
    LOW,
    #[doc = "Sensitive Latency"]
    MEDIUM,
    #[doc = "Critical Latency"]
    HIGH,
}
impl CQOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CQOSW::DISABLE => 0,
            CQOSW::LOW => 1,
            CQOSW::MEDIUM => 2,
            CQOSW::HIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CQOSW<'a> {
    w: &'a mut W,
}
impl<'a> _CQOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CQOSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CQOSW::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CQOSW::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(CQOSW::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CQOSW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 0);
        self.w.bits |= ((value as u8) & 0x03) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `DQOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQOSW {
    #[doc = "Background (no sensitive operation)"]
    DISABLE,
    #[doc = "Sensitive Bandwidth"]
    LOW,
    #[doc = "Sensitive Latency"]
    MEDIUM,
    #[doc = "Critical Latency"]
    HIGH,
}
impl DQOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DQOSW::DISABLE => 0,
            DQOSW::LOW => 1,
            DQOSW::MEDIUM => 2,
            DQOSW::HIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DQOSW<'a> {
    w: &'a mut W,
}
impl<'a> _DQOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DQOSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DQOSW::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DQOSW::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(DQOSW::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DQOSW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 2);
        self.w.bits |= ((value as u8) & 0x03) << 2;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline]
    pub fn cqos(&self) -> CQOSR {
        CQOSR::_from(((self.bits >> 0) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline]
    pub fn dqos(&self) -> DQOSR {
        DQOSR::_from(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline]
    pub fn cqos(&mut self) -> _CQOSW {
        _CQOSW { w: self }
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline]
    pub fn dqos(&mut self) -> _DQOSW {
        _DQOSW { w: self }
    }
}
