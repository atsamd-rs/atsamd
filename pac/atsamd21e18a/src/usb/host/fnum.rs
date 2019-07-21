#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FNUM {
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
    pub const fn reset_value() -> u16 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct MFNUMR {
    bits: u8,
}
impl MFNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FNUMR {
    bits: u16,
}
impl FNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _MFNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _MFNUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 0);
        self.w.bits |= ((value as u16) & 0x07) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _FNUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0x07ff << 3);
        self.w.bits |= ((value as u16) & 0x07ff) << 3;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline]
    pub fn mfnum(&self) -> MFNUMR {
        let bits = ((self.bits >> 0) & 0x07) as u8;
        MFNUMR { bits }
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline]
    pub fn fnum(&self) -> FNUMR {
        let bits = ((self.bits >> 3) & 0x07ff) as u16;
        FNUMR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline]
    pub fn mfnum(&mut self) -> _MFNUMW {
        _MFNUMW { w: self }
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline]
    pub fn fnum(&mut self) -> _FNUMW {
        _FNUMW { w: self }
    }
}
