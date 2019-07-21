#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::RTCCTRL {
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
#[doc = "Possible values of the field `RTCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSELR {
    #[doc = "1.024kHz from 32kHz internal ULP oscillator"]
    ULP1K,
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    ULP32K,
    #[doc = "1.024kHz from 32.768kHz internal oscillator"]
    XOSC1K,
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    XOSC32K,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RTCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTCSELR::ULP1K => 0,
            RTCSELR::ULP32K => 0x01,
            RTCSELR::XOSC1K => 0x04,
            RTCSELR::XOSC32K => 0x05,
            RTCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTCSELR {
        match value {
            0 => RTCSELR::ULP1K,
            1 => RTCSELR::ULP32K,
            4 => RTCSELR::XOSC1K,
            5 => RTCSELR::XOSC32K,
            i => RTCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ULP1K`"]
    #[inline]
    pub fn is_ulp1k(&self) -> bool {
        *self == RTCSELR::ULP1K
    }
    #[doc = "Checks if the value of the field is `ULP32K`"]
    #[inline]
    pub fn is_ulp32k(&self) -> bool {
        *self == RTCSELR::ULP32K
    }
    #[doc = "Checks if the value of the field is `XOSC1K`"]
    #[inline]
    pub fn is_xosc1k(&self) -> bool {
        *self == RTCSELR::XOSC1K
    }
    #[doc = "Checks if the value of the field is `XOSC32K`"]
    #[inline]
    pub fn is_xosc32k(&self) -> bool {
        *self == RTCSELR::XOSC32K
    }
}
#[doc = "Values that can be written to the field `RTCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSELW {
    #[doc = "1.024kHz from 32kHz internal ULP oscillator"]
    ULP1K,
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    ULP32K,
    #[doc = "1.024kHz from 32.768kHz internal oscillator"]
    XOSC1K,
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    XOSC32K,
}
impl RTCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTCSELW::ULP1K => 0,
            RTCSELW::ULP32K => 1,
            RTCSELW::XOSC1K => 4,
            RTCSELW::XOSC32K => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1.024kHz from 32kHz internal ULP oscillator"]
    #[inline]
    pub fn ulp1k(self) -> &'a mut W {
        self.variant(RTCSELW::ULP1K)
    }
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    #[inline]
    pub fn ulp32k(self) -> &'a mut W {
        self.variant(RTCSELW::ULP32K)
    }
    #[doc = "1.024kHz from 32.768kHz internal oscillator"]
    #[inline]
    pub fn xosc1k(self) -> &'a mut W {
        self.variant(RTCSELW::XOSC1K)
    }
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    #[inline]
    pub fn xosc32k(self) -> &'a mut W {
        self.variant(RTCSELW::XOSC32K)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 0);
        self.w.bits |= ((value as u8) & 0x07) << 0;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:2 - RTC Clock Selection"]
    #[inline]
    pub fn rtcsel(&self) -> RTCSELR {
        RTCSELR::_from(((self.bits >> 0) & 0x07) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - RTC Clock Selection"]
    #[inline]
    pub fn rtcsel(&mut self) -> _RTCSELW {
        _RTCSELW { w: self }
    }
}
