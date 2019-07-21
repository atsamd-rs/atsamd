#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PCR {
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
        0x0e
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `SDBPWR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDBPWRR {
    #[doc = "Power off"]
    OFF,
    #[doc = "Power on"]
    ON,
}
impl SDBPWRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SDBPWRR::OFF => false,
            SDBPWRR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDBPWRR {
        match value {
            false => SDBPWRR::OFF,
            true => SDBPWRR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == SDBPWRR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == SDBPWRR::ON
    }
}
#[doc = "Possible values of the field `SDBVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDBVSELR {
    #[doc = "1.8V (Typ.)"]
    _1V8,
    #[doc = "3.0V (Typ.)"]
    _3V0,
    #[doc = "3.3V (Typ.)"]
    _3V3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SDBVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDBVSELR::_1V8 => 0x05,
            SDBVSELR::_3V0 => 0x06,
            SDBVSELR::_3V3 => 0x07,
            SDBVSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDBVSELR {
        match value {
            5 => SDBVSELR::_1V8,
            6 => SDBVSELR::_3V0,
            7 => SDBVSELR::_3V3,
            i => SDBVSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1V8`"]
    #[inline]
    pub fn is_1v8(&self) -> bool {
        *self == SDBVSELR::_1V8
    }
    #[doc = "Checks if the value of the field is `_3V0`"]
    #[inline]
    pub fn is_3v0(&self) -> bool {
        *self == SDBVSELR::_3V0
    }
    #[doc = "Checks if the value of the field is `_3V3`"]
    #[inline]
    pub fn is_3v3(&self) -> bool {
        *self == SDBVSELR::_3V3
    }
}
#[doc = "Values that can be written to the field `SDBPWR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDBPWRW {
    #[doc = "Power off"]
    OFF,
    #[doc = "Power on"]
    ON,
}
impl SDBPWRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDBPWRW::OFF => false,
            SDBPWRW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDBPWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SDBPWRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDBPWRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(SDBPWRW::OFF)
    }
    #[doc = "Power on"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(SDBPWRW::ON)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u8) & 0x01) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDBVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDBVSELW {
    #[doc = "1.8V (Typ.)"]
    _1V8,
    #[doc = "3.0V (Typ.)"]
    _3V0,
    #[doc = "3.3V (Typ.)"]
    _3V3,
}
impl SDBVSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDBVSELW::_1V8 => 5,
            SDBVSELW::_3V0 => 6,
            SDBVSELW::_3V3 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDBVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SDBVSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDBVSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1.8V (Typ.)"]
    #[inline]
    pub fn _1v8(self) -> &'a mut W {
        self.variant(SDBVSELW::_1V8)
    }
    #[doc = "3.0V (Typ.)"]
    #[inline]
    pub fn _3v0(self) -> &'a mut W {
        self.variant(SDBVSELW::_3V0)
    }
    #[doc = "3.3V (Typ.)"]
    #[inline]
    pub fn _3v3(self) -> &'a mut W {
        self.variant(SDBVSELW::_3V3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 1);
        self.w.bits |= ((value as u8) & 0x07) << 1;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline]
    pub fn sdbpwr(&self) -> SDBPWRR {
        SDBPWRR::_from(((self.bits >> 0) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline]
    pub fn sdbvsel(&self) -> SDBVSELR {
        SDBVSELR::_from(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline]
    pub fn sdbpwr(&mut self) -> _SDBPWRW {
        _SDBPWRW { w: self }
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline]
    pub fn sdbvsel(&mut self) -> _SDBVSELW {
        _SDBVSELW { w: self }
    }
}
