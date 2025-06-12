#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Generic Clock Selection ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idselect {
    #[doc = "0: DFLL48"]
    Dfll48 = 0,
    #[doc = "1: FDPLL"]
    Fdpll = 1,
    #[doc = "2: FDPLL32K"]
    Fdpll32k = 2,
    #[doc = "3: WDT"]
    Wdt = 3,
    #[doc = "4: RTC"]
    Rtc = 4,
    #[doc = "5: EIC"]
    Eic = 5,
    #[doc = "6: USB"]
    Usb = 6,
    #[doc = "7: EVSYS_0"]
    Evsys0 = 7,
    #[doc = "8: EVSYS_1"]
    Evsys1 = 8,
    #[doc = "9: EVSYS_2"]
    Evsys2 = 9,
    #[doc = "10: EVSYS_3"]
    Evsys3 = 10,
    #[doc = "11: EVSYS_4"]
    Evsys4 = 11,
    #[doc = "12: EVSYS_5"]
    Evsys5 = 12,
    #[doc = "13: EVSYS_6"]
    Evsys6 = 13,
    #[doc = "14: EVSYS_7"]
    Evsys7 = 14,
    #[doc = "15: EVSYS_8"]
    Evsys8 = 15,
    #[doc = "16: EVSYS_9"]
    Evsys9 = 16,
    #[doc = "17: EVSYS_10"]
    Evsys10 = 17,
    #[doc = "18: EVSYS_11"]
    Evsys11 = 18,
    #[doc = "19: SERCOMX_SLOW"]
    SercomxSlow = 19,
    #[doc = "20: SERCOM0_CORE"]
    Sercom0Core = 20,
    #[doc = "21: SERCOM1_CORE"]
    Sercom1Core = 21,
    #[doc = "22: SERCOM2_CORE"]
    Sercom2Core = 22,
    #[doc = "23: SERCOM3_CORE"]
    Sercom3Core = 23,
    #[doc = "24: SERCOM4_CORE"]
    Sercom4Core = 24,
    #[doc = "25: SERCOM5_CORE"]
    Sercom5Core = 25,
    #[doc = "26: TCC0_TCC1"]
    Tcc0Tcc1 = 26,
    #[doc = "27: TCC2_TC3"]
    Tcc2Tc3 = 27,
    #[doc = "28: TC4_TC5"]
    Tc4Tc5 = 28,
    #[doc = "29: TC6_TC7"]
    Tc6Tc7 = 29,
    #[doc = "30: ADC"]
    Adc = 30,
    #[doc = "31: AC_DIG"]
    AcDig = 31,
    #[doc = "32: AC_ANA"]
    AcAna = 32,
    #[doc = "33: DAC"]
    Dac = 33,
    #[doc = "35: I2S_0"]
    I2s0 = 35,
    #[doc = "36: I2S_1"]
    I2s1 = 36,
}
impl From<Idselect> for u8 {
    #[inline(always)]
    fn from(variant: Idselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idselect {
    type Ux = u8;
}
impl crate::IsEnum for Idselect {}
#[doc = "Field `ID` reader - Generic Clock Selection ID"]
pub type IdR = crate::FieldReader<Idselect>;
impl IdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idselect> {
        match self.bits {
            0 => Some(Idselect::Dfll48),
            1 => Some(Idselect::Fdpll),
            2 => Some(Idselect::Fdpll32k),
            3 => Some(Idselect::Wdt),
            4 => Some(Idselect::Rtc),
            5 => Some(Idselect::Eic),
            6 => Some(Idselect::Usb),
            7 => Some(Idselect::Evsys0),
            8 => Some(Idselect::Evsys1),
            9 => Some(Idselect::Evsys2),
            10 => Some(Idselect::Evsys3),
            11 => Some(Idselect::Evsys4),
            12 => Some(Idselect::Evsys5),
            13 => Some(Idselect::Evsys6),
            14 => Some(Idselect::Evsys7),
            15 => Some(Idselect::Evsys8),
            16 => Some(Idselect::Evsys9),
            17 => Some(Idselect::Evsys10),
            18 => Some(Idselect::Evsys11),
            19 => Some(Idselect::SercomxSlow),
            20 => Some(Idselect::Sercom0Core),
            21 => Some(Idselect::Sercom1Core),
            22 => Some(Idselect::Sercom2Core),
            23 => Some(Idselect::Sercom3Core),
            24 => Some(Idselect::Sercom4Core),
            25 => Some(Idselect::Sercom5Core),
            26 => Some(Idselect::Tcc0Tcc1),
            27 => Some(Idselect::Tcc2Tc3),
            28 => Some(Idselect::Tc4Tc5),
            29 => Some(Idselect::Tc6Tc7),
            30 => Some(Idselect::Adc),
            31 => Some(Idselect::AcDig),
            32 => Some(Idselect::AcAna),
            33 => Some(Idselect::Dac),
            35 => Some(Idselect::I2s0),
            36 => Some(Idselect::I2s1),
            _ => None,
        }
    }
    #[doc = "DFLL48"]
    #[inline(always)]
    pub fn is_dfll48(&self) -> bool {
        *self == Idselect::Dfll48
    }
    #[doc = "FDPLL"]
    #[inline(always)]
    pub fn is_fdpll(&self) -> bool {
        *self == Idselect::Fdpll
    }
    #[doc = "FDPLL32K"]
    #[inline(always)]
    pub fn is_fdpll32k(&self) -> bool {
        *self == Idselect::Fdpll32k
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub fn is_wdt(&self) -> bool {
        *self == Idselect::Wdt
    }
    #[doc = "RTC"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == Idselect::Rtc
    }
    #[doc = "EIC"]
    #[inline(always)]
    pub fn is_eic(&self) -> bool {
        *self == Idselect::Eic
    }
    #[doc = "USB"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == Idselect::Usb
    }
    #[doc = "EVSYS_0"]
    #[inline(always)]
    pub fn is_evsys_0(&self) -> bool {
        *self == Idselect::Evsys0
    }
    #[doc = "EVSYS_1"]
    #[inline(always)]
    pub fn is_evsys_1(&self) -> bool {
        *self == Idselect::Evsys1
    }
    #[doc = "EVSYS_2"]
    #[inline(always)]
    pub fn is_evsys_2(&self) -> bool {
        *self == Idselect::Evsys2
    }
    #[doc = "EVSYS_3"]
    #[inline(always)]
    pub fn is_evsys_3(&self) -> bool {
        *self == Idselect::Evsys3
    }
    #[doc = "EVSYS_4"]
    #[inline(always)]
    pub fn is_evsys_4(&self) -> bool {
        *self == Idselect::Evsys4
    }
    #[doc = "EVSYS_5"]
    #[inline(always)]
    pub fn is_evsys_5(&self) -> bool {
        *self == Idselect::Evsys5
    }
    #[doc = "EVSYS_6"]
    #[inline(always)]
    pub fn is_evsys_6(&self) -> bool {
        *self == Idselect::Evsys6
    }
    #[doc = "EVSYS_7"]
    #[inline(always)]
    pub fn is_evsys_7(&self) -> bool {
        *self == Idselect::Evsys7
    }
    #[doc = "EVSYS_8"]
    #[inline(always)]
    pub fn is_evsys_8(&self) -> bool {
        *self == Idselect::Evsys8
    }
    #[doc = "EVSYS_9"]
    #[inline(always)]
    pub fn is_evsys_9(&self) -> bool {
        *self == Idselect::Evsys9
    }
    #[doc = "EVSYS_10"]
    #[inline(always)]
    pub fn is_evsys_10(&self) -> bool {
        *self == Idselect::Evsys10
    }
    #[doc = "EVSYS_11"]
    #[inline(always)]
    pub fn is_evsys_11(&self) -> bool {
        *self == Idselect::Evsys11
    }
    #[doc = "SERCOMX_SLOW"]
    #[inline(always)]
    pub fn is_sercomx_slow(&self) -> bool {
        *self == Idselect::SercomxSlow
    }
    #[doc = "SERCOM0_CORE"]
    #[inline(always)]
    pub fn is_sercom0_core(&self) -> bool {
        *self == Idselect::Sercom0Core
    }
    #[doc = "SERCOM1_CORE"]
    #[inline(always)]
    pub fn is_sercom1_core(&self) -> bool {
        *self == Idselect::Sercom1Core
    }
    #[doc = "SERCOM2_CORE"]
    #[inline(always)]
    pub fn is_sercom2_core(&self) -> bool {
        *self == Idselect::Sercom2Core
    }
    #[doc = "SERCOM3_CORE"]
    #[inline(always)]
    pub fn is_sercom3_core(&self) -> bool {
        *self == Idselect::Sercom3Core
    }
    #[doc = "SERCOM4_CORE"]
    #[inline(always)]
    pub fn is_sercom4_core(&self) -> bool {
        *self == Idselect::Sercom4Core
    }
    #[doc = "SERCOM5_CORE"]
    #[inline(always)]
    pub fn is_sercom5_core(&self) -> bool {
        *self == Idselect::Sercom5Core
    }
    #[doc = "TCC0_TCC1"]
    #[inline(always)]
    pub fn is_tcc0_tcc1(&self) -> bool {
        *self == Idselect::Tcc0Tcc1
    }
    #[doc = "TCC2_TC3"]
    #[inline(always)]
    pub fn is_tcc2_tc3(&self) -> bool {
        *self == Idselect::Tcc2Tc3
    }
    #[doc = "TC4_TC5"]
    #[inline(always)]
    pub fn is_tc4_tc5(&self) -> bool {
        *self == Idselect::Tc4Tc5
    }
    #[doc = "TC6_TC7"]
    #[inline(always)]
    pub fn is_tc6_tc7(&self) -> bool {
        *self == Idselect::Tc6Tc7
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == Idselect::Adc
    }
    #[doc = "AC_DIG"]
    #[inline(always)]
    pub fn is_ac_dig(&self) -> bool {
        *self == Idselect::AcDig
    }
    #[doc = "AC_ANA"]
    #[inline(always)]
    pub fn is_ac_ana(&self) -> bool {
        *self == Idselect::AcAna
    }
    #[doc = "DAC"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Idselect::Dac
    }
    #[doc = "I2S_0"]
    #[inline(always)]
    pub fn is_i2s_0(&self) -> bool {
        *self == Idselect::I2s0
    }
    #[doc = "I2S_1"]
    #[inline(always)]
    pub fn is_i2s_1(&self) -> bool {
        *self == Idselect::I2s1
    }
}
#[doc = "Field `ID` writer - Generic Clock Selection ID"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 6, Idselect>;
impl<'a, REG> IdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DFLL48"]
    #[inline(always)]
    pub fn dfll48(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Dfll48)
    }
    #[doc = "FDPLL"]
    #[inline(always)]
    pub fn fdpll(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Fdpll)
    }
    #[doc = "FDPLL32K"]
    #[inline(always)]
    pub fn fdpll32k(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Fdpll32k)
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub fn wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Wdt)
    }
    #[doc = "RTC"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Rtc)
    }
    #[doc = "EIC"]
    #[inline(always)]
    pub fn eic(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Eic)
    }
    #[doc = "USB"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Usb)
    }
    #[doc = "EVSYS_0"]
    #[inline(always)]
    pub fn evsys_0(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Evsys0)
    }
    #[doc = "EVSYS_1"]
    #[inline(always)]
    pub fn evsys_1(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Evsys1)
    }
    #[doc = "EVSYS_2"]
    #[inline(always)]
    pub fn evsys_2(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Evsys2)
    }
    #[doc = "EVSYS_3"]
    #[inline(always)]
    pub fn evsys_3(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Evsys3)
    }
    #[doc = "EVSYS_4"]
    #[inline(always)]
    pub fn evsys_4(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Evsys4)
    }
    #[doc = "EVSYS_5"]
    #[inline(always)]
    pub fn evsys_5(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Evsys5)
    }
    #[doc = "EVSYS_6"]
    #[inline(always)]
    pub fn evsys_6(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Evsys6)
    }
    #[doc = "EVSYS_7"]
    #[inline(always)]
    pub fn evsys_7(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Evsys7)
    }
    #[doc = "EVSYS_8"]
    #[inline(always)]
    pub fn evsys_8(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Evsys8)
    }
    #[doc = "EVSYS_9"]
    #[inline(always)]
    pub fn evsys_9(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Evsys9)
    }
    #[doc = "EVSYS_10"]
    #[inline(always)]
    pub fn evsys_10(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Evsys10)
    }
    #[doc = "EVSYS_11"]
    #[inline(always)]
    pub fn evsys_11(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Evsys11)
    }
    #[doc = "SERCOMX_SLOW"]
    #[inline(always)]
    pub fn sercomx_slow(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::SercomxSlow)
    }
    #[doc = "SERCOM0_CORE"]
    #[inline(always)]
    pub fn sercom0_core(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Sercom0Core)
    }
    #[doc = "SERCOM1_CORE"]
    #[inline(always)]
    pub fn sercom1_core(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Sercom1Core)
    }
    #[doc = "SERCOM2_CORE"]
    #[inline(always)]
    pub fn sercom2_core(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Sercom2Core)
    }
    #[doc = "SERCOM3_CORE"]
    #[inline(always)]
    pub fn sercom3_core(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Sercom3Core)
    }
    #[doc = "SERCOM4_CORE"]
    #[inline(always)]
    pub fn sercom4_core(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Sercom4Core)
    }
    #[doc = "SERCOM5_CORE"]
    #[inline(always)]
    pub fn sercom5_core(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Sercom5Core)
    }
    #[doc = "TCC0_TCC1"]
    #[inline(always)]
    pub fn tcc0_tcc1(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Tcc0Tcc1)
    }
    #[doc = "TCC2_TC3"]
    #[inline(always)]
    pub fn tcc2_tc3(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Tcc2Tc3)
    }
    #[doc = "TC4_TC5"]
    #[inline(always)]
    pub fn tc4_tc5(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Tc4Tc5)
    }
    #[doc = "TC6_TC7"]
    #[inline(always)]
    pub fn tc6_tc7(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Tc6Tc7)
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Adc)
    }
    #[doc = "AC_DIG"]
    #[inline(always)]
    pub fn ac_dig(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::AcDig)
    }
    #[doc = "AC_ANA"]
    #[inline(always)]
    pub fn ac_ana(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::AcAna)
    }
    #[doc = "DAC"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Dac)
    }
    #[doc = "I2S_0"]
    #[inline(always)]
    pub fn i2s_0(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::I2s0)
    }
    #[doc = "I2S_1"]
    #[inline(always)]
    pub fn i2s_1(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::I2s1)
    }
}
#[doc = "Generic Clock Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Genselect {
    #[doc = "0: Generic clock generator 0"]
    Gclk0 = 0,
    #[doc = "1: Generic clock generator 1"]
    Gclk1 = 1,
    #[doc = "2: Generic clock generator 2"]
    Gclk2 = 2,
    #[doc = "3: Generic clock generator 3"]
    Gclk3 = 3,
    #[doc = "4: Generic clock generator 4"]
    Gclk4 = 4,
    #[doc = "5: Generic clock generator 5"]
    Gclk5 = 5,
    #[doc = "6: Generic clock generator 6"]
    Gclk6 = 6,
    #[doc = "7: Generic clock generator 7"]
    Gclk7 = 7,
    #[doc = "8: Generic clock generator 8"]
    Gclk8 = 8,
}
impl From<Genselect> for u8 {
    #[inline(always)]
    fn from(variant: Genselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Genselect {
    type Ux = u8;
}
impl crate::IsEnum for Genselect {}
#[doc = "Field `GEN` reader - Generic Clock Generator"]
pub type GenR = crate::FieldReader<Genselect>;
impl GenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Genselect> {
        match self.bits {
            0 => Some(Genselect::Gclk0),
            1 => Some(Genselect::Gclk1),
            2 => Some(Genselect::Gclk2),
            3 => Some(Genselect::Gclk3),
            4 => Some(Genselect::Gclk4),
            5 => Some(Genselect::Gclk5),
            6 => Some(Genselect::Gclk6),
            7 => Some(Genselect::Gclk7),
            8 => Some(Genselect::Gclk8),
            _ => None,
        }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == Genselect::Gclk0
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == Genselect::Gclk1
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == Genselect::Gclk2
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == Genselect::Gclk3
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == Genselect::Gclk4
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == Genselect::Gclk5
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == Genselect::Gclk6
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == Genselect::Gclk7
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == Genselect::Gclk8
    }
}
#[doc = "Field `GEN` writer - Generic Clock Generator"]
pub type GenW<'a, REG> = crate::FieldWriter<'a, REG, 4, Genselect>;
impl<'a, REG> GenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn gclk6(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn gclk7(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk7)
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn gclk8(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk8)
    }
}
#[doc = "Field `CLKEN` reader - Clock Enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock Enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&self) -> GenR {
        GenR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<ClkctrlSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&mut self) -> GenW<ClkctrlSpec> {
        GenW::new(self, 8)
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<ClkctrlSpec> {
        ClkenW::new(self, 14)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WrtlockW<ClkctrlSpec> {
        WrtlockW::new(self, 15)
    }
}
#[doc = "Generic Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctrlSpec;
impl crate::RegisterSpec for ClkctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for ClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for ClkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for ClkctrlSpec {}
