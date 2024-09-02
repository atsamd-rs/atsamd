#[doc = "Register `CHCTRLB` reader"]
pub type R = crate::R<ChctrlbSpec>;
#[doc = "Register `CHCTRLB` writer"]
pub type W = crate::W<ChctrlbSpec>;
#[doc = "Event Input Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evactselect {
    #[doc = "0: No action"]
    Noact = 0,
    #[doc = "1: Transfer and periodic transfer trigger"]
    Trig = 1,
    #[doc = "2: Conditional transfer trigger"]
    Ctrig = 2,
    #[doc = "3: Conditional block transfer"]
    Cblock = 3,
    #[doc = "4: Channel suspend operation"]
    Suspend = 4,
    #[doc = "5: Channel resume operation"]
    Resume = 5,
    #[doc = "6: Skip next block suspend action"]
    Sskip = 6,
}
impl From<Evactselect> for u8 {
    #[inline(always)]
    fn from(variant: Evactselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evactselect {
    type Ux = u8;
}
impl crate::IsEnum for Evactselect {}
#[doc = "Field `EVACT` reader - Event Input Action"]
pub type EvactR = crate::FieldReader<Evactselect>;
impl EvactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Evactselect> {
        match self.bits {
            0 => Some(Evactselect::Noact),
            1 => Some(Evactselect::Trig),
            2 => Some(Evactselect::Ctrig),
            3 => Some(Evactselect::Cblock),
            4 => Some(Evactselect::Suspend),
            5 => Some(Evactselect::Resume),
            6 => Some(Evactselect::Sskip),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == Evactselect::Noact
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Evactselect::Trig
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn is_ctrig(&self) -> bool {
        *self == Evactselect::Ctrig
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn is_cblock(&self) -> bool {
        *self == Evactselect::Cblock
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Evactselect::Suspend
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Evactselect::Resume
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn is_sskip(&self) -> bool {
        *self == Evactselect::Sskip
    }
}
#[doc = "Field `EVACT` writer - Event Input Action"]
pub type EvactW<'a, REG> = crate::FieldWriter<'a, REG, 3, Evactselect>;
impl<'a, REG> EvactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Noact)
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Trig)
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn ctrig(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Ctrig)
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn cblock(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Cblock)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Suspend)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Resume)
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn sskip(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Sskip)
    }
}
#[doc = "Field `EVIE` reader - Channel Event Input Enable"]
pub type EvieR = crate::BitReader;
#[doc = "Field `EVIE` writer - Channel Event Input Enable"]
pub type EvieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVOE` reader - Channel Event Output Enable"]
pub type EvoeR = crate::BitReader;
#[doc = "Field `EVOE` writer - Channel Event Output Enable"]
pub type EvoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel Arbitration Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvlselect {
    #[doc = "0: Channel Priority Level 0"]
    Lvl0 = 0,
    #[doc = "1: Channel Priority Level 1"]
    Lvl1 = 1,
    #[doc = "2: Channel Priority Level 2"]
    Lvl2 = 2,
    #[doc = "3: Channel Priority Level 3"]
    Lvl3 = 3,
}
impl From<Lvlselect> for u8 {
    #[inline(always)]
    fn from(variant: Lvlselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvlselect {
    type Ux = u8;
}
impl crate::IsEnum for Lvlselect {}
#[doc = "Field `LVL` reader - Channel Arbitration Level"]
pub type LvlR = crate::FieldReader<Lvlselect>;
impl LvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvlselect {
        match self.bits {
            0 => Lvlselect::Lvl0,
            1 => Lvlselect::Lvl1,
            2 => Lvlselect::Lvl2,
            3 => Lvlselect::Lvl3,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel Priority Level 0"]
    #[inline(always)]
    pub fn is_lvl0(&self) -> bool {
        *self == Lvlselect::Lvl0
    }
    #[doc = "Channel Priority Level 1"]
    #[inline(always)]
    pub fn is_lvl1(&self) -> bool {
        *self == Lvlselect::Lvl1
    }
    #[doc = "Channel Priority Level 2"]
    #[inline(always)]
    pub fn is_lvl2(&self) -> bool {
        *self == Lvlselect::Lvl2
    }
    #[doc = "Channel Priority Level 3"]
    #[inline(always)]
    pub fn is_lvl3(&self) -> bool {
        *self == Lvlselect::Lvl3
    }
}
#[doc = "Field `LVL` writer - Channel Arbitration Level"]
pub type LvlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lvlselect, crate::Safe>;
impl<'a, REG> LvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel Priority Level 0"]
    #[inline(always)]
    pub fn lvl0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlselect::Lvl0)
    }
    #[doc = "Channel Priority Level 1"]
    #[inline(always)]
    pub fn lvl1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlselect::Lvl1)
    }
    #[doc = "Channel Priority Level 2"]
    #[inline(always)]
    pub fn lvl2(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlselect::Lvl2)
    }
    #[doc = "Channel Priority Level 3"]
    #[inline(always)]
    pub fn lvl3(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlselect::Lvl3)
    }
}
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsrcselect {
    #[doc = "0: Only software/event triggers"]
    Disable = 0,
    #[doc = "1: SERCOM0 RX Trigger"]
    Sercom0Rx = 1,
    #[doc = "2: SERCOM0 TX Trigger"]
    Sercom0Tx = 2,
    #[doc = "3: SERCOM1 RX Trigger"]
    Sercom1Rx = 3,
    #[doc = "4: SERCOM1 TX Trigger"]
    Sercom1Tx = 4,
    #[doc = "5: SERCOM2 RX Trigger"]
    Sercom2Rx = 5,
    #[doc = "6: SERCOM2 TX Trigger"]
    Sercom2Tx = 6,
    #[doc = "7: SERCOM3 RX Trigger"]
    Sercom3Rx = 7,
    #[doc = "8: SERCOM3 TX Trigger"]
    Sercom3Tx = 8,
    #[doc = "9: SERCOM4 RX Trigger"]
    Sercom4Rx = 9,
    #[doc = "10: SERCOM4 TX Trigger"]
    Sercom4Tx = 10,
    #[doc = "11: SERCOM5 RX Trigger"]
    Sercom5Rx = 11,
    #[doc = "12: SERCOM5 TX Trigger"]
    Sercom5Tx = 12,
    #[doc = "13: TCC0 Overflow Trigger"]
    Tcc0Ovf = 13,
    #[doc = "14: TCC0 Match/Compare 0 Trigger"]
    Tcc0Mc0 = 14,
    #[doc = "15: TCC0 Match/Compare 1 Trigger"]
    Tcc0Mc1 = 15,
    #[doc = "16: TCC0 Match/Compare 2 Trigger"]
    Tcc0Mc2 = 16,
    #[doc = "17: TCC0 Match/Compare 3 Trigger"]
    Tcc0Mc3 = 17,
    #[doc = "18: TCC1 Overflow Trigger"]
    Tcc1Ovf = 18,
    #[doc = "19: TCC1 Match/Compare 0 Trigger"]
    Tcc1Mc0 = 19,
    #[doc = "20: TCC1 Match/Compare 1 Trigger"]
    Tcc1Mc1 = 20,
    #[doc = "21: TCC2 Overflow Trigger"]
    Tcc2Ovf = 21,
    #[doc = "22: TCC2 Match/Compare 0 Trigger"]
    Tcc2Mc0 = 22,
    #[doc = "23: TCC2 Match/Compare 1 Trigger"]
    Tcc2Mc1 = 23,
    #[doc = "24: TC3 Overflow Trigger"]
    Tc3Ovf = 24,
    #[doc = "25: TC3 Match/Compare 0 Trigger"]
    Tc3Mc0 = 25,
    #[doc = "26: TC3 Match/Compare 1 Trigger"]
    Tc3Mc1 = 26,
    #[doc = "27: TC4 Overflow Trigger"]
    Tc4Ovf = 27,
    #[doc = "28: TC4 Match/Compare 0 Trigger"]
    Tc4Mc0 = 28,
    #[doc = "29: TC4 Match/Compare 1 Trigger"]
    Tc4Mc1 = 29,
    #[doc = "30: TC5 Overflow Trigger"]
    Tc5Ovf = 30,
    #[doc = "31: TC5 Match/Compare 0 Trigger"]
    Tc5Mc0 = 31,
    #[doc = "32: TC5 Match/Compare 1 Trigger"]
    Tc5Mc1 = 32,
    #[doc = "33: TC6 Overflow Trigger"]
    Tc6Ovf = 33,
    #[doc = "34: TC6 Match/Compare 0 Trigger"]
    Tc6Mc0 = 34,
    #[doc = "35: TC6 Match/Compare 1 Trigger"]
    Tc6Mc1 = 35,
    #[doc = "36: TC7 Overflow Trigger"]
    Tc7Ovf = 36,
    #[doc = "37: TC7 Match/Compare 0 Trigger"]
    Tc7Mc0 = 37,
    #[doc = "38: TC7 Match/Compare 1 Trigger"]
    Tc7Mc1 = 38,
    #[doc = "39: ADC Result Ready Trigger"]
    AdcResrdy = 39,
    #[doc = "40: DAC Empty Trigger"]
    DacEmpty = 40,
    #[doc = "41: I2S RX 0 Trigger"]
    I2sRx0 = 41,
    #[doc = "42: I2S RX 1 Trigger"]
    I2sRx1 = 42,
    #[doc = "43: I2S TX 0 Trigger"]
    I2sTx0 = 43,
    #[doc = "44: I2S TX 1 Trigger"]
    I2sTx1 = 44,
    #[doc = "45: TCC3 Overflow Trigger"]
    Tcc3Ovf = 45,
    #[doc = "46: TCC3 Match/Compare 0 Trigger"]
    Tcc3Mc0 = 46,
    #[doc = "47: TCC3 Match/Compare 1 Trigger"]
    Tcc3Mc1 = 47,
    #[doc = "48: Match/Compare 2 Trigger"]
    Tcc3Mc2 = 48,
    #[doc = "49: Match/Compare 3 Trigger"]
    Tcc3Mc3 = 49,
}
impl From<Trigsrcselect> for u8 {
    #[inline(always)]
    fn from(variant: Trigsrcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsrcselect {
    type Ux = u8;
}
impl crate::IsEnum for Trigsrcselect {}
#[doc = "Field `TRIGSRC` reader - Trigger Source"]
pub type TrigsrcR = crate::FieldReader<Trigsrcselect>;
impl TrigsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigsrcselect> {
        match self.bits {
            0 => Some(Trigsrcselect::Disable),
            1 => Some(Trigsrcselect::Sercom0Rx),
            2 => Some(Trigsrcselect::Sercom0Tx),
            3 => Some(Trigsrcselect::Sercom1Rx),
            4 => Some(Trigsrcselect::Sercom1Tx),
            5 => Some(Trigsrcselect::Sercom2Rx),
            6 => Some(Trigsrcselect::Sercom2Tx),
            7 => Some(Trigsrcselect::Sercom3Rx),
            8 => Some(Trigsrcselect::Sercom3Tx),
            9 => Some(Trigsrcselect::Sercom4Rx),
            10 => Some(Trigsrcselect::Sercom4Tx),
            11 => Some(Trigsrcselect::Sercom5Rx),
            12 => Some(Trigsrcselect::Sercom5Tx),
            13 => Some(Trigsrcselect::Tcc0Ovf),
            14 => Some(Trigsrcselect::Tcc0Mc0),
            15 => Some(Trigsrcselect::Tcc0Mc1),
            16 => Some(Trigsrcselect::Tcc0Mc2),
            17 => Some(Trigsrcselect::Tcc0Mc3),
            18 => Some(Trigsrcselect::Tcc1Ovf),
            19 => Some(Trigsrcselect::Tcc1Mc0),
            20 => Some(Trigsrcselect::Tcc1Mc1),
            21 => Some(Trigsrcselect::Tcc2Ovf),
            22 => Some(Trigsrcselect::Tcc2Mc0),
            23 => Some(Trigsrcselect::Tcc2Mc1),
            24 => Some(Trigsrcselect::Tc3Ovf),
            25 => Some(Trigsrcselect::Tc3Mc0),
            26 => Some(Trigsrcselect::Tc3Mc1),
            27 => Some(Trigsrcselect::Tc4Ovf),
            28 => Some(Trigsrcselect::Tc4Mc0),
            29 => Some(Trigsrcselect::Tc4Mc1),
            30 => Some(Trigsrcselect::Tc5Ovf),
            31 => Some(Trigsrcselect::Tc5Mc0),
            32 => Some(Trigsrcselect::Tc5Mc1),
            33 => Some(Trigsrcselect::Tc6Ovf),
            34 => Some(Trigsrcselect::Tc6Mc0),
            35 => Some(Trigsrcselect::Tc6Mc1),
            36 => Some(Trigsrcselect::Tc7Ovf),
            37 => Some(Trigsrcselect::Tc7Mc0),
            38 => Some(Trigsrcselect::Tc7Mc1),
            39 => Some(Trigsrcselect::AdcResrdy),
            40 => Some(Trigsrcselect::DacEmpty),
            41 => Some(Trigsrcselect::I2sRx0),
            42 => Some(Trigsrcselect::I2sRx1),
            43 => Some(Trigsrcselect::I2sTx0),
            44 => Some(Trigsrcselect::I2sTx1),
            45 => Some(Trigsrcselect::Tcc3Ovf),
            46 => Some(Trigsrcselect::Tcc3Mc0),
            47 => Some(Trigsrcselect::Tcc3Mc1),
            48 => Some(Trigsrcselect::Tcc3Mc2),
            49 => Some(Trigsrcselect::Tcc3Mc3),
            _ => None,
        }
    }
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Trigsrcselect::Disable
    }
    #[doc = "SERCOM0 RX Trigger"]
    #[inline(always)]
    pub fn is_sercom0_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom0Rx
    }
    #[doc = "SERCOM0 TX Trigger"]
    #[inline(always)]
    pub fn is_sercom0_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom0Tx
    }
    #[doc = "SERCOM1 RX Trigger"]
    #[inline(always)]
    pub fn is_sercom1_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom1Rx
    }
    #[doc = "SERCOM1 TX Trigger"]
    #[inline(always)]
    pub fn is_sercom1_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom1Tx
    }
    #[doc = "SERCOM2 RX Trigger"]
    #[inline(always)]
    pub fn is_sercom2_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom2Rx
    }
    #[doc = "SERCOM2 TX Trigger"]
    #[inline(always)]
    pub fn is_sercom2_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom2Tx
    }
    #[doc = "SERCOM3 RX Trigger"]
    #[inline(always)]
    pub fn is_sercom3_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom3Rx
    }
    #[doc = "SERCOM3 TX Trigger"]
    #[inline(always)]
    pub fn is_sercom3_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom3Tx
    }
    #[doc = "SERCOM4 RX Trigger"]
    #[inline(always)]
    pub fn is_sercom4_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom4Rx
    }
    #[doc = "SERCOM4 TX Trigger"]
    #[inline(always)]
    pub fn is_sercom4_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom4Tx
    }
    #[doc = "SERCOM5 RX Trigger"]
    #[inline(always)]
    pub fn is_sercom5_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom5Rx
    }
    #[doc = "SERCOM5 TX Trigger"]
    #[inline(always)]
    pub fn is_sercom5_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom5Tx
    }
    #[doc = "TCC0 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tcc0_ovf(&self) -> bool {
        *self == Trigsrcselect::Tcc0Ovf
    }
    #[doc = "TCC0 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tcc0_mc0(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc0
    }
    #[doc = "TCC0 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tcc0_mc1(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc1
    }
    #[doc = "TCC0 Match/Compare 2 Trigger"]
    #[inline(always)]
    pub fn is_tcc0_mc2(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc2
    }
    #[doc = "TCC0 Match/Compare 3 Trigger"]
    #[inline(always)]
    pub fn is_tcc0_mc3(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc3
    }
    #[doc = "TCC1 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tcc1_ovf(&self) -> bool {
        *self == Trigsrcselect::Tcc1Ovf
    }
    #[doc = "TCC1 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tcc1_mc0(&self) -> bool {
        *self == Trigsrcselect::Tcc1Mc0
    }
    #[doc = "TCC1 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tcc1_mc1(&self) -> bool {
        *self == Trigsrcselect::Tcc1Mc1
    }
    #[doc = "TCC2 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tcc2_ovf(&self) -> bool {
        *self == Trigsrcselect::Tcc2Ovf
    }
    #[doc = "TCC2 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tcc2_mc0(&self) -> bool {
        *self == Trigsrcselect::Tcc2Mc0
    }
    #[doc = "TCC2 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tcc2_mc1(&self) -> bool {
        *self == Trigsrcselect::Tcc2Mc1
    }
    #[doc = "TC3 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tc3_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc3Ovf
    }
    #[doc = "TC3 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tc3_mc0(&self) -> bool {
        *self == Trigsrcselect::Tc3Mc0
    }
    #[doc = "TC3 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tc3_mc1(&self) -> bool {
        *self == Trigsrcselect::Tc3Mc1
    }
    #[doc = "TC4 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tc4_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc4Ovf
    }
    #[doc = "TC4 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tc4_mc0(&self) -> bool {
        *self == Trigsrcselect::Tc4Mc0
    }
    #[doc = "TC4 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tc4_mc1(&self) -> bool {
        *self == Trigsrcselect::Tc4Mc1
    }
    #[doc = "TC5 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tc5_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc5Ovf
    }
    #[doc = "TC5 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tc5_mc0(&self) -> bool {
        *self == Trigsrcselect::Tc5Mc0
    }
    #[doc = "TC5 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tc5_mc1(&self) -> bool {
        *self == Trigsrcselect::Tc5Mc1
    }
    #[doc = "TC6 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tc6_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc6Ovf
    }
    #[doc = "TC6 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tc6_mc0(&self) -> bool {
        *self == Trigsrcselect::Tc6Mc0
    }
    #[doc = "TC6 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tc6_mc1(&self) -> bool {
        *self == Trigsrcselect::Tc6Mc1
    }
    #[doc = "TC7 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tc7_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc7Ovf
    }
    #[doc = "TC7 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tc7_mc0(&self) -> bool {
        *self == Trigsrcselect::Tc7Mc0
    }
    #[doc = "TC7 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tc7_mc1(&self) -> bool {
        *self == Trigsrcselect::Tc7Mc1
    }
    #[doc = "ADC Result Ready Trigger"]
    #[inline(always)]
    pub fn is_adc_resrdy(&self) -> bool {
        *self == Trigsrcselect::AdcResrdy
    }
    #[doc = "DAC Empty Trigger"]
    #[inline(always)]
    pub fn is_dac_empty(&self) -> bool {
        *self == Trigsrcselect::DacEmpty
    }
    #[doc = "I2S RX 0 Trigger"]
    #[inline(always)]
    pub fn is_i2s_rx_0(&self) -> bool {
        *self == Trigsrcselect::I2sRx0
    }
    #[doc = "I2S RX 1 Trigger"]
    #[inline(always)]
    pub fn is_i2s_rx_1(&self) -> bool {
        *self == Trigsrcselect::I2sRx1
    }
    #[doc = "I2S TX 0 Trigger"]
    #[inline(always)]
    pub fn is_i2s_tx_0(&self) -> bool {
        *self == Trigsrcselect::I2sTx0
    }
    #[doc = "I2S TX 1 Trigger"]
    #[inline(always)]
    pub fn is_i2s_tx_1(&self) -> bool {
        *self == Trigsrcselect::I2sTx1
    }
    #[doc = "TCC3 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tcc3_ovf(&self) -> bool {
        *self == Trigsrcselect::Tcc3Ovf
    }
    #[doc = "TCC3 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tcc3_mc0(&self) -> bool {
        *self == Trigsrcselect::Tcc3Mc0
    }
    #[doc = "TCC3 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tcc3_mc1(&self) -> bool {
        *self == Trigsrcselect::Tcc3Mc1
    }
    #[doc = "Match/Compare 2 Trigger"]
    #[inline(always)]
    pub fn is_tcc3_mc2(&self) -> bool {
        *self == Trigsrcselect::Tcc3Mc2
    }
    #[doc = "Match/Compare 3 Trigger"]
    #[inline(always)]
    pub fn is_tcc3_mc3(&self) -> bool {
        *self == Trigsrcselect::Tcc3Mc3
    }
}
#[doc = "Field `TRIGSRC` writer - Trigger Source"]
pub type TrigsrcW<'a, REG> = crate::FieldWriter<'a, REG, 6, Trigsrcselect>;
impl<'a, REG> TrigsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Disable)
    }
    #[doc = "SERCOM0 RX Trigger"]
    #[inline(always)]
    pub fn sercom0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom0Rx)
    }
    #[doc = "SERCOM0 TX Trigger"]
    #[inline(always)]
    pub fn sercom0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom0Tx)
    }
    #[doc = "SERCOM1 RX Trigger"]
    #[inline(always)]
    pub fn sercom1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom1Rx)
    }
    #[doc = "SERCOM1 TX Trigger"]
    #[inline(always)]
    pub fn sercom1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom1Tx)
    }
    #[doc = "SERCOM2 RX Trigger"]
    #[inline(always)]
    pub fn sercom2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom2Rx)
    }
    #[doc = "SERCOM2 TX Trigger"]
    #[inline(always)]
    pub fn sercom2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom2Tx)
    }
    #[doc = "SERCOM3 RX Trigger"]
    #[inline(always)]
    pub fn sercom3_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom3Rx)
    }
    #[doc = "SERCOM3 TX Trigger"]
    #[inline(always)]
    pub fn sercom3_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom3Tx)
    }
    #[doc = "SERCOM4 RX Trigger"]
    #[inline(always)]
    pub fn sercom4_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom4Rx)
    }
    #[doc = "SERCOM4 TX Trigger"]
    #[inline(always)]
    pub fn sercom4_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom4Tx)
    }
    #[doc = "SERCOM5 RX Trigger"]
    #[inline(always)]
    pub fn sercom5_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom5Rx)
    }
    #[doc = "SERCOM5 TX Trigger"]
    #[inline(always)]
    pub fn sercom5_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom5Tx)
    }
    #[doc = "TCC0 Overflow Trigger"]
    #[inline(always)]
    pub fn tcc0_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Ovf)
    }
    #[doc = "TCC0 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc0)
    }
    #[doc = "TCC0 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc1)
    }
    #[doc = "TCC0 Match/Compare 2 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc2)
    }
    #[doc = "TCC0 Match/Compare 3 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc3)
    }
    #[doc = "TCC1 Overflow Trigger"]
    #[inline(always)]
    pub fn tcc1_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc1Ovf)
    }
    #[doc = "TCC1 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tcc1_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc1Mc0)
    }
    #[doc = "TCC1 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tcc1_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc1Mc1)
    }
    #[doc = "TCC2 Overflow Trigger"]
    #[inline(always)]
    pub fn tcc2_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc2Ovf)
    }
    #[doc = "TCC2 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tcc2_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc2Mc0)
    }
    #[doc = "TCC2 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tcc2_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc2Mc1)
    }
    #[doc = "TC3 Overflow Trigger"]
    #[inline(always)]
    pub fn tc3_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc3Ovf)
    }
    #[doc = "TC3 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc3_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc3Mc0)
    }
    #[doc = "TC3 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc3_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc3Mc1)
    }
    #[doc = "TC4 Overflow Trigger"]
    #[inline(always)]
    pub fn tc4_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc4Ovf)
    }
    #[doc = "TC4 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc4_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc4Mc0)
    }
    #[doc = "TC4 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc4_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc4Mc1)
    }
    #[doc = "TC5 Overflow Trigger"]
    #[inline(always)]
    pub fn tc5_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc5Ovf)
    }
    #[doc = "TC5 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc5_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc5Mc0)
    }
    #[doc = "TC5 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc5_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc5Mc1)
    }
    #[doc = "TC6 Overflow Trigger"]
    #[inline(always)]
    pub fn tc6_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc6Ovf)
    }
    #[doc = "TC6 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc6_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc6Mc0)
    }
    #[doc = "TC6 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc6_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc6Mc1)
    }
    #[doc = "TC7 Overflow Trigger"]
    #[inline(always)]
    pub fn tc7_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc7Ovf)
    }
    #[doc = "TC7 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc7_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc7Mc0)
    }
    #[doc = "TC7 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc7_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc7Mc1)
    }
    #[doc = "ADC Result Ready Trigger"]
    #[inline(always)]
    pub fn adc_resrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::AdcResrdy)
    }
    #[doc = "DAC Empty Trigger"]
    #[inline(always)]
    pub fn dac_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::DacEmpty)
    }
    #[doc = "I2S RX 0 Trigger"]
    #[inline(always)]
    pub fn i2s_rx_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::I2sRx0)
    }
    #[doc = "I2S RX 1 Trigger"]
    #[inline(always)]
    pub fn i2s_rx_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::I2sRx1)
    }
    #[doc = "I2S TX 0 Trigger"]
    #[inline(always)]
    pub fn i2s_tx_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::I2sTx0)
    }
    #[doc = "I2S TX 1 Trigger"]
    #[inline(always)]
    pub fn i2s_tx_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::I2sTx1)
    }
    #[doc = "TCC3 Overflow Trigger"]
    #[inline(always)]
    pub fn tcc3_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc3Ovf)
    }
    #[doc = "TCC3 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tcc3_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc3Mc0)
    }
    #[doc = "TCC3 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tcc3_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc3Mc1)
    }
    #[doc = "Match/Compare 2 Trigger"]
    #[inline(always)]
    pub fn tcc3_mc2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc3Mc2)
    }
    #[doc = "Match/Compare 3 Trigger"]
    #[inline(always)]
    pub fn tcc3_mc3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc3Mc3)
    }
}
#[doc = "Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigactselect {
    #[doc = "0: One trigger required for each block transfer"]
    Block = 0,
    #[doc = "2: One trigger required for each beat transfer"]
    Beat = 2,
    #[doc = "3: One trigger required for each transaction"]
    Transaction = 3,
}
impl From<Trigactselect> for u8 {
    #[inline(always)]
    fn from(variant: Trigactselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigactselect {
    type Ux = u8;
}
impl crate::IsEnum for Trigactselect {}
#[doc = "Field `TRIGACT` reader - Trigger Action"]
pub type TrigactR = crate::FieldReader<Trigactselect>;
impl TrigactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigactselect> {
        match self.bits {
            0 => Some(Trigactselect::Block),
            2 => Some(Trigactselect::Beat),
            3 => Some(Trigactselect::Transaction),
            _ => None,
        }
    }
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == Trigactselect::Block
    }
    #[doc = "One trigger required for each beat transfer"]
    #[inline(always)]
    pub fn is_beat(&self) -> bool {
        *self == Trigactselect::Beat
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        *self == Trigactselect::Transaction
    }
}
#[doc = "Field `TRIGACT` writer - Trigger Action"]
pub type TrigactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trigactselect>;
impl<'a, REG> TrigactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Block)
    }
    #[doc = "One trigger required for each beat transfer"]
    #[inline(always)]
    pub fn beat(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Beat)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Transaction)
    }
}
#[doc = "Software Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdselect {
    #[doc = "0: No action"]
    Noact = 0,
    #[doc = "1: Channel suspend operation"]
    Suspend = 1,
    #[doc = "2: Channel resume operation"]
    Resume = 2,
}
impl From<Cmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Cmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Cmdselect {}
#[doc = "Field `CMD` reader - Software Command"]
pub type CmdR = crate::FieldReader<Cmdselect>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdselect> {
        match self.bits {
            0 => Some(Cmdselect::Noact),
            1 => Some(Cmdselect::Suspend),
            2 => Some(Cmdselect::Resume),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == Cmdselect::Noact
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Cmdselect::Suspend
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Cmdselect::Resume
    }
}
#[doc = "Field `CMD` writer - Software Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmdselect>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Noact)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Suspend)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Resume)
    }
}
impl R {
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline(always)]
    pub fn evact(&self) -> EvactR {
        EvactR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&self) -> EvieR {
        EvieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EvoeR {
        EvoeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Channel Arbitration Level"]
    #[inline(always)]
    pub fn lvl(&self) -> LvlR {
        LvlR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TrigsrcR {
        TrigsrcR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&self) -> TrigactR {
        TrigactR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact(&mut self) -> EvactW<ChctrlbSpec> {
        EvactW::new(self, 0)
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evie(&mut self) -> EvieW<ChctrlbSpec> {
        EvieW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evoe(&mut self) -> EvoeW<ChctrlbSpec> {
        EvoeW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Channel Arbitration Level"]
    #[inline(always)]
    #[must_use]
    pub fn lvl(&mut self) -> LvlW<ChctrlbSpec> {
        LvlW::new(self, 5)
    }
    #[doc = "Bits 8:13 - Trigger Source"]
    #[inline(always)]
    #[must_use]
    pub fn trigsrc(&mut self) -> TrigsrcW<ChctrlbSpec> {
        TrigsrcW::new(self, 8)
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline(always)]
    #[must_use]
    pub fn trigact(&mut self) -> TrigactW<ChctrlbSpec> {
        TrigactW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<ChctrlbSpec> {
        CmdW::new(self, 24)
    }
}
#[doc = "Channel Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctrlbSpec;
impl crate::RegisterSpec for ChctrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctrlb::R`](R) reader structure"]
impl crate::Readable for ChctrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`chctrlb::W`](W) writer structure"]
impl crate::Writable for ChctrlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHCTRLB to value 0"]
impl crate::Resettable for ChctrlbSpec {
    const RESET_VALUE: u32 = 0;
}
