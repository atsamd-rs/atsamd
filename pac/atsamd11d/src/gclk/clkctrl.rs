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
    #[doc = "13: SERCOMX_SLOW"]
    SERCOMX_SLOW = 13,
    #[doc = "14: SERCOM0_CORE"]
    SERCOM0_CORE = 14,
    #[doc = "15: SERCOM1_CORE"]
    SERCOM1_CORE = 15,
    #[doc = "16: SERCOM2_CORE"]
    SERCOM2_CORE = 16,
    #[doc = "17: TCC0"]
    TCC0 = 17,
    #[doc = "18: TC1_TC2"]
    TC1_TC2 = 18,
    #[doc = "19: ADC"]
    ADC = 19,
    #[doc = "20: AC_DIG"]
    AC_DIG = 20,
    #[doc = "21: AC_ANA"]
    AC_ANA = 21,
    #[doc = "22: DAC"]
    DAC = 22,
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
            13 => Some(IDSELECT_A::SERCOMX_SLOW),
            14 => Some(IDSELECT_A::SERCOM0_CORE),
            15 => Some(IDSELECT_A::SERCOM1_CORE),
            16 => Some(IDSELECT_A::SERCOM2_CORE),
            17 => Some(IDSELECT_A::TCC0),
            18 => Some(IDSELECT_A::TC1_TC2),
            19 => Some(IDSELECT_A::ADC),
            20 => Some(IDSELECT_A::AC_DIG),
            21 => Some(IDSELECT_A::AC_ANA),
            22 => Some(IDSELECT_A::DAC),
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
    #[doc = "TCC0"]
    #[inline(always)]
    pub fn is_tcc0(&self) -> bool {
        *self == IDSELECT_A::TCC0
    }
    #[doc = "TC1_TC2"]
    #[inline(always)]
    pub fn is_tc1_tc2(&self) -> bool {
        *self == IDSELECT_A::TC1_TC2
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
    #[doc = "TCC0"]
    #[inline(always)]
    pub fn tcc0(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::TCC0)
    }
    #[doc = "TC1_TC2"]
    #[inline(always)]
    pub fn tc1_tc2(self) -> &'a mut crate::W<REG> {
        self.variant(IDSELECT_A::TC1_TC2)
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
