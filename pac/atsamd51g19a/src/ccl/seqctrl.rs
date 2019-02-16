#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SEQCTRL {
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
#[doc = "Possible values of the field `SEQSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQSELR {
    #[doc = "Sequential logic is disabled"]
    DISABLE,
    #[doc = "D flip flop"]
    DFF,
    #[doc = "JK flip flop"]
    JK,
    #[doc = "D latch"]
    LATCH,
    #[doc = "RS latch"]
    RS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEQSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEQSELR::DISABLE => 0,
            SEQSELR::DFF => 1,
            SEQSELR::JK => 2,
            SEQSELR::LATCH => 3,
            SEQSELR::RS => 4,
            SEQSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEQSELR {
        match value {
            0 => SEQSELR::DISABLE,
            1 => SEQSELR::DFF,
            2 => SEQSELR::JK,
            3 => SEQSELR::LATCH,
            4 => SEQSELR::RS,
            i => SEQSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SEQSELR::DISABLE
    }
    #[doc = "Checks if the value of the field is `DFF`"]
    #[inline]
    pub fn is_dff(&self) -> bool {
        *self == SEQSELR::DFF
    }
    #[doc = "Checks if the value of the field is `JK`"]
    #[inline]
    pub fn is_jk(&self) -> bool {
        *self == SEQSELR::JK
    }
    #[doc = "Checks if the value of the field is `LATCH`"]
    #[inline]
    pub fn is_latch(&self) -> bool {
        *self == SEQSELR::LATCH
    }
    #[doc = "Checks if the value of the field is `RS`"]
    #[inline]
    pub fn is_rs(&self) -> bool {
        *self == SEQSELR::RS
    }
}
#[doc = "Values that can be written to the field `SEQSEL`"]
pub enum SEQSELW {
    #[doc = "Sequential logic is disabled"]
    DISABLE,
    #[doc = "D flip flop"]
    DFF,
    #[doc = "JK flip flop"]
    JK,
    #[doc = "D latch"]
    LATCH,
    #[doc = "RS latch"]
    RS,
}
impl SEQSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SEQSELW::DISABLE => 0,
            SEQSELW::DFF => 1,
            SEQSELW::JK => 2,
            SEQSELW::LATCH => 3,
            SEQSELW::RS => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Sequential logic is disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEQSELW::DISABLE)
    }
    #[doc = "D flip flop"]
    #[inline]
    pub fn dff(self) -> &'a mut W {
        self.variant(SEQSELW::DFF)
    }
    #[doc = "JK flip flop"]
    #[inline]
    pub fn jk(self) -> &'a mut W {
        self.variant(SEQSELW::JK)
    }
    #[doc = "D latch"]
    #[inline]
    pub fn latch(self) -> &'a mut W {
        self.variant(SEQSELW::LATCH)
    }
    #[doc = "RS latch"]
    #[inline]
    pub fn rs(self) -> &'a mut W {
        self.variant(SEQSELW::RS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - Sequential Selection"]
    #[inline]
    pub fn seqsel(&self) -> SEQSELR {
        SEQSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Sequential Selection"]
    #[inline]
    pub fn seqsel(&mut self) -> _SEQSELW {
        _SEQSELW { w: self }
    }
}
