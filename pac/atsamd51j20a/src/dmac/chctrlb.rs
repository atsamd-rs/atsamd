#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CHCTRLB {
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
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDR {
    #[doc = "No action"]
    NOACT,
    #[doc = "Channel suspend operation"]
    SUSPEND,
    #[doc = "Channel resume operation"]
    RESUME,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDR::NOACT => 0,
            CMDR::SUSPEND => 0x01,
            CMDR::RESUME => 0x02,
            CMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDR {
        match value {
            0 => CMDR::NOACT,
            1 => CMDR::SUSPEND,
            2 => CMDR::RESUME,
            i => CMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline]
    pub fn is_noact(&self) -> bool {
        *self == CMDR::NOACT
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline]
    pub fn is_suspend(&self) -> bool {
        *self == CMDR::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline]
    pub fn is_resume(&self) -> bool {
        *self == CMDR::RESUME
    }
}
#[doc = "Values that can be written to the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDW {
    #[doc = "No action"]
    NOACT,
    #[doc = "Channel suspend operation"]
    SUSPEND,
    #[doc = "Channel resume operation"]
    RESUME,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::NOACT => 0,
            CMDW::SUSPEND => 1,
            CMDW::RESUME => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No action"]
    #[inline]
    pub fn noact(self) -> &'a mut W {
        self.variant(CMDW::NOACT)
    }
    #[doc = "Channel suspend operation"]
    #[inline]
    pub fn suspend(self) -> &'a mut W {
        self.variant(CMDW::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline]
    pub fn resume(self) -> &'a mut W {
        self.variant(CMDW::RESUME)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 0);
        self.w.bits |= ((value as u8) & 0x03) << 0;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Software Command"]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        CMDR::_from(((self.bits >> 0) & 0x03) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Software Command"]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
}
