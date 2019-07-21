#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::BAUD_FRAC_MODE {
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
pub struct BAUDR {
    bits: u16,
}
impl BAUDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FPR {
    bits: u8,
}
impl FPR {
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
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0x1fff << 0);
        self.w.bits |= ((value as u16) & 0x1fff) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FPW<'a> {
    w: &'a mut W,
}
impl<'a> _FPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 13);
        self.w.bits |= ((value as u16) & 0x07) << 13;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline]
    pub fn baud(&self) -> BAUDR {
        let bits = ((self.bits >> 0) & 0x1fff) as u16;
        BAUDR { bits }
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline]
    pub fn fp(&self) -> FPR {
        let bits = ((self.bits >> 13) & 0x07) as u8;
        FPR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline]
    pub fn baud(&mut self) -> _BAUDW {
        _BAUDW { w: self }
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline]
    pub fn fp(&mut self) -> _FPW {
        _FPW { w: self }
    }
}
