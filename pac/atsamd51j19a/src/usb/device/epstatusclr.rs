#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::EPSTATUSCLR {
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
    pub const fn reset_value() -> u8 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Proxy"]
pub struct _DTGLOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DTGLOUTW<'a> {
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
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u8) & 0x01) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTGLINW<'a> {
    w: &'a mut W,
}
impl<'a> _DTGLINW<'a> {
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
        self.w.bits &= !(0x01 << 1);
        self.w.bits |= ((value as u8) & 0x01) << 1;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CURBKW<'a> {
    w: &'a mut W,
}
impl<'a> _CURBKW<'a> {
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
        self.w.bits &= !(0x01 << 2);
        self.w.bits |= ((value as u8) & 0x01) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STALLRQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _STALLRQ0W<'a> {
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
        self.w.bits &= !(0x01 << 4);
        self.w.bits |= ((value as u8) & 0x01) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STALLRQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _STALLRQ1W<'a> {
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
        self.w.bits &= !(0x01 << 5);
        self.w.bits |= ((value as u8) & 0x01) << 5;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BK0RDYW<'a> {
    w: &'a mut W,
}
impl<'a> _BK0RDYW<'a> {
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
        self.w.bits &= !(0x01 << 6);
        self.w.bits |= ((value as u8) & 0x01) << 6;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BK1RDYW<'a> {
    w: &'a mut W,
}
impl<'a> _BK1RDYW<'a> {
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
        self.w.bits &= !(0x01 << 7);
        self.w.bits |= ((value as u8) & 0x01) << 7;
        self.w
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Data Toggle OUT Clear"]
    #[inline]
    pub fn dtglout(&mut self) -> _DTGLOUTW {
        _DTGLOUTW { w: self }
    }
    #[doc = "Bit 1 - Data Toggle IN Clear"]
    #[inline]
    pub fn dtglin(&mut self) -> _DTGLINW {
        _DTGLINW { w: self }
    }
    #[doc = "Bit 2 - Current Bank Clear"]
    #[inline]
    pub fn curbk(&mut self) -> _CURBKW {
        _CURBKW { w: self }
    }
    #[doc = "Bit 4 - Stall 0 Request Clear"]
    #[inline]
    pub fn stallrq0(&mut self) -> _STALLRQ0W {
        _STALLRQ0W { w: self }
    }
    #[doc = "Bit 5 - Stall 1 Request Clear"]
    #[inline]
    pub fn stallrq1(&mut self) -> _STALLRQ1W {
        _STALLRQ1W { w: self }
    }
    #[doc = "Bit 6 - Bank 0 Ready Clear"]
    #[inline]
    pub fn bk0rdy(&mut self) -> _BK0RDYW {
        _BK0RDYW { w: self }
    }
    #[doc = "Bit 7 - Bank 1 Ready Clear"]
    #[inline]
    pub fn bk1rdy(&mut self) -> _BK1RDYW {
        _BK1RDYW { w: self }
    }
}
