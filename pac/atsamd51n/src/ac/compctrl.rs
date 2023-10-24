#[doc = "Register `COMPCTRL[%s]` reader"]
pub type R = crate::R<COMPCTRL_SPEC>;
#[doc = "Register `COMPCTRL[%s]` writer"]
pub type W = crate::W<COMPCTRL_SPEC>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SINGLE` reader - Single-Shot Mode"]
pub type SINGLE_R = crate::BitReader;
#[doc = "Field `SINGLE` writer - Single-Shot Mode"]
pub type SINGLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTSEL` reader - Interrupt Selection"]
pub type INTSEL_R = crate::FieldReader<INTSELSELECT_A>;
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
impl crate::FieldSpec for INTSELSELECT_A {
    type Ux = u8;
}
impl INTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTSELSELECT_A {
        match self.bits {
            0 => INTSELSELECT_A::TOGGLE,
            1 => INTSELSELECT_A::RISING,
            2 => INTSELSELECT_A::FALLING,
            3 => INTSELSELECT_A::EOC,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt on comparator output toggle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == INTSELSELECT_A::TOGGLE
    }
    #[doc = "Interrupt on comparator output rising"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTSELSELECT_A::RISING
    }
    #[doc = "Interrupt on comparator output falling"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTSELSELECT_A::FALLING
    }
    #[doc = "Interrupt on end of comparison (single-shot mode only)"]
    #[inline(always)]
    pub fn is_eoc(&self) -> bool {
        *self == INTSELSELECT_A::EOC
    }
}
#[doc = "Field `INTSEL` writer - Interrupt Selection"]
pub type INTSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, INTSELSELECT_A>;
impl<'a, REG, const O: u8> INTSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt on comparator output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(INTSELSELECT_A::TOGGLE)
    }
    #[doc = "Interrupt on comparator output rising"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(INTSELSELECT_A::RISING)
    }
    #[doc = "Interrupt on comparator output falling"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(INTSELSELECT_A::FALLING)
    }
    #[doc = "Interrupt on end of comparison (single-shot mode only)"]
    #[inline(always)]
    pub fn eoc(self) -> &'a mut crate::W<REG> {
        self.variant(INTSELSELECT_A::EOC)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MUXNEG` reader - Negative Input Mux Selection"]
pub type MUXNEG_R = crate::FieldReader<MUXNEGSELECT_A>;
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
impl crate::FieldSpec for MUXNEGSELECT_A {
    type Ux = u8;
}
impl MUXNEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MUXNEGSELECT_A {
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
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXNEGSELECT_A::PIN0
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXNEGSELECT_A::PIN1
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == MUXNEGSELECT_A::PIN2
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == MUXNEGSELECT_A::PIN3
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == MUXNEGSELECT_A::GND
    }
    #[doc = "VDD scaler"]
    #[inline(always)]
    pub fn is_vscale(&self) -> bool {
        *self == MUXNEGSELECT_A::VSCALE
    }
    #[doc = "Internal bandgap voltage"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXNEGSELECT_A::BANDGAP
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == MUXNEGSELECT_A::DAC
    }
}
#[doc = "Field `MUXNEG` writer - Negative Input Mux Selection"]
pub type MUXNEG_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, MUXNEGSELECT_A>;
impl<'a, REG, const O: u8> MUXNEG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::PIN0)
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::PIN1)
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::PIN2)
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::PIN3)
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::GND)
    }
    #[doc = "VDD scaler"]
    #[inline(always)]
    pub fn vscale(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::VSCALE)
    }
    #[doc = "Internal bandgap voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::BANDGAP)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::DAC)
    }
}
#[doc = "Field `MUXPOS` reader - Positive Input Mux Selection"]
pub type MUXPOS_R = crate::FieldReader<MUXPOSSELECT_A>;
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
    #[doc = "4: VDD Scaler"]
    VSCALE = 4,
}
impl From<MUXPOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXPOSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MUXPOSSELECT_A {
    type Ux = u8;
}
impl MUXPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MUXPOSSELECT_A> {
        match self.bits {
            0 => Some(MUXPOSSELECT_A::PIN0),
            1 => Some(MUXPOSSELECT_A::PIN1),
            2 => Some(MUXPOSSELECT_A::PIN2),
            3 => Some(MUXPOSSELECT_A::PIN3),
            4 => Some(MUXPOSSELECT_A::VSCALE),
            _ => None,
        }
    }
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN0
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN1
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN2
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == MUXPOSSELECT_A::PIN3
    }
    #[doc = "VDD Scaler"]
    #[inline(always)]
    pub fn is_vscale(&self) -> bool {
        *self == MUXPOSSELECT_A::VSCALE
    }
}
#[doc = "Field `MUXPOS` writer - Positive Input Mux Selection"]
pub type MUXPOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, MUXPOSSELECT_A>;
impl<'a, REG, const O: u8> MUXPOS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::PIN0)
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::PIN1)
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::PIN2)
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::PIN3)
    }
    #[doc = "VDD Scaler"]
    #[inline(always)]
    pub fn vscale(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::VSCALE)
    }
}
#[doc = "Field `SWAP` reader - Swap Inputs and Invert"]
pub type SWAP_R = crate::BitReader;
#[doc = "Field `SWAP` writer - Swap Inputs and Invert"]
pub type SWAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPEED` reader - Speed Selection"]
pub type SPEED_R = crate::FieldReader<SPEEDSELECT_A>;
#[doc = "Speed Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPEEDSELECT_A {
    #[doc = "3: High speed"]
    HIGH = 3,
}
impl From<SPEEDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEEDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPEEDSELECT_A {
    type Ux = u8;
}
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPEEDSELECT_A> {
        match self.bits {
            3 => Some(SPEEDSELECT_A::HIGH),
            _ => None,
        }
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPEEDSELECT_A::HIGH
    }
}
#[doc = "Field `SPEED` writer - Speed Selection"]
pub type SPEED_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SPEEDSELECT_A>;
impl<'a, REG, const O: u8> SPEED_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SPEEDSELECT_A::HIGH)
    }
}
#[doc = "Field `HYSTEN` reader - Hysteresis Enable"]
pub type HYSTEN_R = crate::BitReader;
#[doc = "Field `HYSTEN` writer - Hysteresis Enable"]
pub type HYSTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HYST` reader - Hysteresis Level"]
pub type HYST_R = crate::FieldReader<HYSTSELECT_A>;
#[doc = "Hysteresis Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYSTSELECT_A {
    #[doc = "0: 50mV"]
    HYST50 = 0,
    #[doc = "1: 100mV"]
    HYST100 = 1,
    #[doc = "2: 150mV"]
    HYST150 = 2,
}
impl From<HYSTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: HYSTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HYSTSELECT_A {
    type Ux = u8;
}
impl HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HYSTSELECT_A> {
        match self.bits {
            0 => Some(HYSTSELECT_A::HYST50),
            1 => Some(HYSTSELECT_A::HYST100),
            2 => Some(HYSTSELECT_A::HYST150),
            _ => None,
        }
    }
    #[doc = "50mV"]
    #[inline(always)]
    pub fn is_hyst50(&self) -> bool {
        *self == HYSTSELECT_A::HYST50
    }
    #[doc = "100mV"]
    #[inline(always)]
    pub fn is_hyst100(&self) -> bool {
        *self == HYSTSELECT_A::HYST100
    }
    #[doc = "150mV"]
    #[inline(always)]
    pub fn is_hyst150(&self) -> bool {
        *self == HYSTSELECT_A::HYST150
    }
}
#[doc = "Field `HYST` writer - Hysteresis Level"]
pub type HYST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, HYSTSELECT_A>;
impl<'a, REG, const O: u8> HYST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "50mV"]
    #[inline(always)]
    pub fn hyst50(self) -> &'a mut crate::W<REG> {
        self.variant(HYSTSELECT_A::HYST50)
    }
    #[doc = "100mV"]
    #[inline(always)]
    pub fn hyst100(self) -> &'a mut crate::W<REG> {
        self.variant(HYSTSELECT_A::HYST100)
    }
    #[doc = "150mV"]
    #[inline(always)]
    pub fn hyst150(self) -> &'a mut crate::W<REG> {
        self.variant(HYSTSELECT_A::HYST150)
    }
}
#[doc = "Field `FLEN` reader - Filter Length"]
pub type FLEN_R = crate::FieldReader<FLENSELECT_A>;
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
impl crate::FieldSpec for FLENSELECT_A {
    type Ux = u8;
}
impl FLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FLENSELECT_A> {
        match self.bits {
            0 => Some(FLENSELECT_A::OFF),
            1 => Some(FLENSELECT_A::MAJ3),
            2 => Some(FLENSELECT_A::MAJ5),
            _ => None,
        }
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FLENSELECT_A::OFF
    }
    #[doc = "3-bit majority function (2 of 3)"]
    #[inline(always)]
    pub fn is_maj3(&self) -> bool {
        *self == FLENSELECT_A::MAJ3
    }
    #[doc = "5-bit majority function (3 of 5)"]
    #[inline(always)]
    pub fn is_maj5(&self) -> bool {
        *self == FLENSELECT_A::MAJ5
    }
}
#[doc = "Field `FLEN` writer - Filter Length"]
pub type FLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, FLENSELECT_A>;
impl<'a, REG, const O: u8> FLEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(FLENSELECT_A::OFF)
    }
    #[doc = "3-bit majority function (2 of 3)"]
    #[inline(always)]
    pub fn maj3(self) -> &'a mut crate::W<REG> {
        self.variant(FLENSELECT_A::MAJ3)
    }
    #[doc = "5-bit majority function (3 of 5)"]
    #[inline(always)]
    pub fn maj5(self) -> &'a mut crate::W<REG> {
        self.variant(FLENSELECT_A::MAJ5)
    }
}
#[doc = "Field `OUT` reader - Output"]
pub type OUT_R = crate::FieldReader<OUTSELECT_A>;
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
impl crate::FieldSpec for OUTSELECT_A {
    type Ux = u8;
}
impl OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OUTSELECT_A> {
        match self.bits {
            0 => Some(OUTSELECT_A::OFF),
            1 => Some(OUTSELECT_A::ASYNC),
            2 => Some(OUTSELECT_A::SYNC),
            _ => None,
        }
    }
    #[doc = "The output of COMPn is not routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == OUTSELECT_A::OFF
    }
    #[doc = "The asynchronous output of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == OUTSELECT_A::ASYNC
    }
    #[doc = "The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == OUTSELECT_A::SYNC
    }
}
#[doc = "Field `OUT` writer - Output"]
pub type OUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, OUTSELECT_A>;
impl<'a, REG, const O: u8> OUT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The output of COMPn is not routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(OUTSELECT_A::OFF)
    }
    #[doc = "The asynchronous output of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(OUTSELECT_A::ASYNC)
    }
    #[doc = "The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(OUTSELECT_A::SYNC)
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single-Shot Mode"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&self) -> INTSEL_R {
        INTSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Speed Selection"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&self) -> HYSTEN_R {
        HYSTEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Hysteresis Level"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline(always)]
    pub fn flen(&self) -> FLEN_R {
        FLEN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Output"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<COMPCTRL_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Single-Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SINGLE_W<COMPCTRL_SPEC, 2> {
        SINGLE_W::new(self)
    }
    #[doc = "Bits 3:4 - Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn intsel(&mut self) -> INTSEL_W<COMPCTRL_SPEC, 3> {
        INTSEL_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<COMPCTRL_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<COMPCTRL_SPEC, 8> {
        MUXNEG_W::new(self)
    }
    #[doc = "Bits 12:14 - Positive Input Mux Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<COMPCTRL_SPEC, 12> {
        MUXPOS_W::new(self)
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<COMPCTRL_SPEC, 15> {
        SWAP_W::new(self)
    }
    #[doc = "Bits 16:17 - Speed Selection"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<COMPCTRL_SPEC, 16> {
        SPEED_W::new(self)
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hysten(&mut self) -> HYSTEN_W<COMPCTRL_SPEC, 19> {
        HYSTEN_W::new(self)
    }
    #[doc = "Bits 20:21 - Hysteresis Level"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<COMPCTRL_SPEC, 20> {
        HYST_W::new(self)
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline(always)]
    #[must_use]
    pub fn flen(&mut self) -> FLEN_W<COMPCTRL_SPEC, 24> {
        FLEN_W::new(self)
    }
    #[doc = "Bits 28:29 - Output"]
    #[inline(always)]
    #[must_use]
    pub fn out(&mut self) -> OUT_W<COMPCTRL_SPEC, 28> {
        OUT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Comparator Control n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMPCTRL_SPEC;
impl crate::RegisterSpec for COMPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compctrl::R`](R) reader structure"]
impl crate::Readable for COMPCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`compctrl::W`](W) writer structure"]
impl crate::Writable for COMPCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPCTRL[%s]
to value 0"]
impl crate::Resettable for COMPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
