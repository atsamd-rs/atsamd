#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CC_DITH4 {
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
pub struct DITHERCYR {
    bits: u8,
}
impl DITHERCYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCR {
    bits: u32,
}
impl CCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DITHERCYW<'a> {
    w: &'a mut W,
}
impl<'a> _DITHERCYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 0);
        self.w.bits |= ((value as u32) & 0x0f) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCW<'a> {
    w: &'a mut W,
}
impl<'a> _CCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits &= !(0x000f_ffff << 4);
        self.w.bits |= ((value as u32) & 0x000f_ffff) << 4;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline]
    pub fn dithercy(&self) -> DITHERCYR {
        let bits = ((self.bits >> 0) & 0x0f) as u8;
        DITHERCYR { bits }
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Value"]
    #[inline]
    pub fn cc(&self) -> CCR {
        let bits = ((self.bits >> 4) & 0x000f_ffff) as u32;
        CCR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline]
    pub fn dithercy(&mut self) -> _DITHERCYW {
        _DITHERCYW { w: self }
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Value"]
    #[inline]
    pub fn cc(&mut self) -> _CCW {
        _CCW { w: self }
    }
}
