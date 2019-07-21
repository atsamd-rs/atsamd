#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::WAVE {
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
#[doc = "Possible values of the field `WAVEGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVEGENR {
    #[doc = "Normal frequency"]
    NFRQ,
    #[doc = "Match frequency"]
    MFRQ,
    #[doc = "Normal PWM"]
    NPWM,
    #[doc = "Match PWM"]
    MPWM,
}
impl WAVEGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAVEGENR::NFRQ => 0,
            WAVEGENR::MFRQ => 0x01,
            WAVEGENR::NPWM => 0x02,
            WAVEGENR::MPWM => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAVEGENR {
        match value {
            0 => WAVEGENR::NFRQ,
            1 => WAVEGENR::MFRQ,
            2 => WAVEGENR::NPWM,
            3 => WAVEGENR::MPWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NFRQ`"]
    #[inline]
    pub fn is_nfrq(&self) -> bool {
        *self == WAVEGENR::NFRQ
    }
    #[doc = "Checks if the value of the field is `MFRQ`"]
    #[inline]
    pub fn is_mfrq(&self) -> bool {
        *self == WAVEGENR::MFRQ
    }
    #[doc = "Checks if the value of the field is `NPWM`"]
    #[inline]
    pub fn is_npwm(&self) -> bool {
        *self == WAVEGENR::NPWM
    }
    #[doc = "Checks if the value of the field is `MPWM`"]
    #[inline]
    pub fn is_mpwm(&self) -> bool {
        *self == WAVEGENR::MPWM
    }
}
#[doc = "Values that can be written to the field `WAVEGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVEGENW {
    #[doc = "Normal frequency"]
    NFRQ,
    #[doc = "Match frequency"]
    MFRQ,
    #[doc = "Normal PWM"]
    NPWM,
    #[doc = "Match PWM"]
    MPWM,
}
impl WAVEGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAVEGENW::NFRQ => 0,
            WAVEGENW::MFRQ => 1,
            WAVEGENW::NPWM => 2,
            WAVEGENW::MPWM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAVEGENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAVEGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAVEGENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal frequency"]
    #[inline]
    pub fn nfrq(self) -> &'a mut W {
        self.variant(WAVEGENW::NFRQ)
    }
    #[doc = "Match frequency"]
    #[inline]
    pub fn mfrq(self) -> &'a mut W {
        self.variant(WAVEGENW::MFRQ)
    }
    #[doc = "Normal PWM"]
    #[inline]
    pub fn npwm(self) -> &'a mut W {
        self.variant(WAVEGENW::NPWM)
    }
    #[doc = "Match PWM"]
    #[inline]
    pub fn mpwm(self) -> &'a mut W {
        self.variant(WAVEGENW::MPWM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline]
    pub fn wavegen(&self) -> WAVEGENR {
        WAVEGENR::_from(((self.bits >> 0) & 0x03) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline]
    pub fn wavegen(&mut self) -> _WAVEGENW {
        _WAVEGENW { w: self }
    }
}
