#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BAUD {
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
pub struct BAUDR {
    bits: u8,
}
impl BAUDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BAUDLOWR {
    bits: u8,
}
impl BAUDLOWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSBAUDR {
    bits: u8,
}
impl HSBAUDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSBAUDLOWR {
    bits: u8,
}
impl HSBAUDLOWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BAUDW<'a> {
    w: &'a mut W,
}
impl<'a> _BAUDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0xff << 0);
        self.w.bits |= ((value as u32) & 0xff) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BAUDLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _BAUDLOWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0xff << 8);
        self.w.bits |= ((value as u32) & 0xff) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSBAUDW<'a> {
    w: &'a mut W,
}
impl<'a> _HSBAUDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0xff << 16);
        self.w.bits |= ((value as u32) & 0xff) << 16;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSBAUDLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _HSBAUDLOWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0xff << 24);
        self.w.bits |= ((value as u32) & 0xff) << 24;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Baud Rate Value"]
    #[inline]
    pub fn baud(&self) -> BAUDR {
        let bits = ((self.bits >> 0) & 0xff) as u8;
        BAUDR { bits }
    }
    #[doc = "Bits 8:15 - Baud Rate Value Low"]
    #[inline]
    pub fn baudlow(&self) -> BAUDLOWR {
        let bits = ((self.bits >> 8) & 0xff) as u8;
        BAUDLOWR { bits }
    }
    #[doc = "Bits 16:23 - High Speed Baud Rate Value"]
    #[inline]
    pub fn hsbaud(&self) -> HSBAUDR {
        let bits = ((self.bits >> 16) & 0xff) as u8;
        HSBAUDR { bits }
    }
    #[doc = "Bits 24:31 - High Speed Baud Rate Value Low"]
    #[inline]
    pub fn hsbaudlow(&self) -> HSBAUDLOWR {
        let bits = ((self.bits >> 24) & 0xff) as u8;
        HSBAUDLOWR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Baud Rate Value"]
    #[inline]
    pub fn baud(&mut self) -> _BAUDW {
        _BAUDW { w: self }
    }
    #[doc = "Bits 8:15 - Baud Rate Value Low"]
    #[inline]
    pub fn baudlow(&mut self) -> _BAUDLOWW {
        _BAUDLOWW { w: self }
    }
    #[doc = "Bits 16:23 - High Speed Baud Rate Value"]
    #[inline]
    pub fn hsbaud(&mut self) -> _HSBAUDW {
        _HSBAUDW { w: self }
    }
    #[doc = "Bits 24:31 - High Speed Baud Rate Value Low"]
    #[inline]
    pub fn hsbaudlow(&mut self) -> _HSBAUDLOWW {
        _HSBAUDLOWW { w: self }
    }
}
