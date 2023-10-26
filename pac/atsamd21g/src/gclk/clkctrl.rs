#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<CLKCTRL_SPEC>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<CLKCTRL_SPEC>;
#[doc = "Field `ID` reader - Generic Clock Selection ID"]
pub type ID_R = crate::FieldReader<IDSELECT_A>;
#[doc = "Generic Clock Selection ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDSELECT_A {
    #[doc = "0: DFLL48"]
    DFLL48 = 0,
    #[doc = "1: FDPLL"]
    FDPLL = 1,
    #[doc = "2: FDPLL32K"]
    FDPLL32K = 2,
    #[doc = "3: WDT"]
    WDT = 3,
    #[doc = "4: RTC"]
    RTC = 4,
    #[doc = "5: EIC"]
    EIC = 5,
    #[doc = "6: USB"]
    USB = 6,
    #[doc = "7: EVSYS_0"]
    EVSYS_0 = 7,
    #[doc = "8: EVSYS_1"]
    EVSYS_1 = 8,
    #[doc = "9: EVSYS_2"]
    EVSYS_2 = 9,
    #[doc = "10: EVSYS_3"]
    EVSYS_3 = 10,
    #[doc = "11: EVSYS_4"]
    EVSYS_4 = 11,
    #[doc = "12: EVSYS_5"]
    EVSYS_5 = 12,
    #[doc = "13: EVSYS_6"]
    EVSYS_6 = 13,
    #[doc = "14: EVSYS_7"]
    EVSYS_7 = 14,
    #[doc = "15: EVSYS_8"]
    EVSYS_8 = 15,
    #[doc = "16: EVSYS_9"]
    EVSYS_9 = 16,
    #[doc = "17: EVSYS_10"]
    EVSYS_10 = 17,
    #[doc = "18: EVSYS_11"]
    EVSYS_11 = 18,
    #[doc = "19: SERCOMX_SLOW"]
    SERCOMX_SLOW = 19,
    #[doc = "20: SERCOM0_CORE"]
    SERCOM0_CORE = 20,
    #[doc = "21: SERCOM1_CORE"]
    SERCOM1_CORE = 21,
    #[doc = "22: SERCOM2_CORE"]
    SERCOM2_CORE = 22,
    #[doc = "23: SERCOM3_CORE"]
    SERCOM3_CORE = 23,
    #[doc = "24: SERCOM4_CORE"]
    SERCOM4_CORE = 24,
    #[doc = "25: SERCOM5_CORE"]
    SERCOM5_CORE = 25,
    #[doc = "26: TCC0_TCC1"]
    TCC0_TCC1 = 26,
    #[doc = "27: TCC2_TC3"]
    TCC2_TC3 = 27,
    #[doc = "28: TC4_TC5"]
    TC4_TC5 = 28,
    #[doc = "29: TC6_TC7"]
    TC6_TC7 = 29,
    #[doc = "30: ADC"]
    ADC = 30,
    #[doc = "31: AC_DIG"]
    AC_DIG = 31,
    #[doc = "32: AC_ANA"]
    AC_ANA = 32,
    #[doc = "33: DAC"]
    DAC = 33,
    #[doc = "35: I2S_0"]
    I2S_0 = 35,
    #[doc = "36: I2S_1"]
    I2S_1 = 36,
}
impl From<IDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDSELECT_A {
    type Ux = u8;
}
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IDSELECT_A> {
        match self.bits {
            0 => Some(IDSELECT_A::DFLL48),
            1 => Some(IDSELECT_A::FDPLL),
            2 => Some(IDSELECT_A::FDPLL32K),
            3 => Some(IDSELECT_A::WDT),
            4 => Some(IDSELECT_A::RTC),
            5 => Some(IDSELECT_A::EIC),
            6 => Some(IDSELECT_A::USB),
            7 => Some(IDSELECT_A::EVSYS_0),
            8 => Some(IDSELECT_A::EVSYS_1),
            9 => Some(IDSELECT_A::EVSYS_2),
            10 => Some(IDSELECT_A::EVSYS_3),
            11 => Some(IDSELECT_A::EVSYS_4),
            12 => Some(IDSELECT_A::EVSYS_5),
            13 => Some(IDSELECT_A::EVSYS_6),
            14 => Some(IDSELECT_A::EVSYS_7),
            15 => Some(IDSELECT_A::EVSYS_8),
            16 => Some(IDSELECT_A::EVSYS_9),
            17 => Some(IDSELECT_A::EVSYS_10),
            18 => Some(IDSELECT_A::EVSYS_11),
            19 => Some(IDSELECT_A::SERCOMX_SLOW),
            20 => Some(IDSELECT_A::SERCOM0_CORE),
            21 => Some(IDSELECT_A::SERCOM1_CORE),
            22 => Some(IDSELECT_A::SERCOM2_CORE),
            23 => Some(IDSELECT_A::SERCOM3_CORE),
            24 => Some(IDSELECT_A::SERCOM4_CORE),
            25 => Some(IDSELECT_A::SERCOM5_CORE),
            26 => Some(IDSELECT_A::TCC0_TCC1),
            27 => Some(IDSELECT_A::TCC2_TC3),
            28 => Some(IDSELECT_A::TC4_TC5),
            29 => Some(IDSELECT_A::TC6_TC7),
            30 => Some(IDSELECT_A::ADC),
            31 => Some(IDSELECT_A::AC_DIG),
            32 => Some(IDSELECT_A::AC_ANA),
            33 => Some(IDSELECT_A::DAC),
            35 => Some(IDSELECT_A::I2S_0),
            36 => Some(IDSELECT_A::I2S_1),
            _ => None,
        }
    }
    #[doc = "DFLL48"]
    #[inline(always)]
    pub fn is_dfll48(&self) -> bool {
        *self == IDSELECT_A::DFLL48
    }
    #[doc = "FDPLL"]
    #[inline(always)]
    pub fn is_fdpll(&self) -> bool {
        *self == IDSELECT_A::FDPLL
    }
    #[doc = "FDPLL32K"]
    #[inline(always)]
    pub fn is_fdpll32k(&self) -> bool {
        *self == IDSELECT_A::FDPLL32K
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub fn is_wdt(&self) -> bool {
        *self == IDSELECT_A::WDT
    }
    #[doc = "RTC"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == IDSELECT_A::RTC
    }
    #[doc = "EIC"]
    #[inline(always)]
    pub fn is_eic(&self) -> bool {
        *self == IDSELECT_A::EIC
    }
    #[doc = "USB"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == IDSELECT_A::USB
    }
    #[doc = "EVSYS_0"]
    #[inline(always)]
    pub fn is_evsys_0(&self) -> bool {
        *self == IDSELECT_A::EVSYS_0
    }
    #[doc = "EVSYS_1"]
    #[inline(always)]
    pub fn is_evsys_1(&self) -> bool {
        *self == IDSELECT_A::EVSYS_1
    }
    #[doc = "EVSYS_2"]
    #[inline(always)]
    pub fn is_evsys_2(&self) -> bool {
        *self == IDSELECT_A::EVSYS_2
    }
    #[doc = "EVSYS_3"]
    #[inline(always)]
    pub fn is_evsys_3(&self) -> bool {
        *self == IDSELECT_A::EVSYS_3
    }
    #[doc = "EVSYS_4"]
    #[inline(always)]
    pub fn is_evsys_4(&self) -> bool {
        *self == IDSELECT_A::EVSYS_4
    }
    #[doc = "EVSYS_5"]
    #[inline(always)]
    pub fn is_evsys_5(&self) -> bool {
        *self == IDSELECT_A::EVSYS_5
    }
    #[doc = "EVSYS_6"]
    #[inline(always)]
    pub fn is_evsys_6(&self) -> bool {
        *self == IDSELECT_A::EVSYS_6
    }
    #[doc = "EVSYS_7"]
    #[inline(always)]
    pub fn is_evsys_7(&self) -> bool {
        *self == IDSELECT_A::EVSYS_7
    }
    #[doc = "EVSYS_8"]
    #[inline(always)]
    pub fn is_evsys_8(&self) -> bool {
        *self == IDSELECT_A::EVSYS_8
    }
    #[doc = "EVSYS_9"]
    #[inline(always)]
    pub fn is_evsys_9(&self) -> bool {
        *self == IDSELECT_A::EVSYS_9
    }
    #[doc = "EVSYS_10"]
    #[inline(always)]
    pub fn is_evsys_10(&self) -> bool {
        *self == IDSELECT_A::EVSYS_10
    }
    #[doc = "EVSYS_11"]
    #[inline(always)]
    pub fn is_evsys_11(&self) -> bool {
        *self == IDSELECT_A::EVSYS_11
    }
    #[doc = "SERCOMX_SLOW"]
    #[inline(always)]
    pub fn is_sercomx_slow(&self) -> bool {
        *self == IDSELECT_A::SERCOMX_SLOW
    }
    #[doc = "SERCOM0_CORE"]
    #[inline(always)]
    pub fn is_sercom0_core(&self) -> bool {
        *self == IDSELECT_A::SERCOM0_CORE
    }
    #[doc = "SERCOM1_CORE"]
    #[inline(always)]
    pub fn is_sercom1_core(&self) -> bool {
        *self == IDSELECT_A::SERCOM1_CORE
    }
    #[doc = "SERCOM2_CORE"]
    #[inline(always)]
    pub fn is_sercom2_core(&self) -> bool {
        *self == IDSELECT_A::SERCOM2_CORE
    }
    #[doc = "SERCOM3_CORE"]
    #[inline(always)]
    pub fn is_sercom3_core(&self) -> bool {
        *self == IDSELECT_A::SERCOM3_CORE
    }
    #[doc = "SERCOM4_CORE"]
    #[inline(always)]
    pub fn is_sercom4_core(&self) -> bool {
        *self == IDSELECT_A::SERCOM4_CORE
    }
    #[doc = "SERCOM5_CORE"]
    #[inline(always)]
    pub fn is_sercom5_core(&self) -> bool {
        *self == IDSELECT_A::SERCOM5_CORE
    }
    #[doc = "TCC0_TCC1"]
    #[inline(always)]
    pub fn is_tcc0_tcc1(&self) -> bool {
        *self == IDSELECT_A::TCC0_TCC1
    }
    #[doc = "TCC2_TC3"]
    #[inline(always)]
    pub fn is_tcc2_tc3(&self) -> bool {
        *self == IDSELECT_A::TCC2_TC3
    }
    #[doc = "TC4_TC5"]
    #[inline(always)]
    pub fn is_tc4_tc5(&self) -> bool {
        *self == IDSELECT_A::TC4_TC5
    }
    #[doc = "TC6_TC7"]
    #[inline(always)]
    pub fn is_tc6_tc7(&self) -> bool {
        *self == IDSELECT_A::TC6_TC7
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == IDSELECT_A::ADC
    }
    #[doc = "AC_DIG"]
    #[inline(always)]
    pub fn is_ac_dig(&self) -> bool {
        *self == IDSELECT_A::AC_DIG
    }
    #[doc = "AC_ANA"]
    #[inline(always)]
    pub fn is_ac_ana(&self) -> bool {
        *self == IDSELECT_A::AC_ANA
    }
    #[doc = "DAC"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == IDSELECT_A::DAC
    }
    #[doc = "I2S_0"]
    #[inline(always)]
    pub fn is_i2s_0(&self) -> bool {
        *self == IDSELECT_A::I2S_0
    }
    #[doc = "I2S_1"]
    #[inline(always)]
    pub fn is_i2s_1(&self) -> bool {
        *self == IDSELECT_A::I2S_1
    }
}
#[doc = "Field `ID` writer - Generic Clock Selection ID"]
pub type ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, IDSELECT_A>;
impl<'a, REG, const O: u8> ID_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DFLL48"]
    #[inline(always)]
    pub fn dfll48(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::DFLL48)
    }
    #[doc = "FDPLL"]
    #[inline(always)]
    pub fn fdpll(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::FDPLL)
    }
    #[doc = "FDPLL32K"]
    #[inline(always)]
    pub fn fdpll32k(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::FDPLL32K)
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub fn wdt(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::WDT)
    }
    #[doc = "RTC"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::RTC)
    }
    #[doc = "EIC"]
    #[inline(always)]
    pub fn eic(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EIC)
    }
    #[doc = "USB"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::USB)
    }
    #[doc = "EVSYS_0"]
    #[inline(always)]
    pub fn evsys_0(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_0)
    }
    #[doc = "EVSYS_1"]
    #[inline(always)]
    pub fn evsys_1(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_1)
    }
    #[doc = "EVSYS_2"]
    #[inline(always)]
    pub fn evsys_2(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_2)
    }
    #[doc = "EVSYS_3"]
    #[inline(always)]
    pub fn evsys_3(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_3)
    }
    #[doc = "EVSYS_4"]
    #[inline(always)]
    pub fn evsys_4(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_4)
    }
    #[doc = "EVSYS_5"]
    #[inline(always)]
    pub fn evsys_5(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_5)
    }
    #[doc = "EVSYS_6"]
    #[inline(always)]
    pub fn evsys_6(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_6)
    }
    #[doc = "EVSYS_7"]
    #[inline(always)]
    pub fn evsys_7(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_7)
    }
    #[doc = "EVSYS_8"]
    #[inline(always)]
    pub fn evsys_8(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_8)
    }
    #[doc = "EVSYS_9"]
    #[inline(always)]
    pub fn evsys_9(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_9)
    }
    #[doc = "EVSYS_10"]
    #[inline(always)]
    pub fn evsys_10(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_10)
    }
    #[doc = "EVSYS_11"]
    #[inline(always)]
    pub fn evsys_11(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::EVSYS_11)
    }
    #[doc = "SERCOMX_SLOW"]
    #[inline(always)]
    pub fn sercomx_slow(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOMX_SLOW)
    }
    #[doc = "SERCOM0_CORE"]
    #[inline(always)]
    pub fn sercom0_core(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOM0_CORE)
    }
    #[doc = "SERCOM1_CORE"]
    #[inline(always)]
    pub fn sercom1_core(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOM1_CORE)
    }
    #[doc = "SERCOM2_CORE"]
    #[inline(always)]
    pub fn sercom2_core(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOM2_CORE)
    }
    #[doc = "SERCOM3_CORE"]
    #[inline(always)]
    pub fn sercom3_core(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOM3_CORE)
    }
    #[doc = "SERCOM4_CORE"]
    #[inline(always)]
    pub fn sercom4_core(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOM4_CORE)
    }
    #[doc = "SERCOM5_CORE"]
    #[inline(always)]
    pub fn sercom5_core(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::SERCOM5_CORE)
    }
    #[doc = "TCC0_TCC1"]
    #[inline(always)]
    pub fn tcc0_tcc1(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::TCC0_TCC1)
    }
    #[doc = "TCC2_TC3"]
    #[inline(always)]
    pub fn tcc2_tc3(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::TCC2_TC3)
    }
    #[doc = "TC4_TC5"]
    #[inline(always)]
    pub fn tc4_tc5(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::TC4_TC5)
    }
    #[doc = "TC6_TC7"]
    #[inline(always)]
    pub fn tc6_tc7(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::TC6_TC7)
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::ADC)
    }
    #[doc = "AC_DIG"]
    #[inline(always)]
    pub fn ac_dig(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::AC_DIG)
    }
    #[doc = "AC_ANA"]
    #[inline(always)]
    pub fn ac_ana(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::AC_ANA)
    }
    #[doc = "DAC"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::DAC)
    }
    #[doc = "I2S_0"]
    #[inline(always)]
    pub fn i2s_0(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::I2S_0)
    }
    #[doc = "I2S_1"]
    #[inline(always)]
    pub fn i2s_1(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::I2S_1)
    }
}
#[doc = "Field `GEN` reader - Generic Clock Generator"]
pub type GEN_R = crate::FieldReader<GENSELECT_A>;
#[doc = "Generic Clock Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GENSELECT_A {
    #[doc = "0: Generic clock generator 0"]
    GCLK0 = 0,
    #[doc = "1: Generic clock generator 1"]
    GCLK1 = 1,
    #[doc = "2: Generic clock generator 2"]
    GCLK2 = 2,
    #[doc = "3: Generic clock generator 3"]
    GCLK3 = 3,
    #[doc = "4: Generic clock generator 4"]
    GCLK4 = 4,
    #[doc = "5: Generic clock generator 5"]
    GCLK5 = 5,
    #[doc = "6: Generic clock generator 6"]
    GCLK6 = 6,
    #[doc = "7: Generic clock generator 7"]
    GCLK7 = 7,
    #[doc = "8: Generic clock generator 8"]
    GCLK8 = 8,
}
impl From<GENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GENSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GENSELECT_A {
    type Ux = u8;
}
impl GEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GENSELECT_A> {
        match self.bits {
            0 => Some(GENSELECT_A::GCLK0),
            1 => Some(GENSELECT_A::GCLK1),
            2 => Some(GENSELECT_A::GCLK2),
            3 => Some(GENSELECT_A::GCLK3),
            4 => Some(GENSELECT_A::GCLK4),
            5 => Some(GENSELECT_A::GCLK5),
            6 => Some(GENSELECT_A::GCLK6),
            7 => Some(GENSELECT_A::GCLK7),
            8 => Some(GENSELECT_A::GCLK8),
            _ => None,
        }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENSELECT_A::GCLK0
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENSELECT_A::GCLK1
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENSELECT_A::GCLK2
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENSELECT_A::GCLK3
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENSELECT_A::GCLK4
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENSELECT_A::GCLK5
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENSELECT_A::GCLK6
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENSELECT_A::GCLK7
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENSELECT_A::GCLK8
    }
}
#[doc = "Field `GEN` writer - Generic Clock Generator"]
pub type GEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, GENSELECT_A>;
impl<'a, REG, const O: u8> GEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn gclk6(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn gclk7(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK7)
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn gclk8(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK8)
    }
}
#[doc = "Field `CLKEN` reader - Clock Enable"]
pub type CLKEN_R = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock Enable"]
pub type CLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WRTLOCK_R = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WRTLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<CLKCTRL_SPEC, 0> {
        ID_W::new(self)
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<CLKCTRL_SPEC, 8> {
        GEN_W::new(self)
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CLKCTRL_SPEC, 14> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WRTLOCK_W<CLKCTRL_SPEC, 15> {
        WRTLOCK_W::new(self)
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
#[doc = "Generic Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
