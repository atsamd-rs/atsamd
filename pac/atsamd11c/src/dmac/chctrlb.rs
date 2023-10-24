#[doc = "Register `CHCTRLB` reader"]
pub type R = crate::R<CHCTRLB_SPEC>;
#[doc = "Register `CHCTRLB` writer"]
pub type W = crate::W<CHCTRLB_SPEC>;
#[doc = "Field `EVACT` reader - Event Input Action"]
pub type EVACT_R = crate::FieldReader<EVACTSELECT_A>;
#[doc = "Event Input Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVACTSELECT_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: Transfer and periodic transfer trigger"]
    TRIG = 1,
    #[doc = "2: Conditional transfer trigger"]
    CTRIG = 2,
    #[doc = "3: Conditional block transfer"]
    CBLOCK = 3,
    #[doc = "4: Channel suspend operation"]
    SUSPEND = 4,
    #[doc = "5: Channel resume operation"]
    RESUME = 5,
    #[doc = "6: Skip next block suspend action"]
    SSKIP = 6,
}
impl From<EVACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EVACTSELECT_A {
    type Ux = u8;
}
impl EVACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EVACTSELECT_A> {
        match self.bits {
            0 => Some(EVACTSELECT_A::NOACT),
            1 => Some(EVACTSELECT_A::TRIG),
            2 => Some(EVACTSELECT_A::CTRIG),
            3 => Some(EVACTSELECT_A::CBLOCK),
            4 => Some(EVACTSELECT_A::SUSPEND),
            5 => Some(EVACTSELECT_A::RESUME),
            6 => Some(EVACTSELECT_A::SSKIP),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == EVACTSELECT_A::NOACT
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == EVACTSELECT_A::TRIG
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn is_ctrig(&self) -> bool {
        *self == EVACTSELECT_A::CTRIG
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn is_cblock(&self) -> bool {
        *self == EVACTSELECT_A::CBLOCK
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == EVACTSELECT_A::SUSPEND
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == EVACTSELECT_A::RESUME
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn is_sskip(&self) -> bool {
        *self == EVACTSELECT_A::SSKIP
    }
}
#[doc = "Field `EVACT` writer - Event Input Action"]
pub type EVACT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, EVACTSELECT_A>;
impl<'a, REG, const O: u8> EVACT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::NOACT)
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::TRIG)
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn ctrig(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::CTRIG)
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn cblock(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::CBLOCK)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::RESUME)
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn sskip(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::SSKIP)
    }
}
#[doc = "Field `EVIE` reader - Channel Event Input Enable"]
pub type EVIE_R = crate::BitReader;
#[doc = "Field `EVIE` writer - Channel Event Input Enable"]
pub type EVIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVOE` reader - Channel Event Output Enable"]
pub type EVOE_R = crate::BitReader;
#[doc = "Field `EVOE` writer - Channel Event Output Enable"]
pub type EVOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LVL` reader - Channel Arbitration Level"]
pub type LVL_R = crate::FieldReader<LVLSELECT_A>;
#[doc = "Channel Arbitration Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVLSELECT_A {
    #[doc = "0: Channel Priority Level 0"]
    LVL0 = 0,
    #[doc = "1: Channel Priority Level 1"]
    LVL1 = 1,
    #[doc = "2: Channel Priority Level 2"]
    LVL2 = 2,
    #[doc = "3: Channel Priority Level 3"]
    LVL3 = 3,
}
impl From<LVLSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LVLSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LVLSELECT_A {
    type Ux = u8;
}
impl LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVLSELECT_A {
        match self.bits {
            0 => LVLSELECT_A::LVL0,
            1 => LVLSELECT_A::LVL1,
            2 => LVLSELECT_A::LVL2,
            3 => LVLSELECT_A::LVL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel Priority Level 0"]
    #[inline(always)]
    pub fn is_lvl0(&self) -> bool {
        *self == LVLSELECT_A::LVL0
    }
    #[doc = "Channel Priority Level 1"]
    #[inline(always)]
    pub fn is_lvl1(&self) -> bool {
        *self == LVLSELECT_A::LVL1
    }
    #[doc = "Channel Priority Level 2"]
    #[inline(always)]
    pub fn is_lvl2(&self) -> bool {
        *self == LVLSELECT_A::LVL2
    }
    #[doc = "Channel Priority Level 3"]
    #[inline(always)]
    pub fn is_lvl3(&self) -> bool {
        *self == LVLSELECT_A::LVL3
    }
}
#[doc = "Field `LVL` writer - Channel Arbitration Level"]
pub type LVL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, LVLSELECT_A>;
impl<'a, REG, const O: u8> LVL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel Priority Level 0"]
    #[inline(always)]
    pub fn lvl0(self) -> &'a mut crate::W<REG> {
        self.variant(LVLSELECT_A::LVL0)
    }
    #[doc = "Channel Priority Level 1"]
    #[inline(always)]
    pub fn lvl1(self) -> &'a mut crate::W<REG> {
        self.variant(LVLSELECT_A::LVL1)
    }
    #[doc = "Channel Priority Level 2"]
    #[inline(always)]
    pub fn lvl2(self) -> &'a mut crate::W<REG> {
        self.variant(LVLSELECT_A::LVL2)
    }
    #[doc = "Channel Priority Level 3"]
    #[inline(always)]
    pub fn lvl3(self) -> &'a mut crate::W<REG> {
        self.variant(LVLSELECT_A::LVL3)
    }
}
#[doc = "Field `TRIGSRC` reader - Trigger Source"]
pub type TRIGSRC_R = crate::FieldReader<TRIGSRCSELECT_A>;
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSRCSELECT_A {
    #[doc = "0: Only software/event triggers"]
    DISABLE = 0,
    #[doc = "1: SERCOM0 RX Trigger"]
    SERCOM0_RX = 1,
    #[doc = "2: SERCOM0 TX Trigger"]
    SERCOM0_TX = 2,
    #[doc = "3: SERCOM1 RX Trigger"]
    SERCOM1_RX = 3,
    #[doc = "4: SERCOM1 TX Trigger"]
    SERCOM1_TX = 4,
    #[doc = "5: SERCOM2 RX Trigger"]
    SERCOM2_RX = 5,
    #[doc = "6: SERCOM2 TX Trigger"]
    SERCOM2_TX = 6,
    #[doc = "7: TCC0 Overflow Trigger"]
    TCC0_OVF = 7,
    #[doc = "8: TCC0 Match/Compare 0 Trigger"]
    TCC0_MC0 = 8,
    #[doc = "9: TCC0 Match/Compare 1 Trigger"]
    TCC0_MC1 = 9,
    #[doc = "10: TCC0 Match/Compare 2 Trigger"]
    TCC0_MC2 = 10,
    #[doc = "11: TCC0 Match/Compare 3 Trigger"]
    TCC0_MC3 = 11,
    #[doc = "12: TC1 Overflow Trigger"]
    TC1_OVF = 12,
    #[doc = "13: TC1 Match/Compare 0 Trigger"]
    TC1_MC0 = 13,
    #[doc = "14: TC1 Match/Compare 1 Trigger"]
    TC1_MC1 = 14,
    #[doc = "15: TC2 Overflow Trigger"]
    TC2_OVF = 15,
    #[doc = "16: TC2 Match/Compare 0 Trigger"]
    TC2_MC0 = 16,
    #[doc = "17: TC2 Match/Compare 1 Trigger"]
    TC2_MC1 = 17,
    #[doc = "18: ADC Result Ready Trigger"]
    ADC_RESRDY = 18,
    #[doc = "19: DAC Empty Trigger"]
    DAC_EMPTY = 19,
}
impl From<TRIGSRCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRCSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGSRCSELECT_A {
    type Ux = u8;
}
impl TRIGSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRIGSRCSELECT_A> {
        match self.bits {
            0 => Some(TRIGSRCSELECT_A::DISABLE),
            1 => Some(TRIGSRCSELECT_A::SERCOM0_RX),
            2 => Some(TRIGSRCSELECT_A::SERCOM0_TX),
            3 => Some(TRIGSRCSELECT_A::SERCOM1_RX),
            4 => Some(TRIGSRCSELECT_A::SERCOM1_TX),
            5 => Some(TRIGSRCSELECT_A::SERCOM2_RX),
            6 => Some(TRIGSRCSELECT_A::SERCOM2_TX),
            7 => Some(TRIGSRCSELECT_A::TCC0_OVF),
            8 => Some(TRIGSRCSELECT_A::TCC0_MC0),
            9 => Some(TRIGSRCSELECT_A::TCC0_MC1),
            10 => Some(TRIGSRCSELECT_A::TCC0_MC2),
            11 => Some(TRIGSRCSELECT_A::TCC0_MC3),
            12 => Some(TRIGSRCSELECT_A::TC1_OVF),
            13 => Some(TRIGSRCSELECT_A::TC1_MC0),
            14 => Some(TRIGSRCSELECT_A::TC1_MC1),
            15 => Some(TRIGSRCSELECT_A::TC2_OVF),
            16 => Some(TRIGSRCSELECT_A::TC2_MC0),
            17 => Some(TRIGSRCSELECT_A::TC2_MC1),
            18 => Some(TRIGSRCSELECT_A::ADC_RESRDY),
            19 => Some(TRIGSRCSELECT_A::DAC_EMPTY),
            _ => None,
        }
    }
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TRIGSRCSELECT_A::DISABLE
    }
    #[doc = "SERCOM0 RX Trigger"]
    #[inline(always)]
    pub fn is_sercom0_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM0_RX
    }
    #[doc = "SERCOM0 TX Trigger"]
    #[inline(always)]
    pub fn is_sercom0_tx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM0_TX
    }
    #[doc = "SERCOM1 RX Trigger"]
    #[inline(always)]
    pub fn is_sercom1_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM1_RX
    }
    #[doc = "SERCOM1 TX Trigger"]
    #[inline(always)]
    pub fn is_sercom1_tx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM1_TX
    }
    #[doc = "SERCOM2 RX Trigger"]
    #[inline(always)]
    pub fn is_sercom2_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM2_RX
    }
    #[doc = "SERCOM2 TX Trigger"]
    #[inline(always)]
    pub fn is_sercom2_tx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM2_TX
    }
    #[doc = "TCC0 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tcc0_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC0_OVF
    }
    #[doc = "TCC0 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tcc0_mc0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC0_MC0
    }
    #[doc = "TCC0 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tcc0_mc1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC0_MC1
    }
    #[doc = "TCC0 Match/Compare 2 Trigger"]
    #[inline(always)]
    pub fn is_tcc0_mc2(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC0_MC2
    }
    #[doc = "TCC0 Match/Compare 3 Trigger"]
    #[inline(always)]
    pub fn is_tcc0_mc3(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC0_MC3
    }
    #[doc = "TC1 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tc1_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC1_OVF
    }
    #[doc = "TC1 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tc1_mc0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC1_MC0
    }
    #[doc = "TC1 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tc1_mc1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC1_MC1
    }
    #[doc = "TC2 Overflow Trigger"]
    #[inline(always)]
    pub fn is_tc2_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC2_OVF
    }
    #[doc = "TC2 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn is_tc2_mc0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC2_MC0
    }
    #[doc = "TC2 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn is_tc2_mc1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC2_MC1
    }
    #[doc = "ADC Result Ready Trigger"]
    #[inline(always)]
    pub fn is_adc_resrdy(&self) -> bool {
        *self == TRIGSRCSELECT_A::ADC_RESRDY
    }
    #[doc = "DAC Empty Trigger"]
    #[inline(always)]
    pub fn is_dac_empty(&self) -> bool {
        *self == TRIGSRCSELECT_A::DAC_EMPTY
    }
}
#[doc = "Field `TRIGSRC` writer - Trigger Source"]
pub type TRIGSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, TRIGSRCSELECT_A>;
impl<'a, REG, const O: u8> TRIGSRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::DISABLE)
    }
    #[doc = "SERCOM0 RX Trigger"]
    #[inline(always)]
    pub fn sercom0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::SERCOM0_RX)
    }
    #[doc = "SERCOM0 TX Trigger"]
    #[inline(always)]
    pub fn sercom0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::SERCOM0_TX)
    }
    #[doc = "SERCOM1 RX Trigger"]
    #[inline(always)]
    pub fn sercom1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::SERCOM1_RX)
    }
    #[doc = "SERCOM1 TX Trigger"]
    #[inline(always)]
    pub fn sercom1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::SERCOM1_TX)
    }
    #[doc = "SERCOM2 RX Trigger"]
    #[inline(always)]
    pub fn sercom2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::SERCOM2_RX)
    }
    #[doc = "SERCOM2 TX Trigger"]
    #[inline(always)]
    pub fn sercom2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::SERCOM2_TX)
    }
    #[doc = "TCC0 Overflow Trigger"]
    #[inline(always)]
    pub fn tcc0_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::TCC0_OVF)
    }
    #[doc = "TCC0 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::TCC0_MC0)
    }
    #[doc = "TCC0 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::TCC0_MC1)
    }
    #[doc = "TCC0 Match/Compare 2 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::TCC0_MC2)
    }
    #[doc = "TCC0 Match/Compare 3 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc3(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::TCC0_MC3)
    }
    #[doc = "TC1 Overflow Trigger"]
    #[inline(always)]
    pub fn tc1_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::TC1_OVF)
    }
    #[doc = "TC1 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc1_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::TC1_MC0)
    }
    #[doc = "TC1 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc1_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::TC1_MC1)
    }
    #[doc = "TC2 Overflow Trigger"]
    #[inline(always)]
    pub fn tc2_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::TC2_OVF)
    }
    #[doc = "TC2 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc2_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::TC2_MC0)
    }
    #[doc = "TC2 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc2_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::TC2_MC1)
    }
    #[doc = "ADC Result Ready Trigger"]
    #[inline(always)]
    pub fn adc_resrdy(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::ADC_RESRDY)
    }
    #[doc = "DAC Empty Trigger"]
    #[inline(always)]
    pub fn dac_empty(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCSELECT_A::DAC_EMPTY)
    }
}
#[doc = "Field `TRIGACT` reader - Trigger Action"]
pub type TRIGACT_R = crate::FieldReader<TRIGACTSELECT_A>;
#[doc = "Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGACTSELECT_A {
    #[doc = "0: One trigger required for each block transfer"]
    BLOCK = 0,
    #[doc = "2: One trigger required for each beat transfer"]
    BEAT = 2,
    #[doc = "3: One trigger required for each transaction"]
    TRANSACTION = 3,
}
impl From<TRIGACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGACTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGACTSELECT_A {
    type Ux = u8;
}
impl TRIGACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRIGACTSELECT_A> {
        match self.bits {
            0 => Some(TRIGACTSELECT_A::BLOCK),
            2 => Some(TRIGACTSELECT_A::BEAT),
            3 => Some(TRIGACTSELECT_A::TRANSACTION),
            _ => None,
        }
    }
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == TRIGACTSELECT_A::BLOCK
    }
    #[doc = "One trigger required for each beat transfer"]
    #[inline(always)]
    pub fn is_beat(&self) -> bool {
        *self == TRIGACTSELECT_A::BEAT
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        *self == TRIGACTSELECT_A::TRANSACTION
    }
}
#[doc = "Field `TRIGACT` writer - Trigger Action"]
pub type TRIGACT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, TRIGACTSELECT_A>;
impl<'a, REG, const O: u8> TRIGACT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGACTSELECT_A::BLOCK)
    }
    #[doc = "One trigger required for each beat transfer"]
    #[inline(always)]
    pub fn beat(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGACTSELECT_A::BEAT)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGACTSELECT_A::TRANSACTION)
    }
}
#[doc = "Field `CMD` reader - Software Command"]
pub type CMD_R = crate::FieldReader<CMDSELECT_A>;
#[doc = "Software Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDSELECT_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: Channel suspend operation"]
    SUSPEND = 1,
    #[doc = "2: Channel resume operation"]
    RESUME = 2,
}
impl From<CMDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDSELECT_A {
    type Ux = u8;
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMDSELECT_A> {
        match self.bits {
            0 => Some(CMDSELECT_A::NOACT),
            1 => Some(CMDSELECT_A::SUSPEND),
            2 => Some(CMDSELECT_A::RESUME),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == CMDSELECT_A::NOACT
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == CMDSELECT_A::SUSPEND
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == CMDSELECT_A::RESUME
    }
}
#[doc = "Field `CMD` writer - Software Command"]
pub type CMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CMDSELECT_A>;
impl<'a, REG, const O: u8> CMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::NOACT)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::RESUME)
    }
}
impl R {
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&self) -> EVIE_R {
        EVIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EVOE_R {
        EVOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Channel Arbitration Level"]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TRIGSRC_R {
        TRIGSRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&self) -> TRIGACT_R {
        TRIGACT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact(&mut self) -> EVACT_W<CHCTRLB_SPEC, 0> {
        EVACT_W::new(self)
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evie(&mut self) -> EVIE_W<CHCTRLB_SPEC, 3> {
        EVIE_W::new(self)
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evoe(&mut self) -> EVOE_W<CHCTRLB_SPEC, 4> {
        EVOE_W::new(self)
    }
    #[doc = "Bits 5:6 - Channel Arbitration Level"]
    #[inline(always)]
    #[must_use]
    pub fn lvl(&mut self) -> LVL_W<CHCTRLB_SPEC, 5> {
        LVL_W::new(self)
    }
    #[doc = "Bits 8:12 - Trigger Source"]
    #[inline(always)]
    #[must_use]
    pub fn trigsrc(&mut self) -> TRIGSRC_W<CHCTRLB_SPEC, 8> {
        TRIGSRC_W::new(self)
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline(always)]
    #[must_use]
    pub fn trigact(&mut self) -> TRIGACT_W<CHCTRLB_SPEC, 22> {
        TRIGACT_W::new(self)
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<CHCTRLB_SPEC, 24> {
        CMD_W::new(self)
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
#[doc = "Channel Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTRLB_SPEC;
impl crate::RegisterSpec for CHCTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctrlb::R`](R) reader structure"]
impl crate::Readable for CHCTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctrlb::W`](W) writer structure"]
impl crate::Writable for CHCTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTRLB to value 0"]
impl crate::Resettable for CHCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
