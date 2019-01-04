#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAINT1 {
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
pub struct _INDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _INDEXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAY`"]
pub enum WAYW {
    #[doc = "Way 0 is selection for index invalidation"]
    WAY0,
    #[doc = "Way 1 is selection for index invalidation"]
    WAY1,
    #[doc = "Way 2 is selection for index invalidation"]
    WAY2,
    #[doc = "Way 3 is selection for index invalidation"]
    WAY3,
}
impl WAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAYW::WAY0 => 0,
            WAYW::WAY1 => 1,
            WAYW::WAY2 => 2,
            WAYW::WAY3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAYW<'a> {
    w: &'a mut W,
}
impl<'a> _WAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Way 0 is selection for index invalidation"]
    #[inline]
    pub fn way0(self) -> &'a mut W {
        self.variant(WAYW::WAY0)
    }
    #[doc = "Way 1 is selection for index invalidation"]
    #[inline]
    pub fn way1(self) -> &'a mut W {
        self.variant(WAYW::WAY1)
    }
    #[doc = "Way 2 is selection for index invalidation"]
    #[inline]
    pub fn way2(self) -> &'a mut W {
        self.variant(WAYW::WAY2)
    }
    #[doc = "Way 3 is selection for index invalidation"]
    #[inline]
    pub fn way3(self) -> &'a mut W {
        self.variant(WAYW::WAY3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 4:11 - Invalidate Index"]
    #[inline]
    pub fn index(&mut self) -> _INDEXW {
        _INDEXW { w: self }
    }
    #[doc = "Bits 28:31 - Invalidate Way"]
    #[inline]
    pub fn way(&mut self) -> _WAYW {
        _WAYW { w: self }
    }
}
