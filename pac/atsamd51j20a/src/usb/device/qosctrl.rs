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
        0x0f
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct CQOSR {
    bits: u8,
}
impl CQOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DQOSR {
    bits: u8,
}
impl DQOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CQOSW<'a> {
    w: &'a mut W,
}
impl<'a> _CQOSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 0);
        self.w.bits |= ((value as u8) & 0x03) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DQOSW<'a> {
    w: &'a mut W,
}
impl<'a> _DQOSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
        let bits = ((self.bits >> 0) & 0x03) as u8;
        CQOSR { bits }
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline]
    pub fn dqos(&self) -> DQOSR {
        let bits = ((self.bits >> 2) & 0x03) as u8;
        DQOSR { bits }
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
