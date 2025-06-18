#[doc = "Register `COMPCTRL[%s]` reader"]
pub type R = crate::R<CompctrlSpec>;
#[doc = "Register `COMPCTRL[%s]` writer"]
pub type W = crate::W<CompctrlSpec>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLE` reader - Single-Shot Mode"]
pub type SingleR = crate::BitReader;
#[doc = "Field `SINGLE` writer - Single-Shot Mode"]
pub type SingleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Intselselect {
    #[doc = "0: Interrupt on comparator output toggle"]
    Toggle = 0,
    #[doc = "1: Interrupt on comparator output rising"]
    Rising = 1,
    #[doc = "2: Interrupt on comparator output falling"]
    Falling = 2,
    #[doc = "3: Interrupt on end of comparison (single-shot mode only)"]
    Eoc = 3,
}
impl From<Intselselect> for u8 {
    #[inline(always)]
    fn from(variant: Intselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Intselselect {
    type Ux = u8;
}
impl crate::IsEnum for Intselselect {}
#[doc = "Field `INTSEL` reader - Interrupt Selection"]
pub type IntselR = crate::FieldReader<Intselselect>;
impl IntselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intselselect {
        match self.bits {
            0 => Intselselect::Toggle,
            1 => Intselselect::Rising,
            2 => Intselselect::Falling,
            3 => Intselselect::Eoc,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt on comparator output toggle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Intselselect::Toggle
    }
    #[doc = "Interrupt on comparator output rising"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Intselselect::Rising
    }
    #[doc = "Interrupt on comparator output falling"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Intselselect::Falling
    }
    #[doc = "Interrupt on end of comparison (single-shot mode only)"]
    #[inline(always)]
    pub fn is_eoc(&self) -> bool {
        *self == Intselselect::Eoc
    }
}
#[doc = "Field `INTSEL` writer - Interrupt Selection"]
pub type IntselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Intselselect, crate::Safe>;
impl<'a, REG> IntselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt on comparator output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Intselselect::Toggle)
    }
    #[doc = "Interrupt on comparator output rising"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Intselselect::Rising)
    }
    #[doc = "Interrupt on comparator output falling"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Intselselect::Falling)
    }
    #[doc = "Interrupt on end of comparison (single-shot mode only)"]
    #[inline(always)]
    pub fn eoc(self) -> &'a mut crate::W<REG> {
        self.variant(Intselselect::Eoc)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Negative Input Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Muxnegselect {
    #[doc = "0: I/O pin 0"]
    Pin0 = 0,
    #[doc = "1: I/O pin 1"]
    Pin1 = 1,
    #[doc = "2: I/O pin 2"]
    Pin2 = 2,
    #[doc = "3: I/O pin 3"]
    Pin3 = 3,
    #[doc = "4: Ground"]
    Gnd = 4,
    #[doc = "5: VDD scaler"]
    Vscale = 5,
    #[doc = "6: Internal bandgap voltage"]
    Bandgap = 6,
    #[doc = "7: DAC output"]
    Dac = 7,
}
impl From<Muxnegselect> for u8 {
    #[inline(always)]
    fn from(variant: Muxnegselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Muxnegselect {
    type Ux = u8;
}
impl crate::IsEnum for Muxnegselect {}
#[doc = "Field `MUXNEG` reader - Negative Input Mux Selection"]
pub type MuxnegR = crate::FieldReader<Muxnegselect>;
impl MuxnegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Muxnegselect {
        match self.bits {
            0 => Muxnegselect::Pin0,
            1 => Muxnegselect::Pin1,
            2 => Muxnegselect::Pin2,
            3 => Muxnegselect::Pin3,
            4 => Muxnegselect::Gnd,
            5 => Muxnegselect::Vscale,
            6 => Muxnegselect::Bandgap,
            7 => Muxnegselect::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Muxnegselect::Pin0
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Muxnegselect::Pin1
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Muxnegselect::Pin2
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Muxnegselect::Pin3
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == Muxnegselect::Gnd
    }
    #[doc = "VDD scaler"]
    #[inline(always)]
    pub fn is_vscale(&self) -> bool {
        *self == Muxnegselect::Vscale
    }
    #[doc = "Internal bandgap voltage"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == Muxnegselect::Bandgap
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Muxnegselect::Dac
    }
}
#[doc = "Field `MUXNEG` writer - Negative Input Mux Selection"]
pub type MuxnegW<'a, REG> = crate::FieldWriter<'a, REG, 3, Muxnegselect, crate::Safe>;
impl<'a, REG> MuxnegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Pin0)
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Pin1)
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Pin2)
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Pin3)
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Gnd)
    }
    #[doc = "VDD scaler"]
    #[inline(always)]
    pub fn vscale(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Vscale)
    }
    #[doc = "Internal bandgap voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Bandgap)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Dac)
    }
}
#[doc = "Positive Input Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Muxposselect {
    #[doc = "0: I/O pin 0"]
    Pin0 = 0,
    #[doc = "1: I/O pin 1"]
    Pin1 = 1,
    #[doc = "2: I/O pin 2"]
    Pin2 = 2,
    #[doc = "3: I/O pin 3"]
    Pin3 = 3,
    #[doc = "4: VDD Scaler"]
    Vscale = 4,
}
impl From<Muxposselect> for u8 {
    #[inline(always)]
    fn from(variant: Muxposselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Muxposselect {
    type Ux = u8;
}
impl crate::IsEnum for Muxposselect {}
#[doc = "Field `MUXPOS` reader - Positive Input Mux Selection"]
pub type MuxposR = crate::FieldReader<Muxposselect>;
impl MuxposR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Muxposselect> {
        match self.bits {
            0 => Some(Muxposselect::Pin0),
            1 => Some(Muxposselect::Pin1),
            2 => Some(Muxposselect::Pin2),
            3 => Some(Muxposselect::Pin3),
            4 => Some(Muxposselect::Vscale),
            _ => None,
        }
    }
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Muxposselect::Pin0
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Muxposselect::Pin1
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Muxposselect::Pin2
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Muxposselect::Pin3
    }
    #[doc = "VDD Scaler"]
    #[inline(always)]
    pub fn is_vscale(&self) -> bool {
        *self == Muxposselect::Vscale
    }
}
#[doc = "Field `MUXPOS` writer - Positive Input Mux Selection"]
pub type MuxposW<'a, REG> = crate::FieldWriter<'a, REG, 3, Muxposselect>;
impl<'a, REG> MuxposW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin0)
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin1)
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin2)
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin3)
    }
    #[doc = "VDD Scaler"]
    #[inline(always)]
    pub fn vscale(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Vscale)
    }
}
#[doc = "Field `SWAP` reader - Swap Inputs and Invert"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - Swap Inputs and Invert"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Speed Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Speedselect {
    #[doc = "3: High speed"]
    High = 3,
}
impl From<Speedselect> for u8 {
    #[inline(always)]
    fn from(variant: Speedselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Speedselect {
    type Ux = u8;
}
impl crate::IsEnum for Speedselect {}
#[doc = "Field `SPEED` reader - Speed Selection"]
pub type SpeedR = crate::FieldReader<Speedselect>;
impl SpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Speedselect> {
        match self.bits {
            3 => Some(Speedselect::High),
            _ => None,
        }
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Speedselect::High
    }
}
#[doc = "Field `SPEED` writer - Speed Selection"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2, Speedselect>;
impl<'a, REG> SpeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Speedselect::High)
    }
}
#[doc = "Field `HYSTEN` reader - Hysteresis Enable"]
pub type HystenR = crate::BitReader;
#[doc = "Field `HYSTEN` writer - Hysteresis Enable"]
pub type HystenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Hysteresis Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hystselect {
    #[doc = "0: 25mV"]
    Hyst25 = 0,
    #[doc = "1: 50mV"]
    Hyst50 = 1,
    #[doc = "2: 75mV"]
    Hyst75 = 2,
    #[doc = "3: 100mV"]
    Hyst100 = 3,
}
impl From<Hystselect> for u8 {
    #[inline(always)]
    fn from(variant: Hystselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hystselect {
    type Ux = u8;
}
impl crate::IsEnum for Hystselect {}
#[doc = "Field `HYST` reader - Hysteresis Level"]
pub type HystR = crate::FieldReader<Hystselect>;
impl HystR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hystselect {
        match self.bits {
            0 => Hystselect::Hyst25,
            1 => Hystselect::Hyst50,
            2 => Hystselect::Hyst75,
            3 => Hystselect::Hyst100,
            _ => unreachable!(),
        }
    }
    #[doc = "25mV"]
    #[inline(always)]
    pub fn is_hyst25(&self) -> bool {
        *self == Hystselect::Hyst25
    }
    #[doc = "50mV"]
    #[inline(always)]
    pub fn is_hyst50(&self) -> bool {
        *self == Hystselect::Hyst50
    }
    #[doc = "75mV"]
    #[inline(always)]
    pub fn is_hyst75(&self) -> bool {
        *self == Hystselect::Hyst75
    }
    #[doc = "100mV"]
    #[inline(always)]
    pub fn is_hyst100(&self) -> bool {
        *self == Hystselect::Hyst100
    }
}
#[doc = "Field `HYST` writer - Hysteresis Level"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hystselect, crate::Safe>;
impl<'a, REG> HystW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "25mV"]
    #[inline(always)]
    pub fn hyst25(self) -> &'a mut crate::W<REG> {
        self.variant(Hystselect::Hyst25)
    }
    #[doc = "50mV"]
    #[inline(always)]
    pub fn hyst50(self) -> &'a mut crate::W<REG> {
        self.variant(Hystselect::Hyst50)
    }
    #[doc = "75mV"]
    #[inline(always)]
    pub fn hyst75(self) -> &'a mut crate::W<REG> {
        self.variant(Hystselect::Hyst75)
    }
    #[doc = "100mV"]
    #[inline(always)]
    pub fn hyst100(self) -> &'a mut crate::W<REG> {
        self.variant(Hystselect::Hyst100)
    }
}
#[doc = "Filter Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flenselect {
    #[doc = "0: No filtering"]
    Off = 0,
    #[doc = "1: 3-bit majority function (2 of 3)"]
    Maj3 = 1,
    #[doc = "2: 5-bit majority function (3 of 5)"]
    Maj5 = 2,
}
impl From<Flenselect> for u8 {
    #[inline(always)]
    fn from(variant: Flenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flenselect {
    type Ux = u8;
}
impl crate::IsEnum for Flenselect {}
#[doc = "Field `FLEN` reader - Filter Length"]
pub type FlenR = crate::FieldReader<Flenselect>;
impl FlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flenselect> {
        match self.bits {
            0 => Some(Flenselect::Off),
            1 => Some(Flenselect::Maj3),
            2 => Some(Flenselect::Maj5),
            _ => None,
        }
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Flenselect::Off
    }
    #[doc = "3-bit majority function (2 of 3)"]
    #[inline(always)]
    pub fn is_maj3(&self) -> bool {
        *self == Flenselect::Maj3
    }
    #[doc = "5-bit majority function (3 of 5)"]
    #[inline(always)]
    pub fn is_maj5(&self) -> bool {
        *self == Flenselect::Maj5
    }
}
#[doc = "Field `FLEN` writer - Filter Length"]
pub type FlenW<'a, REG> = crate::FieldWriter<'a, REG, 3, Flenselect>;
impl<'a, REG> FlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Flenselect::Off)
    }
    #[doc = "3-bit majority function (2 of 3)"]
    #[inline(always)]
    pub fn maj3(self) -> &'a mut crate::W<REG> {
        self.variant(Flenselect::Maj3)
    }
    #[doc = "5-bit majority function (3 of 5)"]
    #[inline(always)]
    pub fn maj5(self) -> &'a mut crate::W<REG> {
        self.variant(Flenselect::Maj5)
    }
}
#[doc = "Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outselect {
    #[doc = "0: The output of COMPn is not routed to the COMPn I/O port"]
    Off = 0,
    #[doc = "1: The asynchronous output of COMPn is routed to the COMPn I/O port"]
    Async = 1,
    #[doc = "2: The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    Sync = 2,
}
impl From<Outselect> for u8 {
    #[inline(always)]
    fn from(variant: Outselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outselect {
    type Ux = u8;
}
impl crate::IsEnum for Outselect {}
#[doc = "Field `OUT` reader - Output"]
pub type OutR = crate::FieldReader<Outselect>;
impl OutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outselect> {
        match self.bits {
            0 => Some(Outselect::Off),
            1 => Some(Outselect::Async),
            2 => Some(Outselect::Sync),
            _ => None,
        }
    }
    #[doc = "The output of COMPn is not routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Outselect::Off
    }
    #[doc = "The asynchronous output of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == Outselect::Async
    }
    #[doc = "The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Outselect::Sync
    }
}
#[doc = "Field `OUT` writer - Output"]
pub type OutW<'a, REG> = crate::FieldWriter<'a, REG, 2, Outselect>;
impl<'a, REG> OutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The output of COMPn is not routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Outselect::Off)
    }
    #[doc = "The asynchronous output of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(Outselect::Async)
    }
    #[doc = "The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Outselect::Sync)
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single-Shot Mode"]
    #[inline(always)]
    pub fn single(&self) -> SingleR {
        SingleR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&self) -> IntselR {
        IntselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MuxnegR {
        MuxnegR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MuxposR {
        MuxposR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Speed Selection"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&self) -> HystenR {
        HystenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Hysteresis Level"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline(always)]
    pub fn flen(&self) -> FlenR {
        FlenR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Output"]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CompctrlSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Single-Shot Mode"]
    #[inline(always)]
    pub fn single(&mut self) -> SingleW<CompctrlSpec> {
        SingleW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&mut self) -> IntselW<CompctrlSpec> {
        IntselW::new(self, 3)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<CompctrlSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MuxnegW<CompctrlSpec> {
        MuxnegW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MuxposW<CompctrlSpec> {
        MuxposW::new(self, 12)
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<CompctrlSpec> {
        SwapW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Speed Selection"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<CompctrlSpec> {
        SpeedW::new(self, 16)
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&mut self) -> HystenW<CompctrlSpec> {
        HystenW::new(self, 19)
    }
    #[doc = "Bits 20:21 - Hysteresis Level"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<CompctrlSpec> {
        HystW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline(always)]
    pub fn flen(&mut self) -> FlenW<CompctrlSpec> {
        FlenW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Output"]
    #[inline(always)]
    pub fn out(&mut self) -> OutW<CompctrlSpec> {
        OutW::new(self, 28)
    }
}
#[doc = "Comparator Control n\n\nYou can [`read`](crate::Reg::read) this register and get [`compctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompctrlSpec;
impl crate::RegisterSpec for CompctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compctrl::R`](R) reader structure"]
impl crate::Readable for CompctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`compctrl::W`](W) writer structure"]
impl crate::Writable for CompctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPCTRL[%s] to value 0"]
impl crate::Resettable for CompctrlSpec {}
