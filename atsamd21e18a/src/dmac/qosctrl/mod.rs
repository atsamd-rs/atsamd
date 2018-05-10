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
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
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
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `WRBQOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRBQOSR {
    #[doc = "Background (no sensitive operation)"]
    DISABLE,
    #[doc = "Sensitive Bandwidth"]
    LOW,
    #[doc = "Sensitive Latency"]
    MEDIUM,
    #[doc = "Critical Latency"]
    HIGH,
}
impl WRBQOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WRBQOSR::DISABLE => 0,
            WRBQOSR::LOW => 1,
            WRBQOSR::MEDIUM => 2,
            WRBQOSR::HIGH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WRBQOSR {
        match value {
            0 => WRBQOSR::DISABLE,
            1 => WRBQOSR::LOW,
            2 => WRBQOSR::MEDIUM,
            3 => WRBQOSR::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WRBQOSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WRBQOSR::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline]
    pub fn is_medium(&self) -> bool {
        *self == WRBQOSR::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WRBQOSR::HIGH
    }
}
#[doc = "Possible values of the field `FQOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FQOSR {
    #[doc = "Background (no sensitive operation)"]
    DISABLE,
    #[doc = "Sensitive Bandwidth"]
    LOW,
    #[doc = "Sensitive Latency"]
    MEDIUM,
    #[doc = "Critical Latency"]
    HIGH,
}
impl FQOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FQOSR::DISABLE => 0,
            FQOSR::LOW => 1,
            FQOSR::MEDIUM => 2,
            FQOSR::HIGH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FQOSR {
        match value {
            0 => FQOSR::DISABLE,
            1 => FQOSR::LOW,
            2 => FQOSR::MEDIUM,
            3 => FQOSR::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FQOSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == FQOSR::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline]
    pub fn is_medium(&self) -> bool {
        *self == FQOSR::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == FQOSR::HIGH
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
            DQOSR::LOW => 1,
            DQOSR::MEDIUM => 2,
            DQOSR::HIGH => 3,
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
#[doc = "Values that can be written to the field `WRBQOS`"]
pub enum WRBQOSW {
    #[doc = "Background (no sensitive operation)"]
    DISABLE,
    #[doc = "Sensitive Bandwidth"]
    LOW,
    #[doc = "Sensitive Latency"]
    MEDIUM,
    #[doc = "Critical Latency"]
    HIGH,
}
impl WRBQOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WRBQOSW::DISABLE => 0,
            WRBQOSW::LOW => 1,
            WRBQOSW::MEDIUM => 2,
            WRBQOSW::HIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRBQOSW<'a> {
    w: &'a mut W,
}
impl<'a> _WRBQOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRBQOSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WRBQOSW::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WRBQOSW::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(WRBQOSW::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WRBQOSW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FQOS`"]
pub enum FQOSW {
    #[doc = "Background (no sensitive operation)"]
    DISABLE,
    #[doc = "Sensitive Bandwidth"]
    LOW,
    #[doc = "Sensitive Latency"]
    MEDIUM,
    #[doc = "Critical Latency"]
    HIGH,
}
impl FQOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FQOSW::DISABLE => 0,
            FQOSW::LOW => 1,
            FQOSW::MEDIUM => 2,
            FQOSW::HIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FQOSW<'a> {
    w: &'a mut W,
}
impl<'a> _FQOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FQOSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FQOSW::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(FQOSW::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(FQOSW::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(FQOSW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DQOS`"]
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
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Write-Back Quality of Service"]
    #[inline]
    pub fn wrbqos(&self) -> WRBQOSR {
        WRBQOSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 2:3 - Fetch Quality of Service"]
    #[inline]
    pub fn fqos(&self) -> FQOSR {
        FQOSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Data Transfer Quality of Service"]
    #[inline]
    pub fn dqos(&self) -> DQOSR {
        DQOSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 21 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Write-Back Quality of Service"]
    #[inline]
    pub fn wrbqos(&mut self) -> _WRBQOSW {
        _WRBQOSW { w: self }
    }
    #[doc = "Bits 2:3 - Fetch Quality of Service"]
    #[inline]
    pub fn fqos(&mut self) -> _FQOSW {
        _FQOSW { w: self }
    }
    #[doc = "Bits 4:5 - Data Transfer Quality of Service"]
    #[inline]
    pub fn dqos(&mut self) -> _DQOSW {
        _DQOSW { w: self }
    }
}
