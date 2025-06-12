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
    #[doc = "13: SERCOMX_SLOW"]
    SercomxSlow = 13,
    #[doc = "14: SERCOM0_CORE"]
    Sercom0Core = 14,
    #[doc = "15: SERCOM1_CORE"]
    Sercom1Core = 15,
    #[doc = "16: SERCOM2_CORE"]
    Sercom2Core = 16,
    #[doc = "17: TCC0"]
    Tcc0 = 17,
    #[doc = "18: TC1_TC2"]
    Tc1Tc2 = 18,
    #[doc = "19: ADC"]
    Adc = 19,
    #[doc = "20: AC_DIG"]
    AcDig = 20,
    #[doc = "21: AC_ANA"]
    AcAna = 21,
    #[doc = "22: DAC"]
    Dac = 22,
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
            13 => Some(Idselect::SercomxSlow),
            14 => Some(Idselect::Sercom0Core),
            15 => Some(Idselect::Sercom1Core),
            16 => Some(Idselect::Sercom2Core),
            17 => Some(Idselect::Tcc0),
            18 => Some(Idselect::Tc1Tc2),
            19 => Some(Idselect::Adc),
            20 => Some(Idselect::AcDig),
            21 => Some(Idselect::AcAna),
            22 => Some(Idselect::Dac),
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
    #[doc = "TCC0"]
    #[inline(always)]
    pub fn is_tcc0(&self) -> bool {
        *self == Idselect::Tcc0
    }
    #[doc = "TC1_TC2"]
    #[inline(always)]
    pub fn is_tc1_tc2(&self) -> bool {
        *self == Idselect::Tc1Tc2
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
    #[doc = "TCC0"]
    #[inline(always)]
    pub fn tcc0(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Tcc0)
    }
    #[doc = "TC1_TC2"]
    #[inline(always)]
    pub fn tc1_tc2(self) -> &'a mut crate::W<REG> {
        self.variant(Idselect::Tc1Tc2)
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
