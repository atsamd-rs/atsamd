#[doc = "Reader of register INPUTCTRL"]
pub type R = crate::R<u32, super::INPUTCTRL>;
#[doc = "Writer for register INPUTCTRL"]
pub type W = crate::W<u32, super::INPUTCTRL>;
#[doc = "Register INPUTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Positive Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUXPOS_A {
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
    #[doc = "26: 1/4  Scaled Core Supply"]
    SCALEDCOREVCC = 26,
    #[doc = "27: 1/4  Scaled I/O Supply"]
    SCALEDIOVCC = 27,
    #[doc = "28: DAC Output"]
    DAC = 28,
}
impl From<MUXPOS_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXPOS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUXPOS`"]
pub type MUXPOS_R = crate::R<u8, MUXPOS_A>;
impl MUXPOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MUXPOS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MUXPOS_A::PIN0),
            1 => Val(MUXPOS_A::PIN1),
            2 => Val(MUXPOS_A::PIN2),
            3 => Val(MUXPOS_A::PIN3),
            4 => Val(MUXPOS_A::PIN4),
            5 => Val(MUXPOS_A::PIN5),
            6 => Val(MUXPOS_A::PIN6),
            7 => Val(MUXPOS_A::PIN7),
            8 => Val(MUXPOS_A::PIN8),
            9 => Val(MUXPOS_A::PIN9),
            10 => Val(MUXPOS_A::PIN10),
            11 => Val(MUXPOS_A::PIN11),
            12 => Val(MUXPOS_A::PIN12),
            13 => Val(MUXPOS_A::PIN13),
            14 => Val(MUXPOS_A::PIN14),
            15 => Val(MUXPOS_A::PIN15),
            16 => Val(MUXPOS_A::PIN16),
            17 => Val(MUXPOS_A::PIN17),
            18 => Val(MUXPOS_A::PIN18),
            19 => Val(MUXPOS_A::PIN19),
            24 => Val(MUXPOS_A::TEMP),
            25 => Val(MUXPOS_A::BANDGAP),
            26 => Val(MUXPOS_A::SCALEDCOREVCC),
            27 => Val(MUXPOS_A::SCALEDIOVCC),
            28 => Val(MUXPOS_A::DAC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXPOS_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXPOS_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == MUXPOS_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == MUXPOS_A::PIN3
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == MUXPOS_A::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == MUXPOS_A::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == MUXPOS_A::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == MUXPOS_A::PIN7
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == MUXPOS_A::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == MUXPOS_A::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == MUXPOS_A::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == MUXPOS_A::PIN11
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == MUXPOS_A::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == MUXPOS_A::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == MUXPOS_A::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == MUXPOS_A::PIN15
    }
    #[doc = "Checks if the value of the field is `PIN16`"]
    #[inline(always)]
    pub fn is_pin16(&self) -> bool {
        *self == MUXPOS_A::PIN16
    }
    #[doc = "Checks if the value of the field is `PIN17`"]
    #[inline(always)]
    pub fn is_pin17(&self) -> bool {
        *self == MUXPOS_A::PIN17
    }
    #[doc = "Checks if the value of the field is `PIN18`"]
    #[inline(always)]
    pub fn is_pin18(&self) -> bool {
        *self == MUXPOS_A::PIN18
    }
    #[doc = "Checks if the value of the field is `PIN19`"]
    #[inline(always)]
    pub fn is_pin19(&self) -> bool {
        *self == MUXPOS_A::PIN19
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == MUXPOS_A::TEMP
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXPOS_A::BANDGAP
    }
    #[doc = "Checks if the value of the field is `SCALEDCOREVCC`"]
    #[inline(always)]
    pub fn is_scaledcorevcc(&self) -> bool {
        *self == MUXPOS_A::SCALEDCOREVCC
    }
    #[doc = "Checks if the value of the field is `SCALEDIOVCC`"]
    #[inline(always)]
    pub fn is_scalediovcc(&self) -> bool {
        *self == MUXPOS_A::SCALEDIOVCC
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == MUXPOS_A::DAC
    }
}
#[doc = "Write proxy for field `MUXPOS`"]
pub struct MUXPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXPOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXPOS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN7)
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN8)
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN9)
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN10)
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN11)
    }
    #[doc = "ADC AIN12 Pin"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN12)
    }
    #[doc = "ADC AIN13 Pin"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN13)
    }
    #[doc = "ADC AIN14 Pin"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN14)
    }
    #[doc = "ADC AIN15 Pin"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN15)
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline(always)]
    pub fn pin16(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN16)
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline(always)]
    pub fn pin17(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN17)
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline(always)]
    pub fn pin18(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN18)
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline(always)]
    pub fn pin19(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN19)
    }
    #[doc = "Temperature Reference"]
    #[inline(always)]
    pub fn temp(self) -> &'a mut W {
        self.variant(MUXPOS_A::TEMP)
    }
    #[doc = "Bandgap Voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(MUXPOS_A::BANDGAP)
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn scaledcorevcc(self) -> &'a mut W {
        self.variant(MUXPOS_A::SCALEDCOREVCC)
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline(always)]
    pub fn scalediovcc(self) -> &'a mut W {
        self.variant(MUXPOS_A::SCALEDIOVCC)
    }
    #[doc = "DAC Output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(MUXPOS_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Negative Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUXNEG_A {
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
impl From<MUXNEG_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXNEG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUXNEG`"]
pub type MUXNEG_R = crate::R<u8, MUXNEG_A>;
impl MUXNEG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MUXNEG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MUXNEG_A::PIN0),
            1 => Val(MUXNEG_A::PIN1),
            2 => Val(MUXNEG_A::PIN2),
            3 => Val(MUXNEG_A::PIN3),
            4 => Val(MUXNEG_A::PIN4),
            5 => Val(MUXNEG_A::PIN5),
            6 => Val(MUXNEG_A::PIN6),
            7 => Val(MUXNEG_A::PIN7),
            24 => Val(MUXNEG_A::GND),
            25 => Val(MUXNEG_A::IOGND),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXNEG_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXNEG_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == MUXNEG_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == MUXNEG_A::PIN3
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == MUXNEG_A::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == MUXNEG_A::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == MUXNEG_A::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == MUXNEG_A::PIN7
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == MUXNEG_A::GND
    }
    #[doc = "Checks if the value of the field is `IOGND`"]
    #[inline(always)]
    pub fn is_iognd(&self) -> bool {
        *self == MUXNEG_A::IOGND
    }
}
#[doc = "Write proxy for field `MUXNEG`"]
pub struct MUXNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXNEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXNEG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN7)
    }
    #[doc = "Internal Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(MUXNEG_A::GND)
    }
    #[doc = "I/O Ground"]
    #[inline(always)]
    pub fn iognd(self) -> &'a mut W {
        self.variant(MUXNEG_A::IOGND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `INPUTSCAN`"]
pub type INPUTSCAN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INPUTSCAN`"]
pub struct INPUTSCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTSCAN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `INPUTOFFSET`"]
pub type INPUTOFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INPUTOFFSET`"]
pub struct INPUTOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Gain Factor Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN_A {
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
impl From<GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN`"]
pub type GAIN_R = crate::R<u8, GAIN_A>;
impl GAIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAIN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAIN_A::_1X),
            1 => Val(GAIN_A::_2X),
            2 => Val(GAIN_A::_4X),
            3 => Val(GAIN_A::_8X),
            4 => Val(GAIN_A::_16X),
            15 => Val(GAIN_A::DIV2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1X`"]
    #[inline(always)]
    pub fn is_1x(&self) -> bool {
        *self == GAIN_A::_1X
    }
    #[doc = "Checks if the value of the field is `_2X`"]
    #[inline(always)]
    pub fn is_2x(&self) -> bool {
        *self == GAIN_A::_2X
    }
    #[doc = "Checks if the value of the field is `_4X`"]
    #[inline(always)]
    pub fn is_4x(&self) -> bool {
        *self == GAIN_A::_4X
    }
    #[doc = "Checks if the value of the field is `_8X`"]
    #[inline(always)]
    pub fn is_8x(&self) -> bool {
        *self == GAIN_A::_8X
    }
    #[doc = "Checks if the value of the field is `_16X`"]
    #[inline(always)]
    pub fn is_16x(&self) -> bool {
        *self == GAIN_A::_16X
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == GAIN_A::DIV2
    }
}
#[doc = "Write proxy for field `GAIN`"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1x"]
    #[inline(always)]
    pub fn _1x(self) -> &'a mut W {
        self.variant(GAIN_A::_1X)
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn _2x(self) -> &'a mut W {
        self.variant(GAIN_A::_2X)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn _4x(self) -> &'a mut W {
        self.variant(GAIN_A::_4X)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn _8x(self) -> &'a mut W {
        self.variant(GAIN_A::_8X)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn _16x(self) -> &'a mut W {
        self.variant(GAIN_A::_16X)
    }
    #[doc = "1/2x"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(GAIN_A::DIV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
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
    pub fn muxpos(&mut self) -> MUXPOS_W {
        MUXPOS_W { w: self }
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MUXNEG_W {
        MUXNEG_W { w: self }
    }
    #[doc = "Bits 16:19 - Number of Input Channels Included in Scan"]
    #[inline(always)]
    pub fn inputscan(&mut self) -> INPUTSCAN_W {
        INPUTSCAN_W { w: self }
    }
    #[doc = "Bits 20:23 - Positive Mux Setting Offset"]
    #[inline(always)]
    pub fn inputoffset(&mut self) -> INPUTOFFSET_W {
        INPUTOFFSET_W { w: self }
    }
    #[doc = "Bits 24:27 - Gain Factor Selection"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
}
