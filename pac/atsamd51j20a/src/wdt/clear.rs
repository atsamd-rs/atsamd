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
#[doc = "Values that can be written to the field `CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
        self.w.bits &= !(0xff << 0);
        self.w.bits |= ((value as u8) & 0xff) << 0;
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
    #[doc = "Bits 0:7 - Watchdog Clear"]
    #[inline]
    pub fn clear(&mut self) -> _CLEARW {
        _CLEARW { w: self }
    }
}
