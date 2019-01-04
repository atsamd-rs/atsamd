#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::INPUTCTRL {
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
#[doc = "Possible values of the field `MUXPOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXPOSR {
    #[doc = "ADC AIN0 Pin"]
    AIN0,
    #[doc = "ADC AIN1 Pin"]
    AIN1,
    #[doc = "ADC AIN2 Pin"]
    AIN2,
    #[doc = "ADC AIN3 Pin"]
    AIN3,
    #[doc = "ADC AIN4 Pin"]
    AIN4,
    #[doc = "ADC AIN5 Pin"]
    AIN5,
    #[doc = "ADC AIN6 Pin"]
    AIN6,
    #[doc = "ADC AIN7 Pin"]
    AIN7,
    #[doc = "ADC AIN8 Pin"]
    AIN8,
    #[doc = "ADC AIN9 Pin"]
    AIN9,
    #[doc = "ADC AIN10 Pin"]
    AIN10,
    #[doc = "ADC AIN11 Pin"]
    AIN11,
    #[doc = "ADC AIN12 Pin"]
    AIN12,
    #[doc = "ADC AIN13 Pin"]
    AIN13,
    #[doc = "ADC AIN14 Pin"]
    AIN14,
    #[doc = "ADC AIN15 Pin"]
    AIN15,
    #[doc = "ADC AIN16 Pin"]
    AIN16,
    #[doc = "ADC AIN17 Pin"]
    AIN17,
    #[doc = "ADC AIN18 Pin"]
    AIN18,
    #[doc = "ADC AIN19 Pin"]
    AIN19,
    #[doc = "ADC AIN20 Pin"]
    AIN20,
    #[doc = "ADC AIN21 Pin"]
    AIN21,
    #[doc = "ADC AIN22 Pin"]
    AIN22,
    #[doc = "ADC AIN23 Pin"]
    AIN23,
    #[doc = "1/4 Scaled Core Supply"]
    SCALEDCOREVCC,
    #[doc = "1/4 Scaled VBAT Supply"]
    SCALEDVBAT,
    #[doc = "1/4 Scaled I/O Supply"]
    SCALEDIOVCC,
    #[doc = "Bandgap Voltage"]
    BANDGAP,
    #[doc = "Temperature Sensor"]
    PTAT,
    #[doc = "Temperature Sensor"]
    CTAT,
    #[doc = "DAC Output"]
    DAC,
    #[doc = "PTC output (only on ADC0)"]
    PTC,
}
impl MUXPOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUXPOSR::AIN0 => 0,
            MUXPOSR::AIN1 => 1,
            MUXPOSR::AIN2 => 2,
            MUXPOSR::AIN3 => 3,
            MUXPOSR::AIN4 => 4,
            MUXPOSR::AIN5 => 5,
            MUXPOSR::AIN6 => 6,
            MUXPOSR::AIN7 => 7,
            MUXPOSR::AIN8 => 8,
            MUXPOSR::AIN9 => 9,
            MUXPOSR::AIN10 => 10,
            MUXPOSR::AIN11 => 11,
            MUXPOSR::AIN12 => 12,
            MUXPOSR::AIN13 => 13,
            MUXPOSR::AIN14 => 14,
            MUXPOSR::AIN15 => 15,
            MUXPOSR::AIN16 => 16,
            MUXPOSR::AIN17 => 17,
            MUXPOSR::AIN18 => 18,
            MUXPOSR::AIN19 => 19,
            MUXPOSR::AIN20 => 20,
            MUXPOSR::AIN21 => 21,
            MUXPOSR::AIN22 => 22,
            MUXPOSR::AIN23 => 23,
            MUXPOSR::SCALEDCOREVCC => 24,
            MUXPOSR::SCALEDVBAT => 25,
            MUXPOSR::SCALEDIOVCC => 26,
            MUXPOSR::BANDGAP => 27,
            MUXPOSR::PTAT => 28,
            MUXPOSR::CTAT => 29,
            MUXPOSR::DAC => 30,
            MUXPOSR::PTC => 31,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUXPOSR {
        match value {
            0 => MUXPOSR::AIN0,
            1 => MUXPOSR::AIN1,
            2 => MUXPOSR::AIN2,
            3 => MUXPOSR::AIN3,
            4 => MUXPOSR::AIN4,
            5 => MUXPOSR::AIN5,
            6 => MUXPOSR::AIN6,
            7 => MUXPOSR::AIN7,
            8 => MUXPOSR::AIN8,
            9 => MUXPOSR::AIN9,
            10 => MUXPOSR::AIN10,
            11 => MUXPOSR::AIN11,
            12 => MUXPOSR::AIN12,
            13 => MUXPOSR::AIN13,
            14 => MUXPOSR::AIN14,
            15 => MUXPOSR::AIN15,
            16 => MUXPOSR::AIN16,
            17 => MUXPOSR::AIN17,
            18 => MUXPOSR::AIN18,
            19 => MUXPOSR::AIN19,
            20 => MUXPOSR::AIN20,
            21 => MUXPOSR::AIN21,
            22 => MUXPOSR::AIN22,
            23 => MUXPOSR::AIN23,
            24 => MUXPOSR::SCALEDCOREVCC,
            25 => MUXPOSR::SCALEDVBAT,
            26 => MUXPOSR::SCALEDIOVCC,
            27 => MUXPOSR::BANDGAP,
            28 => MUXPOSR::PTAT,
            29 => MUXPOSR::CTAT,
            30 => MUXPOSR::DAC,
            31 => MUXPOSR::PTC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AIN0`"]
    #[inline]
    pub fn is_ain0(&self) -> bool {
        *self == MUXPOSR::AIN0
    }
    #[doc = "Checks if the value of the field is `AIN1`"]
    #[inline]
    pub fn is_ain1(&self) -> bool {
        *self == MUXPOSR::AIN1
    }
    #[doc = "Checks if the value of the field is `AIN2`"]
    #[inline]
    pub fn is_ain2(&self) -> bool {
        *self == MUXPOSR::AIN2
    }
    #[doc = "Checks if the value of the field is `AIN3`"]
    #[inline]
    pub fn is_ain3(&self) -> bool {
        *self == MUXPOSR::AIN3
    }
    #[doc = "Checks if the value of the field is `AIN4`"]
    #[inline]
    pub fn is_ain4(&self) -> bool {
        *self == MUXPOSR::AIN4
    }
    #[doc = "Checks if the value of the field is `AIN5`"]
    #[inline]
    pub fn is_ain5(&self) -> bool {
        *self == MUXPOSR::AIN5
    }
    #[doc = "Checks if the value of the field is `AIN6`"]
    #[inline]
    pub fn is_ain6(&self) -> bool {
        *self == MUXPOSR::AIN6
    }
    #[doc = "Checks if the value of the field is `AIN7`"]
    #[inline]
    pub fn is_ain7(&self) -> bool {
        *self == MUXPOSR::AIN7
    }
    #[doc = "Checks if the value of the field is `AIN8`"]
    #[inline]
    pub fn is_ain8(&self) -> bool {
        *self == MUXPOSR::AIN8
    }
    #[doc = "Checks if the value of the field is `AIN9`"]
    #[inline]
    pub fn is_ain9(&self) -> bool {
        *self == MUXPOSR::AIN9
    }
    #[doc = "Checks if the value of the field is `AIN10`"]
    #[inline]
    pub fn is_ain10(&self) -> bool {
        *self == MUXPOSR::AIN10
    }
    #[doc = "Checks if the value of the field is `AIN11`"]
    #[inline]
    pub fn is_ain11(&self) -> bool {
        *self == MUXPOSR::AIN11
    }
    #[doc = "Checks if the value of the field is `AIN12`"]
    #[inline]
    pub fn is_ain12(&self) -> bool {
        *self == MUXPOSR::AIN12
    }
    #[doc = "Checks if the value of the field is `AIN13`"]
    #[inline]
    pub fn is_ain13(&self) -> bool {
        *self == MUXPOSR::AIN13
    }
    #[doc = "Checks if the value of the field is `AIN14`"]
    #[inline]
    pub fn is_ain14(&self) -> bool {
        *self == MUXPOSR::AIN14
    }
    #[doc = "Checks if the value of the field is `AIN15`"]
    #[inline]
    pub fn is_ain15(&self) -> bool {
        *self == MUXPOSR::AIN15
    }
    #[doc = "Checks if the value of the field is `AIN16`"]
    #[inline]
    pub fn is_ain16(&self) -> bool {
        *self == MUXPOSR::AIN16
    }
    #[doc = "Checks if the value of the field is `AIN17`"]
    #[inline]
    pub fn is_ain17(&self) -> bool {
        *self == MUXPOSR::AIN17
    }
    #[doc = "Checks if the value of the field is `AIN18`"]
    #[inline]
    pub fn is_ain18(&self) -> bool {
        *self == MUXPOSR::AIN18
    }
    #[doc = "Checks if the value of the field is `AIN19`"]
    #[inline]
    pub fn is_ain19(&self) -> bool {
        *self == MUXPOSR::AIN19
    }
    #[doc = "Checks if the value of the field is `AIN20`"]
    #[inline]
    pub fn is_ain20(&self) -> bool {
        *self == MUXPOSR::AIN20
    }
    #[doc = "Checks if the value of the field is `AIN21`"]
    #[inline]
    pub fn is_ain21(&self) -> bool {
        *self == MUXPOSR::AIN21
    }
    #[doc = "Checks if the value of the field is `AIN22`"]
    #[inline]
    pub fn is_ain22(&self) -> bool {
        *self == MUXPOSR::AIN22
    }
    #[doc = "Checks if the value of the field is `AIN23`"]
    #[inline]
    pub fn is_ain23(&self) -> bool {
        *self == MUXPOSR::AIN23
    }
    #[doc = "Checks if the value of the field is `SCALEDCOREVCC`"]
    #[inline]
    pub fn is_scaledcorevcc(&self) -> bool {
        *self == MUXPOSR::SCALEDCOREVCC
    }
    #[doc = "Checks if the value of the field is `SCALEDVBAT`"]
    #[inline]
    pub fn is_scaledvbat(&self) -> bool {
        *self == MUXPOSR::SCALEDVBAT
    }
    #[doc = "Checks if the value of the field is `SCALEDIOVCC`"]
    #[inline]
    pub fn is_scalediovcc(&self) -> bool {
        *self == MUXPOSR::SCALEDIOVCC
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXPOSR::BANDGAP
    }
    #[doc = "Checks if the value of the field is `PTAT`"]
    #[inline]
    pub fn is_ptat(&self) -> bool {
        *self == MUXPOSR::PTAT
    }
    #[doc = "Checks if the value of the field is `CTAT`"]
    #[inline]
    pub fn is_ctat(&self) -> bool {
        *self == MUXPOSR::CTAT
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == MUXPOSR::DAC
    }
    #[doc = "Checks if the value of the field is `PTC`"]
    #[inline]
    pub fn is_ptc(&self) -> bool {
        *self == MUXPOSR::PTC
    }
}
#[doc = r" Value of the field"]
pub struct DIFFMODER {
    bits: bool,
}
impl DIFFMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `MUXNEG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXNEGR {
    #[doc = "ADC AIN0 Pin"]
    AIN0,
    #[doc = "ADC AIN1 Pin"]
    AIN1,
    #[doc = "ADC AIN2 Pin"]
    AIN2,
    #[doc = "ADC AIN3 Pin"]
    AIN3,
    #[doc = "ADC AIN4 Pin"]
    AIN4,
    #[doc = "ADC AIN5 Pin"]
    AIN5,
    #[doc = "ADC AIN6 Pin"]
    AIN6,
    #[doc = "ADC AIN7 Pin"]
    AIN7,
    #[doc = "Internal Ground"]
    GND,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MUXNEGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUXNEGR::AIN0 => 0,
            MUXNEGR::AIN1 => 1,
            MUXNEGR::AIN2 => 2,
            MUXNEGR::AIN3 => 3,
            MUXNEGR::AIN4 => 4,
            MUXNEGR::AIN5 => 5,
            MUXNEGR::AIN6 => 6,
            MUXNEGR::AIN7 => 7,
            MUXNEGR::GND => 24,
            MUXNEGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUXNEGR {
        match value {
            0 => MUXNEGR::AIN0,
            1 => MUXNEGR::AIN1,
            2 => MUXNEGR::AIN2,
            3 => MUXNEGR::AIN3,
            4 => MUXNEGR::AIN4,
            5 => MUXNEGR::AIN5,
            6 => MUXNEGR::AIN6,
            7 => MUXNEGR::AIN7,
            24 => MUXNEGR::GND,
            i => MUXNEGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AIN0`"]
    #[inline]
    pub fn is_ain0(&self) -> bool {
        *self == MUXNEGR::AIN0
    }
    #[doc = "Checks if the value of the field is `AIN1`"]
    #[inline]
    pub fn is_ain1(&self) -> bool {
        *self == MUXNEGR::AIN1
    }
    #[doc = "Checks if the value of the field is `AIN2`"]
    #[inline]
    pub fn is_ain2(&self) -> bool {
        *self == MUXNEGR::AIN2
    }
    #[doc = "Checks if the value of the field is `AIN3`"]
    #[inline]
    pub fn is_ain3(&self) -> bool {
        *self == MUXNEGR::AIN3
    }
    #[doc = "Checks if the value of the field is `AIN4`"]
    #[inline]
    pub fn is_ain4(&self) -> bool {
        *self == MUXNEGR::AIN4
    }
    #[doc = "Checks if the value of the field is `AIN5`"]
    #[inline]
    pub fn is_ain5(&self) -> bool {
        *self == MUXNEGR::AIN5
    }
    #[doc = "Checks if the value of the field is `AIN6`"]
    #[inline]
    pub fn is_ain6(&self) -> bool {
        *self == MUXNEGR::AIN6
    }
    #[doc = "Checks if the value of the field is `AIN7`"]
    #[inline]
    pub fn is_ain7(&self) -> bool {
        *self == MUXNEGR::AIN7
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline]
    pub fn is_gnd(&self) -> bool {
        *self == MUXNEGR::GND
    }
}
#[doc = r" Value of the field"]
pub struct DSEQSTOPR {
    bits: bool,
}
impl DSEQSTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `MUXPOS`"]
pub enum MUXPOSW {
    #[doc = "ADC AIN0 Pin"]
    AIN0,
    #[doc = "ADC AIN1 Pin"]
    AIN1,
    #[doc = "ADC AIN2 Pin"]
    AIN2,
    #[doc = "ADC AIN3 Pin"]
    AIN3,
    #[doc = "ADC AIN4 Pin"]
    AIN4,
    #[doc = "ADC AIN5 Pin"]
    AIN5,
    #[doc = "ADC AIN6 Pin"]
    AIN6,
    #[doc = "ADC AIN7 Pin"]
    AIN7,
    #[doc = "ADC AIN8 Pin"]
    AIN8,
    #[doc = "ADC AIN9 Pin"]
    AIN9,
    #[doc = "ADC AIN10 Pin"]
    AIN10,
    #[doc = "ADC AIN11 Pin"]
    AIN11,
    #[doc = "ADC AIN12 Pin"]
    AIN12,
    #[doc = "ADC AIN13 Pin"]
    AIN13,
    #[doc = "ADC AIN14 Pin"]
    AIN14,
    #[doc = "ADC AIN15 Pin"]
    AIN15,
    #[doc = "ADC AIN16 Pin"]
    AIN16,
    #[doc = "ADC AIN17 Pin"]
    AIN17,
    #[doc = "ADC AIN18 Pin"]
    AIN18,
    #[doc = "ADC AIN19 Pin"]
    AIN19,
    #[doc = "ADC AIN20 Pin"]
    AIN20,
    #[doc = "ADC AIN21 Pin"]
    AIN21,
    #[doc = "ADC AIN22 Pin"]
    AIN22,
    #[doc = "ADC AIN23 Pin"]
    AIN23,
    #[doc = "1/4 Scaled Core Supply"]
    SCALEDCOREVCC,
    #[doc = "1/4 Scaled VBAT Supply"]
    SCALEDVBAT,
    #[doc = "1/4 Scaled I/O Supply"]
    SCALEDIOVCC,
    #[doc = "Bandgap Voltage"]
    BANDGAP,
    #[doc = "Temperature Sensor"]
    PTAT,
    #[doc = "Temperature Sensor"]
    CTAT,
    #[doc = "DAC Output"]
    DAC,
    #[doc = "PTC output (only on ADC0)"]
    PTC,
}
impl MUXPOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUXPOSW::AIN0 => 0,
            MUXPOSW::AIN1 => 1,
            MUXPOSW::AIN2 => 2,
            MUXPOSW::AIN3 => 3,
            MUXPOSW::AIN4 => 4,
            MUXPOSW::AIN5 => 5,
            MUXPOSW::AIN6 => 6,
            MUXPOSW::AIN7 => 7,
            MUXPOSW::AIN8 => 8,
            MUXPOSW::AIN9 => 9,
            MUXPOSW::AIN10 => 10,
            MUXPOSW::AIN11 => 11,
            MUXPOSW::AIN12 => 12,
            MUXPOSW::AIN13 => 13,
            MUXPOSW::AIN14 => 14,
            MUXPOSW::AIN15 => 15,
            MUXPOSW::AIN16 => 16,
            MUXPOSW::AIN17 => 17,
            MUXPOSW::AIN18 => 18,
            MUXPOSW::AIN19 => 19,
            MUXPOSW::AIN20 => 20,
            MUXPOSW::AIN21 => 21,
            MUXPOSW::AIN22 => 22,
            MUXPOSW::AIN23 => 23,
            MUXPOSW::SCALEDCOREVCC => 24,
            MUXPOSW::SCALEDVBAT => 25,
            MUXPOSW::SCALEDIOVCC => 26,
            MUXPOSW::BANDGAP => 27,
            MUXPOSW::PTAT => 28,
            MUXPOSW::CTAT => 29,
            MUXPOSW::DAC => 30,
            MUXPOSW::PTC => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUXPOSW<'a> {
    w: &'a mut W,
}
impl<'a> _MUXPOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUXPOSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline]
    pub fn ain0(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline]
    pub fn ain1(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline]
    pub fn ain2(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline]
    pub fn ain3(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline]
    pub fn ain4(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline]
    pub fn ain5(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline]
    pub fn ain6(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline]
    pub fn ain7(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN7)
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline]
    pub fn ain8(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN8)
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline]
    pub fn ain9(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN9)
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline]
    pub fn ain10(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN10)
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline]
    pub fn ain11(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN11)
    }
    #[doc = "ADC AIN12 Pin"]
    #[inline]
    pub fn ain12(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN12)
    }
    #[doc = "ADC AIN13 Pin"]
    #[inline]
    pub fn ain13(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN13)
    }
    #[doc = "ADC AIN14 Pin"]
    #[inline]
    pub fn ain14(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN14)
    }
    #[doc = "ADC AIN15 Pin"]
    #[inline]
    pub fn ain15(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN15)
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline]
    pub fn ain16(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN16)
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline]
    pub fn ain17(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN17)
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline]
    pub fn ain18(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN18)
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline]
    pub fn ain19(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN19)
    }
    #[doc = "ADC AIN20 Pin"]
    #[inline]
    pub fn ain20(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN20)
    }
    #[doc = "ADC AIN21 Pin"]
    #[inline]
    pub fn ain21(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN21)
    }
    #[doc = "ADC AIN22 Pin"]
    #[inline]
    pub fn ain22(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN22)
    }
    #[doc = "ADC AIN23 Pin"]
    #[inline]
    pub fn ain23(self) -> &'a mut W {
        self.variant(MUXPOSW::AIN23)
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline]
    pub fn scaledcorevcc(self) -> &'a mut W {
        self.variant(MUXPOSW::SCALEDCOREVCC)
    }
    #[doc = "1/4 Scaled VBAT Supply"]
    #[inline]
    pub fn scaledvbat(self) -> &'a mut W {
        self.variant(MUXPOSW::SCALEDVBAT)
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline]
    pub fn scalediovcc(self) -> &'a mut W {
        self.variant(MUXPOSW::SCALEDIOVCC)
    }
    #[doc = "Bandgap Voltage"]
    #[inline]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(MUXPOSW::BANDGAP)
    }
    #[doc = "Temperature Sensor"]
    #[inline]
    pub fn ptat(self) -> &'a mut W {
        self.variant(MUXPOSW::PTAT)
    }
    #[doc = "Temperature Sensor"]
    #[inline]
    pub fn ctat(self) -> &'a mut W {
        self.variant(MUXPOSW::CTAT)
    }
    #[doc = "DAC Output"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(MUXPOSW::DAC)
    }
    #[doc = "PTC output (only on ADC0)"]
    #[inline]
    pub fn ptc(self) -> &'a mut W {
        self.variant(MUXPOSW::PTC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIFFMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFFMODEW<'a> {
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
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MUXNEG`"]
pub enum MUXNEGW {
    #[doc = "ADC AIN0 Pin"]
    AIN0,
    #[doc = "ADC AIN1 Pin"]
    AIN1,
    #[doc = "ADC AIN2 Pin"]
    AIN2,
    #[doc = "ADC AIN3 Pin"]
    AIN3,
    #[doc = "ADC AIN4 Pin"]
    AIN4,
    #[doc = "ADC AIN5 Pin"]
    AIN5,
    #[doc = "ADC AIN6 Pin"]
    AIN6,
    #[doc = "ADC AIN7 Pin"]
    AIN7,
    #[doc = "Internal Ground"]
    GND,
}
impl MUXNEGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUXNEGW::AIN0 => 0,
            MUXNEGW::AIN1 => 1,
            MUXNEGW::AIN2 => 2,
            MUXNEGW::AIN3 => 3,
            MUXNEGW::AIN4 => 4,
            MUXNEGW::AIN5 => 5,
            MUXNEGW::AIN6 => 6,
            MUXNEGW::AIN7 => 7,
            MUXNEGW::GND => 24,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUXNEGW<'a> {
    w: &'a mut W,
}
impl<'a> _MUXNEGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUXNEGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline]
    pub fn ain0(self) -> &'a mut W {
        self.variant(MUXNEGW::AIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline]
    pub fn ain1(self) -> &'a mut W {
        self.variant(MUXNEGW::AIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline]
    pub fn ain2(self) -> &'a mut W {
        self.variant(MUXNEGW::AIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline]
    pub fn ain3(self) -> &'a mut W {
        self.variant(MUXNEGW::AIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline]
    pub fn ain4(self) -> &'a mut W {
        self.variant(MUXNEGW::AIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline]
    pub fn ain5(self) -> &'a mut W {
        self.variant(MUXNEGW::AIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline]
    pub fn ain6(self) -> &'a mut W {
        self.variant(MUXNEGW::AIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline]
    pub fn ain7(self) -> &'a mut W {
        self.variant(MUXNEGW::AIN7)
    }
    #[doc = "Internal Ground"]
    #[inline]
    pub fn gnd(self) -> &'a mut W {
        self.variant(MUXNEGW::GND)
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
#[doc = r" Proxy"]
pub struct _DSEQSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DSEQSTOPW<'a> {
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
        const MASK: bool = true;
        const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline]
    pub fn muxpos(&self) -> MUXPOSR {
        MUXPOSR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 7 - Differential Mode"]
    #[inline]
    pub fn diffmode(&self) -> DIFFMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DIFFMODER { bits }
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline]
    pub fn muxneg(&self) -> MUXNEGR {
        MUXNEGR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 15 - Stop DMA Sequencing"]
    #[inline]
    pub fn dseqstop(&self) -> DSEQSTOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DSEQSTOPR { bits }
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
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline]
    pub fn muxpos(&mut self) -> _MUXPOSW {
        _MUXPOSW { w: self }
    }
    #[doc = "Bit 7 - Differential Mode"]
    #[inline]
    pub fn diffmode(&mut self) -> _DIFFMODEW {
        _DIFFMODEW { w: self }
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline]
    pub fn muxneg(&mut self) -> _MUXNEGW {
        _MUXNEGW { w: self }
    }
    #[doc = "Bit 15 - Stop DMA Sequencing"]
    #[inline]
    pub fn dseqstop(&mut self) -> _DSEQSTOPW {
        _DSEQSTOPW { w: self }
    }
}
