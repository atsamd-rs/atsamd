#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CLEAR {
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
#[doc = "Values that can be written to the field `CLEAR`"]
pub enum CLEARW {
    #[doc = "Clear Key"]
    KEY,
}
impl CLEARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLEARW::KEY => 165,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _CLEARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLEARW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clear Key"]
    #[inline]
    pub fn key(self) -> &'a mut W {
        self.variant(CLEARW::KEY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Watchdog Clear"]
    #[inline]
    pub fn clear(&mut self) -> _CLEARW {
        _CLEARW { w: self }
    }
}
