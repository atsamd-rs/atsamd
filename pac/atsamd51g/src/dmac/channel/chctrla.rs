#[doc = "Reader of register CHCTRLA"]
pub type R = crate::R<u32, super::CHCTRLA>;
#[doc = "Writer for register CHCTRLA"]
pub type W = crate::W<u32, super::CHCTRLA>;
#[doc = "Register CHCTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CHCTRLA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNSTDBY`"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSRC_A {
    #[doc = "0: Only software/event triggers"]
    DISABLE = 0,
    #[doc = "1: DMA RTC timestamp trigger"]
    RTC_TIMESTAMP = 1,
    #[doc = "2: DMAC ID for DCC0 register"]
    DSU_DCC0 = 2,
    #[doc = "3: DMAC ID for DCC1 register"]
    DSU_DCC1 = 3,
    #[doc = "4: Index of DMA RX trigger"]
    SERCOM0_RX = 4,
    #[doc = "5: Index of DMA TX trigger"]
    SERCOM0_TX = 5,
    #[doc = "6: Index of DMA RX trigger"]
    SERCOM1_RX = 6,
    #[doc = "7: Index of DMA TX trigger"]
    SERCOM1_TX = 7,
    #[doc = "8: Index of DMA RX trigger"]
    SERCOM2_RX = 8,
    #[doc = "9: Index of DMA TX trigger"]
    SERCOM2_TX = 9,
    #[doc = "10: Index of DMA RX trigger"]
    SERCOM3_RX = 10,
    #[doc = "11: Index of DMA TX trigger"]
    SERCOM3_TX = 11,
    #[doc = "12: Index of DMA RX trigger"]
    SERCOM4_RX = 12,
    #[doc = "13: Index of DMA TX trigger"]
    SERCOM4_TX = 13,
    #[doc = "14: Index of DMA RX trigger"]
    SERCOM5_RX = 14,
    #[doc = "15: Index of DMA TX trigger"]
    SERCOM5_TX = 15,
    #[doc = "16: Index of DMA RX trigger"]
    SERCOM6_RX = 16,
    #[doc = "17: Index of DMA TX trigger"]
    SERCOM6_TX = 17,
    #[doc = "18: Index of DMA RX trigger"]
    SERCOM7_RX = 18,
    #[doc = "19: Index of DMA TX trigger"]
    SERCOM7_TX = 19,
    #[doc = "20: DMA CAN Debug Req"]
    CAN0_DEBUG = 20,
    #[doc = "21: DMA CAN Debug Req"]
    CAN1_DEBUG = 21,
    #[doc = "22: DMA overflow/underflow/retrigger trigger"]
    TCC0_OVF = 22,
    #[doc = "23: Indexes of DMA Match/Compare triggers"]
    TCC0_MC_0 = 23,
    #[doc = "24: Indexes of DMA Match/Compare triggers"]
    TCC0_MC_1 = 24,
    #[doc = "25: Indexes of DMA Match/Compare triggers"]
    TCC0_MC_2 = 25,
    #[doc = "26: Indexes of DMA Match/Compare triggers"]
    TCC0_MC_3 = 26,
    #[doc = "27: Indexes of DMA Match/Compare triggers"]
    TCC0_MC_4 = 27,
    #[doc = "28: Indexes of DMA Match/Compare triggers"]
    TCC0_MC_5 = 28,
    #[doc = "29: DMA overflow/underflow/retrigger trigger"]
    TCC1_OVF = 29,
    #[doc = "30: Indexes of DMA Match/Compare triggers"]
    TCC1_MC_0 = 30,
    #[doc = "31: Indexes of DMA Match/Compare triggers"]
    TCC1_MC_1 = 31,
    #[doc = "32: Indexes of DMA Match/Compare triggers"]
    TCC1_MC_2 = 32,
    #[doc = "33: Indexes of DMA Match/Compare triggers"]
    TCC1_MC_3 = 33,
    #[doc = "34: DMA overflow/underflow/retrigger trigger"]
    TCC2_OVF = 34,
    #[doc = "35: Indexes of DMA Match/Compare triggers"]
    TCC2_MC_0 = 35,
    #[doc = "36: Indexes of DMA Match/Compare triggers"]
    TCC2_MC_1 = 36,
    #[doc = "37: Indexes of DMA Match/Compare triggers"]
    TCC2_MC_2 = 37,
    #[doc = "38: DMA overflow/underflow/retrigger trigger"]
    TCC3_OVF = 38,
    #[doc = "39: Indexes of DMA Match/Compare triggers"]
    TCC3_MC_0 = 39,
    #[doc = "40: Indexes of DMA Match/Compare triggers"]
    TCC3_MC_1 = 40,
    #[doc = "41: DMA overflow/underflow/retrigger trigger"]
    TCC4_OVF = 41,
    #[doc = "42: Indexes of DMA Match/Compare triggers"]
    TCC4_MC_0 = 42,
    #[doc = "43: Indexes of DMA Match/Compare triggers"]
    TCC4_MC_1 = 43,
    #[doc = "44: Indexes of DMA Overflow trigger"]
    TC0_OVF = 44,
    #[doc = "45: Indexes of DMA Match/Compare triggers"]
    TC0_MC_0 = 45,
    #[doc = "46: Indexes of DMA Match/Compare triggers"]
    TC0_MC_1 = 46,
    #[doc = "47: Indexes of DMA Overflow trigger"]
    TC1_OVF = 47,
    #[doc = "48: Indexes of DMA Match/Compare triggers"]
    TC1_MC_0 = 48,
    #[doc = "49: Indexes of DMA Match/Compare triggers"]
    TC1_MC_1 = 49,
    #[doc = "50: Indexes of DMA Overflow trigger"]
    TC2_OVF = 50,
    #[doc = "51: Indexes of DMA Match/Compare triggers"]
    TC2_MC_0 = 51,
    #[doc = "52: Indexes of DMA Match/Compare triggers"]
    TC2_MC_1 = 52,
    #[doc = "53: Indexes of DMA Overflow trigger"]
    TC3_OVF = 53,
    #[doc = "54: Indexes of DMA Match/Compare triggers"]
    TC3_MC_0 = 54,
    #[doc = "55: Indexes of DMA Match/Compare triggers"]
    TC3_MC_1 = 55,
    #[doc = "56: Indexes of DMA Overflow trigger"]
    TC4_OVF = 56,
    #[doc = "57: Indexes of DMA Match/Compare triggers"]
    TC4_MC_0 = 57,
    #[doc = "58: Indexes of DMA Match/Compare triggers"]
    TC4_MC_1 = 58,
    #[doc = "59: Indexes of DMA Overflow trigger"]
    TC5_OVF = 59,
    #[doc = "60: Indexes of DMA Match/Compare triggers"]
    TC5_MC_0 = 60,
    #[doc = "61: Indexes of DMA Match/Compare triggers"]
    TC5_MC_1 = 61,
    #[doc = "62: Indexes of DMA Overflow trigger"]
    TC6_OVF = 62,
    #[doc = "63: Indexes of DMA Match/Compare triggers"]
    TC6_MC_0 = 63,
    #[doc = "64: Indexes of DMA Match/Compare triggers"]
    TC6_MC_1 = 64,
    #[doc = "65: Indexes of DMA Overflow trigger"]
    TC7_OVF = 65,
    #[doc = "66: Indexes of DMA Match/Compare triggers"]
    TC7_MC_0 = 66,
    #[doc = "67: Indexes of DMA Match/Compare triggers"]
    TC7_MC_1 = 67,
    #[doc = "68: index of DMA RESRDY trigger"]
    ADC0_RESRDY = 68,
    #[doc = "69: Index of DMA SEQ trigger"]
    ADC0_SEQ = 69,
    #[doc = "70: Index of DMA RESRDY trigger"]
    ADC1_RESRDY = 70,
    #[doc = "71: Index of DMA SEQ trigger"]
    ADC1_SEQ = 71,
    #[doc = "72: DMA DAC Empty Req"]
    DAC_EMPTY_0 = 72,
    #[doc = "73: DMA DAC Empty Req"]
    DAC_EMPTY_1 = 73,
    #[doc = "74: DMA DAC Result Ready Req"]
    DAC_RESRDY_0 = 74,
    #[doc = "75: DMA DAC Result Ready Req"]
    DAC_RESRDY_1 = 75,
    #[doc = "76: Indexes of DMA RX triggers"]
    I2S_RX_0 = 76,
    #[doc = "77: Indexes of DMA RX triggers"]
    I2S_RX_1 = 77,
    #[doc = "78: Indexes of DMA TX triggers"]
    I2S_TX_0 = 78,
    #[doc = "79: Indexes of DMA TX triggers"]
    I2S_TX_1 = 79,
    #[doc = "80: Indexes of PCC RX trigger"]
    PCC_RX = 80,
    #[doc = "81: DMA DATA Write trigger"]
    AES_WR = 81,
    #[doc = "82: DMA DATA Read trigger"]
    AES_RD = 82,
    #[doc = "83: Indexes of QSPI RX trigger"]
    QSPI_RX = 83,
    #[doc = "84: Indexes of QSPI TX trigger"]
    QSPI_TX = 84,
}
impl From<TRIGSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGSRC`"]
pub type TRIGSRC_R = crate::R<u8, TRIGSRC_A>;
impl TRIGSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRIGSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRIGSRC_A::DISABLE),
            1 => Val(TRIGSRC_A::RTC_TIMESTAMP),
            2 => Val(TRIGSRC_A::DSU_DCC0),
            3 => Val(TRIGSRC_A::DSU_DCC1),
            4 => Val(TRIGSRC_A::SERCOM0_RX),
            5 => Val(TRIGSRC_A::SERCOM0_TX),
            6 => Val(TRIGSRC_A::SERCOM1_RX),
            7 => Val(TRIGSRC_A::SERCOM1_TX),
            8 => Val(TRIGSRC_A::SERCOM2_RX),
            9 => Val(TRIGSRC_A::SERCOM2_TX),
            10 => Val(TRIGSRC_A::SERCOM3_RX),
            11 => Val(TRIGSRC_A::SERCOM3_TX),
            12 => Val(TRIGSRC_A::SERCOM4_RX),
            13 => Val(TRIGSRC_A::SERCOM4_TX),
            14 => Val(TRIGSRC_A::SERCOM5_RX),
            15 => Val(TRIGSRC_A::SERCOM5_TX),
            16 => Val(TRIGSRC_A::SERCOM6_RX),
            17 => Val(TRIGSRC_A::SERCOM6_TX),
            18 => Val(TRIGSRC_A::SERCOM7_RX),
            19 => Val(TRIGSRC_A::SERCOM7_TX),
            20 => Val(TRIGSRC_A::CAN0_DEBUG),
            21 => Val(TRIGSRC_A::CAN1_DEBUG),
            22 => Val(TRIGSRC_A::TCC0_OVF),
            23 => Val(TRIGSRC_A::TCC0_MC_0),
            24 => Val(TRIGSRC_A::TCC0_MC_1),
            25 => Val(TRIGSRC_A::TCC0_MC_2),
            26 => Val(TRIGSRC_A::TCC0_MC_3),
            27 => Val(TRIGSRC_A::TCC0_MC_4),
            28 => Val(TRIGSRC_A::TCC0_MC_5),
            29 => Val(TRIGSRC_A::TCC1_OVF),
            30 => Val(TRIGSRC_A::TCC1_MC_0),
            31 => Val(TRIGSRC_A::TCC1_MC_1),
            32 => Val(TRIGSRC_A::TCC1_MC_2),
            33 => Val(TRIGSRC_A::TCC1_MC_3),
            34 => Val(TRIGSRC_A::TCC2_OVF),
            35 => Val(TRIGSRC_A::TCC2_MC_0),
            36 => Val(TRIGSRC_A::TCC2_MC_1),
            37 => Val(TRIGSRC_A::TCC2_MC_2),
            38 => Val(TRIGSRC_A::TCC3_OVF),
            39 => Val(TRIGSRC_A::TCC3_MC_0),
            40 => Val(TRIGSRC_A::TCC3_MC_1),
            41 => Val(TRIGSRC_A::TCC4_OVF),
            42 => Val(TRIGSRC_A::TCC4_MC_0),
            43 => Val(TRIGSRC_A::TCC4_MC_1),
            44 => Val(TRIGSRC_A::TC0_OVF),
            45 => Val(TRIGSRC_A::TC0_MC_0),
            46 => Val(TRIGSRC_A::TC0_MC_1),
            47 => Val(TRIGSRC_A::TC1_OVF),
            48 => Val(TRIGSRC_A::TC1_MC_0),
            49 => Val(TRIGSRC_A::TC1_MC_1),
            50 => Val(TRIGSRC_A::TC2_OVF),
            51 => Val(TRIGSRC_A::TC2_MC_0),
            52 => Val(TRIGSRC_A::TC2_MC_1),
            53 => Val(TRIGSRC_A::TC3_OVF),
            54 => Val(TRIGSRC_A::TC3_MC_0),
            55 => Val(TRIGSRC_A::TC3_MC_1),
            56 => Val(TRIGSRC_A::TC4_OVF),
            57 => Val(TRIGSRC_A::TC4_MC_0),
            58 => Val(TRIGSRC_A::TC4_MC_1),
            59 => Val(TRIGSRC_A::TC5_OVF),
            60 => Val(TRIGSRC_A::TC5_MC_0),
            61 => Val(TRIGSRC_A::TC5_MC_1),
            62 => Val(TRIGSRC_A::TC6_OVF),
            63 => Val(TRIGSRC_A::TC6_MC_0),
            64 => Val(TRIGSRC_A::TC6_MC_1),
            65 => Val(TRIGSRC_A::TC7_OVF),
            66 => Val(TRIGSRC_A::TC7_MC_0),
            67 => Val(TRIGSRC_A::TC7_MC_1),
            68 => Val(TRIGSRC_A::ADC0_RESRDY),
            69 => Val(TRIGSRC_A::ADC0_SEQ),
            70 => Val(TRIGSRC_A::ADC1_RESRDY),
            71 => Val(TRIGSRC_A::ADC1_SEQ),
            72 => Val(TRIGSRC_A::DAC_EMPTY_0),
            73 => Val(TRIGSRC_A::DAC_EMPTY_1),
            74 => Val(TRIGSRC_A::DAC_RESRDY_0),
            75 => Val(TRIGSRC_A::DAC_RESRDY_1),
            76 => Val(TRIGSRC_A::I2S_RX_0),
            77 => Val(TRIGSRC_A::I2S_RX_1),
            78 => Val(TRIGSRC_A::I2S_TX_0),
            79 => Val(TRIGSRC_A::I2S_TX_1),
            80 => Val(TRIGSRC_A::PCC_RX),
            81 => Val(TRIGSRC_A::AES_WR),
            82 => Val(TRIGSRC_A::AES_RD),
            83 => Val(TRIGSRC_A::QSPI_RX),
            84 => Val(TRIGSRC_A::QSPI_TX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TRIGSRC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RTC_TIMESTAMP`"]
    #[inline(always)]
    pub fn is_rtc_timestamp(&self) -> bool {
        *self == TRIGSRC_A::RTC_TIMESTAMP
    }
    #[doc = "Checks if the value of the field is `DSU_DCC0`"]
    #[inline(always)]
    pub fn is_dsu_dcc0(&self) -> bool {
        *self == TRIGSRC_A::DSU_DCC0
    }
    #[doc = "Checks if the value of the field is `DSU_DCC1`"]
    #[inline(always)]
    pub fn is_dsu_dcc1(&self) -> bool {
        *self == TRIGSRC_A::DSU_DCC1
    }
    #[doc = "Checks if the value of the field is `SERCOM0_RX`"]
    #[inline(always)]
    pub fn is_sercom0_rx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM0_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM0_TX`"]
    #[inline(always)]
    pub fn is_sercom0_tx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM0_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM1_RX`"]
    #[inline(always)]
    pub fn is_sercom1_rx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM1_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM1_TX`"]
    #[inline(always)]
    pub fn is_sercom1_tx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM1_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM2_RX`"]
    #[inline(always)]
    pub fn is_sercom2_rx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM2_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM2_TX`"]
    #[inline(always)]
    pub fn is_sercom2_tx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM2_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM3_RX`"]
    #[inline(always)]
    pub fn is_sercom3_rx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM3_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM3_TX`"]
    #[inline(always)]
    pub fn is_sercom3_tx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM3_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM4_RX`"]
    #[inline(always)]
    pub fn is_sercom4_rx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM4_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM4_TX`"]
    #[inline(always)]
    pub fn is_sercom4_tx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM4_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM5_RX`"]
    #[inline(always)]
    pub fn is_sercom5_rx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM5_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM5_TX`"]
    #[inline(always)]
    pub fn is_sercom5_tx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM5_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM6_RX`"]
    #[inline(always)]
    pub fn is_sercom6_rx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM6_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM6_TX`"]
    #[inline(always)]
    pub fn is_sercom6_tx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM6_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM7_RX`"]
    #[inline(always)]
    pub fn is_sercom7_rx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM7_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM7_TX`"]
    #[inline(always)]
    pub fn is_sercom7_tx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM7_TX
    }
    #[doc = "Checks if the value of the field is `CAN0_DEBUG`"]
    #[inline(always)]
    pub fn is_can0_debug(&self) -> bool {
        *self == TRIGSRC_A::CAN0_DEBUG
    }
    #[doc = "Checks if the value of the field is `CAN1_DEBUG`"]
    #[inline(always)]
    pub fn is_can1_debug(&self) -> bool {
        *self == TRIGSRC_A::CAN1_DEBUG
    }
    #[doc = "Checks if the value of the field is `TCC0_OVF`"]
    #[inline(always)]
    pub fn is_tcc0_ovf(&self) -> bool {
        *self == TRIGSRC_A::TCC0_OVF
    }
    #[doc = "Checks if the value of the field is `TCC0_MC_0`"]
    #[inline(always)]
    pub fn is_tcc0_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TCC0_MC_0
    }
    #[doc = "Checks if the value of the field is `TCC0_MC_1`"]
    #[inline(always)]
    pub fn is_tcc0_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TCC0_MC_1
    }
    #[doc = "Checks if the value of the field is `TCC0_MC_2`"]
    #[inline(always)]
    pub fn is_tcc0_mc_2(&self) -> bool {
        *self == TRIGSRC_A::TCC0_MC_2
    }
    #[doc = "Checks if the value of the field is `TCC0_MC_3`"]
    #[inline(always)]
    pub fn is_tcc0_mc_3(&self) -> bool {
        *self == TRIGSRC_A::TCC0_MC_3
    }
    #[doc = "Checks if the value of the field is `TCC0_MC_4`"]
    #[inline(always)]
    pub fn is_tcc0_mc_4(&self) -> bool {
        *self == TRIGSRC_A::TCC0_MC_4
    }
    #[doc = "Checks if the value of the field is `TCC0_MC_5`"]
    #[inline(always)]
    pub fn is_tcc0_mc_5(&self) -> bool {
        *self == TRIGSRC_A::TCC0_MC_5
    }
    #[doc = "Checks if the value of the field is `TCC1_OVF`"]
    #[inline(always)]
    pub fn is_tcc1_ovf(&self) -> bool {
        *self == TRIGSRC_A::TCC1_OVF
    }
    #[doc = "Checks if the value of the field is `TCC1_MC_0`"]
    #[inline(always)]
    pub fn is_tcc1_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TCC1_MC_0
    }
    #[doc = "Checks if the value of the field is `TCC1_MC_1`"]
    #[inline(always)]
    pub fn is_tcc1_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TCC1_MC_1
    }
    #[doc = "Checks if the value of the field is `TCC1_MC_2`"]
    #[inline(always)]
    pub fn is_tcc1_mc_2(&self) -> bool {
        *self == TRIGSRC_A::TCC1_MC_2
    }
    #[doc = "Checks if the value of the field is `TCC1_MC_3`"]
    #[inline(always)]
    pub fn is_tcc1_mc_3(&self) -> bool {
        *self == TRIGSRC_A::TCC1_MC_3
    }
    #[doc = "Checks if the value of the field is `TCC2_OVF`"]
    #[inline(always)]
    pub fn is_tcc2_ovf(&self) -> bool {
        *self == TRIGSRC_A::TCC2_OVF
    }
    #[doc = "Checks if the value of the field is `TCC2_MC_0`"]
    #[inline(always)]
    pub fn is_tcc2_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TCC2_MC_0
    }
    #[doc = "Checks if the value of the field is `TCC2_MC_1`"]
    #[inline(always)]
    pub fn is_tcc2_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TCC2_MC_1
    }
    #[doc = "Checks if the value of the field is `TCC2_MC_2`"]
    #[inline(always)]
    pub fn is_tcc2_mc_2(&self) -> bool {
        *self == TRIGSRC_A::TCC2_MC_2
    }
    #[doc = "Checks if the value of the field is `TCC3_OVF`"]
    #[inline(always)]
    pub fn is_tcc3_ovf(&self) -> bool {
        *self == TRIGSRC_A::TCC3_OVF
    }
    #[doc = "Checks if the value of the field is `TCC3_MC_0`"]
    #[inline(always)]
    pub fn is_tcc3_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TCC3_MC_0
    }
    #[doc = "Checks if the value of the field is `TCC3_MC_1`"]
    #[inline(always)]
    pub fn is_tcc3_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TCC3_MC_1
    }
    #[doc = "Checks if the value of the field is `TCC4_OVF`"]
    #[inline(always)]
    pub fn is_tcc4_ovf(&self) -> bool {
        *self == TRIGSRC_A::TCC4_OVF
    }
    #[doc = "Checks if the value of the field is `TCC4_MC_0`"]
    #[inline(always)]
    pub fn is_tcc4_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TCC4_MC_0
    }
    #[doc = "Checks if the value of the field is `TCC4_MC_1`"]
    #[inline(always)]
    pub fn is_tcc4_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TCC4_MC_1
    }
    #[doc = "Checks if the value of the field is `TC0_OVF`"]
    #[inline(always)]
    pub fn is_tc0_ovf(&self) -> bool {
        *self == TRIGSRC_A::TC0_OVF
    }
    #[doc = "Checks if the value of the field is `TC0_MC_0`"]
    #[inline(always)]
    pub fn is_tc0_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TC0_MC_0
    }
    #[doc = "Checks if the value of the field is `TC0_MC_1`"]
    #[inline(always)]
    pub fn is_tc0_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TC0_MC_1
    }
    #[doc = "Checks if the value of the field is `TC1_OVF`"]
    #[inline(always)]
    pub fn is_tc1_ovf(&self) -> bool {
        *self == TRIGSRC_A::TC1_OVF
    }
    #[doc = "Checks if the value of the field is `TC1_MC_0`"]
    #[inline(always)]
    pub fn is_tc1_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TC1_MC_0
    }
    #[doc = "Checks if the value of the field is `TC1_MC_1`"]
    #[inline(always)]
    pub fn is_tc1_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TC1_MC_1
    }
    #[doc = "Checks if the value of the field is `TC2_OVF`"]
    #[inline(always)]
    pub fn is_tc2_ovf(&self) -> bool {
        *self == TRIGSRC_A::TC2_OVF
    }
    #[doc = "Checks if the value of the field is `TC2_MC_0`"]
    #[inline(always)]
    pub fn is_tc2_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TC2_MC_0
    }
    #[doc = "Checks if the value of the field is `TC2_MC_1`"]
    #[inline(always)]
    pub fn is_tc2_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TC2_MC_1
    }
    #[doc = "Checks if the value of the field is `TC3_OVF`"]
    #[inline(always)]
    pub fn is_tc3_ovf(&self) -> bool {
        *self == TRIGSRC_A::TC3_OVF
    }
    #[doc = "Checks if the value of the field is `TC3_MC_0`"]
    #[inline(always)]
    pub fn is_tc3_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TC3_MC_0
    }
    #[doc = "Checks if the value of the field is `TC3_MC_1`"]
    #[inline(always)]
    pub fn is_tc3_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TC3_MC_1
    }
    #[doc = "Checks if the value of the field is `TC4_OVF`"]
    #[inline(always)]
    pub fn is_tc4_ovf(&self) -> bool {
        *self == TRIGSRC_A::TC4_OVF
    }
    #[doc = "Checks if the value of the field is `TC4_MC_0`"]
    #[inline(always)]
    pub fn is_tc4_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TC4_MC_0
    }
    #[doc = "Checks if the value of the field is `TC4_MC_1`"]
    #[inline(always)]
    pub fn is_tc4_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TC4_MC_1
    }
    #[doc = "Checks if the value of the field is `TC5_OVF`"]
    #[inline(always)]
    pub fn is_tc5_ovf(&self) -> bool {
        *self == TRIGSRC_A::TC5_OVF
    }
    #[doc = "Checks if the value of the field is `TC5_MC_0`"]
    #[inline(always)]
    pub fn is_tc5_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TC5_MC_0
    }
    #[doc = "Checks if the value of the field is `TC5_MC_1`"]
    #[inline(always)]
    pub fn is_tc5_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TC5_MC_1
    }
    #[doc = "Checks if the value of the field is `TC6_OVF`"]
    #[inline(always)]
    pub fn is_tc6_ovf(&self) -> bool {
        *self == TRIGSRC_A::TC6_OVF
    }
    #[doc = "Checks if the value of the field is `TC6_MC_0`"]
    #[inline(always)]
    pub fn is_tc6_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TC6_MC_0
    }
    #[doc = "Checks if the value of the field is `TC6_MC_1`"]
    #[inline(always)]
    pub fn is_tc6_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TC6_MC_1
    }
    #[doc = "Checks if the value of the field is `TC7_OVF`"]
    #[inline(always)]
    pub fn is_tc7_ovf(&self) -> bool {
        *self == TRIGSRC_A::TC7_OVF
    }
    #[doc = "Checks if the value of the field is `TC7_MC_0`"]
    #[inline(always)]
    pub fn is_tc7_mc_0(&self) -> bool {
        *self == TRIGSRC_A::TC7_MC_0
    }
    #[doc = "Checks if the value of the field is `TC7_MC_1`"]
    #[inline(always)]
    pub fn is_tc7_mc_1(&self) -> bool {
        *self == TRIGSRC_A::TC7_MC_1
    }
    #[doc = "Checks if the value of the field is `ADC0_RESRDY`"]
    #[inline(always)]
    pub fn is_adc0_resrdy(&self) -> bool {
        *self == TRIGSRC_A::ADC0_RESRDY
    }
    #[doc = "Checks if the value of the field is `ADC0_SEQ`"]
    #[inline(always)]
    pub fn is_adc0_seq(&self) -> bool {
        *self == TRIGSRC_A::ADC0_SEQ
    }
    #[doc = "Checks if the value of the field is `ADC1_RESRDY`"]
    #[inline(always)]
    pub fn is_adc1_resrdy(&self) -> bool {
        *self == TRIGSRC_A::ADC1_RESRDY
    }
    #[doc = "Checks if the value of the field is `ADC1_SEQ`"]
    #[inline(always)]
    pub fn is_adc1_seq(&self) -> bool {
        *self == TRIGSRC_A::ADC1_SEQ
    }
    #[doc = "Checks if the value of the field is `DAC_EMPTY_0`"]
    #[inline(always)]
    pub fn is_dac_empty_0(&self) -> bool {
        *self == TRIGSRC_A::DAC_EMPTY_0
    }
    #[doc = "Checks if the value of the field is `DAC_EMPTY_1`"]
    #[inline(always)]
    pub fn is_dac_empty_1(&self) -> bool {
        *self == TRIGSRC_A::DAC_EMPTY_1
    }
    #[doc = "Checks if the value of the field is `DAC_RESRDY_0`"]
    #[inline(always)]
    pub fn is_dac_resrdy_0(&self) -> bool {
        *self == TRIGSRC_A::DAC_RESRDY_0
    }
    #[doc = "Checks if the value of the field is `DAC_RESRDY_1`"]
    #[inline(always)]
    pub fn is_dac_resrdy_1(&self) -> bool {
        *self == TRIGSRC_A::DAC_RESRDY_1
    }
    #[doc = "Checks if the value of the field is `I2S_RX_0`"]
    #[inline(always)]
    pub fn is_i2s_rx_0(&self) -> bool {
        *self == TRIGSRC_A::I2S_RX_0
    }
    #[doc = "Checks if the value of the field is `I2S_RX_1`"]
    #[inline(always)]
    pub fn is_i2s_rx_1(&self) -> bool {
        *self == TRIGSRC_A::I2S_RX_1
    }
    #[doc = "Checks if the value of the field is `I2S_TX_0`"]
    #[inline(always)]
    pub fn is_i2s_tx_0(&self) -> bool {
        *self == TRIGSRC_A::I2S_TX_0
    }
    #[doc = "Checks if the value of the field is `I2S_TX_1`"]
    #[inline(always)]
    pub fn is_i2s_tx_1(&self) -> bool {
        *self == TRIGSRC_A::I2S_TX_1
    }
    #[doc = "Checks if the value of the field is `PCC_RX`"]
    #[inline(always)]
    pub fn is_pcc_rx(&self) -> bool {
        *self == TRIGSRC_A::PCC_RX
    }
    #[doc = "Checks if the value of the field is `AES_WR`"]
    #[inline(always)]
    pub fn is_aes_wr(&self) -> bool {
        *self == TRIGSRC_A::AES_WR
    }
    #[doc = "Checks if the value of the field is `AES_RD`"]
    #[inline(always)]
    pub fn is_aes_rd(&self) -> bool {
        *self == TRIGSRC_A::AES_RD
    }
    #[doc = "Checks if the value of the field is `QSPI_RX`"]
    #[inline(always)]
    pub fn is_qspi_rx(&self) -> bool {
        *self == TRIGSRC_A::QSPI_RX
    }
    #[doc = "Checks if the value of the field is `QSPI_TX`"]
    #[inline(always)]
    pub fn is_qspi_tx(&self) -> bool {
        *self == TRIGSRC_A::QSPI_TX
    }
}
#[doc = "Write proxy for field `TRIGSRC`"]
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
    #[doc = "DMA RTC timestamp trigger"]
    #[inline(always)]
    pub fn rtc_timestamp(self) -> &'a mut W {
        self.variant(TRIGSRC_A::RTC_TIMESTAMP)
    }
    #[doc = "DMAC ID for DCC0 register"]
    #[inline(always)]
    pub fn dsu_dcc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DSU_DCC0)
    }
    #[doc = "DMAC ID for DCC1 register"]
    #[inline(always)]
    pub fn dsu_dcc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DSU_DCC1)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom0_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM0_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom0_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM0_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom1_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM1_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom1_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM1_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom2_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM2_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom2_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM2_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom3_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM3_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom3_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM3_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom4_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM4_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom4_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM4_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom5_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM5_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom5_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM5_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom6_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM6_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom6_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM6_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom7_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM7_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom7_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM7_TX)
    }
    #[doc = "DMA CAN Debug Req"]
    #[inline(always)]
    pub fn can0_debug(self) -> &'a mut W {
        self.variant(TRIGSRC_A::CAN0_DEBUG)
    }
    #[doc = "DMA CAN Debug Req"]
    #[inline(always)]
    pub fn can1_debug(self) -> &'a mut W {
        self.variant(TRIGSRC_A::CAN1_DEBUG)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc0_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC0_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC0_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC0_MC_1)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_2(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC0_MC_2)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_3(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC0_MC_3)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_4(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC0_MC_4)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_5(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC0_MC_5)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc1_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC1_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc1_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC1_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc1_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC1_MC_1)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc1_mc_2(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC1_MC_2)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc1_mc_3(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC1_MC_3)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc2_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC2_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc2_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC2_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc2_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC2_MC_1)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc2_mc_2(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC2_MC_2)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc3_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC3_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc3_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC3_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc3_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC3_MC_1)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc4_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC4_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc4_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC4_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc4_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TCC4_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc0_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC0_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc0_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC0_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc0_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC0_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc1_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC1_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc1_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC1_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc1_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC1_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc2_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC2_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc2_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC2_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc2_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC2_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc3_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC3_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc3_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC3_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc3_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC3_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc4_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC4_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc4_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC4_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc4_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC4_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc5_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC5_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc5_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC5_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc5_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC5_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc6_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC6_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc6_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC6_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc6_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC6_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc7_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC7_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc7_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC7_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc7_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC7_MC_1)
    }
    #[doc = "index of DMA RESRDY trigger"]
    #[inline(always)]
    pub fn adc0_resrdy(self) -> &'a mut W {
        self.variant(TRIGSRC_A::ADC0_RESRDY)
    }
    #[doc = "Index of DMA SEQ trigger"]
    #[inline(always)]
    pub fn adc0_seq(self) -> &'a mut W {
        self.variant(TRIGSRC_A::ADC0_SEQ)
    }
    #[doc = "Index of DMA RESRDY trigger"]
    #[inline(always)]
    pub fn adc1_resrdy(self) -> &'a mut W {
        self.variant(TRIGSRC_A::ADC1_RESRDY)
    }
    #[doc = "Index of DMA SEQ trigger"]
    #[inline(always)]
    pub fn adc1_seq(self) -> &'a mut W {
        self.variant(TRIGSRC_A::ADC1_SEQ)
    }
    #[doc = "DMA DAC Empty Req"]
    #[inline(always)]
    pub fn dac_empty_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DAC_EMPTY_0)
    }
    #[doc = "DMA DAC Empty Req"]
    #[inline(always)]
    pub fn dac_empty_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DAC_EMPTY_1)
    }
    #[doc = "DMA DAC Result Ready Req"]
    #[inline(always)]
    pub fn dac_resrdy_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DAC_RESRDY_0)
    }
    #[doc = "DMA DAC Result Ready Req"]
    #[inline(always)]
    pub fn dac_resrdy_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DAC_RESRDY_1)
    }
    #[doc = "Indexes of DMA RX triggers"]
    #[inline(always)]
    pub fn i2s_rx_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::I2S_RX_0)
    }
    #[doc = "Indexes of DMA RX triggers"]
    #[inline(always)]
    pub fn i2s_rx_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::I2S_RX_1)
    }
    #[doc = "Indexes of DMA TX triggers"]
    #[inline(always)]
    pub fn i2s_tx_0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::I2S_TX_0)
    }
    #[doc = "Indexes of DMA TX triggers"]
    #[inline(always)]
    pub fn i2s_tx_1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::I2S_TX_1)
    }
    #[doc = "Indexes of PCC RX trigger"]
    #[inline(always)]
    pub fn pcc_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::PCC_RX)
    }
    #[doc = "DMA DATA Write trigger"]
    #[inline(always)]
    pub fn aes_wr(self) -> &'a mut W {
        self.variant(TRIGSRC_A::AES_WR)
    }
    #[doc = "DMA DATA Read trigger"]
    #[inline(always)]
    pub fn aes_rd(self) -> &'a mut W {
        self.variant(TRIGSRC_A::AES_RD)
    }
    #[doc = "Indexes of QSPI RX trigger"]
    #[inline(always)]
    pub fn qspi_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::QSPI_RX)
    }
    #[doc = "Indexes of QSPI TX trigger"]
    #[inline(always)]
    pub fn qspi_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::QSPI_TX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGACT_A {
    #[doc = "0: One trigger required for each block transfer"]
    BLOCK = 0,
    #[doc = "2: One trigger required for each burst transfer"]
    BURST = 2,
    #[doc = "3: One trigger required for each transaction"]
    TRANSACTION = 3,
}
impl From<TRIGACT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGACT`"]
pub type TRIGACT_R = crate::R<u8, TRIGACT_A>;
impl TRIGACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRIGACT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRIGACT_A::BLOCK),
            2 => Val(TRIGACT_A::BURST),
            3 => Val(TRIGACT_A::TRANSACTION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == TRIGACT_A::BLOCK
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == TRIGACT_A::BURST
    }
    #[doc = "Checks if the value of the field is `TRANSACTION`"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        *self == TRIGACT_A::TRANSACTION
    }
}
#[doc = "Write proxy for field `TRIGACT`"]
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
    #[doc = "One trigger required for each burst transfer"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(TRIGACT_A::BURST)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut W {
        self.variant(TRIGACT_A::TRANSACTION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Burst Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BURSTLEN_A {
    #[doc = "0: Single-beat burst length"]
    SINGLE = 0,
    #[doc = "1: 2-beats burst length"]
    _2BEAT = 1,
    #[doc = "2: 3-beats burst length"]
    _3BEAT = 2,
    #[doc = "3: 4-beats burst length"]
    _4BEAT = 3,
    #[doc = "4: 5-beats burst length"]
    _5BEAT = 4,
    #[doc = "5: 6-beats burst length"]
    _6BEAT = 5,
    #[doc = "6: 7-beats burst length"]
    _7BEAT = 6,
    #[doc = "7: 8-beats burst length"]
    _8BEAT = 7,
    #[doc = "8: 9-beats burst length"]
    _9BEAT = 8,
    #[doc = "9: 10-beats burst length"]
    _10BEAT = 9,
    #[doc = "10: 11-beats burst length"]
    _11BEAT = 10,
    #[doc = "11: 12-beats burst length"]
    _12BEAT = 11,
    #[doc = "12: 13-beats burst length"]
    _13BEAT = 12,
    #[doc = "13: 14-beats burst length"]
    _14BEAT = 13,
    #[doc = "14: 15-beats burst length"]
    _15BEAT = 14,
    #[doc = "15: 16-beats burst length"]
    _16BEAT = 15,
}
impl From<BURSTLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: BURSTLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BURSTLEN`"]
pub type BURSTLEN_R = crate::R<u8, BURSTLEN_A>;
impl BURSTLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTLEN_A {
        match self.bits {
            0 => BURSTLEN_A::SINGLE,
            1 => BURSTLEN_A::_2BEAT,
            2 => BURSTLEN_A::_3BEAT,
            3 => BURSTLEN_A::_4BEAT,
            4 => BURSTLEN_A::_5BEAT,
            5 => BURSTLEN_A::_6BEAT,
            6 => BURSTLEN_A::_7BEAT,
            7 => BURSTLEN_A::_8BEAT,
            8 => BURSTLEN_A::_9BEAT,
            9 => BURSTLEN_A::_10BEAT,
            10 => BURSTLEN_A::_11BEAT,
            11 => BURSTLEN_A::_12BEAT,
            12 => BURSTLEN_A::_13BEAT,
            13 => BURSTLEN_A::_14BEAT,
            14 => BURSTLEN_A::_15BEAT,
            15 => BURSTLEN_A::_16BEAT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == BURSTLEN_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `_2BEAT`"]
    #[inline(always)]
    pub fn is_2beat(&self) -> bool {
        *self == BURSTLEN_A::_2BEAT
    }
    #[doc = "Checks if the value of the field is `_3BEAT`"]
    #[inline(always)]
    pub fn is_3beat(&self) -> bool {
        *self == BURSTLEN_A::_3BEAT
    }
    #[doc = "Checks if the value of the field is `_4BEAT`"]
    #[inline(always)]
    pub fn is_4beat(&self) -> bool {
        *self == BURSTLEN_A::_4BEAT
    }
    #[doc = "Checks if the value of the field is `_5BEAT`"]
    #[inline(always)]
    pub fn is_5beat(&self) -> bool {
        *self == BURSTLEN_A::_5BEAT
    }
    #[doc = "Checks if the value of the field is `_6BEAT`"]
    #[inline(always)]
    pub fn is_6beat(&self) -> bool {
        *self == BURSTLEN_A::_6BEAT
    }
    #[doc = "Checks if the value of the field is `_7BEAT`"]
    #[inline(always)]
    pub fn is_7beat(&self) -> bool {
        *self == BURSTLEN_A::_7BEAT
    }
    #[doc = "Checks if the value of the field is `_8BEAT`"]
    #[inline(always)]
    pub fn is_8beat(&self) -> bool {
        *self == BURSTLEN_A::_8BEAT
    }
    #[doc = "Checks if the value of the field is `_9BEAT`"]
    #[inline(always)]
    pub fn is_9beat(&self) -> bool {
        *self == BURSTLEN_A::_9BEAT
    }
    #[doc = "Checks if the value of the field is `_10BEAT`"]
    #[inline(always)]
    pub fn is_10beat(&self) -> bool {
        *self == BURSTLEN_A::_10BEAT
    }
    #[doc = "Checks if the value of the field is `_11BEAT`"]
    #[inline(always)]
    pub fn is_11beat(&self) -> bool {
        *self == BURSTLEN_A::_11BEAT
    }
    #[doc = "Checks if the value of the field is `_12BEAT`"]
    #[inline(always)]
    pub fn is_12beat(&self) -> bool {
        *self == BURSTLEN_A::_12BEAT
    }
    #[doc = "Checks if the value of the field is `_13BEAT`"]
    #[inline(always)]
    pub fn is_13beat(&self) -> bool {
        *self == BURSTLEN_A::_13BEAT
    }
    #[doc = "Checks if the value of the field is `_14BEAT`"]
    #[inline(always)]
    pub fn is_14beat(&self) -> bool {
        *self == BURSTLEN_A::_14BEAT
    }
    #[doc = "Checks if the value of the field is `_15BEAT`"]
    #[inline(always)]
    pub fn is_15beat(&self) -> bool {
        *self == BURSTLEN_A::_15BEAT
    }
    #[doc = "Checks if the value of the field is `_16BEAT`"]
    #[inline(always)]
    pub fn is_16beat(&self) -> bool {
        *self == BURSTLEN_A::_16BEAT
    }
}
#[doc = "Write proxy for field `BURSTLEN`"]
pub struct BURSTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTLEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-beat burst length"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(BURSTLEN_A::SINGLE)
    }
    #[doc = "2-beats burst length"]
    #[inline(always)]
    pub fn _2beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_2BEAT)
    }
    #[doc = "3-beats burst length"]
    #[inline(always)]
    pub fn _3beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_3BEAT)
    }
    #[doc = "4-beats burst length"]
    #[inline(always)]
    pub fn _4beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_4BEAT)
    }
    #[doc = "5-beats burst length"]
    #[inline(always)]
    pub fn _5beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_5BEAT)
    }
    #[doc = "6-beats burst length"]
    #[inline(always)]
    pub fn _6beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_6BEAT)
    }
    #[doc = "7-beats burst length"]
    #[inline(always)]
    pub fn _7beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_7BEAT)
    }
    #[doc = "8-beats burst length"]
    #[inline(always)]
    pub fn _8beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_8BEAT)
    }
    #[doc = "9-beats burst length"]
    #[inline(always)]
    pub fn _9beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_9BEAT)
    }
    #[doc = "10-beats burst length"]
    #[inline(always)]
    pub fn _10beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_10BEAT)
    }
    #[doc = "11-beats burst length"]
    #[inline(always)]
    pub fn _11beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_11BEAT)
    }
    #[doc = "12-beats burst length"]
    #[inline(always)]
    pub fn _12beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_12BEAT)
    }
    #[doc = "13-beats burst length"]
    #[inline(always)]
    pub fn _13beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_13BEAT)
    }
    #[doc = "14-beats burst length"]
    #[inline(always)]
    pub fn _14beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_14BEAT)
    }
    #[doc = "15-beats burst length"]
    #[inline(always)]
    pub fn _15beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_15BEAT)
    }
    #[doc = "16-beats burst length"]
    #[inline(always)]
    pub fn _16beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_16BEAT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "FIFO Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum THRESHOLD_A {
    #[doc = "0: Destination write starts after each beat source address read"]
    _1BEAT = 0,
    #[doc = "1: Destination write starts after 2-beats source address read"]
    _2BEATS = 1,
    #[doc = "2: Destination write starts after 4-beats source address read"]
    _4BEATS = 2,
    #[doc = "3: Destination write starts after 8-beats source address read"]
    _8BEATS = 3,
}
impl From<THRESHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: THRESHOLD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `THRESHOLD`"]
pub type THRESHOLD_R = crate::R<u8, THRESHOLD_A>;
impl THRESHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THRESHOLD_A {
        match self.bits {
            0 => THRESHOLD_A::_1BEAT,
            1 => THRESHOLD_A::_2BEATS,
            2 => THRESHOLD_A::_4BEATS,
            3 => THRESHOLD_A::_8BEATS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1BEAT`"]
    #[inline(always)]
    pub fn is_1beat(&self) -> bool {
        *self == THRESHOLD_A::_1BEAT
    }
    #[doc = "Checks if the value of the field is `_2BEATS`"]
    #[inline(always)]
    pub fn is_2beats(&self) -> bool {
        *self == THRESHOLD_A::_2BEATS
    }
    #[doc = "Checks if the value of the field is `_4BEATS`"]
    #[inline(always)]
    pub fn is_4beats(&self) -> bool {
        *self == THRESHOLD_A::_4BEATS
    }
    #[doc = "Checks if the value of the field is `_8BEATS`"]
    #[inline(always)]
    pub fn is_8beats(&self) -> bool {
        *self == THRESHOLD_A::_8BEATS
    }
}
#[doc = "Write proxy for field `THRESHOLD`"]
pub struct THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THRESHOLD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Destination write starts after each beat source address read"]
    #[inline(always)]
    pub fn _1beat(self) -> &'a mut W {
        self.variant(THRESHOLD_A::_1BEAT)
    }
    #[doc = "Destination write starts after 2-beats source address read"]
    #[inline(always)]
    pub fn _2beats(self) -> &'a mut W {
        self.variant(THRESHOLD_A::_2BEATS)
    }
    #[doc = "Destination write starts after 4-beats source address read"]
    #[inline(always)]
    pub fn _4beats(self) -> &'a mut W {
        self.variant(THRESHOLD_A::_4BEATS)
    }
    #[doc = "Destination write starts after 8-beats source address read"]
    #[inline(always)]
    pub fn _8beats(self) -> &'a mut W {
        self.variant(THRESHOLD_A::_8BEATS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TRIGSRC_R {
        TRIGSRC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&self) -> TRIGACT_R {
        TRIGACT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline(always)]
    pub fn burstlen(&self) -> BURSTLEN_R {
        BURSTLEN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&mut self) -> TRIGSRC_W {
        TRIGSRC_W { w: self }
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&mut self) -> TRIGACT_W {
        TRIGACT_W { w: self }
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline(always)]
    pub fn burstlen(&mut self) -> BURSTLEN_W {
        BURSTLEN_W { w: self }
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline(always)]
    pub fn threshold(&mut self) -> THRESHOLD_W {
        THRESHOLD_W { w: self }
    }
}
