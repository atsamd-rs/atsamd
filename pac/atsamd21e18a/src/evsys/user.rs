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
    pub const fn reset_value() -> u16 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
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
        self.w.bits &= !(0x1f << 0);
        self.w.bits |= ((value as u16) & 0x1f) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
        self.w.bits &= !(0x1f << 8);
        self.w.bits |= ((value as u16) & 0x1f) << 8;
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
        let bits = ((self.bits >> 0) & 0x1f) as u8;
        USERR { bits }
    }
    #[doc = "Bits 8:12 - Channel Event Selection"]
    #[inline]
    pub fn channel(&self) -> CHANNELR {
        CHANNELR::_from(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
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
