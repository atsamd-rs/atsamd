#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DFLLVAL {
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
pub struct FINER {
    bits: u8,
}
impl FINER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COARSER {
    bits: u8,
}
impl COARSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIFFR {
    bits: u16,
}
impl DIFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _FINEW<'a> {
    w: &'a mut W,
}
impl<'a> _FINEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0xff << 0);
        self.w.bits |= ((value as u32) & 0xff) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COARSEW<'a> {
    w: &'a mut W,
}
impl<'a> _COARSEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x3f << 10);
        self.w.bits |= ((value as u32) & 0x3f) << 10;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Fine Value"]
    #[inline]
    pub fn fine(&self) -> FINER {
        let bits = ((self.bits >> 0) & 0xff) as u8;
        FINER { bits }
    }
    #[doc = "Bits 10:15 - Coarse Value"]
    #[inline]
    pub fn coarse(&self) -> COARSER {
        let bits = ((self.bits >> 10) & 0x3f) as u8;
        COARSER { bits }
    }
    #[doc = "Bits 16:31 - Multiplication Ratio Difference"]
    #[inline]
    pub fn diff(&self) -> DIFFR {
        let bits = ((self.bits >> 16) & 0xffff) as u16;
        DIFFR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Fine Value"]
    #[inline]
    pub fn fine(&mut self) -> _FINEW {
        _FINEW { w: self }
    }
    #[doc = "Bits 10:15 - Coarse Value"]
    #[inline]
    pub fn coarse(&mut self) -> _COARSEW {
        _COARSEW { w: self }
    }
}
