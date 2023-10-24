#[doc = "Register `INPUTCTRL` reader"]
pub type R = crate::R<INPUTCTRL_SPEC>;
#[doc = "Register `INPUTCTRL` writer"]
pub type W = crate::W<INPUTCTRL_SPEC>;
#[doc = "Field `MUXPOS` reader - Positive Mux Input Selection"]
pub type MUXPOS_R = crate::FieldReader<MUXPOSSELECT_A>;
#[doc = "Positive Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXPOSSELECT_A {
    #[doc = "0: ADC AIN0 Pin"]
    AIN0 = 0,
    #[doc = "1: ADC AIN1 Pin"]
    AIN1 = 1,
    #[doc = "2: ADC AIN2 Pin"]
    AIN2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    AIN3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    AIN4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    AIN5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    AIN6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    AIN7 = 7,
    #[doc = "8: ADC AIN8 Pin"]
    AIN8 = 8,
    #[doc = "9: ADC AIN9 Pin"]
    AIN9 = 9,
    #[doc = "10: ADC AIN10 Pin"]
    AIN10 = 10,
    #[doc = "11: ADC AIN11 Pin"]
    AIN11 = 11,
    #[doc = "12: ADC AIN12 Pin"]
    AIN12 = 12,
    #[doc = "13: ADC AIN13 Pin"]
    AIN13 = 13,
    #[doc = "14: ADC AIN14 Pin"]
    AIN14 = 14,
    #[doc = "15: ADC AIN15 Pin"]
    AIN15 = 15,
    #[doc = "16: ADC AIN16 Pin"]
    AIN16 = 16,
    #[doc = "17: ADC AIN17 Pin"]
    AIN17 = 17,
    #[doc = "18: ADC AIN18 Pin"]
    AIN18 = 18,
    #[doc = "19: ADC AIN19 Pin"]
    AIN19 = 19,
    #[doc = "20: ADC AIN20 Pin"]
    AIN20 = 20,
    #[doc = "21: ADC AIN21 Pin"]
    AIN21 = 21,
    #[doc = "22: ADC AIN22 Pin"]
    AIN22 = 22,
    #[doc = "23: ADC AIN23 Pin"]
    AIN23 = 23,
    #[doc = "24: 1/4 Scaled Core Supply"]
    SCALEDCOREVCC = 24,
    #[doc = "25: 1/4 Scaled VBAT Supply"]
    SCALEDVBAT = 25,
    #[doc = "26: 1/4 Scaled I/O Supply"]
    SCALEDIOVCC = 26,
    #[doc = "27: Bandgap Voltage"]
    BANDGAP = 27,
    #[doc = "28: Temperature Sensor TSENSP"]
    PTAT = 28,
    #[doc = "29: Temperature Sensor TSENSC"]
    CTAT = 29,
    #[doc = "30: DAC Output"]
    DAC = 30,
    #[doc = "31: PTC output (only on ADC0)"]
    PTC = 31,
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
    pub const fn variant(&self) -> MUXPOSSELECT_A {
        match self.bits {
            0 => MUXPOSSELECT_A::AIN0,
            1 => MUXPOSSELECT_A::AIN1,
            2 => MUXPOSSELECT_A::AIN2,
            3 => MUXPOSSELECT_A::AIN3,
            4 => MUXPOSSELECT_A::AIN4,
            5 => MUXPOSSELECT_A::AIN5,
            6 => MUXPOSSELECT_A::AIN6,
            7 => MUXPOSSELECT_A::AIN7,
            8 => MUXPOSSELECT_A::AIN8,
            9 => MUXPOSSELECT_A::AIN9,
            10 => MUXPOSSELECT_A::AIN10,
            11 => MUXPOSSELECT_A::AIN11,
            12 => MUXPOSSELECT_A::AIN12,
            13 => MUXPOSSELECT_A::AIN13,
            14 => MUXPOSSELECT_A::AIN14,
            15 => MUXPOSSELECT_A::AIN15,
            16 => MUXPOSSELECT_A::AIN16,
            17 => MUXPOSSELECT_A::AIN17,
            18 => MUXPOSSELECT_A::AIN18,
            19 => MUXPOSSELECT_A::AIN19,
            20 => MUXPOSSELECT_A::AIN20,
            21 => MUXPOSSELECT_A::AIN21,
            22 => MUXPOSSELECT_A::AIN22,
            23 => MUXPOSSELECT_A::AIN23,
            24 => MUXPOSSELECT_A::SCALEDCOREVCC,
            25 => MUXPOSSELECT_A::SCALEDVBAT,
            26 => MUXPOSSELECT_A::SCALEDIOVCC,
            27 => MUXPOSSELECT_A::BANDGAP,
            28 => MUXPOSSELECT_A::PTAT,
            29 => MUXPOSSELECT_A::CTAT,
            30 => MUXPOSSELECT_A::DAC,
            31 => MUXPOSSELECT_A::PTC,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN0
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN1
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN2
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN3
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN4
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN5
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN6
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN7
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline(always)]
    pub fn is_ain8(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN8
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline(always)]
    pub fn is_ain9(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN9
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline(always)]
    pub fn is_ain10(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN10
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline(always)]
    pub fn is_ain11(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN11
    }
    #[doc = "ADC AIN12 Pin"]
    #[inline(always)]
    pub fn is_ain12(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN12
    }
    #[doc = "ADC AIN13 Pin"]
    #[inline(always)]
    pub fn is_ain13(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN13
    }
    #[doc = "ADC AIN14 Pin"]
    #[inline(always)]
    pub fn is_ain14(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN14
    }
    #[doc = "ADC AIN15 Pin"]
    #[inline(always)]
    pub fn is_ain15(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN15
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline(always)]
    pub fn is_ain16(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN16
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline(always)]
    pub fn is_ain17(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN17
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline(always)]
    pub fn is_ain18(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN18
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline(always)]
    pub fn is_ain19(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN19
    }
    #[doc = "ADC AIN20 Pin"]
    #[inline(always)]
    pub fn is_ain20(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN20
    }
    #[doc = "ADC AIN21 Pin"]
    #[inline(always)]
    pub fn is_ain21(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN21
    }
    #[doc = "ADC AIN22 Pin"]
    #[inline(always)]
    pub fn is_ain22(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN22
    }
    #[doc = "ADC AIN23 Pin"]
    #[inline(always)]
    pub fn is_ain23(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN23
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn is_scaledcorevcc(&self) -> bool {
        *self == MUXPOSSELECT_A::SCALEDCOREVCC
    }
    #[doc = "1/4 Scaled VBAT Supply"]
    #[inline(always)]
    pub fn is_scaledvbat(&self) -> bool {
        *self == MUXPOSSELECT_A::SCALEDVBAT
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline(always)]
    pub fn is_scalediovcc(&self) -> bool {
        *self == MUXPOSSELECT_A::SCALEDIOVCC
    }
    #[doc = "Bandgap Voltage"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXPOSSELECT_A::BANDGAP
    }
    #[doc = "Temperature Sensor TSENSP"]
    #[inline(always)]
    pub fn is_ptat(&self) -> bool {
        *self == MUXPOSSELECT_A::PTAT
    }
    #[doc = "Temperature Sensor TSENSC"]
    #[inline(always)]
    pub fn is_ctat(&self) -> bool {
        *self == MUXPOSSELECT_A::CTAT
    }
    #[doc = "DAC Output"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == MUXPOSSELECT_A::DAC
    }
    #[doc = "PTC output (only on ADC0)"]
    #[inline(always)]
    pub fn is_ptc(&self) -> bool {
        *self == MUXPOSSELECT_A::PTC
    }
}
#[doc = "Field `MUXPOS` writer - Positive Mux Input Selection"]
pub type MUXPOS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O, MUXPOSSELECT_A>;
impl<'a, REG, const O: u8> MUXPOS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN7)
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline(always)]
    pub fn ain8(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN8)
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline(always)]
    pub fn ain9(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN9)
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline(always)]
    pub fn ain10(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN10)
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline(always)]
    pub fn ain11(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN11)
    }
    #[doc = "ADC AIN12 Pin"]
    #[inline(always)]
    pub fn ain12(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN12)
    }
    #[doc = "ADC AIN13 Pin"]
    #[inline(always)]
    pub fn ain13(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN13)
    }
    #[doc = "ADC AIN14 Pin"]
    #[inline(always)]
    pub fn ain14(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN14)
    }
    #[doc = "ADC AIN15 Pin"]
    #[inline(always)]
    pub fn ain15(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN15)
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline(always)]
    pub fn ain16(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN16)
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline(always)]
    pub fn ain17(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN17)
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline(always)]
    pub fn ain18(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN18)
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline(always)]
    pub fn ain19(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN19)
    }
    #[doc = "ADC AIN20 Pin"]
    #[inline(always)]
    pub fn ain20(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN20)
    }
    #[doc = "ADC AIN21 Pin"]
    #[inline(always)]
    pub fn ain21(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN21)
    }
    #[doc = "ADC AIN22 Pin"]
    #[inline(always)]
    pub fn ain22(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN22)
    }
    #[doc = "ADC AIN23 Pin"]
    #[inline(always)]
    pub fn ain23(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::AIN23)
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn scaledcorevcc(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::SCALEDCOREVCC)
    }
    #[doc = "1/4 Scaled VBAT Supply"]
    #[inline(always)]
    pub fn scaledvbat(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::SCALEDVBAT)
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline(always)]
    pub fn scalediovcc(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::SCALEDIOVCC)
    }
    #[doc = "Bandgap Voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::BANDGAP)
    }
    #[doc = "Temperature Sensor TSENSP"]
    #[inline(always)]
    pub fn ptat(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::PTAT)
    }
    #[doc = "Temperature Sensor TSENSC"]
    #[inline(always)]
    pub fn ctat(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::CTAT)
    }
    #[doc = "DAC Output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::DAC)
    }
    #[doc = "PTC output (only on ADC0)"]
    #[inline(always)]
    pub fn ptc(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOSSELECT_A::PTC)
    }
}
#[doc = "Field `DIFFMODE` reader - Differential Mode"]
pub type DIFFMODE_R = crate::BitReader;
#[doc = "Field `DIFFMODE` writer - Differential Mode"]
pub type DIFFMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MUXNEG` reader - Negative Mux Input Selection"]
pub type MUXNEG_R = crate::FieldReader<MUXNEGSELECT_A>;
#[doc = "Negative Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXNEGSELECT_A {
    #[doc = "0: ADC AIN0 Pin"]
    AIN0 = 0,
    #[doc = "1: ADC AIN1 Pin"]
    AIN1 = 1,
    #[doc = "2: ADC AIN2 Pin"]
    AIN2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    AIN3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    AIN4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    AIN5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    AIN6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    AIN7 = 7,
    #[doc = "24: Internal Ground"]
    GND = 24,
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
    pub const fn variant(&self) -> Option<MUXNEGSELECT_A> {
        match self.bits {
            0 => Some(MUXNEGSELECT_A::AIN0),
            1 => Some(MUXNEGSELECT_A::AIN1),
            2 => Some(MUXNEGSELECT_A::AIN2),
            3 => Some(MUXNEGSELECT_A::AIN3),
            4 => Some(MUXNEGSELECT_A::AIN4),
            5 => Some(MUXNEGSELECT_A::AIN5),
            6 => Some(MUXNEGSELECT_A::AIN6),
            7 => Some(MUXNEGSELECT_A::AIN7),
            24 => Some(MUXNEGSELECT_A::GND),
            _ => None,
        }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN0
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN1
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN2
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN3
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN4
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN5
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN6
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN7
    }
    #[doc = "Internal Ground"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == MUXNEGSELECT_A::GND
    }
}
#[doc = "Field `MUXNEG` writer - Negative Mux Input Selection"]
pub type MUXNEG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, MUXNEGSELECT_A>;
impl<'a, REG, const O: u8> MUXNEG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::AIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::AIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::AIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::AIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::AIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::AIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::AIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::AIN7)
    }
    #[doc = "Internal Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEGSELECT_A::GND)
    }
}
#[doc = "Field `DSEQSTOP` reader - Stop DMA Sequencing"]
pub type DSEQSTOP_R = crate::BitReader;
#[doc = "Field `DSEQSTOP` writer - Stop DMA Sequencing"]
pub type DSEQSTOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Differential Mode"]
    #[inline(always)]
    pub fn diffmode(&self) -> DIFFMODE_R {
        DIFFMODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Stop DMA Sequencing"]
    #[inline(always)]
    pub fn dseqstop(&self) -> DSEQSTOP_R {
        DSEQSTOP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<INPUTCTRL_SPEC, 0> {
        MUXPOS_W::new(self)
    }
    #[doc = "Bit 7 - Differential Mode"]
    #[inline(always)]
    #[must_use]
    pub fn diffmode(&mut self) -> DIFFMODE_W<INPUTCTRL_SPEC, 7> {
        DIFFMODE_W::new(self)
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<INPUTCTRL_SPEC, 8> {
        MUXNEG_W::new(self)
    }
    #[doc = "Bit 15 - Stop DMA Sequencing"]
    #[inline(always)]
    #[must_use]
    pub fn dseqstop(&mut self) -> DSEQSTOP_W<INPUTCTRL_SPEC, 15> {
        DSEQSTOP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Input Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUTCTRL_SPEC;
impl crate::RegisterSpec for INPUTCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`inputctrl::R`](R) reader structure"]
impl crate::Readable for INPUTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inputctrl::W`](W) writer structure"]
impl crate::Writable for INPUTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTCTRL to value 0"]
impl crate::Resettable for INPUTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
