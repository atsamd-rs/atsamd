#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
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
    PIN0,
    #[doc = "ADC AIN1 Pin"]
    PIN1,
    #[doc = "ADC AIN2 Pin"]
    PIN2,
    #[doc = "ADC AIN3 Pin"]
    PIN3,
    #[doc = "ADC AIN4 Pin"]
    PIN4,
    #[doc = "ADC AIN5 Pin"]
    PIN5,
    #[doc = "ADC AIN6 Pin"]
    PIN6,
    #[doc = "ADC AIN7 Pin"]
    PIN7,
    #[doc = "ADC AIN8 Pin"]
    PIN8,
    #[doc = "ADC AIN9 Pin"]
    PIN9,
    #[doc = "ADC AIN10 Pin"]
    PIN10,
    #[doc = "ADC AIN11 Pin"]
    PIN11,
    #[doc = "ADC AIN12 Pin"]
    PIN12,
    #[doc = "ADC AIN13 Pin"]
    PIN13,
    #[doc = "ADC AIN14 Pin"]
    PIN14,
    #[doc = "ADC AIN15 Pin"]
    PIN15,
    #[doc = "ADC AIN16 Pin"]
    PIN16,
    #[doc = "ADC AIN17 Pin"]
    PIN17,
    #[doc = "ADC AIN18 Pin"]
    PIN18,
    #[doc = "ADC AIN19 Pin"]
    PIN19,
    #[doc = "Temperature Reference"]
    TEMP,
    #[doc = "Bandgap Voltage"]
    BANDGAP,
    #[doc = "1/4  Scaled Core Supply"]
    SCALEDCOREVCC,
    #[doc = "1/4  Scaled I/O Supply"]
    SCALEDIOVCC,
    #[doc = "DAC Output"]
    DAC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MUXPOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUXPOSR::PIN0 => 0,
            MUXPOSR::PIN1 => 1,
            MUXPOSR::PIN2 => 2,
            MUXPOSR::PIN3 => 3,
            MUXPOSR::PIN4 => 4,
            MUXPOSR::PIN5 => 5,
            MUXPOSR::PIN6 => 6,
            MUXPOSR::PIN7 => 7,
            MUXPOSR::PIN8 => 8,
            MUXPOSR::PIN9 => 9,
            MUXPOSR::PIN10 => 10,
            MUXPOSR::PIN11 => 11,
            MUXPOSR::PIN12 => 12,
            MUXPOSR::PIN13 => 13,
            MUXPOSR::PIN14 => 14,
            MUXPOSR::PIN15 => 15,
            MUXPOSR::PIN16 => 16,
            MUXPOSR::PIN17 => 17,
            MUXPOSR::PIN18 => 18,
            MUXPOSR::PIN19 => 19,
            MUXPOSR::TEMP => 24,
            MUXPOSR::BANDGAP => 25,
            MUXPOSR::SCALEDCOREVCC => 26,
            MUXPOSR::SCALEDIOVCC => 27,
            MUXPOSR::DAC => 28,
            MUXPOSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUXPOSR {
        match value {
            0 => MUXPOSR::PIN0,
            1 => MUXPOSR::PIN1,
            2 => MUXPOSR::PIN2,
            3 => MUXPOSR::PIN3,
            4 => MUXPOSR::PIN4,
            5 => MUXPOSR::PIN5,
            6 => MUXPOSR::PIN6,
            7 => MUXPOSR::PIN7,
            8 => MUXPOSR::PIN8,
            9 => MUXPOSR::PIN9,
            10 => MUXPOSR::PIN10,
            11 => MUXPOSR::PIN11,
            12 => MUXPOSR::PIN12,
            13 => MUXPOSR::PIN13,
            14 => MUXPOSR::PIN14,
            15 => MUXPOSR::PIN15,
            16 => MUXPOSR::PIN16,
            17 => MUXPOSR::PIN17,
            18 => MUXPOSR::PIN18,
            19 => MUXPOSR::PIN19,
            24 => MUXPOSR::TEMP,
            25 => MUXPOSR::BANDGAP,
            26 => MUXPOSR::SCALEDCOREVCC,
            27 => MUXPOSR::SCALEDIOVCC,
            28 => MUXPOSR::DAC,
            i => MUXPOSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline]
    pub fn is_pin0(&self) -> bool {
        *self == MUXPOSR::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline]
    pub fn is_pin1(&self) -> bool {
        *self == MUXPOSR::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline]
    pub fn is_pin2(&self) -> bool {
        *self == MUXPOSR::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline]
    pub fn is_pin3(&self) -> bool {
        *self == MUXPOSR::PIN3
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline]
    pub fn is_pin4(&self) -> bool {
        *self == MUXPOSR::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline]
    pub fn is_pin5(&self) -> bool {
        *self == MUXPOSR::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline]
    pub fn is_pin6(&self) -> bool {
        *self == MUXPOSR::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline]
    pub fn is_pin7(&self) -> bool {
        *self == MUXPOSR::PIN7
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline]
    pub fn is_pin8(&self) -> bool {
        *self == MUXPOSR::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline]
    pub fn is_pin9(&self) -> bool {
        *self == MUXPOSR::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline]
    pub fn is_pin10(&self) -> bool {
        *self == MUXPOSR::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline]
    pub fn is_pin11(&self) -> bool {
        *self == MUXPOSR::PIN11
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline]
    pub fn is_pin12(&self) -> bool {
        *self == MUXPOSR::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline]
    pub fn is_pin13(&self) -> bool {
        *self == MUXPOSR::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline]
    pub fn is_pin14(&self) -> bool {
        *self == MUXPOSR::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline]
    pub fn is_pin15(&self) -> bool {
        *self == MUXPOSR::PIN15
    }
    #[doc = "Checks if the value of the field is `PIN16`"]
    #[inline]
    pub fn is_pin16(&self) -> bool {
        *self == MUXPOSR::PIN16
    }
    #[doc = "Checks if the value of the field is `PIN17`"]
    #[inline]
    pub fn is_pin17(&self) -> bool {
        *self == MUXPOSR::PIN17
    }
    #[doc = "Checks if the value of the field is `PIN18`"]
    #[inline]
    pub fn is_pin18(&self) -> bool {
        *self == MUXPOSR::PIN18
    }
    #[doc = "Checks if the value of the field is `PIN19`"]
    #[inline]
    pub fn is_pin19(&self) -> bool {
        *self == MUXPOSR::PIN19
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline]
    pub fn is_temp(&self) -> bool {
        *self == MUXPOSR::TEMP
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXPOSR::BANDGAP
    }
    #[doc = "Checks if the value of the field is `SCALEDCOREVCC`"]
    #[inline]
    pub fn is_scaledcorevcc(&self) -> bool {
        *self == MUXPOSR::SCALEDCOREVCC
    }
    #[doc = "Checks if the value of the field is `SCALEDIOVCC`"]
    #[inline]
    pub fn is_scalediovcc(&self) -> bool {
        *self == MUXPOSR::SCALEDIOVCC
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == MUXPOSR::DAC
    }
}
#[doc = "Possible values of the field `MUXNEG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXNEGR {
    #[doc = "ADC AIN0 Pin"]
    PIN0,
    #[doc = "ADC AIN1 Pin"]
    PIN1,
    #[doc = "ADC AIN2 Pin"]
    PIN2,
    #[doc = "ADC AIN3 Pin"]
    PIN3,
    #[doc = "ADC AIN4 Pin"]
    PIN4,
    #[doc = "ADC AIN5 Pin"]
    PIN5,
    #[doc = "ADC AIN6 Pin"]
    PIN6,
    #[doc = "ADC AIN7 Pin"]
    PIN7,
    #[doc = "Internal Ground"]
    GND,
    #[doc = "I/O Ground"]
    IOGND,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MUXNEGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUXNEGR::PIN0 => 0,
            MUXNEGR::PIN1 => 1,
            MUXNEGR::PIN2 => 2,
            MUXNEGR::PIN3 => 3,
            MUXNEGR::PIN4 => 4,
            MUXNEGR::PIN5 => 5,
            MUXNEGR::PIN6 => 6,
            MUXNEGR::PIN7 => 7,
            MUXNEGR::GND => 24,
            MUXNEGR::IOGND => 25,
            MUXNEGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUXNEGR {
        match value {
            0 => MUXNEGR::PIN0,
            1 => MUXNEGR::PIN1,
            2 => MUXNEGR::PIN2,
            3 => MUXNEGR::PIN3,
            4 => MUXNEGR::PIN4,
            5 => MUXNEGR::PIN5,
            6 => MUXNEGR::PIN6,
            7 => MUXNEGR::PIN7,
            24 => MUXNEGR::GND,
            25 => MUXNEGR::IOGND,
            i => MUXNEGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline]
    pub fn is_pin0(&self) -> bool {
        *self == MUXNEGR::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline]
    pub fn is_pin1(&self) -> bool {
        *self == MUXNEGR::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline]
    pub fn is_pin2(&self) -> bool {
        *self == MUXNEGR::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline]
    pub fn is_pin3(&self) -> bool {
        *self == MUXNEGR::PIN3
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline]
    pub fn is_pin4(&self) -> bool {
        *self == MUXNEGR::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline]
    pub fn is_pin5(&self) -> bool {
        *self == MUXNEGR::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline]
    pub fn is_pin6(&self) -> bool {
        *self == MUXNEGR::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline]
    pub fn is_pin7(&self) -> bool {
        *self == MUXNEGR::PIN7
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline]
    pub fn is_gnd(&self) -> bool {
        *self == MUXNEGR::GND
    }
    #[doc = "Checks if the value of the field is `IOGND`"]
    #[inline]
    pub fn is_iognd(&self) -> bool {
        *self == MUXNEGR::IOGND
    }
}
#[doc = r" Value of the field"]
pub struct INPUTSCANR {
    bits: u8,
}
impl INPUTSCANR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INPUTOFFSETR {
    bits: u8,
}
impl INPUTOFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `GAIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAINR {
    #[doc = "1x"]
    _1X,
    #[doc = "2x"]
    _2X,
    #[doc = "4x"]
    _4X,
    #[doc = "8x"]
    _8X,
    #[doc = "16x"]
    _16X,
    #[doc = "1/2x"]
    DIV2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAINR::_1X => 0,
            GAINR::_2X => 1,
            GAINR::_4X => 2,
            GAINR::_8X => 3,
            GAINR::_16X => 4,
            GAINR::DIV2 => 15,
            GAINR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAINR {
        match value {
            0 => GAINR::_1X,
            1 => GAINR::_2X,
            2 => GAINR::_4X,
            3 => GAINR::_8X,
            4 => GAINR::_16X,
            15 => GAINR::DIV2,
            i => GAINR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1X`"]
    #[inline]
    pub fn is_1x(&self) -> bool {
        *self == GAINR::_1X
    }
    #[doc = "Checks if the value of the field is `_2X`"]
    #[inline]
    pub fn is_2x(&self) -> bool {
        *self == GAINR::_2X
    }
    #[doc = "Checks if the value of the field is `_4X`"]
    #[inline]
    pub fn is_4x(&self) -> bool {
        *self == GAINR::_4X
    }
    #[doc = "Checks if the value of the field is `_8X`"]
    #[inline]
    pub fn is_8x(&self) -> bool {
        *self == GAINR::_8X
    }
    #[doc = "Checks if the value of the field is `_16X`"]
    #[inline]
    pub fn is_16x(&self) -> bool {
        *self == GAINR::_16X
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == GAINR::DIV2
    }
}
#[doc = "Values that can be written to the field `MUXPOS`"]
pub enum MUXPOSW {
    #[doc = "ADC AIN0 Pin"]
    PIN0,
    #[doc = "ADC AIN1 Pin"]
    PIN1,
    #[doc = "ADC AIN2 Pin"]
    PIN2,
    #[doc = "ADC AIN3 Pin"]
    PIN3,
    #[doc = "ADC AIN4 Pin"]
    PIN4,
    #[doc = "ADC AIN5 Pin"]
    PIN5,
    #[doc = "ADC AIN6 Pin"]
    PIN6,
    #[doc = "ADC AIN7 Pin"]
    PIN7,
    #[doc = "ADC AIN8 Pin"]
    PIN8,
    #[doc = "ADC AIN9 Pin"]
    PIN9,
    #[doc = "ADC AIN10 Pin"]
    PIN10,
    #[doc = "ADC AIN11 Pin"]
    PIN11,
    #[doc = "ADC AIN12 Pin"]
    PIN12,
    #[doc = "ADC AIN13 Pin"]
    PIN13,
    #[doc = "ADC AIN14 Pin"]
    PIN14,
    #[doc = "ADC AIN15 Pin"]
    PIN15,
    #[doc = "ADC AIN16 Pin"]
    PIN16,
    #[doc = "ADC AIN17 Pin"]
    PIN17,
    #[doc = "ADC AIN18 Pin"]
    PIN18,
    #[doc = "ADC AIN19 Pin"]
    PIN19,
    #[doc = "Temperature Reference"]
    TEMP,
    #[doc = "Bandgap Voltage"]
    BANDGAP,
    #[doc = "1/4  Scaled Core Supply"]
    SCALEDCOREVCC,
    #[doc = "1/4  Scaled I/O Supply"]
    SCALEDIOVCC,
    #[doc = "DAC Output"]
    DAC,
}
impl MUXPOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUXPOSW::PIN0 => 0,
            MUXPOSW::PIN1 => 1,
            MUXPOSW::PIN2 => 2,
            MUXPOSW::PIN3 => 3,
            MUXPOSW::PIN4 => 4,
            MUXPOSW::PIN5 => 5,
            MUXPOSW::PIN6 => 6,
            MUXPOSW::PIN7 => 7,
            MUXPOSW::PIN8 => 8,
            MUXPOSW::PIN9 => 9,
            MUXPOSW::PIN10 => 10,
            MUXPOSW::PIN11 => 11,
            MUXPOSW::PIN12 => 12,
            MUXPOSW::PIN13 => 13,
            MUXPOSW::PIN14 => 14,
            MUXPOSW::PIN15 => 15,
            MUXPOSW::PIN16 => 16,
            MUXPOSW::PIN17 => 17,
            MUXPOSW::PIN18 => 18,
            MUXPOSW::PIN19 => 19,
            MUXPOSW::TEMP => 24,
            MUXPOSW::BANDGAP => 25,
            MUXPOSW::SCALEDCOREVCC => 26,
            MUXPOSW::SCALEDIOVCC => 27,
            MUXPOSW::DAC => 28,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline]
    pub fn pin4(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline]
    pub fn pin5(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline]
    pub fn pin6(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline]
    pub fn pin7(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN7)
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline]
    pub fn pin8(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN8)
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline]
    pub fn pin9(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN9)
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline]
    pub fn pin10(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN10)
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline]
    pub fn pin11(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN11)
    }
    #[doc = "ADC AIN12 Pin"]
    #[inline]
    pub fn pin12(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN12)
    }
    #[doc = "ADC AIN13 Pin"]
    #[inline]
    pub fn pin13(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN13)
    }
    #[doc = "ADC AIN14 Pin"]
    #[inline]
    pub fn pin14(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN14)
    }
    #[doc = "ADC AIN15 Pin"]
    #[inline]
    pub fn pin15(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN15)
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline]
    pub fn pin16(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN16)
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline]
    pub fn pin17(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN17)
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline]
    pub fn pin18(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN18)
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline]
    pub fn pin19(self) -> &'a mut W {
        self.variant(MUXPOSW::PIN19)
    }
    #[doc = "Temperature Reference"]
    #[inline]
    pub fn temp(self) -> &'a mut W {
        self.variant(MUXPOSW::TEMP)
    }
    #[doc = "Bandgap Voltage"]
    #[inline]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(MUXPOSW::BANDGAP)
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline]
    pub fn scaledcorevcc(self) -> &'a mut W {
        self.variant(MUXPOSW::SCALEDCOREVCC)
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline]
    pub fn scalediovcc(self) -> &'a mut W {
        self.variant(MUXPOSW::SCALEDIOVCC)
    }
    #[doc = "DAC Output"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(MUXPOSW::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MUXNEG`"]
