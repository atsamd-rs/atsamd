#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WRCTRL {
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
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct PERIDR {
    bits: u16,
}
impl PERIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYR {
    #[doc = "No action"]
    OFF,
    #[doc = "Clear protection"]
    CLR,
    #[doc = "Set protection"]
    SET,
    #[doc = "Set and lock protection"]
    SETLCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KEYR::OFF => 0,
            KEYR::CLR => 0x01,
            KEYR::SET => 0x02,
            KEYR::SETLCK => 0x03,
            KEYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KEYR {
        match value {
            0 => KEYR::OFF,
            1 => KEYR::CLR,
            2 => KEYR::SET,
            3 => KEYR::SETLCK,
            i => KEYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == KEYR::OFF
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline]
    pub fn is_clr(&self) -> bool {
        *self == KEYR::CLR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == KEYR::SET
    }
    #[doc = "Checks if the value of the field is `SETLCK`"]
    #[inline]
    pub fn is_setlck(&self) -> bool {
        *self == KEYR::SETLCK
    }
}
#[doc = r" Proxy"]
pub struct _PERIDW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0xffff << 0);
        self.w.bits |= ((value as u32) & 0xffff) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYW {
    #[doc = "No action"]
    OFF,
    #[doc = "Clear protection"]
    CLR,
    #[doc = "Set protection"]
    SET,
    #[doc = "Set and lock protection"]
    SETLCK,
}
impl KEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            KEYW::OFF => 0,
            KEYW::CLR => 1,
            KEYW::SET => 2,
            KEYW::SETLCK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No action"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(KEYW::OFF)
    }
    #[doc = "Clear protection"]
    #[inline]
    pub fn clr(self) -> &'a mut W {
        self.variant(KEYW::CLR)
    }
    #[doc = "Set protection"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(KEYW::SET)
    }
    #[doc = "Set and lock protection"]
    #[inline]
    pub fn setlck(self) -> &'a mut W {
        self.variant(KEYW::SETLCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0xff << 16);
        self.w.bits |= ((value as u32) & 0xff) << 16;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Peripheral identifier"]
    #[inline]
    pub fn perid(&self) -> PERIDR {
        let bits = ((self.bits >> 0) & 0xffff) as u16;
        PERIDR { bits }
    }
    #[doc = "Bits 16:23 - Peripheral access control key"]
    #[inline]
    pub fn key(&self) -> KEYR {
        KEYR::_from(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Peripheral identifier"]
    #[inline]
    pub fn perid(&mut self) -> _PERIDW {
        _PERIDW { w: self }
    }
    #[doc = "Bits 16:23 - Peripheral access control key"]
    #[inline]
    pub fn key(&mut self) -> _KEYW {
        _KEYW { w: self }
    }
}
