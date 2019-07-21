#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::PADCAL {
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
pub struct TRANSPR {
    bits: u8,
}
impl TRANSPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRANSNR {
    bits: u8,
}
impl TRANSNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIMR {
    bits: u8,
}
impl TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TRANSPW<'a> {
    w: &'a mut W,
}
impl<'a> _TRANSPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x1f << 0);
        self.w.bits |= ((value as u16) & 0x1f) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRANSNW<'a> {
    w: &'a mut W,
}
impl<'a> _TRANSNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x1f << 6);
        self.w.bits |= ((value as u16) & 0x1f) << 6;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 12);
        self.w.bits |= ((value as u16) & 0x07) << 12;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:4 - USB Pad Transp calibration"]
    #[inline]
    pub fn transp(&self) -> TRANSPR {
        let bits = ((self.bits >> 0) & 0x1f) as u8;
        TRANSPR { bits }
    }
    #[doc = "Bits 6:10 - USB Pad Transn calibration"]
    #[inline]
    pub fn transn(&self) -> TRANSNR {
        let bits = ((self.bits >> 6) & 0x1f) as u8;
        TRANSNR { bits }
    }
    #[doc = "Bits 12:14 - USB Pad Trim calibration"]
    #[inline]
    pub fn trim(&self) -> TRIMR {
        let bits = ((self.bits >> 12) & 0x07) as u8;
        TRIMR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - USB Pad Transp calibration"]
    #[inline]
    pub fn transp(&mut self) -> _TRANSPW {
        _TRANSPW { w: self }
    }
    #[doc = "Bits 6:10 - USB Pad Transn calibration"]
    #[inline]
    pub fn transn(&mut self) -> _TRANSNW {
        _TRANSNW { w: self }
    }
    #[doc = "Bits 12:14 - USB Pad Trim calibration"]
    #[inline]
    pub fn trim(&mut self) -> _TRIMW {
        _TRIMW { w: self }
    }
}
