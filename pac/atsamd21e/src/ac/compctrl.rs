#[doc = "Register `COMPCTRL%s` reader"]
pub struct R(crate::R<COMPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPCTRL%s` writer"]
pub struct W(crate::W<COMPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPCTRL_SPEC>;
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
impl From<crate::W<COMPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SINGLE` reader - Single-Shot Mode"]
pub struct SINGLE_R(crate::FieldReader<bool, bool>);
impl SINGLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SINGLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINGLE` writer - Single-Shot Mode"]
pub struct SINGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Speed Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: Low speed"]
    LOW = 0,
    #[doc = "1: High speed"]
    HIGH = 1,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPEED` reader - Speed Selection"]
pub struct SPEED_R(crate::FieldReader<u8, SPEED_A>);
impl SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPEED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPEED_A> {
        match self.bits {
            0 => Some(SPEED_A::LOW),
            1 => Some(SPEED_A::HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SPEED_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SPEED_A::HIGH
    }
}
impl core::ops::Deref for SPEED_R {
    type Target = crate::FieldReader<u8, SPEED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPEED` writer - Speed Selection"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPEED_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPEED_A::LOW)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPEED_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Interrupt Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTSEL_A {
    #[doc = "0: Interrupt on comparator output toggle"]
    TOGGLE = 0,
    #[doc = "1: Interrupt on comparator output rising"]
    RISING = 1,
    #[doc = "2: Interrupt on comparator output falling"]
    FALLING = 2,
    #[doc = "3: Interrupt on end of comparison (single-shot mode only)"]
    EOC = 3,
}
impl From<INTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTSEL` reader - Interrupt Selection"]
pub struct INTSEL_R(crate::FieldReader<u8, INTSEL_A>);
impl INTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSEL_A {
        match self.bits {
            0 => INTSEL_A::TOGGLE,
            1 => INTSEL_A::RISING,
            2 => INTSEL_A::FALLING,
            3 => INTSEL_A::EOC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == INTSEL_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == INTSEL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == INTSEL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EOC`"]
    #[inline(always)]
    pub fn is_eoc(&self) -> bool {
        **self == INTSEL_A::EOC
    }
}
impl core::ops::Deref for INTSEL_R {
    type Target = crate::FieldReader<u8, INTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSEL` writer - Interrupt Selection"]
pub struct INTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Interrupt on comparator output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(INTSEL_A::TOGGLE)
    }
    #[doc = "Interrupt on comparator output rising"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTSEL_A::RISING)
    }
    #[doc = "Interrupt on comparator output falling"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTSEL_A::FALLING)
    }
    #[doc = "Interrupt on end of comparison (single-shot mode only)"]
    #[inline(always)]
    pub fn eoc(self) -> &'a mut W {
        self.variant(INTSEL_A::EOC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Negative Input Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUXNEG_A {
    #[doc = "0: I/O pin 0"]
    PIN0 = 0,
    #[doc = "1: I/O pin 1"]
    PIN1 = 1,
    #[doc = "2: I/O pin 2"]
    PIN2 = 2,
    #[doc = "3: I/O pin 3"]
    PIN3 = 3,
    #[doc = "4: Ground"]
    GND = 4,
    #[doc = "5: VDD scaler"]
    VSCALE = 5,
    #[doc = "6: Internal bandgap voltage"]
    BANDGAP = 6,
    #[doc = "7: DAC output"]
    DAC = 7,
}
impl From<MUXNEG_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXNEG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MUXNEG` reader - Negative Input Mux Selection"]
pub struct MUXNEG_R(crate::FieldReader<u8, MUXNEG_A>);
impl MUXNEG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MUXNEG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUXNEG_A {
        match self.bits {
            0 => MUXNEG_A::PIN0,
            1 => MUXNEG_A::PIN1,
            2 => MUXNEG_A::PIN2,
            3 => MUXNEG_A::PIN3,
            4 => MUXNEG_A::GND,
            5 => MUXNEG_A::VSCALE,
            6 => MUXNEG_A::BANDGAP,
            7 => MUXNEG_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        **self == MUXNEG_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        **self == MUXNEG_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        **self == MUXNEG_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        **self == MUXNEG_A::PIN3
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        **self == MUXNEG_A::GND
    }
    #[doc = "Checks if the value of the field is `VSCALE`"]
    #[inline(always)]
    pub fn is_vscale(&self) -> bool {
        **self == MUXNEG_A::VSCALE
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        **self == MUXNEG_A::BANDGAP
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        **self == MUXNEG_A::DAC
    }
}
impl core::ops::Deref for MUXNEG_R {
    type Target = crate::FieldReader<u8, MUXNEG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUXNEG` writer - Negative Input Mux Selection"]
pub struct MUXNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXNEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXNEG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN0)
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN1)
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN2)
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN3)
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(MUXNEG_A::GND)
    }
    #[doc = "VDD scaler"]
    #[inline(always)]
    pub fn vscale(self) -> &'a mut W {
        self.variant(MUXNEG_A::VSCALE)
    }
    #[doc = "Internal bandgap voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(MUXNEG_A::BANDGAP)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(MUXNEG_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Positive Input Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUXPOS_A {
    #[doc = "0: I/O pin 0"]
    PIN0 = 0,
    #[doc = "1: I/O pin 1"]
    PIN1 = 1,
    #[doc = "2: I/O pin 2"]
    PIN2 = 2,
    #[doc = "3: I/O pin 3"]
    PIN3 = 3,
}
impl From<MUXPOS_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXPOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MUXPOS` reader - Positive Input Mux Selection"]
pub struct MUXPOS_R(crate::FieldReader<u8, MUXPOS_A>);
impl MUXPOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MUXPOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUXPOS_A {
        match self.bits {
            0 => MUXPOS_A::PIN0,
            1 => MUXPOS_A::PIN1,
            2 => MUXPOS_A::PIN2,
            3 => MUXPOS_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        **self == MUXPOS_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        **self == MUXPOS_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        **self == MUXPOS_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        **self == MUXPOS_A::PIN3
    }
}
impl core::ops::Deref for MUXPOS_R {
    type Target = crate::FieldReader<u8, MUXPOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUXPOS` writer - Positive Input Mux Selection"]
