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
    pub const fn reset_value() -> u8 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
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
            RAMCFGR::PARTIAL => 0x01,
            RAMCFGR::OFF => 0x02,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
        self.w.bits &= !(0x03 << 0);
        self.w.bits |= ((value as u8) & 0x03) << 0;
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
        self.w.bits &= !(0x03 << 4);
        self.w.bits |= ((value as u8) & 0x03) << 4;
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
        RAMCFGR::_from(((self.bits >> 0) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Fast Wakeup"]
    #[inline]
    pub fn fastwkup(&self) -> FASTWKUPR {
        let bits = ((self.bits >> 4) & 0x03) as u8;
        FASTWKUPR { bits }
    }
}
impl W {
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
