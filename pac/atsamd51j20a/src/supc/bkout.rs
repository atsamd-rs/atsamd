#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BKOUT {
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
pub struct ENR {
    bits: u8,
}
impl ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RTCTGLR {
    bits: u8,
}
impl RTCTGLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 0);
        self.w.bits |= ((value as u32) & 0x03) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 8);
        self.w.bits |= ((value as u32) & 0x03) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 16);
        self.w.bits |= ((value as u32) & 0x03) << 16;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTCTGLW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCTGLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 24);
        self.w.bits |= ((value as u32) & 0x03) << 24;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Enable Output"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = ((self.bits >> 0) & 0x03) as u8;
        ENR { bits }
    }
    #[doc = "Bits 24:25 - RTC Toggle Output"]
    #[inline]
    pub fn rtctgl(&self) -> RTCTGLR {
        let bits = ((self.bits >> 24) & 0x03) as u8;
        RTCTGLR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Enable Output"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bits 8:9 - Clear Output"]
    #[inline]
    pub fn clr(&mut self) -> _CLRW {
        _CLRW { w: self }
    }
    #[doc = "Bits 16:17 - Set Output"]
    #[inline]
    pub fn set(&mut self) -> _SETW {
        _SETW { w: self }
    }
    #[doc = "Bits 24:25 - RTC Toggle Output"]
    #[inline]
    pub fn rtctgl(&mut self) -> _RTCTGLW {
        _RTCTGLW { w: self }
    }
}
