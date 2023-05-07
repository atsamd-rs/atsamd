#[doc = "Register `INPUTCTRL` reader"]
pub struct R(crate::R<INPUTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUTCTRL` writer"]
pub struct W(crate::W<INPUTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INPUTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUXPOS` reader - Positive Mux Input Selection"]
pub type MUXPOS_R = crate::FieldReader<u8, MUXPOSSELECT_A>;
#[doc = "Positive Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXPOSSELECT_A {
    #[doc = "0: ADC AIN0 Pin"]
    PIN0 = 0,
    #[doc = "1: ADC AIN1 Pin"]
    PIN1 = 1,
    #[doc = "2: ADC AIN2 Pin"]
    PIN2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    PIN3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    PIN4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    PIN5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    PIN6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    PIN7 = 7,
    #[doc = "8: ADC AIN8 Pin"]
    PIN8 = 8,
    #[doc = "9: ADC AIN9 Pin"]
    PIN9 = 9,
    #[doc = "10: ADC AIN10 Pin"]
    PIN10 = 10,
    #[doc = "11: ADC AIN11 Pin"]
    PIN11 = 11,
    #[doc = "12: ADC AIN12 Pin"]
    PIN12 = 12,
    #[doc = "13: ADC AIN13 Pin"]
    PIN13 = 13,
    #[doc = "14: ADC AIN14 Pin"]
    PIN14 = 14,
    #[doc = "15: ADC AIN15 Pin"]
    PIN15 = 15,
    #[doc = "16: ADC AIN16 Pin"]
    PIN16 = 16,
    #[doc = "17: ADC AIN17 Pin"]
    PIN17 = 17,
    #[doc = "18: ADC AIN18 Pin"]
    PIN18 = 18,
    #[doc = "19: ADC AIN19 Pin"]
    PIN19 = 19,
    #[doc = "24: Temperature Reference"]
    TEMP = 24,
    #[doc = "25: Bandgap Voltage"]
    BANDGAP = 25,
    #[doc = "26: 1/4 Scaled Core Supply"]
    SCALEDCOREVCC = 26,
    #[doc = "27: 1/4 Scaled I/O Supply"]
    SCALEDIOVCC = 27,
    #[doc = "28: DAC Output"]
    DAC = 28,
}
impl From<MUXPOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXPOSSELECT_A) -> Self {
        variant as _
    }
}
impl MUXPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUXPOSSELECT_A> {
        match self.bits {
            0 => Some(MUXPOSSELECT_A::PIN0),
            1 => Some(MUXPOSSELECT_A::PIN1),
            2 => Some(MUXPOSSELECT_A::PIN2),
            3 => Some(MUXPOSSELECT_A::PIN3),
            4 => Some(MUXPOSSELECT_A::PIN4),
            5 => Some(MUXPOSSELECT_A::PIN5),
            6 => Some(MUXPOSSELECT_A::PIN6),
            7 => Some(MUXPOSSELECT_A::PIN7),
            8 => Some(MUXPOSSELECT_A::PIN8),
            9 => Some(MUXPOSSELECT_A::PIN9),
            10 => Some(MUXPOSSELECT_A::PIN10),
            11 => Some(MUXPOSSELECT_A::PIN11),
            12 => Some(MUXPOSSELECT_A::PIN12),
            13 => Some(MUXPOSSELECT_A::PIN13),
            14 => Some(MUXPOSSELECT_A::PIN14),
            15 => Some(MUXPOSSELECT_A::PIN15),
            16 => Some(MUXPOSSELECT_A::PIN16),
            17 => Some(MUXPOSSELECT_A::PIN17),
            18 => Some(MUXPOSSELECT_A::PIN18),
            19 => Some(MUXPOSSELECT_A::PIN19),
            24 => Some(MUXPOSSELECT_A::TEMP),
            25 => Some(MUXPOSSELECT_A::BANDGAP),
            26 => Some(MUXPOSSELECT_A::SCALEDCOREVCC),
            27 => Some(MUXPOSSELECT_A::SCALEDIOVCC),
            28 => Some(MUXPOSSELECT_A::DAC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN3
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN7
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN11
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN15
    }
    #[doc = "Checks if the value of the field is `PIN16`"]
    #[inline(always)]
    pub fn is_pin16(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN16
    }
    #[doc = "Checks if the value of the field is `PIN17`"]
    #[inline(always)]
    pub fn is_pin17(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN17
    }
    #[doc = "Checks if the value of the field is `PIN18`"]
    #[inline(always)]
    pub fn is_pin18(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN18
    }
    #[doc = "Checks if the value of the field is `PIN19`"]
    #[inline(always)]
    pub fn is_pin19(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN19
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == MUXPOSSELECT_A::TEMP
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXPOSSELECT_A::BANDGAP
    }
    #[doc = "Checks if the value of the field is `SCALEDCOREVCC`"]
    #[inline(always)]
    pub fn is_scaledcorevcc(&self) -> bool {
        *self == MUXPOSSELECT_A::SCALEDCOREVCC
    }
    #[doc = "Checks if the value of the field is `SCALEDIOVCC`"]
    #[inline(always)]
    pub fn is_scalediovcc(&self) -> bool {
        *self == MUXPOSSELECT_A::SCALEDIOVCC
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == MUXPOSSELECT_A::DAC
    }
}
#[doc = "Field `MUXPOS` writer - Positive Mux Input Selection"]
pub type MUXPOS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INPUTCTRL_SPEC, u8, MUXPOSSELECT_A, 5, O>;
impl<'a, const O: u8> MUXPOS_W<'a, O> {
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN7)
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN8)
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN9)
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN10)
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN11)
    }
    #[doc = "ADC AIN12 Pin"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN12)
    }
    #[doc = "ADC AIN13 Pin"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN13)
    }
    #[doc = "ADC AIN14 Pin"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN14)
    }
    #[doc = "ADC AIN15 Pin"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN15)
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline(always)]
    pub fn pin16(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN16)
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline(always)]
    pub fn pin17(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN17)
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline(always)]
    pub fn pin18(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN18)
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline(always)]
    pub fn pin19(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN19)
    }
    #[doc = "Temperature Reference"]
    #[inline(always)]
    pub fn temp(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::TEMP)
    }
    #[doc = "Bandgap Voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::BANDGAP)
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn scaledcorevcc(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::SCALEDCOREVCC)
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline(always)]
    pub fn scalediovcc(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::SCALEDIOVCC)
    }
    #[doc = "DAC Output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::DAC)
    }
}
#[doc = "Field `MUXNEG` reader - Negative Mux Input Selection"]
pub type MUXNEG_R = crate::FieldReader<u8, MUXNEGSELECT_A>;
#[doc = "Negative Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXNEGSELECT_A {
    #[doc = "0: ADC AIN0 Pin"]
    PIN0 = 0,
    #[doc = "1: ADC AIN1 Pin"]
    PIN1 = 1,
    #[doc = "2: ADC AIN2 Pin"]
    PIN2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    PIN3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    PIN4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    PIN5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    PIN6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    PIN7 = 7,
    #[doc = "24: Internal Ground"]
    GND = 24,
    #[doc = "25: I/O Ground"]
    IOGND = 25,
}
impl From<MUXNEGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXNEGSELECT_A) -> Self {
        variant as _
    }
}
impl MUXNEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUXNEGSELECT_A> {
        match self.bits {
            0 => Some(MUXNEGSELECT_A::PIN0),
            1 => Some(MUXNEGSELECT_A::PIN1),
            2 => Some(MUXNEGSELECT_A::PIN2),
            3 => Some(MUXNEGSELECT_A::PIN3),
            4 => Some(MUXNEGSELECT_A::PIN4),
            5 => Some(MUXNEGSELECT_A::PIN5),
            6 => Some(MUXNEGSELECT_A::PIN6),
            7 => Some(MUXNEGSELECT_A::PIN7),
            24 => Some(MUXNEGSELECT_A::GND),
            25 => Some(MUXNEGSELECT_A::IOGND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXNEGSELECT_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXNEGSELECT_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == MUXNEGSELECT_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == MUXNEGSELECT_A::PIN3
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == MUXNEGSELECT_A::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == MUXNEGSELECT_A::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == MUXNEGSELECT_A::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == MUXNEGSELECT_A::PIN7
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == MUXNEGSELECT_A::GND
    }
    #[doc = "Checks if the value of the field is `IOGND`"]
    #[inline(always)]
    pub fn is_iognd(&self) -> bool {
        *self == MUXNEGSELECT_A::IOGND
    }
}
#[doc = "Field `MUXNEG` writer - Negative Mux Input Selection"]
pub type MUXNEG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INPUTCTRL_SPEC, u8, MUXNEGSELECT_A, 5, O>;
impl<'a, const O: u8> MUXNEG_W<'a, O> {
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::PIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::PIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::PIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::PIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::PIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::PIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::PIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::PIN7)
    }
    #[doc = "Internal Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::GND)
    }
    #[doc = "I/O Ground"]
    #[inline(always)]
    pub fn iognd(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::IOGND)
    }
}
#[doc = "Field `INPUTSCAN` reader - Number of Input Channels Included in Scan"]
pub type INPUTSCAN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPUTSCAN` writer - Number of Input Channels Included in Scan"]
pub type INPUTSCAN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INPUTCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `INPUTOFFSET` reader - Positive Mux Setting Offset"]
pub type INPUTOFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPUTOFFSET` writer - Positive Mux Setting Offset"]
pub type INPUTOFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INPUTCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `GAIN` reader - Gain Factor Selection"]
pub type GAIN_R = crate::FieldReader<u8, GAINSELECT_A>;
#[doc = "Gain Factor Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GAINSELECT_A {
    #[doc = "0: 1x"]
    _1X = 0,
    #[doc = "1: 2x"]
    _2X = 1,
    #[doc = "2: 4x"]
    _4X = 2,
    #[doc = "3: 8x"]
    _8X = 3,
    #[doc = "4: 16x"]
    _16X = 4,
    #[doc = "15: 1/2x"]
    DIV2 = 15,
}
impl From<GAINSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINSELECT_A) -> Self {
        variant as _
    }
}
impl GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GAINSELECT_A> {
        match self.bits {
            0 => Some(GAINSELECT_A::_1X),
            1 => Some(GAINSELECT_A::_2X),
            2 => Some(GAINSELECT_A::_4X),
            3 => Some(GAINSELECT_A::_8X),
            4 => Some(GAINSELECT_A::_16X),
            15 => Some(GAINSELECT_A::DIV2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1X`"]
    #[inline(always)]
    pub fn is_1x(&self) -> bool {
        *self == GAINSELECT_A::_1X
    }
    #[doc = "Checks if the value of the field is `_2X`"]
    #[inline(always)]
    pub fn is_2x(&self) -> bool {
        *self == GAINSELECT_A::_2X
    }
    #[doc = "Checks if the value of the field is `_4X`"]
    #[inline(always)]
    pub fn is_4x(&self) -> bool {
        *self == GAINSELECT_A::_4X
    }
    #[doc = "Checks if the value of the field is `_8X`"]
    #[inline(always)]
    pub fn is_8x(&self) -> bool {
        *self == GAINSELECT_A::_8X
    }
    #[doc = "Checks if the value of the field is `_16X`"]
    #[inline(always)]
    pub fn is_16x(&self) -> bool {
        *self == GAINSELECT_A::_16X
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == GAINSELECT_A::DIV2
    }
}
#[doc = "Field `GAIN` writer - Gain Factor Selection"]
pub type GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INPUTCTRL_SPEC, u8, GAINSELECT_A, 4, O>;
impl<'a, const O: u8> GAIN_W<'a, O> {
    #[doc = "1x"]
    #[inline(always)]
    pub fn _1x(self) -> &'a mut W {
        self.variant(GAINSELECT_A::_1X)
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn _2x(self) -> &'a mut W {
        self.variant(GAINSELECT_A::_2X)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn _4x(self) -> &'a mut W {
        self.variant(GAINSELECT_A::_4X)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn _8x(self) -> &'a mut W {
        self.variant(GAINSELECT_A::_8X)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn _16x(self) -> &'a mut W {
        self.variant(GAINSELECT_A::_16X)
    }
    #[doc = "1/2x"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(GAINSELECT_A::DIV2)
    }
}
impl R {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Number of Input Channels Included in Scan"]
    #[inline(always)]
    pub fn inputscan(&self) -> INPUTSCAN_R {
        INPUTSCAN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Positive Mux Setting Offset"]
    #[inline(always)]
    pub fn inputoffset(&self) -> INPUTOFFSET_R {
        INPUTOFFSET_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Gain Factor Selection"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<0> {
        MUXPOS_W::new(self)
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<8> {
        MUXNEG_W::new(self)
    }
    #[doc = "Bits 16:19 - Number of Input Channels Included in Scan"]
    #[inline(always)]
    #[must_use]
    pub fn inputscan(&mut self) -> INPUTSCAN_W<16> {
        INPUTSCAN_W::new(self)
    }
    #[doc = "Bits 20:23 - Positive Mux Setting Offset"]
    #[inline(always)]
    #[must_use]
    pub fn inputoffset(&mut self) -> INPUTOFFSET_W<20> {
        INPUTOFFSET_W::new(self)
    }
    #[doc = "Bits 24:27 - Gain Factor Selection"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<24> {
        GAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputctrl](index.html) module"]
pub struct INPUTCTRL_SPEC;
impl crate::RegisterSpec for INPUTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inputctrl::R](R) reader structure"]
impl crate::Readable for INPUTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inputctrl::W](W) writer structure"]
impl crate::Writable for INPUTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTCTRL to value 0"]
impl crate::Resettable for INPUTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
