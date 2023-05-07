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
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPCTRL_SPEC, bool, O>;
#[doc = "Field `SINGLE` reader - Single-Shot Mode"]
pub type SINGLE_R = crate::BitReader<bool>;
#[doc = "Field `SINGLE` writer - Single-Shot Mode"]
pub type SINGLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPCTRL_SPEC, bool, O>;
#[doc = "Field `SPEED` reader - Speed Selection"]
pub type SPEED_R = crate::FieldReader<u8, SPEEDSELECT_A>;
#[doc = "Speed Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPEEDSELECT_A {
    #[doc = "0: Low speed"]
    LOW = 0,
    #[doc = "1: High speed"]
    HIGH = 1,
}
impl From<SPEEDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEEDSELECT_A) -> Self {
        variant as _
    }
}
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPEEDSELECT_A> {
        match self.bits {
            0 => Some(SPEEDSELECT_A::LOW),
            1 => Some(SPEEDSELECT_A::HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPEEDSELECT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPEEDSELECT_A::HIGH
    }
}
#[doc = "Field `SPEED` writer - Speed Selection"]
pub type SPEED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMPCTRL_SPEC, u8, SPEEDSELECT_A, 2, O>;
impl<'a, const O: u8> SPEED_W<'a, O> {
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPEEDSELECT_A::LOW)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPEEDSELECT_A::HIGH)
    }
}
#[doc = "Field `INTSEL` reader - Interrupt Selection"]
pub type INTSEL_R = crate::FieldReader<u8, INTSELSELECT_A>;
#[doc = "Interrupt Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTSELSELECT_A {
    #[doc = "0: Interrupt on comparator output toggle"]
    TOGGLE = 0,
    #[doc = "1: Interrupt on comparator output rising"]
    RISING = 1,
    #[doc = "2: Interrupt on comparator output falling"]
    FALLING = 2,
    #[doc = "3: Interrupt on end of comparison (single-shot mode only)"]
    EOC = 3,
}
impl From<INTSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: INTSELSELECT_A) -> Self {
        variant as _
    }
}
impl INTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSELSELECT_A {
        match self.bits {
            0 => INTSELSELECT_A::TOGGLE,
            1 => INTSELSELECT_A::RISING,
            2 => INTSELSELECT_A::FALLING,
            3 => INTSELSELECT_A::EOC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == INTSELSELECT_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTSELSELECT_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTSELSELECT_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EOC`"]
    #[inline(always)]
    pub fn is_eoc(&self) -> bool {
        *self == INTSELSELECT_A::EOC
    }
}
#[doc = "Field `INTSEL` writer - Interrupt Selection"]
pub type INTSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMPCTRL_SPEC, u8, INTSELSELECT_A, 2, O>;
impl<'a, const O: u8> INTSEL_W<'a, O> {
    #[doc = "Interrupt on comparator output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(INTSELSELECT_A::TOGGLE)
    }
    #[doc = "Interrupt on comparator output rising"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTSELSELECT_A::RISING)
    }
    #[doc = "Interrupt on comparator output falling"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTSELSELECT_A::FALLING)
    }
    #[doc = "Interrupt on end of comparison (single-shot mode only)"]
    #[inline(always)]
    pub fn eoc(self) -> &'a mut W {
        self.variant(INTSELSELECT_A::EOC)
    }
}
#[doc = "Field `MUXNEG` reader - Negative Input Mux Selection"]
pub type MUXNEG_R = crate::FieldReader<u8, MUXNEGSELECT_A>;
#[doc = "Negative Input Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXNEGSELECT_A {
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
impl From<MUXNEGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXNEGSELECT_A) -> Self {
        variant as _
    }
}
impl MUXNEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUXNEGSELECT_A {
        match self.bits {
            0 => MUXNEGSELECT_A::PIN0,
            1 => MUXNEGSELECT_A::PIN1,
            2 => MUXNEGSELECT_A::PIN2,
            3 => MUXNEGSELECT_A::PIN3,
            4 => MUXNEGSELECT_A::GND,
            5 => MUXNEGSELECT_A::VSCALE,
            6 => MUXNEGSELECT_A::BANDGAP,
            7 => MUXNEGSELECT_A::DAC,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == MUXNEGSELECT_A::GND
    }
    #[doc = "Checks if the value of the field is `VSCALE`"]
    #[inline(always)]
    pub fn is_vscale(&self) -> bool {
        *self == MUXNEGSELECT_A::VSCALE
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXNEGSELECT_A::BANDGAP
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == MUXNEGSELECT_A::DAC
    }
}
#[doc = "Field `MUXNEG` writer - Negative Input Mux Selection"]
pub type MUXNEG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMPCTRL_SPEC, u8, MUXNEGSELECT_A, 3, O>;
impl<'a, const O: u8> MUXNEG_W<'a, O> {
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::PIN0)
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::PIN1)
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::PIN2)
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::PIN3)
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::GND)
    }
    #[doc = "VDD scaler"]
    #[inline(always)]
    pub fn vscale(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::VSCALE)
    }
    #[doc = "Internal bandgap voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::BANDGAP)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::DAC)
    }
}
#[doc = "Field `MUXPOS` reader - Positive Input Mux Selection"]
pub type MUXPOS_R = crate::FieldReader<u8, MUXPOSSELECT_A>;
#[doc = "Positive Input Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXPOSSELECT_A {
    #[doc = "0: I/O pin 0"]
    PIN0 = 0,
    #[doc = "1: I/O pin 1"]
    PIN1 = 1,
    #[doc = "2: I/O pin 2"]
    PIN2 = 2,
    #[doc = "3: I/O pin 3"]
    PIN3 = 3,
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
    pub fn variant(&self) -> MUXPOSSELECT_A {
        match self.bits {
            0 => MUXPOSSELECT_A::PIN0,
            1 => MUXPOSSELECT_A::PIN1,
            2 => MUXPOSSELECT_A::PIN2,
            3 => MUXPOSSELECT_A::PIN3,
            _ => unreachable!(),
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
}
#[doc = "Field `MUXPOS` writer - Positive Input Mux Selection"]
pub type MUXPOS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMPCTRL_SPEC, u8, MUXPOSSELECT_A, 2, O>;
impl<'a, const O: u8> MUXPOS_W<'a, O> {
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN0)
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN1)
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN2)
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::PIN3)
    }
}
#[doc = "Field `SWAP` reader - Swap Inputs and Invert"]
pub type SWAP_R = crate::BitReader<bool>;
#[doc = "Field `SWAP` writer - Swap Inputs and Invert"]
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPCTRL_SPEC, bool, O>;
#[doc = "Field `OUT` reader - Output"]
pub type OUT_R = crate::FieldReader<u8, OUTSELECT_A>;
#[doc = "Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTSELECT_A {
    #[doc = "0: The output of COMPn is not routed to the COMPn I/O port"]
    OFF = 0,
    #[doc = "1: The asynchronous output of COMPn is routed to the COMPn I/O port"]
    ASYNC = 1,
    #[doc = "2: The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    SYNC = 2,
}
impl From<OUTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTSELECT_A) -> Self {
        variant as _
    }
}
impl OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUTSELECT_A> {
        match self.bits {
            0 => Some(OUTSELECT_A::OFF),
            1 => Some(OUTSELECT_A::ASYNC),
            2 => Some(OUTSELECT_A::SYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == OUTSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == OUTSELECT_A::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == OUTSELECT_A::SYNC
    }
}
#[doc = "Field `OUT` writer - Output"]
pub type OUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMPCTRL_SPEC, u8, OUTSELECT_A, 2, O>;
impl<'a, const O: u8> OUT_W<'a, O> {
    #[doc = "The output of COMPn is not routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(OUTSELECT_A::OFF)
    }
    #[doc = "The asynchronous output of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(OUTSELECT_A::ASYNC)
    }
    #[doc = "The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(OUTSELECT_A::SYNC)
    }
}
#[doc = "Field `HYST` reader - Hysteresis Enable"]
pub type HYST_R = crate::BitReader<bool>;
#[doc = "Field `HYST` writer - Hysteresis Enable"]
pub type HYST_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPCTRL_SPEC, bool, O>;
#[doc = "Field `FLEN` reader - Filter Length"]
pub type FLEN_R = crate::FieldReader<u8, FLENSELECT_A>;
#[doc = "Filter Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLENSELECT_A {
    #[doc = "0: No filtering"]
    OFF = 0,
    #[doc = "1: 3-bit majority function (2 of 3)"]
    MAJ3 = 1,
    #[doc = "2: 5-bit majority function (3 of 5)"]
    MAJ5 = 2,
}
impl From<FLENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FLENSELECT_A) -> Self {
        variant as _
    }
}
impl FLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLENSELECT_A> {
        match self.bits {
            0 => Some(FLENSELECT_A::OFF),
            1 => Some(FLENSELECT_A::MAJ3),
            2 => Some(FLENSELECT_A::MAJ5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FLENSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `MAJ3`"]
    #[inline(always)]
    pub fn is_maj3(&self) -> bool {
        *self == FLENSELECT_A::MAJ3
    }
    #[doc = "Checks if the value of the field is `MAJ5`"]
    #[inline(always)]
    pub fn is_maj5(&self) -> bool {
        *self == FLENSELECT_A::MAJ5
    }
}
#[doc = "Field `FLEN` writer - Filter Length"]
pub type FLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMPCTRL_SPEC, u8, FLENSELECT_A, 3, O>;
impl<'a, const O: u8> FLEN_W<'a, O> {
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FLENSELECT_A::OFF)
    }
    #[doc = "3-bit majority function (2 of 3)"]
    #[inline(always)]
    pub fn maj3(self) -> &'a mut W {
        self.variant(FLENSELECT_A::MAJ3)
    }
    #[doc = "5-bit majority function (3 of 5)"]
    #[inline(always)]
    pub fn maj5(self) -> &'a mut W {
        self.variant(FLENSELECT_A::MAJ5)
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single-Shot Mode"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Speed Selection"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&self) -> INTSEL_R {
        INTSEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Output"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline(always)]
    pub fn flen(&self) -> FLEN_R {
        FLEN_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Single-Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SINGLE_W<1> {
        SINGLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Speed Selection"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<2> {
        SPEED_W::new(self)
    }
    #[doc = "Bits 5:6 - Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn intsel(&mut self) -> INTSEL_W<5> {
        INTSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<8> {
        MUXNEG_W::new(self)
    }
    #[doc = "Bits 12:13 - Positive Input Mux Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<12> {
        MUXPOS_W::new(self)
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<15> {
        SWAP_W::new(self)
    }
    #[doc = "Bits 16:17 - Output"]
    #[inline(always)]
    #[must_use]
    pub fn out(&mut self) -> OUT_W<16> {
        OUT_W::new(self)
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<19> {
        HYST_W::new(self)
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline(always)]
    #[must_use]
    pub fn flen(&mut self) -> FLEN_W<24> {
        FLEN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPCTRL%s to value 0"]
impl crate::Resettable for COMPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
