#[doc = "Register `INPUTCTRL` reader"]
pub type R = crate::R<InputctrlSpec>;
#[doc = "Register `INPUTCTRL` writer"]
pub type W = crate::W<InputctrlSpec>;
#[doc = "Positive Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Muxposselect {
    #[doc = "0: ADC AIN0 Pin"]
    Pin0 = 0,
    #[doc = "1: ADC AIN1 Pin"]
    Pin1 = 1,
    #[doc = "2: ADC AIN2 Pin"]
    Pin2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    Pin3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    Pin4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    Pin5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    Pin6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    Pin7 = 7,
    #[doc = "8: ADC AIN8 Pin"]
    Pin8 = 8,
    #[doc = "9: ADC AIN9 Pin"]
    Pin9 = 9,
    #[doc = "10: ADC AIN10 Pin"]
    Pin10 = 10,
    #[doc = "11: ADC AIN11 Pin"]
    Pin11 = 11,
    #[doc = "12: ADC AIN12 Pin"]
    Pin12 = 12,
    #[doc = "13: ADC AIN13 Pin"]
    Pin13 = 13,
    #[doc = "14: ADC AIN14 Pin"]
    Pin14 = 14,
    #[doc = "15: ADC AIN15 Pin"]
    Pin15 = 15,
    #[doc = "16: ADC AIN16 Pin"]
    Pin16 = 16,
    #[doc = "17: ADC AIN17 Pin"]
    Pin17 = 17,
    #[doc = "18: ADC AIN18 Pin"]
    Pin18 = 18,
    #[doc = "19: ADC AIN19 Pin"]
    Pin19 = 19,
    #[doc = "24: Temperature Reference"]
    Temp = 24,
    #[doc = "25: Bandgap Voltage"]
    Bandgap = 25,
    #[doc = "26: 1/4 Scaled Core Supply"]
    Scaledcorevcc = 26,
    #[doc = "27: 1/4 Scaled I/O Supply"]
    Scalediovcc = 27,
    #[doc = "28: DAC Output"]
    Dac = 28,
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
#[doc = "Field `MUXPOS` reader - Positive Mux Input Selection"]
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
            4 => Some(Muxposselect::Pin4),
            5 => Some(Muxposselect::Pin5),
            6 => Some(Muxposselect::Pin6),
            7 => Some(Muxposselect::Pin7),
            8 => Some(Muxposselect::Pin8),
            9 => Some(Muxposselect::Pin9),
            10 => Some(Muxposselect::Pin10),
            11 => Some(Muxposselect::Pin11),
            12 => Some(Muxposselect::Pin12),
            13 => Some(Muxposselect::Pin13),
            14 => Some(Muxposselect::Pin14),
            15 => Some(Muxposselect::Pin15),
            16 => Some(Muxposselect::Pin16),
            17 => Some(Muxposselect::Pin17),
            18 => Some(Muxposselect::Pin18),
            19 => Some(Muxposselect::Pin19),
            24 => Some(Muxposselect::Temp),
            25 => Some(Muxposselect::Bandgap),
            26 => Some(Muxposselect::Scaledcorevcc),
            27 => Some(Muxposselect::Scalediovcc),
            28 => Some(Muxposselect::Dac),
            _ => None,
        }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Muxposselect::Pin0
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Muxposselect::Pin1
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Muxposselect::Pin2
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Muxposselect::Pin3
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == Muxposselect::Pin4
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == Muxposselect::Pin5
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == Muxposselect::Pin6
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == Muxposselect::Pin7
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Muxposselect::Pin8
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Muxposselect::Pin9
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Muxposselect::Pin10
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Muxposselect::Pin11
    }
    #[doc = "ADC AIN12 Pin"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == Muxposselect::Pin12
    }
    #[doc = "ADC AIN13 Pin"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == Muxposselect::Pin13
    }
    #[doc = "ADC AIN14 Pin"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == Muxposselect::Pin14
    }
    #[doc = "ADC AIN15 Pin"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == Muxposselect::Pin15
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline(always)]
    pub fn is_pin16(&self) -> bool {
        *self == Muxposselect::Pin16
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline(always)]
    pub fn is_pin17(&self) -> bool {
        *self == Muxposselect::Pin17
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline(always)]
    pub fn is_pin18(&self) -> bool {
        *self == Muxposselect::Pin18
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline(always)]
    pub fn is_pin19(&self) -> bool {
        *self == Muxposselect::Pin19
    }
    #[doc = "Temperature Reference"]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == Muxposselect::Temp
    }
    #[doc = "Bandgap Voltage"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == Muxposselect::Bandgap
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn is_scaledcorevcc(&self) -> bool {
        *self == Muxposselect::Scaledcorevcc
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline(always)]
    pub fn is_scalediovcc(&self) -> bool {
        *self == Muxposselect::Scalediovcc
    }
    #[doc = "DAC Output"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Muxposselect::Dac
    }
}
#[doc = "Field `MUXPOS` writer - Positive Mux Input Selection"]
pub type MuxposW<'a, REG> = crate::FieldWriter<'a, REG, 5, Muxposselect>;
impl<'a, REG> MuxposW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin7)
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin8)
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin9)
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin10)
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin11)
    }
    #[doc = "ADC AIN12 Pin"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin12)
    }
    #[doc = "ADC AIN13 Pin"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin13)
    }
    #[doc = "ADC AIN14 Pin"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin14)
    }
    #[doc = "ADC AIN15 Pin"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin15)
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline(always)]
    pub fn pin16(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin16)
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline(always)]
    pub fn pin17(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin17)
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline(always)]
    pub fn pin18(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin18)
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline(always)]
    pub fn pin19(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Pin19)
    }
    #[doc = "Temperature Reference"]
    #[inline(always)]
    pub fn temp(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Temp)
    }
    #[doc = "Bandgap Voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Bandgap)
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn scaledcorevcc(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Scaledcorevcc)
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline(always)]
    pub fn scalediovcc(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Scalediovcc)
    }
    #[doc = "DAC Output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Dac)
    }
}
#[doc = "Negative Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Muxnegselect {
    #[doc = "0: ADC AIN0 Pin"]
    Pin0 = 0,
    #[doc = "1: ADC AIN1 Pin"]
    Pin1 = 1,
    #[doc = "2: ADC AIN2 Pin"]
    Pin2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    Pin3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    Pin4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    Pin5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    Pin6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    Pin7 = 7,
    #[doc = "24: Internal Ground"]
    Gnd = 24,
    #[doc = "25: I/O Ground"]
    Iognd = 25,
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
#[doc = "Field `MUXNEG` reader - Negative Mux Input Selection"]
pub type MuxnegR = crate::FieldReader<Muxnegselect>;
impl MuxnegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Muxnegselect> {
        match self.bits {
            0 => Some(Muxnegselect::Pin0),
            1 => Some(Muxnegselect::Pin1),
            2 => Some(Muxnegselect::Pin2),
            3 => Some(Muxnegselect::Pin3),
            4 => Some(Muxnegselect::Pin4),
            5 => Some(Muxnegselect::Pin5),
            6 => Some(Muxnegselect::Pin6),
            7 => Some(Muxnegselect::Pin7),
            24 => Some(Muxnegselect::Gnd),
            25 => Some(Muxnegselect::Iognd),
            _ => None,
        }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Muxnegselect::Pin0
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Muxnegselect::Pin1
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Muxnegselect::Pin2
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Muxnegselect::Pin3
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == Muxnegselect::Pin4
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == Muxnegselect::Pin5
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == Muxnegselect::Pin6
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == Muxnegselect::Pin7
    }
    #[doc = "Internal Ground"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == Muxnegselect::Gnd
    }
    #[doc = "I/O Ground"]
    #[inline(always)]
    pub fn is_iognd(&self) -> bool {
        *self == Muxnegselect::Iognd
    }
}
#[doc = "Field `MUXNEG` writer - Negative Mux Input Selection"]
pub type MuxnegW<'a, REG> = crate::FieldWriter<'a, REG, 5, Muxnegselect>;
impl<'a, REG> MuxnegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Pin0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Pin1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Pin2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Pin3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Pin4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Pin5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Pin6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Pin7)
    }
    #[doc = "Internal Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Gnd)
    }
    #[doc = "I/O Ground"]
    #[inline(always)]
    pub fn iognd(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Iognd)
    }
}
#[doc = "Field `INPUTSCAN` reader - Number of Input Channels Included in Scan"]
pub type InputscanR = crate::FieldReader;
#[doc = "Field `INPUTSCAN` writer - Number of Input Channels Included in Scan"]
pub type InputscanW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INPUTOFFSET` reader - Positive Mux Setting Offset"]
pub type InputoffsetR = crate::FieldReader;
#[doc = "Field `INPUTOFFSET` writer - Positive Mux Setting Offset"]
pub type InputoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Gain Factor Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gainselect {
    #[doc = "0: 1x"]
    _1x = 0,
    #[doc = "1: 2x"]
    _2x = 1,
    #[doc = "2: 4x"]
    _4x = 2,
    #[doc = "3: 8x"]
    _8x = 3,
    #[doc = "4: 16x"]
    _16x = 4,
    #[doc = "15: 1/2x"]
    Div2 = 15,
}
impl From<Gainselect> for u8 {
    #[inline(always)]
    fn from(variant: Gainselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gainselect {
    type Ux = u8;
}
impl crate::IsEnum for Gainselect {}
#[doc = "Field `GAIN` reader - Gain Factor Selection"]
pub type GainR = crate::FieldReader<Gainselect>;
impl GainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gainselect> {
        match self.bits {
            0 => Some(Gainselect::_1x),
            1 => Some(Gainselect::_2x),
            2 => Some(Gainselect::_4x),
            3 => Some(Gainselect::_8x),
            4 => Some(Gainselect::_16x),
            15 => Some(Gainselect::Div2),
            _ => None,
        }
    }
    #[doc = "1x"]
    #[inline(always)]
    pub fn is_1x(&self) -> bool {
        *self == Gainselect::_1x
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn is_2x(&self) -> bool {
        *self == Gainselect::_2x
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn is_4x(&self) -> bool {
        *self == Gainselect::_4x
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn is_8x(&self) -> bool {
        *self == Gainselect::_8x
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn is_16x(&self) -> bool {
        *self == Gainselect::_16x
    }
    #[doc = "1/2x"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Gainselect::Div2
    }
}
#[doc = "Field `GAIN` writer - Gain Factor Selection"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gainselect>;
impl<'a, REG> GainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1x"]
    #[inline(always)]
    pub fn _1x(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::_1x)
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn _2x(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::_2x)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn _4x(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::_4x)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn _8x(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::_8x)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn _16x(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::_16x)
    }
    #[doc = "1/2x"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::Div2)
    }
}
impl R {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MuxposR {
        MuxposR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MuxnegR {
        MuxnegR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Number of Input Channels Included in Scan"]
    #[inline(always)]
    pub fn inputscan(&self) -> InputscanR {
        InputscanR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Positive Mux Setting Offset"]
    #[inline(always)]
    pub fn inputoffset(&self) -> InputoffsetR {
        InputoffsetR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Gain Factor Selection"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MuxposW<InputctrlSpec> {
        MuxposW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MuxnegW<InputctrlSpec> {
        MuxnegW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Number of Input Channels Included in Scan"]
    #[inline(always)]
    pub fn inputscan(&mut self) -> InputscanW<InputctrlSpec> {
        InputscanW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Positive Mux Setting Offset"]
    #[inline(always)]
    pub fn inputoffset(&mut self) -> InputoffsetW<InputctrlSpec> {
        InputoffsetW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Gain Factor Selection"]
    #[inline(always)]
    pub fn gain(&mut self) -> GainW<InputctrlSpec> {
        GainW::new(self, 24)
    }
}
#[doc = "Input Control\n\nYou can [`read`](crate::Reg::read) this register and get [`inputctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputctrlSpec;
impl crate::RegisterSpec for InputctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputctrl::R`](R) reader structure"]
impl crate::Readable for InputctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`inputctrl::W`](W) writer structure"]
impl crate::Writable for InputctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTCTRL to value 0"]
impl crate::Resettable for InputctrlSpec {}
