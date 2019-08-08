#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
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
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `BMAX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMAXR {
    #[doc = "undocumented"]
    INCR16,
    #[doc = "undocumented"]
    INCR8,
    #[doc = "undocumented"]
    INCR4,
    #[doc = "undocumented"]
    SINGLE,
}
impl BMAXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BMAXR::INCR16 => 0,
            BMAXR::INCR8 => 1,
            BMAXR::INCR4 => 2,
            BMAXR::SINGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BMAXR {
        match value {
            0 => BMAXR::INCR16,
            1 => BMAXR::INCR8,
            2 => BMAXR::INCR4,
            3 => BMAXR::SINGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INCR16`"]
    #[inline]
    pub fn is_incr16(&self) -> bool {
        *self == BMAXR::INCR16
    }
    #[doc = "Checks if the value of the field is `INCR8`"]
    #[inline]
    pub fn is_incr8(&self) -> bool {
        *self == BMAXR::INCR8
    }
    #[doc = "Checks if the value of the field is `INCR4`"]
    #[inline]
    pub fn is_incr4(&self) -> bool {
        *self == BMAXR::INCR4
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == BMAXR::SINGLE
    }
}
#[doc = "Values that can be written to the field `BMAX`"]
pub enum BMAXW {
    #[doc = "`0`"]
    INCR16,
    #[doc = "`1`"]
    INCR8,
    #[doc = "`10`"]
    INCR4,
    #[doc = "`11`"]
    SINGLE,
}
impl BMAXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BMAXW::INCR16 => 0,
            BMAXW::INCR8 => 1,
            BMAXW::INCR4 => 2,
            BMAXW::SINGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BMAXW<'a> {
    w: &'a mut W,
}
impl<'a> _BMAXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BMAXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn incr16(self) -> &'a mut W {
        self.variant(BMAXW::INCR16)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn incr8(self) -> &'a mut W {
        self.variant(BMAXW::INCR8)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn incr4(self) -> &'a mut W {
        self.variant(BMAXW::INCR4)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(BMAXW::SINGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - AHB Maximum Burst"]
    #[inline]
    pub fn bmax(&self) -> BMAXR {
        BMAXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - AHB Maximum Burst"]
    #[inline]
    pub fn bmax(&mut self) -> _BMAXW {
        _BMAXW { w: self }
    }
}
