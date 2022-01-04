#[doc = "Register `CHCTRLB` reader"]
pub struct R(crate::R<CHCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTRLB` writer"]
pub struct W(crate::W<CHCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CHCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Event Input Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVACT_A {
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
impl From<EVACT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EVACT` reader - Event Input Action"]
pub struct EVACT_R(crate::FieldReader<u8, EVACT_A>);
impl EVACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EVACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EVACT_A> {
        match self.bits {
            0 => Some(EVACT_A::NOACT),
            1 => Some(EVACT_A::TRIG),
            2 => Some(EVACT_A::CTRIG),
            3 => Some(EVACT_A::CBLOCK),
            4 => Some(EVACT_A::SUSPEND),
            5 => Some(EVACT_A::RESUME),
            6 => Some(EVACT_A::SSKIP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        **self == EVACT_A::NOACT
    }
    #[doc = "Checks if the value of the field is `TRIG`"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        **self == EVACT_A::TRIG
    }
    #[doc = "Checks if the value of the field is `CTRIG`"]
    #[inline(always)]
    pub fn is_ctrig(&self) -> bool {
        **self == EVACT_A::CTRIG
    }
    #[doc = "Checks if the value of the field is `CBLOCK`"]
    #[inline(always)]
    pub fn is_cblock(&self) -> bool {
        **self == EVACT_A::CBLOCK
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        **self == EVACT_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        **self == EVACT_A::RESUME
    }
    #[doc = "Checks if the value of the field is `SSKIP`"]
    #[inline(always)]
    pub fn is_sskip(&self) -> bool {
        **self == EVACT_A::SSKIP
    }
}
impl core::ops::Deref for EVACT_R {
    type Target = crate::FieldReader<u8, EVACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVACT` writer - Event Input Action"]
pub struct EVACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EVACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVACT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut W {
        self.variant(EVACT_A::NOACT)
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut W {
        self.variant(EVACT_A::TRIG)
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn ctrig(self) -> &'a mut W {
        self.variant(EVACT_A::CTRIG)
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn cblock(self) -> &'a mut W {
        self.variant(EVACT_A::CBLOCK)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(EVACT_A::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(EVACT_A::RESUME)
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn sskip(self) -> &'a mut W {
        self.variant(EVACT_A::SSKIP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `EVIE` reader - Channel Event Input Enable"]
pub struct EVIE_R(crate::FieldReader<bool, bool>);
impl EVIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVIE` writer - Channel Event Input Enable"]
pub struct EVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EVIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `EVOE` reader - Channel Event Output Enable"]
pub struct EVOE_R(crate::FieldReader<bool, bool>);
impl EVOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVOE` writer - Channel Event Output Enable"]
pub struct EVOE_W<'a> {
    w: &'a mut W,
}
impl<'a> EVOE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Channel Arbitration Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVL_A {
    #[doc = "0: Channel Priority Level 0"]
    LVL0 = 0,
    #[doc = "1: Channel Priority Level 1"]
    LVL1 = 1,
    #[doc = "2: Channel Priority Level 2"]
    LVL2 = 2,
    #[doc = "3: Channel Priority Level 3"]
    LVL3 = 3,
}
impl From<LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LVL` reader - Channel Arbitration Level"]
pub struct LVL_R(crate::FieldReader<u8, LVL_A>);
impl LVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LVL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVL_A {
        match self.bits {
            0 => LVL_A::LVL0,
            1 => LVL_A::LVL1,
            2 => LVL_A::LVL2,
            3 => LVL_A::LVL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LVL0`"]
    #[inline(always)]
    pub fn is_lvl0(&self) -> bool {
        **self == LVL_A::LVL0
    }
    #[doc = "Checks if the value of the field is `LVL1`"]
    #[inline(always)]
    pub fn is_lvl1(&self) -> bool {
        **self == LVL_A::LVL1
    }
    #[doc = "Checks if the value of the field is `LVL2`"]
    #[inline(always)]
    pub fn is_lvl2(&self) -> bool {
        **self == LVL_A::LVL2
    }
    #[doc = "Checks if the value of the field is `LVL3`"]
    #[inline(always)]
    pub fn is_lvl3(&self) -> bool {
        **self == LVL_A::LVL3
    }
}
impl core::ops::Deref for LVL_R {
    type Target = crate::FieldReader<u8, LVL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVL` writer - Channel Arbitration Level"]
pub struct LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> LVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel Priority Level 0"]
    #[inline(always)]
    pub fn lvl0(self) -> &'a mut W {
        self.variant(LVL_A::LVL0)
    }
    #[doc = "Channel Priority Level 1"]
    #[inline(always)]
    pub fn lvl1(self) -> &'a mut W {
        self.variant(LVL_A::LVL1)
    }
    #[doc = "Channel Priority Level 2"]
    #[inline(always)]
    pub fn lvl2(self) -> &'a mut W {
        self.variant(LVL_A::LVL2)
    }
    #[doc = "Channel Priority Level 3"]
    #[inline(always)]
    pub fn lvl3(self) -> &'a mut W {
        self.variant(LVL_A::LVL3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSRC_A {
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
    #[doc = "7: SERCOM3 RX Trigger"]
    SERCOM3_RX = 7,
    #[doc = "8: SERCOM3 TX Trigger"]
    SERCOM3_TX = 8,
    #[doc = "9: SERCOM4 RX Trigger"]
    SERCOM4_RX = 9,
    #[doc = "10: SERCOM4 TX Trigger"]
    SERCOM4_TX = 10,
    #[doc = "11: SERCOM5 RX Trigger"]
    SERCOM5_RX = 11,
    #[doc = "12: SERCOM5 TX Trigger"]
    SERCOM5_TX = 12,
    #[doc = "13: TCC0 Overflow Trigger"]
    TCC0_OVF = 13,
    #[doc = "14: TCC0 Match/Compare 0 Trigger"]
    TCC0_MC0 = 14,
    #[doc = "15: TCC0 Match/Compare 1 Trigger"]
    TCC0_MC1 = 15,
    #[doc = "16: TCC0 Match/Compare 2 Trigger"]
    TCC0_MC2 = 16,
    #[doc = "17: TCC0 Match/Compare 3 Trigger"]
    TCC0_MC3 = 17,
    #[doc = "18: TCC1 Overflow Trigger"]
    TCC1_OVF = 18,
    #[doc = "19: TCC1 Match/Compare 0 Trigger"]
    TCC1_MC0 = 19,
    #[doc = "20: TCC1 Match/Compare 1 Trigger"]
    TCC1_MC1 = 20,
    #[doc = "21: TCC2 Overflow Trigger"]
    TCC2_OVF = 21,
    #[doc = "22: TCC2 Match/Compare 0 Trigger"]
    TCC2_MC0 = 22,
    #[doc = "23: TCC2 Match/Compare 1 Trigger"]
    TCC2_MC1 = 23,
    #[doc = "24: TC3 Overflow Trigger"]
    TC3_OVF = 24,
    #[doc = "25: TC3 Match/Compare 0 Trigger"]
    TC3_MC0 = 25,
    #[doc = "26: TC3 Match/Compare 1 Trigger"]
    TC3_MC1 = 26,
    #[doc = "27: TC4 Overflow Trigger"]
    TC4_OVF = 27,
    #[doc = "28: TC4 Match/Compare 0 Trigger"]
    TC4_MC0 = 28,
    #[doc = "29: TC4 Match/Compare 1 Trigger"]
    TC4_MC1 = 29,
    #[doc = "30: TC5 Overflow Trigger"]
    TC5_OVF = 30,
    #[doc = "31: TC5 Match/Compare 0 Trigger"]
    TC5_MC0 = 31,
    #[doc = "32: TC5 Match/Compare 1 Trigger"]
    TC5_MC1 = 32,
    #[doc = "33: TC6 Overflow Trigger"]
    TC6_OVF = 33,
    #[doc = "34: TC6 Match/Compare 0 Trigger"]
    TC6_MC0 = 34,
    #[doc = "35: TC6 Match/Compare 1 Trigger"]
    TC6_MC1 = 35,
    #[doc = "36: TC7 Overflow Trigger"]
    TC7_OVF = 36,
    #[doc = "37: TC7 Match/Compare 0 Trigger"]
    TC7_MC0 = 37,
    #[doc = "38: TC7 Match/Compare 1 Trigger"]
    TC7_MC1 = 38,
    #[doc = "39: ADC Result Ready Trigger"]
    ADC_RESRDY = 39,
    #[doc = "40: DAC Empty Trigger"]
    DAC_EMPTY = 40,
    #[doc = "41: I2S RX 0 Trigger"]
    I2S_RX_0 = 41,
    #[doc = "42: I2S RX 1 Trigger"]
    I2S_RX_1 = 42,
    #[doc = "43: I2S TX 0 Trigger"]
    I2S_TX_0 = 43,
    #[doc = "44: I2S TX 1 Trigger"]
    I2S_TX_1 = 44,
    #[doc = "45: TCC3 Overflow Trigger"]
    TCC3_OVF = 45,
    #[doc = "46: TCC3 Match/Compare 0 Trigger"]
    TCC3_MC0 = 46,
    #[doc = "47: TCC3 Match/Compare 1 Trigger"]
    TCC3_MC1 = 47,
    #[doc = "48: Match/Compare 2 Trigger"]
    TCC3_MC2 = 48,
    #[doc = "49: Match/Compare 3 Trigger"]
    TCC3_MC3 = 49,
}
impl From<TRIGSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGSRC` reader - Trigger Source"]
pub struct TRIGSRC_R(crate::FieldReader<u8, TRIGSRC_A>);
impl TRIGSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRIGSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGSRC_A> {
        match self.bits {
            0 => Some(TRIGSRC_A::DISABLE),
            1 => Some(TRIGSRC_A::SERCOM0_RX),
            2 => Some(TRIGSRC_A::SERCOM0_TX),
            3 => Some(TRIGSRC_A::SERCOM1_RX),
            4 => Some(TRIGSRC_A::SERCOM1_TX),
            5 => Some(TRIGSRC_A::SERCOM2_RX),
            6 => Some(TRIGSRC_A::SERCOM2_TX),
            7 => Some(TRIGSRC_A::SERCOM3_RX),
            8 => Some(TRIGSRC_A::SERCOM3_TX),
            9 => Some(TRIGSRC_A::SERCOM4_RX),
            10 => Some(TRIGSRC_A::SERCOM4_TX),
            11 => Some(TRIGSRC_A::SERCOM5_RX),
            12 => Some(TRIGSRC_A::SERCOM5_TX),
            13 => Some(TRIGSRC_A::TCC0_OVF),
            14 => Some(TRIGSRC_A::TCC0_MC0),
            15 => Some(TRIGSRC_A::TCC0_MC1),
            16 => Some(TRIGSRC_A::TCC0_MC2),
            17 => Some(TRIGSRC_A::TCC0_MC3),
            18 => Some(TRIGSRC_A::TCC1_OVF),
            19 => Some(TRIGSRC_A::TCC1_MC0),
            20 => Some(TRIGSRC_A::TCC1_MC1),
            21 => Some(TRIGSRC_A::TCC2_OVF),
            22 => Some(TRIGSRC_A::TCC2_MC0),
            23 => Some(TRIGSRC_A::TCC2_MC1),
            24 => Some(TRIGSRC_A::TC3_OVF),
            25 => Some(TRIGSRC_A::TC3_MC0),
            26 => Some(TRIGSRC_A::TC3_MC1),
            27 => Some(TRIGSRC_A::TC4_OVF),
            28 => Some(TRIGSRC_A::TC4_MC0),
            29 => Some(TRIGSRC_A::TC4_MC1),
            30 => Some(TRIGSRC_A::TC5_OVF),
            31 => Some(TRIGSRC_A::TC5_MC0),
            32 => Some(TRIGSRC_A::TC5_MC1),
            33 => Some(TRIGSRC_A::TC6_OVF),
            34 => Some(TRIGSRC_A::TC6_MC0),
            35 => Some(TRIGSRC_A::TC6_MC1),
            36 => Some(TRIGSRC_A::TC7_OVF),
            37 => Some(TRIGSRC_A::TC7_MC0),
            38 => Some(TRIGSRC_A::TC7_MC1),
            39 => Some(TRIGSRC_A::ADC_RESRDY),
            40 => Some(TRIGSRC_A::DAC_EMPTY),
            41 => Some(TRIGSRC_A::I2S_RX_0),
            42 => Some(TRIGSRC_A::I2S_RX_1),
            43 => Some(TRIGSRC_A::I2S_TX_0),
            44 => Some(TRIGSRC_A::I2S_TX_1),
            45 => Some(TRIGSRC_A::TCC3_OVF),
            46 => Some(TRIGSRC_A::TCC3_MC0),
            47 => Some(TRIGSRC_A::TCC3_MC1),
            48 => Some(TRIGSRC_A::TCC3_MC2),
            49 => Some(TRIGSRC_A::TCC3_MC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TRIGSRC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `SERCOM0_RX`"]
    #[inline(always)]
    pub fn is_sercom0_rx(&self) -> bool {
        **self == TRIGSRC_A::SERCOM0_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM0_TX`"]
    #[inline(always)]
    pub fn is_sercom0_tx(&self) -> bool {
        **self == TRIGSRC_A::SERCOM0_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM1_RX`"]
    #[inline(always)]
    pub fn is_sercom1_rx(&self) -> bool {
        **self == TRIGSRC_A::SERCOM1_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM1_TX`"]
    #[inline(always)]
    pub fn is_sercom1_tx(&self) -> bool {
        **self == TRIGSRC_A::SERCOM1_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM2_RX`"]
    #[inline(always)]
    pub fn is_sercom2_rx(&self) -> bool {
        **self == TRIGSRC_A::SERCOM2_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM2_TX`"]
    #[inline(always)]
    pub fn is_sercom2_tx(&self) -> bool {
        **self == TRIGSRC_A::SERCOM2_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM3_RX`"]
    #[inline(always)]
    pub fn is_sercom3_rx(&self) -> bool {
        **self == TRIGSRC_A::SERCOM3_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM3_TX`"]
    #[inline(always)]
    pub fn is_sercom3_tx(&self) -> bool {
        **self == TRIGSRC_A::SERCOM3_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM4_RX`"]
    #[inline(always)]
    pub fn is_sercom4_rx(&self) -> bool {
        **self == TRIGSRC_A::SERCOM4_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM4_TX`"]
    #[inline(always)]
    pub fn is_sercom4_tx(&self) -> bool {
        **self == TRIGSRC_A::SERCOM4_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM5_RX`"]
    #[inline(always)]
    pub fn is_sercom5_rx(&self) -> bool {
        **self == TRIGSRC_A::SERCOM5_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM5_TX`"]
    #[inline(always)]
    pub fn is_sercom5_tx(&self) -> bool {
        **self == TRIGSRC_A::SERCOM5_TX
    }
    #[doc = "Checks if the value of the field is `TCC0_OVF`"]
    #[inline(always)]
    pub fn is_tcc0_ovf(&self) -> bool {
        **self == TRIGSRC_A::TCC0_OVF
    }
    #[doc = "Checks if the value of the field is `TCC0_MC0`"]
    #[inline(always)]
    pub fn is_tcc0_mc0(&self) -> bool {
        **self == TRIGSRC_A::TCC0_MC0
    }
    #[doc = "Checks if the value of the field is `TCC0_MC1`"]
    #[inline(always)]
    pub fn is_tcc0_mc1(&self) -> bool {
        **self == TRIGSRC_A::TCC0_MC1
    }
    #[doc = "Checks if the value of the field is `TCC0_MC2`"]
    #[inline(always)]
    pub fn is_tcc0_mc2(&self) -> bool {
        **self == TRIGSRC_A::TCC0_MC2
    }
    #[doc = "Checks if the value of the field is `TCC0_MC3`"]
    #[inline(always)]
    pub fn is_tcc0_mc3(&self) -> bool {
        **self == TRIGSRC_A::TCC0_MC3
    }
    #[doc = "Checks if the value of the field is `TCC1_OVF`"]
    #[inline(always)]
    pub fn is_tcc1_ovf(&self) -> bool {
        **self == TRIGSRC_A::TCC1_OVF
    }
    #[doc = "Checks if the value of the field is `TCC1_MC0`"]
    #[inline(always)]
    pub fn is_tcc1_mc0(&self) -> bool {
        **self == TRIGSRC_A::TCC1_MC0
    }
    #[doc = "Checks if the value of the field is `TCC1_MC1`"]
    #[inline(always)]
    pub fn is_tcc1_mc1(&self) -> bool {
        **self == TRIGSRC_A::TCC1_MC1
    }
    #[doc = "Checks if the value of the field is `TCC2_OVF`"]
    #[inline(always)]
    pub fn is_tcc2_ovf(&self) -> bool {
        **self == TRIGSRC_A::TCC2_OVF
    }
    #[doc = "Checks if the value of the field is `TCC2_MC0`"]
    #[inline(always)]
    pub fn is_tcc2_mc0(&self) -> bool {
        **self == TRIGSRC_A::TCC2_MC0
    }
    #[doc = "Checks if the value of the field is `TCC2_MC1`"]
    #[inline(always)]
    pub fn is_tcc2_mc1(&self) -> bool {
        **self == TRIGSRC_A::TCC2_MC1
    }
    #[doc = "Checks if the value of the field is `TC3_OVF`"]
    #[inline(always)]
    pub fn is_tc3_ovf(&self) -> bool {
        **self == TRIGSRC_A::TC3_OVF
    }
    #[doc = "Checks if the value of the field is `TC3_MC0`"]
    #[inline(always)]
    pub fn is_tc3_mc0(&self) -> bool {
        **self == TRIGSRC_A::TC3_MC0
    }
    #[doc = "Checks if the value of the field is `TC3_MC1`"]
    #[inline(always)]
    pub fn is_tc3_mc1(&self) -> bool {
        **self == TRIGSRC_A::TC3_MC1
    }
    #[doc = "Checks if the value of the field is `TC4_OVF`"]
    #[inline(always)]
    pub fn is_tc4_ovf(&self) -> bool {
        **self == TRIGSRC_A::TC4_OVF
    }
    #[doc = "Checks if the value of the field is `TC4_MC0`"]
    #[inline(always)]
    pub fn is_tc4_mc0(&self) -> bool {
        **self == TRIGSRC_A::TC4_MC0
    }
    #[doc = "Checks if the value of the field is `TC4_MC1`"]
    #[inline(always)]
    pub fn is_tc4_mc1(&self) -> bool {
        **self == TRIGSRC_A::TC4_MC1
    }
    #[doc = "Checks if the value of the field is `TC5_OVF`"]
    #[inline(always)]
    pub fn is_tc5_ovf(&self) -> bool {
        **self == TRIGSRC_A::TC5_OVF
    }
    #[doc = "Checks if the value of the field is `TC5_MC0`"]
    #[inline(always)]
    pub fn is_tc5_mc0(&self) -> bool {
        **self == TRIGSRC_A::TC5_MC0
    }
    #[doc = "Checks if the value of the field is `TC5_MC1`"]
    #[inline(always)]
    pub fn is_tc5_mc1(&self) -> bool {
        **self == TRIGSRC_A::TC5_MC1
    }
    #[doc = "Checks if the value of the field is `TC6_OVF`"]
    #[inline(always)]
    pub fn is_tc6_ovf(&self) -> bool {
        **self == TRIGSRC_A::TC6_OVF
    }
    #[doc = "Checks if the value of the field is `TC6_MC0`"]
    #[inline(always)]
    pub fn is_tc6_mc0(&self) -> bool {
        **self == TRIGSRC_A::TC6_MC0
    }
    #[doc = "Checks if the value of the field is `TC6_MC1`"]
    #[inline(always)]
    pub fn is_tc6_mc1(&self) -> bool {
        **self == TRIGSRC_A::TC6_MC1
    }
    #[doc = "Checks if the value of the field is `TC7_OVF`"]
    #[inline(always)]
    pub fn is_tc7_ovf(&self) -> bool {
        **self == TRIGSRC_A::TC7_OVF
    }
    #[doc = "Checks if the value of the field is `TC7_MC0`"]
    #[inline(always)]
    pub fn is_tc7_mc0(&self) -> bool {
        **self == TRIGSRC_A::TC7_MC0
    }
    #[doc = "Checks if the value of the field is `TC7_MC1`"]
    #[inline(always)]
    pub fn is_tc7_mc1(&self) -> bool {
        **self == TRIGSRC_A::TC7_MC1
    }
    #[doc = "Checks if the value of the field is `ADC_RESRDY`"]
    #[inline(always)]
    pub fn is_adc_resrdy(&self) -> bool {
        **self == TRIGSRC_A::ADC_RESRDY
    }
    #[doc = "Checks if the value of the field is `DAC_EMPTY`"]
    #[inline(always)]
    pub fn is_dac_empty(&self) -> bool {
        **self == TRIGSRC_A::DAC_EMPTY
    }
    #[doc = "Checks if the value of the field is `I2S_RX_0`"]
    #[inline(always)]
    pub fn is_i2s_rx_0(&self) -> bool {
        **self == TRIGSRC_A::I2S_RX_0
    }
    #[doc = "Checks if the value of the field is `I2S_RX_1`"]
    #[inline(always)]
    pub fn is_i2s_rx_1(&self) -> bool {
        **self == TRIGSRC_A::I2S_RX_1
    }
    #[doc = "Checks if the value of the field is `I2S_TX_0`"]
    #[inline(always)]
    pub fn is_i2s_tx_0(&self) -> bool {
        **self == TRIGSRC_A::I2S_TX_0
    }
    #[doc = "Checks if the value of the field is `I2S_TX_1`"]
    #[inline(always)]
    pub fn is_i2s_tx_1(&self) -> bool {
        **self == TRIGSRC_A::I2S_TX_1
    }
    #[doc = "Checks if the value of the field is `TCC3_OVF`"]
    #[inline(always)]
    pub fn is_tcc3_ovf(&self) -> bool {
        **self == TRIGSRC_A::TCC3_OVF
    }
    #[doc = "Checks if the value of the field is `TCC3_MC0`"]
    #[inline(always)]
    pub fn is_tcc3_mc0(&self) -> bool {
        **self == TRIGSRC_A::TCC3_MC0
    }
    #[doc = "Checks if the value of the field is `TCC3_MC1`"]
    #[inline(always)]
    pub fn is_tcc3_mc1(&self) -> bool {
        **self == TRIGSRC_A::TCC3_MC1
    }
    #[doc = "Checks if the value of the field is `TCC3_MC2`"]
    #[inline(always)]
    pub fn is_tcc3_mc2(&self) -> bool {
        **self == TRIGSRC_A::TCC3_MC2
    }
    #[doc = "Checks if the value of the field is `TCC3_MC3`"]
    #[inline(always)]
    pub fn is_tcc3_mc3(&self) -> bool {
        **self == TRIGSRC_A::TCC3_MC3
    }
}
impl core::ops::Deref for TRIGSRC_R {
    type Target = crate::FieldReader<u8, TRIGSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGSRC` writer - Trigger Source"]
pub struct TRIGSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DISABLE)
    }
    #[doc = "SERCOM0 RX Trigger"]
    #[inline(always)]
    pub fn sercom0_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM0_RX)
    }
    #[doc = "SERCOM0 TX Trigger"]
    #[inline(always)]
    pub fn sercom0_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM0_TX)
    }
    #[doc = "SERCOM1 RX Trigger"]
    #[inline(always)]
    pub fn sercom1_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM1_RX)
    }
    #[doc = "SERCOM1 TX Trigger"]
    #[inline(always)]
    pub fn sercom1_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM1_TX)
    }
    #[doc = "SERCOM2 RX Trigger"]
    #[inline(always)]
    pub fn sercom2_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM2_RX)
    }
    #[doc = "SERCOM2 TX Trigger"]
    #[inline(always)]
    pub fn sercom2_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM2_TX)
    }
    #[doc = "SERCOM3 RX Trigger"]
    #[inline(always)]
    pub fn sercom3_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM3_RX)
    }
    #[doc = "SERCOM3 TX Trigger"]
    #[inline(always)]
    pub fn sercom3_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM3_TX)
    }
    #[doc = "SERCOM4 RX Trigger"]
    #[inline(always)]
    pub fn sercom4_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM4_RX)
    }
    #[doc = "SERCOM4 TX Trigger"]
    #[inline(always)]
    pub fn sercom4_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM4_TX)
    }
    #[doc = "SERCOM5 RX Trigger"]
    #[inline(always)]
    pub fn sercom5_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM5_RX)
    }
    #[doc = "SERCOM5 TX Trigger"]
    #[inline(always)]
    pub fn sercom5_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM5_TX)
    }
    #[doc = "TCC0 Overflow Trigger"]
    #[inline(always)]
    pub fn tcc0_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC0_OVF)
    }
    #[doc = "TCC0 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC0_MC0)
    }
    #[doc = "TCC0 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC0_MC1)
    }
    #[doc = "TCC0 Match/Compare 2 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc2(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC0_MC2)
    }
    #[doc = "TCC0 Match/Compare 3 Trigger"]
    #[inline(always)]
    pub fn tcc0_mc3(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC0_MC3)
    }
    #[doc = "TCC1 Overflow Trigger"]
    #[inline(always)]
    pub fn tcc1_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC1_OVF)
    }
    #[doc = "TCC1 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tcc1_mc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC1_MC0)
    }
    #[doc = "TCC1 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tcc1_mc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC1_MC1)
    }
    #[doc = "TCC2 Overflow Trigger"]
    #[inline(always)]
    pub fn tcc2_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC2_OVF)
    }
    #[doc = "TCC2 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tcc2_mc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC2_MC0)
    }
    #[doc = "TCC2 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tcc2_mc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC2_MC1)
    }
    #[doc = "TC3 Overflow Trigger"]
    #[inline(always)]
    pub fn tc3_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC3_OVF)
    }
    #[doc = "TC3 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc3_mc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC3_MC0)
    }
    #[doc = "TC3 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc3_mc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC3_MC1)
    }
    #[doc = "TC4 Overflow Trigger"]
    #[inline(always)]
    pub fn tc4_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC4_OVF)
    }
    #[doc = "TC4 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc4_mc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC4_MC0)
    }
    #[doc = "TC4 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc4_mc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC4_MC1)
    }
    #[doc = "TC5 Overflow Trigger"]
    #[inline(always)]
    pub fn tc5_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC5_OVF)
    }
    #[doc = "TC5 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc5_mc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC5_MC0)
    }
    #[doc = "TC5 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc5_mc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC5_MC1)
    }
    #[doc = "TC6 Overflow Trigger"]
    #[inline(always)]
    pub fn tc6_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC6_OVF)
    }
    #[doc = "TC6 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc6_mc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC6_MC0)
    }
    #[doc = "TC6 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc6_mc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC6_MC1)
    }
    #[doc = "TC7 Overflow Trigger"]
    #[inline(always)]
    pub fn tc7_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC7_OVF)
    }
    #[doc = "TC7 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc7_mc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC7_MC0)
    }
    #[doc = "TC7 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc7_mc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC7_MC1)
    }
    #[doc = "ADC Result Ready Trigger"]
    #[inline(always)]
    pub fn adc_resrdy(self) -> &'a mut W {
        self.variant(TRIGSRC_A::ADC_RESRDY)
    }
    #[doc = "DAC Empty Trigger"]
    #[inline(always)]
    pub fn dac_empty(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DAC_EMPTY)
    }
    #[doc = "I2S RX 0 Trigger"]
    #[inline(always)]
    pub fn i2s_rx_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::I2S_RX_0)
    }
    #[doc = "I2S RX 1 Trigger"]
    #[inline(always)]
    pub fn i2s_rx_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::I2S_RX_1)
    }
    #[doc = "I2S TX 0 Trigger"]
    #[inline(always)]
    pub fn i2s_tx_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::I2S_TX_0)
    }
    #[doc = "I2S TX 1 Trigger"]
    #[inline(always)]
    pub fn i2s_tx_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::I2S_TX_1)
    }
    #[doc = "TCC3 Overflow Trigger"]
    #[inline(always)]
    pub fn tcc3_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC3_OVF)
    }
    #[doc = "TCC3 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tcc3_mc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC3_MC0)
    }
    #[doc = "TCC3 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tcc3_mc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC3_MC1)
    }
    #[doc = "Match/Compare 2 Trigger"]
    #[inline(always)]
    pub fn tcc3_mc2(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC3_MC2)
    }
    #[doc = "Match/Compare 3 Trigger"]
    #[inline(always)]
    pub fn tcc3_mc3(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC3_MC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGACT_A {
    #[doc = "0: One trigger required for each block transfer"]
    BLOCK = 0,
    #[doc = "2: One trigger required for each beat transfer"]
    BEAT = 2,
    #[doc = "3: One trigger required for each transaction"]
    TRANSACTION = 3,
}
impl From<TRIGACT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGACT` reader - Trigger Action"]
pub struct TRIGACT_R(crate::FieldReader<u8, TRIGACT_A>);
impl TRIGACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRIGACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGACT_A> {
        match self.bits {
            0 => Some(TRIGACT_A::BLOCK),
            2 => Some(TRIGACT_A::BEAT),
            3 => Some(TRIGACT_A::TRANSACTION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        **self == TRIGACT_A::BLOCK
    }
    #[doc = "Checks if the value of the field is `BEAT`"]
    #[inline(always)]
    pub fn is_beat(&self) -> bool {
        **self == TRIGACT_A::BEAT
    }
    #[doc = "Checks if the value of the field is `TRANSACTION`"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        **self == TRIGACT_A::TRANSACTION
    }
}
impl core::ops::Deref for TRIGACT_R {
    type Target = crate::FieldReader<u8, TRIGACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGACT` writer - Trigger Action"]
pub struct TRIGACT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGACT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn block(self) -> &'a mut W {
        self.variant(TRIGACT_A::BLOCK)
    }
    #[doc = "One trigger required for each beat transfer"]
    #[inline(always)]
    pub fn beat(self) -> &'a mut W {
        self.variant(TRIGACT_A::BEAT)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut W {
        self.variant(TRIGACT_A::TRANSACTION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Software Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: Channel suspend operation"]
    SUSPEND = 1,
    #[doc = "2: Channel resume operation"]
    RESUME = 2,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD` reader - Software Command"]
pub struct CMD_R(crate::FieldReader<u8, CMD_A>);
impl CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            0 => Some(CMD_A::NOACT),
            1 => Some(CMD_A::SUSPEND),
            2 => Some(CMD_A::RESUME),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        **self == CMD_A::NOACT
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        **self == CMD_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        **self == CMD_A::RESUME
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<u8, CMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - Software Command"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut W {
        self.variant(CMD_A::NOACT)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(CMD_A::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(CMD_A::RESUME)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&self) -> EVIE_R {
        EVIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EVOE_R {
        EVOE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Channel Arbitration Level"]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TRIGSRC_R {
        TRIGSRC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&self) -> TRIGACT_R {
        TRIGACT_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline(always)]
    pub fn evact(&mut self) -> EVACT_W {
        EVACT_W { w: self }
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&mut self) -> EVIE_W {
        EVIE_W { w: self }
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&mut self) -> EVOE_W {
        EVOE_W { w: self }
    }
    #[doc = "Bits 5:6 - Channel Arbitration Level"]
    #[inline(always)]
    pub fn lvl(&mut self) -> LVL_W {
        LVL_W { w: self }
    }
    #[doc = "Bits 8:13 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&mut self) -> TRIGSRC_W {
        TRIGSRC_W { w: self }
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&mut self) -> TRIGACT_W {
        TRIGACT_W { w: self }
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrlb](index.html) module"]
pub struct CHCTRLB_SPEC;
impl crate::RegisterSpec for CHCTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctrlb::R](R) reader structure"]
impl crate::Readable for CHCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctrlb::W](W) writer structure"]
impl crate::Writable for CHCTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCTRLB to value 0"]
impl crate::Resettable for CHCTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
