#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DFLLMUL {
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
pub struct MULR {
    bits: u16,
}
impl MULR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FSTEPR {
    bits: u16,
}
impl FSTEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CSTEPR {
    bits: u8,
}
impl CSTEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _MULW<'a> {
    w: &'a mut W,
}
impl<'a> _MULW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0xffff << 0);
        self.w.bits |= ((value as u32) & 0xffff) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTEPW<'a> {
    w: &'a mut W,
}
impl<'a> _FSTEPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0x03ff << 16);
        self.w.bits |= ((value as u32) & 0x03ff) << 16;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSTEPW<'a> {
    w: &'a mut W,
}
impl<'a> _CSTEPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x3f << 26);
        self.w.bits |= ((value as u32) & 0x3f) << 26;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline]
    pub fn mul(&self) -> MULR {
        let bits = ((self.bits >> 0) & 0xffff) as u16;
        MULR { bits }
    }
    #[doc = "Bits 16:25 - Fine Maximum Step"]
    #[inline]
    pub fn fstep(&self) -> FSTEPR {
        let bits = ((self.bits >> 16) & 0x03ff) as u16;
        FSTEPR { bits }
    }
    #[doc = "Bits 26:31 - Coarse Maximum Step"]
    #[inline]
    pub fn cstep(&self) -> CSTEPR {
        let bits = ((self.bits >> 26) & 0x3f) as u8;
        CSTEPR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline]
    pub fn mul(&mut self) -> _MULW {
        _MULW { w: self }
    }
    #[doc = "Bits 16:25 - Fine Maximum Step"]
    #[inline]
    pub fn fstep(&mut self) -> _FSTEPW {
        _FSTEPW { w: self }
    }
    #[doc = "Bits 26:31 - Coarse Maximum Step"]
    #[inline]
    pub fn cstep(&mut self) -> _CSTEPW {
        _CSTEPW { w: self }
    }
}