pub struct MUXPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXPOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXPOS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN0)
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN1)
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN2)
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `SWAP` reader - Swap Inputs and Invert"]
pub struct SWAP_R(crate::FieldReader<bool, bool>);
impl SWAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAP` writer - Swap Inputs and Invert"]
pub struct SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUT_A {
    #[doc = "0: The output of COMPn is not routed to the COMPn I/O port"]
    OFF = 0,
    #[doc = "1: The asynchronous output of COMPn is routed to the COMPn I/O port"]
    ASYNC = 1,
    #[doc = "2: The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    SYNC = 2,
}
impl From<OUT_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OUT` reader - Output"]
pub struct OUT_R(crate::FieldReader<u8, OUT_A>);
impl OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUT_A> {
        match self.bits {
            0 => Some(OUT_A::OFF),
            1 => Some(OUT_A::ASYNC),
            2 => Some(OUT_A::SYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == OUT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        **self == OUT_A::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        **self == OUT_A::SYNC
    }
}
impl core::ops::Deref for OUT_R {
    type Target = crate::FieldReader<u8, OUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT` writer - Output"]
pub struct OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The output of COMPn is not routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(OUT_A::OFF)
    }
    #[doc = "The asynchronous output of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(OUT_A::ASYNC)
    }
    #[doc = "The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(OUT_A::SYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `HYST` reader - Hysteresis Enable"]
pub struct HYST_R(crate::FieldReader<bool, bool>);
impl HYST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HYST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYST` writer - Hysteresis Enable"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Filter Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEN_A {
    #[doc = "0: No filtering"]
    OFF = 0,
    #[doc = "1: 3-bit majority function (2 of 3)"]
    MAJ3 = 1,
    #[doc = "2: 5-bit majority function (3 of 5)"]
    MAJ5 = 2,
}
impl From<FLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLEN` reader - Filter Length"]
pub struct FLEN_R(crate::FieldReader<u8, FLEN_A>);
impl FLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLEN_A> {
        match self.bits {
            0 => Some(FLEN_A::OFF),
            1 => Some(FLEN_A::MAJ3),
            2 => Some(FLEN_A::MAJ5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == FLEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `MAJ3`"]
    #[inline(always)]
    pub fn is_maj3(&self) -> bool {
        **self == FLEN_A::MAJ3
    }
    #[doc = "Checks if the value of the field is `MAJ5`"]
    #[inline(always)]
    pub fn is_maj5(&self) -> bool {
        **self == FLEN_A::MAJ5
    }
}
impl core::ops::Deref for FLEN_R {
    type Target = crate::FieldReader<u8, FLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEN` writer - Filter Length"]
pub struct FLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FLEN_A::OFF)
    }
    #[doc = "3-bit majority function (2 of 3)"]
    #[inline(always)]
    pub fn maj3(self) -> &'a mut W {
        self.variant(FLEN_A::MAJ3)
    }
    #[doc = "5-bit majority function (3 of 5)"]
    #[inline(always)]
    pub fn maj5(self) -> &'a mut W {
        self.variant(FLEN_A::MAJ5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Single-Shot Mode"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Speed Selection"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&self) -> INTSEL_R {
        INTSEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Output"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline(always)]
    pub fn flen(&self) -> FLEN_R {
        FLEN_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Single-Shot Mode"]
    #[inline(always)]
    pub fn single(&mut self) -> SINGLE_W {
        SINGLE_W { w: self }
    }
    #[doc = "Bits 2:3 - Speed Selection"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bits 5:6 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&mut self) -> INTSEL_W {
        INTSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MUXNEG_W {
        MUXNEG_W { w: self }
    }
    #[doc = "Bits 12:13 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MUXPOS_W {
        MUXPOS_W { w: self }
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W {
        SWAP_W { w: self }
    }
    #[doc = "Bits 16:17 - Output"]
    #[inline(always)]
    pub fn out(&mut self) -> OUT_W {
        OUT_W { w: self }
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline(always)]
    pub fn flen(&mut self) -> FLEN_W {
        FLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compctrl](index.html) module"]
pub struct COMPCTRL_SPEC;
impl crate::RegisterSpec for COMPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [compctrl::R](R) reader structure"]
impl crate::Readable for COMPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compctrl::W](W) writer structure"]
impl crate::Writable for COMPCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMPCTRL%s to value 0"]
impl crate::Resettable for COMPCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
