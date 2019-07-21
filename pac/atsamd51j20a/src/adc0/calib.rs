#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CALIB {
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
pub struct BIASCOMPR {
    bits: u8,
}
impl BIASCOMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BIASR2RR {
    bits: u8,
}
impl BIASR2RR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BIASREFBUFR {
    bits: u8,
}
impl BIASREFBUFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BIASCOMPW<'a> {
    w: &'a mut W,
}
impl<'a> _BIASCOMPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 0);
        self.w.bits |= ((value as u16) & 0x07) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BIASR2RW<'a> {
    w: &'a mut W,
}
impl<'a> _BIASR2RW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 4);
        self.w.bits |= ((value as u16) & 0x07) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BIASREFBUFW<'a> {
    w: &'a mut W,
}
impl<'a> _BIASREFBUFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 8);
        self.w.bits |= ((value as u16) & 0x07) << 8;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:2 - Bias Comparator Scaling"]
    #[inline]
    pub fn biascomp(&self) -> BIASCOMPR {
        let bits = ((self.bits >> 0) & 0x07) as u8;
        BIASCOMPR { bits }
    }
    #[doc = "Bits 4:6 - Bias R2R Ampli scaling"]
    #[inline]
    pub fn biasr2r(&self) -> BIASR2RR {
        let bits = ((self.bits >> 4) & 0x07) as u8;
        BIASR2RR { bits }
    }
    #[doc = "Bits 8:10 - Bias Reference Buffer Scaling"]
    #[inline]
    pub fn biasrefbuf(&self) -> BIASREFBUFR {
        let bits = ((self.bits >> 8) & 0x07) as u8;
        BIASREFBUFR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Bias Comparator Scaling"]
    #[inline]
    pub fn biascomp(&mut self) -> _BIASCOMPW {
        _BIASCOMPW { w: self }
    }
    #[doc = "Bits 4:6 - Bias R2R Ampli scaling"]
    #[inline]
    pub fn biasr2r(&mut self) -> _BIASR2RW {
        _BIASR2RW { w: self }
    }
    #[doc = "Bits 8:10 - Bias Reference Buffer Scaling"]
    #[inline]
    pub fn biasrefbuf(&mut self) -> _BIASREFBUFW {
        _BIASREFBUFW { w: self }
    }
}
