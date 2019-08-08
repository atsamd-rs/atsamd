#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::USER {
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
#[doc = r" Value of the field"]
pub struct USERR {
    bits: u8,
}
impl USERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CHANNEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNELR {
    #[doc = "No Channel Output Selected"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CHANNELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHANNELR::_0 => 0,
            CHANNELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHANNELR {
        match value {
            0 => CHANNELR::_0,
            i => CHANNELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CHANNELR::_0
    }
}
#[doc = r" Proxy"]
pub struct _USERW<'a> {
    w: &'a mut W,
}
impl<'a> _USERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL`"]
pub enum CHANNELW {
    #[doc = "No Channel Output Selected"]
    _0,
}
impl CHANNELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHANNELW::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNELW<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Channel Output Selected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHANNELW::_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:4 - User Multiplexer Selection"]
    #[inline]
    pub fn user(&self) -> USERR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        USERR { bits }
    }
    #[doc = "Bits 8:12 - Channel Event Selection"]
    #[inline]
    pub fn channel(&self) -> CHANNELR {
        CHANNELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - User Multiplexer Selection"]
    #[inline]
    pub fn user(&mut self) -> _USERW {
        _USERW { w: self }
    }
    #[doc = "Bits 8:12 - Channel Event Selection"]
    #[inline]
    pub fn channel(&mut self) -> _CHANNELW {
        _CHANNELW { w: self }
    }
}