pub enum MUXNEGW {
    #[doc = "ADC AIN0 Pin"]
    PIN0,
    #[doc = "ADC AIN1 Pin"]
    PIN1,
    #[doc = "ADC AIN2 Pin"]
    PIN2,
    #[doc = "ADC AIN3 Pin"]
    PIN3,
    #[doc = "ADC AIN4 Pin"]
    PIN4,
    #[doc = "ADC AIN5 Pin"]
    PIN5,
    #[doc = "ADC AIN6 Pin"]
    PIN6,
    #[doc = "ADC AIN7 Pin"]
    PIN7,
    #[doc = "Internal Ground"]
    GND,
    #[doc = "I/O Ground"]
    IOGND,
}
impl MUXNEGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUXNEGW::PIN0 => 0,
            MUXNEGW::PIN1 => 1,
            MUXNEGW::PIN2 => 2,
            MUXNEGW::PIN3 => 3,
            MUXNEGW::PIN4 => 4,
            MUXNEGW::PIN5 => 5,
            MUXNEGW::PIN6 => 6,
            MUXNEGW::PIN7 => 7,
            MUXNEGW::GND => 24,
            MUXNEGW::IOGND => 25,
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
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXNEGW::PIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXNEGW::PIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXNEGW::PIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXNEGW::PIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline]
    pub fn pin4(self) -> &'a mut W {
        self.variant(MUXNEGW::PIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline]
    pub fn pin5(self) -> &'a mut W {
        self.variant(MUXNEGW::PIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline]
    pub fn pin6(self) -> &'a mut W {
        self.variant(MUXNEGW::PIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline]
    pub fn pin7(self) -> &'a mut W {
        self.variant(MUXNEGW::PIN7)
    }
    #[doc = "Internal Ground"]
    #[inline]
    pub fn gnd(self) -> &'a mut W {
        self.variant(MUXNEGW::GND)
    }
    #[doc = "I/O Ground"]
    #[inline]
    pub fn iognd(self) -> &'a mut W {
        self.variant(MUXNEGW::IOGND)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INPUTSCANW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUTSCANW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INPUTOFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUTOFFSETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GAIN`"]
pub enum GAINW {
    #[doc = "1x"]
    _1X,
    #[doc = "2x"]
    _2X,
    #[doc = "4x"]
    _4X,
    #[doc = "8x"]
    _8X,
    #[doc = "16x"]
    _16X,
    #[doc = "1/2x"]
    DIV2,
}
impl GAINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAINW::_1X => 0,
            GAINW::_2X => 1,
            GAINW::_4X => 2,
            GAINW::_8X => 3,
            GAINW::_16X => 4,
            GAINW::DIV2 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAINW<'a> {
    w: &'a mut W,
}
impl<'a> _GAINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAINW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1x"]
    #[inline]
    pub fn _1x(self) -> &'a mut W {
        self.variant(GAINW::_1X)
    }
    #[doc = "2x"]
    #[inline]
    pub fn _2x(self) -> &'a mut W {
        self.variant(GAINW::_2X)
    }
    #[doc = "4x"]
    #[inline]
    pub fn _4x(self) -> &'a mut W {
        self.variant(GAINW::_4X)
    }
    #[doc = "8x"]
    #[inline]
    pub fn _8x(self) -> &'a mut W {
        self.variant(GAINW::_8X)
    }
    #[doc = "16x"]
    #[inline]
    pub fn _16x(self) -> &'a mut W {
        self.variant(GAINW::_16X)
    }
    #[doc = "1/2x"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(GAINW::DIV2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline]
    pub fn muxpos(&self) -> MUXPOSR {
        MUXPOSR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline]
    pub fn muxneg(&self) -> MUXNEGR {
        MUXNEGR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Number of Input Channels Included in Scan"]
    #[inline]
    pub fn inputscan(&self) -> INPUTSCANR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INPUTSCANR { bits }
    }
    #[doc = "Bits 20:23 - Positive Mux Setting Offset"]
    #[inline]
    pub fn inputoffset(&self) -> INPUTOFFSETR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INPUTOFFSETR { bits }
    }
    #[doc = "Bits 24:27 - Gain Factor Selection"]
    #[inline]
    pub fn gain(&self) -> GAINR {
        GAINR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline]
    pub fn muxpos(&mut self) -> _MUXPOSW {
        _MUXPOSW { w: self }
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline]
    pub fn muxneg(&mut self) -> _MUXNEGW {
        _MUXNEGW { w: self }
    }
    #[doc = "Bits 16:19 - Number of Input Channels Included in Scan"]
    #[inline]
    pub fn inputscan(&mut self) -> _INPUTSCANW {
        _INPUTSCANW { w: self }
    }
    #[doc = "Bits 20:23 - Positive Mux Setting Offset"]
    #[inline]
    pub fn inputoffset(&mut self) -> _INPUTOFFSETW {
        _INPUTOFFSETW { w: self }
    }
    #[doc = "Bits 24:27 - Gain Factor Selection"]
    #[inline]
    pub fn gain(&mut self) -> _GAINW {
        _GAINW { w: self }
    }
}
