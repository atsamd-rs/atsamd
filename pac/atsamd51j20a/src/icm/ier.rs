#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IER {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = r" Proxy"]
pub struct _RHCW<'a> {
    w: &'a mut W,
}
impl<'a> _RHCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RDMW<'a> {
    w: &'a mut W,
}
impl<'a> _RDMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RBEW<'a> {
    w: &'a mut W,
}
impl<'a> _RBEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RWCW<'a> {
    w: &'a mut W,
}
impl<'a> _RWCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RECW<'a> {
    w: &'a mut W,
}
impl<'a> _RECW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSUW<'a> {
    w: &'a mut W,
}
impl<'a> _RSUW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _URADW<'a> {
    w: &'a mut W,
}
impl<'a> _URADW<'a> {
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
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Enable"]
    #[inline]
    pub fn rhc(&mut self) -> _RHCW {
        _RHCW { w: self }
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Enable"]
    #[inline]
    pub fn rdm(&mut self) -> _RDMW {
        _RDMW { w: self }
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Enable"]
    #[inline]
    pub fn rbe(&mut self) -> _RBEW {
        _RBEW { w: self }
    }
    #[doc = "Bits 12:15 - Region Wrap Condition detected Interrupt Enable"]
    #[inline]
    pub fn rwc(&mut self) -> _RWCW {
        _RWCW { w: self }
    }
    #[doc = "Bits 16:19 - Region End bit Condition Detected Interrupt Enable"]
    #[inline]
    pub fn rec(&mut self) -> _RECW {
        _RECW { w: self }
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Disable"]
    #[inline]
    pub fn rsu(&mut self) -> _RSUW {
        _RSUW { w: self }
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Enable"]
    #[inline]
    pub fn urad(&mut self) -> _URADW {
        _URADW { w: self }
    }
}
