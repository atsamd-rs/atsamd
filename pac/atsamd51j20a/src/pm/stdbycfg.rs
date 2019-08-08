#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::STDBYCFG {
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
#[doc = "Possible values of the field `RAMCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMCFGR {
    #[doc = "All the RAMs are retained"]
    RET,
    #[doc = "Only the first 32K bytes are retained"]
    PARTIAL,
    #[doc = "All the RAMs are OFF"]
    OFF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAMCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMCFGR::RET => 0,
            RAMCFGR::PARTIAL => 1,
            RAMCFGR::OFF => 2,
            RAMCFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMCFGR {
        match value {
            0 => RAMCFGR::RET,
            1 => RAMCFGR::PARTIAL,
            2 => RAMCFGR::OFF,
            i => RAMCFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RET`"]
    #[inline]
    pub fn is_ret(&self) -> bool {
        *self == RAMCFGR::RET
    }
    #[doc = "Checks if the value of the field is `PARTIAL`"]
    #[inline]
    pub fn is_partial(&self) -> bool {
        *self == RAMCFGR::PARTIAL
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == RAMCFGR::OFF
    }
}
#[doc = r" Value of the field"]
pub struct FASTWKUPR {
    bits: u8,
}
impl FASTWKUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `RAMCFG`"]
pub enum RAMCFGW {
    #[doc = "All the RAMs are retained"]
    RET,
    #[doc = "Only the first 32K bytes are retained"]
    PARTIAL,
    #[doc = "All the RAMs are OFF"]
    OFF,
}
impl RAMCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAMCFGW::RET => 0,
            RAMCFGW::PARTIAL => 1,
            RAMCFGW::OFF => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "All the RAMs are retained"]
    #[inline]
    pub fn ret(self) -> &'a mut W {
        self.variant(RAMCFGW::RET)
    }
    #[doc = "Only the first 32K bytes are retained"]
    #[inline]
    pub fn partial(self) -> &'a mut W {
        self.variant(RAMCFGW::PARTIAL)
    }
    #[doc = "All the RAMs are OFF"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(RAMCFGW::OFF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FASTWKUPW<'a> {
    w: &'a mut W,
}
impl<'a> _FASTWKUPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline]
    pub fn ramcfg(&self) -> RAMCFGR {
        RAMCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Fast Wakeup"]
    #[inline]
    pub fn fastwkup(&self) -> FASTWKUPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        FASTWKUPR { bits }
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
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline]
    pub fn ramcfg(&mut self) -> _RAMCFGW {
        _RAMCFGW { w: self }
    }
    #[doc = "Bits 4:5 - Fast Wakeup"]
    #[inline]
    pub fn fastwkup(&mut self) -> _FASTWKUPW {
        _FASTWKUPW { w: self }
    }
}
