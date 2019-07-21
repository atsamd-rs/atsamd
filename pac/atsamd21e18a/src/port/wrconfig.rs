#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WRCONFIG {
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
#[doc = r" Proxy"]
pub struct _PINMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PINMASKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0xffff << 0);
        self.w.bits |= ((value as u32) & 0xffff) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PMUXENW<'a> {
    w: &'a mut W,
}
impl<'a> _PMUXENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 16);
        self.w.bits |= ((value as u32) & 0x01) << 16;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INENW<'a> {
    w: &'a mut W,
}
impl<'a> _INENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 17);
        self.w.bits |= ((value as u32) & 0x01) << 17;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PULLENW<'a> {
    w: &'a mut W,
}
impl<'a> _PULLENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 18);
        self.w.bits |= ((value as u32) & 0x01) << 18;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DRVSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _DRVSTRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 22);
        self.w.bits |= ((value as u32) & 0x01) << 22;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PMUXW<'a> {
    w: &'a mut W,
}
impl<'a> _PMUXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 24);
        self.w.bits |= ((value as u32) & 0x0f) << 24;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRPMUXW<'a> {
    w: &'a mut W,
}
impl<'a> _WRPMUXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 28);
        self.w.bits |= ((value as u32) & 0x01) << 28;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRPINCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _WRPINCFGW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 30);
        self.w.bits |= ((value as u32) & 0x01) << 30;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HWSELW<'a> {
    w: &'a mut W,
}
impl<'a> _HWSELW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 31);
        self.w.bits |= ((value as u32) & 0x01) << 31;
        self.w
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Pin Mask for Multiple Pin Configuration"]
    #[inline]
    pub fn pinmask(&mut self) -> _PINMASKW {
        _PINMASKW { w: self }
    }
    #[doc = "Bit 16 - Peripheral Multiplexer Enable"]
    #[inline]
    pub fn pmuxen(&mut self) -> _PMUXENW {
        _PMUXENW { w: self }
    }
    #[doc = "Bit 17 - Input Enable"]
    #[inline]
    pub fn inen(&mut self) -> _INENW {
        _INENW { w: self }
    }
    #[doc = "Bit 18 - Pull Enable"]
    #[inline]
    pub fn pullen(&mut self) -> _PULLENW {
        _PULLENW { w: self }
    }
    #[doc = "Bit 22 - Output Driver Strength Selection"]
    #[inline]
    pub fn drvstr(&mut self) -> _DRVSTRW {
        _DRVSTRW { w: self }
    }
    #[doc = "Bits 24:27 - Peripheral Multiplexing"]
    #[inline]
    pub fn pmux(&mut self) -> _PMUXW {
        _PMUXW { w: self }
    }
    #[doc = "Bit 28 - Write PMUX"]
    #[inline]
    pub fn wrpmux(&mut self) -> _WRPMUXW {
        _WRPMUXW { w: self }
    }
    #[doc = "Bit 30 - Write PINCFG"]
    #[inline]
    pub fn wrpincfg(&mut self) -> _WRPINCFGW {
        _WRPINCFGW { w: self }
    }
    #[doc = "Bit 31 - Half-Word Select"]
    #[inline]
    pub fn hwsel(&mut self) -> _HWSELW {
        _HWSELW { w: self }
    }
}
