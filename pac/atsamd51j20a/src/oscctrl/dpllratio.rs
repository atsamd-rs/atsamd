#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DPLLRATIO {
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
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct LDRR {
    bits: u16,
}
impl LDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LDRFRACR {
    bits: u8,
}
impl LDRFRACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LDRW<'a> {
    w: &'a mut W,
}
impl<'a> _LDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0x1fff << 0);
        self.w.bits |= ((value as u32) & 0x1fff) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LDRFRACW<'a> {
    w: &'a mut W,
}
impl<'a> _LDRFRACW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x1f << 16);
        self.w.bits |= ((value as u32) & 0x1f) << 16;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:12 - Loop Divider Ratio"]
    #[inline]
    pub fn ldr(&self) -> LDRR {
        let bits = ((self.bits >> 0) & 0x1fff) as u16;
        LDRR { bits }
    }
    #[doc = "Bits 16:20 - Loop Divider Ratio Fractional Part"]
    #[inline]
    pub fn ldrfrac(&self) -> LDRFRACR {
        let bits = ((self.bits >> 16) & 0x1f) as u8;
        LDRFRACR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:12 - Loop Divider Ratio"]
    #[inline]
    pub fn ldr(&mut self) -> _LDRW {
        _LDRW { w: self }
    }
    #[doc = "Bits 16:20 - Loop Divider Ratio Fractional Part"]
    #[inline]
    pub fn ldrfrac(&mut self) -> _LDRFRACW {
        _LDRFRACW { w: self }
    }
}
