#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PER_DITH6 {
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
        0xffff_ffff
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct DITHERR {
    bits: u8,
}
impl DITHERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PERR {
    bits: u32,
}
impl PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DITHERW<'a> {
    w: &'a mut W,
}
impl<'a> _DITHERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x3f << 0);
        self.w.bits |= ((value as u32) & 0x3f) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERW<'a> {
    w: &'a mut W,
}
impl<'a> _PERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits &= !(0x0003_ffff << 6);
        self.w.bits |= ((value as u32) & 0x0003_ffff) << 6;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Dithering Cycle Number"]
    #[inline]
    pub fn dither(&self) -> DITHERR {
        let bits = ((self.bits >> 0) & 0x3f) as u8;
        DITHERR { bits }
    }
    #[doc = "Bits 6:23 - Period Value"]
    #[inline]
    pub fn per(&self) -> PERR {
        let bits = ((self.bits >> 6) & 0x0003_ffff) as u32;
        PERR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Dithering Cycle Number"]
    #[inline]
    pub fn dither(&mut self) -> _DITHERW {
        _DITHERW { w: self }
    }
    #[doc = "Bits 6:23 - Period Value"]
    #[inline]
    pub fn per(&mut self) -> _PERW {
        _PERW { w: self }
    }
}
