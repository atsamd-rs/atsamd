#[doc = "Register `CHCTRLA` reader"]
pub struct R(crate::R<CHCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTRLA` writer"]
pub struct W(crate::W<CHCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTRLA_SPEC>;
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
impl From<crate::W<CHCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Channel Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Channel Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTRLA_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Channel Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Channel Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTRLA_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Channel Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Channel Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTRLA_SPEC, bool, O>;
#[doc = "Field `TRIGSRC` reader - Trigger Source"]
pub type TRIGSRC_R = crate::FieldReader<u8, TRIGSRCSELECT_A>;
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSRCSELECT_A {
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
impl From<TRIGSRCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRCSELECT_A) -> Self {
        variant as _
    }
}
impl TRIGSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGSRCSELECT_A> {
        match self.bits {
            0 => Some(TRIGSRCSELECT_A::DISABLE),
            1 => Some(TRIGSRCSELECT_A::RTC_TIMESTAMP),
            2 => Some(TRIGSRCSELECT_A::DSU_DCC0),
            3 => Some(TRIGSRCSELECT_A::DSU_DCC1),
            4 => Some(TRIGSRCSELECT_A::SERCOM0_RX),
            5 => Some(TRIGSRCSELECT_A::SERCOM0_TX),
            6 => Some(TRIGSRCSELECT_A::SERCOM1_RX),
            7 => Some(TRIGSRCSELECT_A::SERCOM1_TX),
            8 => Some(TRIGSRCSELECT_A::SERCOM2_RX),
            9 => Some(TRIGSRCSELECT_A::SERCOM2_TX),
            10 => Some(TRIGSRCSELECT_A::SERCOM3_RX),
            11 => Some(TRIGSRCSELECT_A::SERCOM3_TX),
            12 => Some(TRIGSRCSELECT_A::SERCOM4_RX),
            13 => Some(TRIGSRCSELECT_A::SERCOM4_TX),
            14 => Some(TRIGSRCSELECT_A::SERCOM5_RX),
            15 => Some(TRIGSRCSELECT_A::SERCOM5_TX),
            16 => Some(TRIGSRCSELECT_A::SERCOM6_RX),
            17 => Some(TRIGSRCSELECT_A::SERCOM6_TX),
            18 => Some(TRIGSRCSELECT_A::SERCOM7_RX),
            19 => Some(TRIGSRCSELECT_A::SERCOM7_TX),
            20 => Some(TRIGSRCSELECT_A::CAN0_DEBUG),
            21 => Some(TRIGSRCSELECT_A::CAN1_DEBUG),
            22 => Some(TRIGSRCSELECT_A::TCC0_OVF),
            23 => Some(TRIGSRCSELECT_A::TCC0_MC_0),
            24 => Some(TRIGSRCSELECT_A::TCC0_MC_1),
            25 => Some(TRIGSRCSELECT_A::TCC0_MC_2),
            26 => Some(TRIGSRCSELECT_A::TCC0_MC_3),
            27 => Some(TRIGSRCSELECT_A::TCC0_MC_4),
            28 => Some(TRIGSRCSELECT_A::TCC0_MC_5),
            29 => Some(TRIGSRCSELECT_A::TCC1_OVF),
            30 => Some(TRIGSRCSELECT_A::TCC1_MC_0),
            31 => Some(TRIGSRCSELECT_A::TCC1_MC_1),
            32 => Some(TRIGSRCSELECT_A::TCC1_MC_2),
            33 => Some(TRIGSRCSELECT_A::TCC1_MC_3),
            34 => Some(TRIGSRCSELECT_A::TCC2_OVF),
            35 => Some(TRIGSRCSELECT_A::TCC2_MC_0),
            36 => Some(TRIGSRCSELECT_A::TCC2_MC_1),
            37 => Some(TRIGSRCSELECT_A::TCC2_MC_2),
            38 => Some(TRIGSRCSELECT_A::TCC3_OVF),
            39 => Some(TRIGSRCSELECT_A::TCC3_MC_0),
            40 => Some(TRIGSRCSELECT_A::TCC3_MC_1),
            41 => Some(TRIGSRCSELECT_A::TCC4_OVF),
            42 => Some(TRIGSRCSELECT_A::TCC4_MC_0),
            43 => Some(TRIGSRCSELECT_A::TCC4_MC_1),
            44 => Some(TRIGSRCSELECT_A::TC0_OVF),
            45 => Some(TRIGSRCSELECT_A::TC0_MC_0),
            46 => Some(TRIGSRCSELECT_A::TC0_MC_1),
            47 => Some(TRIGSRCSELECT_A::TC1_OVF),
            48 => Some(TRIGSRCSELECT_A::TC1_MC_0),
            49 => Some(TRIGSRCSELECT_A::TC1_MC_1),
            50 => Some(TRIGSRCSELECT_A::TC2_OVF),
            51 => Some(TRIGSRCSELECT_A::TC2_MC_0),
            52 => Some(TRIGSRCSELECT_A::TC2_MC_1),
            53 => Some(TRIGSRCSELECT_A::TC3_OVF),
            54 => Some(TRIGSRCSELECT_A::TC3_MC_0),
            55 => Some(TRIGSRCSELECT_A::TC3_MC_1),
            56 => Some(TRIGSRCSELECT_A::TC4_OVF),
            57 => Some(TRIGSRCSELECT_A::TC4_MC_0),
            58 => Some(TRIGSRCSELECT_A::TC4_MC_1),
            59 => Some(TRIGSRCSELECT_A::TC5_OVF),
            60 => Some(TRIGSRCSELECT_A::TC5_MC_0),
            61 => Some(TRIGSRCSELECT_A::TC5_MC_1),
            62 => Some(TRIGSRCSELECT_A::TC6_OVF),
            63 => Some(TRIGSRCSELECT_A::TC6_MC_0),
            64 => Some(TRIGSRCSELECT_A::TC6_MC_1),
            65 => Some(TRIGSRCSELECT_A::TC7_OVF),
            66 => Some(TRIGSRCSELECT_A::TC7_MC_0),
            67 => Some(TRIGSRCSELECT_A::TC7_MC_1),
            68 => Some(TRIGSRCSELECT_A::ADC0_RESRDY),
            69 => Some(TRIGSRCSELECT_A::ADC0_SEQ),
            70 => Some(TRIGSRCSELECT_A::ADC1_RESRDY),
            71 => Some(TRIGSRCSELECT_A::ADC1_SEQ),
            72 => Some(TRIGSRCSELECT_A::DAC_EMPTY_0),
            73 => Some(TRIGSRCSELECT_A::DAC_EMPTY_1),
            74 => Some(TRIGSRCSELECT_A::DAC_RESRDY_0),
            75 => Some(TRIGSRCSELECT_A::DAC_RESRDY_1),
            76 => Some(TRIGSRCSELECT_A::I2S_RX_0),
            77 => Some(TRIGSRCSELECT_A::I2S_RX_1),
            78 => Some(TRIGSRCSELECT_A::I2S_TX_0),
            79 => Some(TRIGSRCSELECT_A::I2S_TX_1),
            80 => Some(TRIGSRCSELECT_A::PCC_RX),
            81 => Some(TRIGSRCSELECT_A::AES_WR),
            82 => Some(TRIGSRCSELECT_A::AES_RD),
            83 => Some(TRIGSRCSELECT_A::QSPI_RX),
            84 => Some(TRIGSRCSELECT_A::QSPI_TX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TRIGSRCSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RTC_TIMESTAMP`"]
    #[inline(always)]
    pub fn is_rtc_timestamp(&self) -> bool {
        *self == TRIGSRCSELECT_A::RTC_TIMESTAMP
    }
    #[doc = "Checks if the value of the field is `DSU_DCC0`"]
    #[inline(always)]
    pub fn is_dsu_dcc0(&self) -> bool {
        *self == TRIGSRCSELECT_A::DSU_DCC0
    }
    #[doc = "Checks if the value of the field is `DSU_DCC1`"]
    #[inline(always)]
    pub fn is_dsu_dcc1(&self) -> bool {
        *self == TRIGSRCSELECT_A::DSU_DCC1
    }
    #[doc = "Checks if the value of the field is `SERCOM0_RX`"]
    #[inline(always)]
    pub fn is_sercom0_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM0_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM0_TX`"]
    #[inline(always)]
    pub fn is_sercom0_tx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM0_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM1_RX`"]
    #[inline(always)]
    pub fn is_sercom1_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM1_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM1_TX`"]
    #[inline(always)]
    pub fn is_sercom1_tx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM1_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM2_RX`"]
    #[inline(always)]
    pub fn is_sercom2_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM2_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM2_TX`"]
    #[inline(always)]
    pub fn is_sercom2_tx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM2_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM3_RX`"]
    #[inline(always)]
    pub fn is_sercom3_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM3_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM3_TX`"]
    #[inline(always)]
    pub fn is_sercom3_tx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM3_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM4_RX`"]
    #[inline(always)]
    pub fn is_sercom4_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM4_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM4_TX`"]
    #[inline(always)]
    pub fn is_sercom4_tx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM4_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM5_RX`"]
    #[inline(always)]
    pub fn is_sercom5_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM5_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM5_TX`"]
    #[inline(always)]
    pub fn is_sercom5_tx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM5_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM6_RX`"]
    #[inline(always)]
    pub fn is_sercom6_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM6_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM6_TX`"]
    #[inline(always)]
    pub fn is_sercom6_tx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM6_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM7_RX`"]
    #[inline(always)]
    pub fn is_sercom7_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM7_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM7_TX`"]
    #[inline(always)]
    pub fn is_sercom7_tx(&self) -> bool {
        *self == TRIGSRCSELECT_A::SERCOM7_TX
    }
    #[doc = "Checks if the value of the field is `CAN0_DEBUG`"]
    #[inline(always)]
    pub fn is_can0_debug(&self) -> bool {
        *self == TRIGSRCSELECT_A::CAN0_DEBUG
    }
    #[doc = "Checks if the value of the field is `CAN1_DEBUG`"]
    #[inline(always)]
    pub fn is_can1_debug(&self) -> bool {
        *self == TRIGSRCSELECT_A::CAN1_DEBUG
    }
    #[doc = "Checks if the value of the field is `TCC0_OVF`"]
    #[inline(always)]
    pub fn is_tcc0_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC0_OVF
    }
    #[doc = "Checks if the value of the field is `TCC0_MC_0`"]
    #[inline(always)]
    pub fn is_tcc0_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC0_MC_0
    }
    #[doc = "Checks if the value of the field is `TCC0_MC_1`"]
    #[inline(always)]
    pub fn is_tcc0_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC0_MC_1
    }
    #[doc = "Checks if the value of the field is `TCC0_MC_2`"]
    #[inline(always)]
    pub fn is_tcc0_mc_2(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC0_MC_2
    }
    #[doc = "Checks if the value of the field is `TCC0_MC_3`"]
    #[inline(always)]
    pub fn is_tcc0_mc_3(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC0_MC_3
    }
    #[doc = "Checks if the value of the field is `TCC0_MC_4`"]
    #[inline(always)]
    pub fn is_tcc0_mc_4(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC0_MC_4
    }
    #[doc = "Checks if the value of the field is `TCC0_MC_5`"]
    #[inline(always)]
    pub fn is_tcc0_mc_5(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC0_MC_5
    }
    #[doc = "Checks if the value of the field is `TCC1_OVF`"]
    #[inline(always)]
    pub fn is_tcc1_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC1_OVF
    }
    #[doc = "Checks if the value of the field is `TCC1_MC_0`"]
    #[inline(always)]
    pub fn is_tcc1_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC1_MC_0
    }
    #[doc = "Checks if the value of the field is `TCC1_MC_1`"]
    #[inline(always)]
    pub fn is_tcc1_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC1_MC_1
    }
    #[doc = "Checks if the value of the field is `TCC1_MC_2`"]
    #[inline(always)]
    pub fn is_tcc1_mc_2(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC1_MC_2
    }
    #[doc = "Checks if the value of the field is `TCC1_MC_3`"]
    #[inline(always)]
    pub fn is_tcc1_mc_3(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC1_MC_3
    }
    #[doc = "Checks if the value of the field is `TCC2_OVF`"]
    #[inline(always)]
    pub fn is_tcc2_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC2_OVF
    }
    #[doc = "Checks if the value of the field is `TCC2_MC_0`"]
    #[inline(always)]
    pub fn is_tcc2_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC2_MC_0
    }
    #[doc = "Checks if the value of the field is `TCC2_MC_1`"]
    #[inline(always)]
    pub fn is_tcc2_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC2_MC_1
    }
    #[doc = "Checks if the value of the field is `TCC2_MC_2`"]
    #[inline(always)]
    pub fn is_tcc2_mc_2(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC2_MC_2
    }
    #[doc = "Checks if the value of the field is `TCC3_OVF`"]
    #[inline(always)]
    pub fn is_tcc3_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC3_OVF
    }
    #[doc = "Checks if the value of the field is `TCC3_MC_0`"]
    #[inline(always)]
    pub fn is_tcc3_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC3_MC_0
    }
    #[doc = "Checks if the value of the field is `TCC3_MC_1`"]
    #[inline(always)]
    pub fn is_tcc3_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC3_MC_1
    }
    #[doc = "Checks if the value of the field is `TCC4_OVF`"]
    #[inline(always)]
    pub fn is_tcc4_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC4_OVF
    }
    #[doc = "Checks if the value of the field is `TCC4_MC_0`"]
    #[inline(always)]
    pub fn is_tcc4_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC4_MC_0
    }
    #[doc = "Checks if the value of the field is `TCC4_MC_1`"]
    #[inline(always)]
    pub fn is_tcc4_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TCC4_MC_1
    }
    #[doc = "Checks if the value of the field is `TC0_OVF`"]
    #[inline(always)]
    pub fn is_tc0_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC0_OVF
    }
    #[doc = "Checks if the value of the field is `TC0_MC_0`"]
    #[inline(always)]
    pub fn is_tc0_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC0_MC_0
    }
    #[doc = "Checks if the value of the field is `TC0_MC_1`"]
    #[inline(always)]
    pub fn is_tc0_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC0_MC_1
    }
    #[doc = "Checks if the value of the field is `TC1_OVF`"]
    #[inline(always)]
    pub fn is_tc1_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC1_OVF
    }
    #[doc = "Checks if the value of the field is `TC1_MC_0`"]
    #[inline(always)]
    pub fn is_tc1_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC1_MC_0
    }
    #[doc = "Checks if the value of the field is `TC1_MC_1`"]
    #[inline(always)]
    pub fn is_tc1_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC1_MC_1
    }
    #[doc = "Checks if the value of the field is `TC2_OVF`"]
    #[inline(always)]
    pub fn is_tc2_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC2_OVF
    }
    #[doc = "Checks if the value of the field is `TC2_MC_0`"]
    #[inline(always)]
    pub fn is_tc2_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC2_MC_0
    }
    #[doc = "Checks if the value of the field is `TC2_MC_1`"]
    #[inline(always)]
    pub fn is_tc2_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC2_MC_1
    }
    #[doc = "Checks if the value of the field is `TC3_OVF`"]
    #[inline(always)]
    pub fn is_tc3_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC3_OVF
    }
    #[doc = "Checks if the value of the field is `TC3_MC_0`"]
    #[inline(always)]
    pub fn is_tc3_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC3_MC_0
    }
    #[doc = "Checks if the value of the field is `TC3_MC_1`"]
    #[inline(always)]
    pub fn is_tc3_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC3_MC_1
    }
    #[doc = "Checks if the value of the field is `TC4_OVF`"]
    #[inline(always)]
    pub fn is_tc4_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC4_OVF
    }
    #[doc = "Checks if the value of the field is `TC4_MC_0`"]
    #[inline(always)]
    pub fn is_tc4_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC4_MC_0
    }
    #[doc = "Checks if the value of the field is `TC4_MC_1`"]
    #[inline(always)]
    pub fn is_tc4_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC4_MC_1
    }
    #[doc = "Checks if the value of the field is `TC5_OVF`"]
    #[inline(always)]
    pub fn is_tc5_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC5_OVF
    }
    #[doc = "Checks if the value of the field is `TC5_MC_0`"]
    #[inline(always)]
    pub fn is_tc5_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC5_MC_0
    }
    #[doc = "Checks if the value of the field is `TC5_MC_1`"]
    #[inline(always)]
    pub fn is_tc5_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC5_MC_1
    }
    #[doc = "Checks if the value of the field is `TC6_OVF`"]
    #[inline(always)]
    pub fn is_tc6_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC6_OVF
    }
    #[doc = "Checks if the value of the field is `TC6_MC_0`"]
    #[inline(always)]
    pub fn is_tc6_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC6_MC_0
    }
    #[doc = "Checks if the value of the field is `TC6_MC_1`"]
    #[inline(always)]
    pub fn is_tc6_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC6_MC_1
    }
    #[doc = "Checks if the value of the field is `TC7_OVF`"]
    #[inline(always)]
    pub fn is_tc7_ovf(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC7_OVF
    }
    #[doc = "Checks if the value of the field is `TC7_MC_0`"]
    #[inline(always)]
    pub fn is_tc7_mc_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC7_MC_0
    }
    #[doc = "Checks if the value of the field is `TC7_MC_1`"]
    #[inline(always)]
    pub fn is_tc7_mc_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::TC7_MC_1
    }
    #[doc = "Checks if the value of the field is `ADC0_RESRDY`"]
    #[inline(always)]
    pub fn is_adc0_resrdy(&self) -> bool {
        *self == TRIGSRCSELECT_A::ADC0_RESRDY
    }
    #[doc = "Checks if the value of the field is `ADC0_SEQ`"]
    #[inline(always)]
    pub fn is_adc0_seq(&self) -> bool {
        *self == TRIGSRCSELECT_A::ADC0_SEQ
    }
    #[doc = "Checks if the value of the field is `ADC1_RESRDY`"]
    #[inline(always)]
    pub fn is_adc1_resrdy(&self) -> bool {
        *self == TRIGSRCSELECT_A::ADC1_RESRDY
    }
    #[doc = "Checks if the value of the field is `ADC1_SEQ`"]
    #[inline(always)]
    pub fn is_adc1_seq(&self) -> bool {
        *self == TRIGSRCSELECT_A::ADC1_SEQ
    }
    #[doc = "Checks if the value of the field is `DAC_EMPTY_0`"]
    #[inline(always)]
    pub fn is_dac_empty_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::DAC_EMPTY_0
    }
    #[doc = "Checks if the value of the field is `DAC_EMPTY_1`"]
    #[inline(always)]
    pub fn is_dac_empty_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::DAC_EMPTY_1
    }
    #[doc = "Checks if the value of the field is `DAC_RESRDY_0`"]
    #[inline(always)]
    pub fn is_dac_resrdy_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::DAC_RESRDY_0
    }
    #[doc = "Checks if the value of the field is `DAC_RESRDY_1`"]
    #[inline(always)]
    pub fn is_dac_resrdy_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::DAC_RESRDY_1
    }
    #[doc = "Checks if the value of the field is `I2S_RX_0`"]
    #[inline(always)]
    pub fn is_i2s_rx_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::I2S_RX_0
    }
    #[doc = "Checks if the value of the field is `I2S_RX_1`"]
    #[inline(always)]
    pub fn is_i2s_rx_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::I2S_RX_1
    }
    #[doc = "Checks if the value of the field is `I2S_TX_0`"]
    #[inline(always)]
    pub fn is_i2s_tx_0(&self) -> bool {
        *self == TRIGSRCSELECT_A::I2S_TX_0
    }
    #[doc = "Checks if the value of the field is `I2S_TX_1`"]
    #[inline(always)]
    pub fn is_i2s_tx_1(&self) -> bool {
        *self == TRIGSRCSELECT_A::I2S_TX_1
    }
    #[doc = "Checks if the value of the field is `PCC_RX`"]
    #[inline(always)]
    pub fn is_pcc_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::PCC_RX
    }
    #[doc = "Checks if the value of the field is `AES_WR`"]
    #[inline(always)]
    pub fn is_aes_wr(&self) -> bool {
        *self == TRIGSRCSELECT_A::AES_WR
    }
    #[doc = "Checks if the value of the field is `AES_RD`"]
    #[inline(always)]
    pub fn is_aes_rd(&self) -> bool {
        *self == TRIGSRCSELECT_A::AES_RD
    }
    #[doc = "Checks if the value of the field is `QSPI_RX`"]
    #[inline(always)]
    pub fn is_qspi_rx(&self) -> bool {
        *self == TRIGSRCSELECT_A::QSPI_RX
    }
    #[doc = "Checks if the value of the field is `QSPI_TX`"]
    #[inline(always)]
    pub fn is_qspi_tx(&self) -> bool {
        *self == TRIGSRCSELECT_A::QSPI_TX
    }
}
#[doc = "Field `TRIGSRC` writer - Trigger Source"]
pub type TRIGSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHCTRLA_SPEC, u8, TRIGSRCSELECT_A, 7, O>;
impl<'a, const O: u8> TRIGSRC_W<'a, O> {
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::DISABLE)
    }
    #[doc = "DMA RTC timestamp trigger"]
    #[inline(always)]
    pub fn rtc_timestamp(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::RTC_TIMESTAMP)
    }
    #[doc = "DMAC ID for DCC0 register"]
    #[inline(always)]
    pub fn dsu_dcc0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::DSU_DCC0)
    }
    #[doc = "DMAC ID for DCC1 register"]
    #[inline(always)]
    pub fn dsu_dcc1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::DSU_DCC1)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom0_rx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM0_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom0_tx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM0_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom1_rx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM1_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom1_tx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM1_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom2_rx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM2_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom2_tx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM2_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom3_rx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM3_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom3_tx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM3_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom4_rx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM4_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom4_tx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM4_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom5_rx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM5_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom5_tx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM5_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom6_rx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM6_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom6_tx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM6_TX)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom7_rx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM7_RX)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom7_tx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::SERCOM7_TX)
    }
    #[doc = "DMA CAN Debug Req"]
    #[inline(always)]
    pub fn can0_debug(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::CAN0_DEBUG)
    }
    #[doc = "DMA CAN Debug Req"]
    #[inline(always)]
    pub fn can1_debug(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::CAN1_DEBUG)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc0_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC0_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC0_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC0_MC_1)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_2(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC0_MC_2)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_3(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC0_MC_3)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_4(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC0_MC_4)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_5(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC0_MC_5)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc1_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC1_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc1_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC1_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc1_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC1_MC_1)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc1_mc_2(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC1_MC_2)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc1_mc_3(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC1_MC_3)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc2_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC2_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc2_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC2_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc2_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC2_MC_1)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc2_mc_2(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC2_MC_2)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc3_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC3_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc3_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC3_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc3_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC3_MC_1)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc4_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC4_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc4_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC4_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc4_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TCC4_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc0_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC0_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc0_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC0_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc0_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC0_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc1_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC1_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc1_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC1_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc1_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC1_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc2_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC2_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc2_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC2_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc2_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC2_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc3_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC3_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc3_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC3_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc3_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC3_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc4_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC4_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc4_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC4_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc4_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC4_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc5_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC5_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc5_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC5_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc5_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC5_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc6_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC6_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc6_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC6_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc6_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC6_MC_1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc7_ovf(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC7_OVF)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc7_mc_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC7_MC_0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc7_mc_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::TC7_MC_1)
    }
    #[doc = "index of DMA RESRDY trigger"]
    #[inline(always)]
    pub fn adc0_resrdy(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::ADC0_RESRDY)
    }
    #[doc = "Index of DMA SEQ trigger"]
    #[inline(always)]
    pub fn adc0_seq(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::ADC0_SEQ)
    }
    #[doc = "Index of DMA RESRDY trigger"]
    #[inline(always)]
    pub fn adc1_resrdy(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::ADC1_RESRDY)
    }
    #[doc = "Index of DMA SEQ trigger"]
    #[inline(always)]
    pub fn adc1_seq(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::ADC1_SEQ)
    }
    #[doc = "DMA DAC Empty Req"]
    #[inline(always)]
    pub fn dac_empty_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::DAC_EMPTY_0)
    }
    #[doc = "DMA DAC Empty Req"]
    #[inline(always)]
    pub fn dac_empty_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::DAC_EMPTY_1)
    }
    #[doc = "DMA DAC Result Ready Req"]
    #[inline(always)]
    pub fn dac_resrdy_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::DAC_RESRDY_0)
    }
    #[doc = "DMA DAC Result Ready Req"]
    #[inline(always)]
    pub fn dac_resrdy_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::DAC_RESRDY_1)
    }
    #[doc = "Indexes of DMA RX triggers"]
    #[inline(always)]
    pub fn i2s_rx_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::I2S_RX_0)
    }
    #[doc = "Indexes of DMA RX triggers"]
    #[inline(always)]
    pub fn i2s_rx_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::I2S_RX_1)
    }
    #[doc = "Indexes of DMA TX triggers"]
    #[inline(always)]
    pub fn i2s_tx_0(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::I2S_TX_0)
    }
    #[doc = "Indexes of DMA TX triggers"]
    #[inline(always)]
    pub fn i2s_tx_1(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::I2S_TX_1)
    }
    #[doc = "Indexes of PCC RX trigger"]
    #[inline(always)]
    pub fn pcc_rx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::PCC_RX)
    }
    #[doc = "DMA DATA Write trigger"]
    #[inline(always)]
    pub fn aes_wr(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::AES_WR)
    }
    #[doc = "DMA DATA Read trigger"]
    #[inline(always)]
    pub fn aes_rd(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::AES_RD)
    }
    #[doc = "Indexes of QSPI RX trigger"]
    #[inline(always)]
    pub fn qspi_rx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::QSPI_RX)
    }
    #[doc = "Indexes of QSPI TX trigger"]
    #[inline(always)]
    pub fn qspi_tx(self) -> &'a mut W {
        self.variant(TRIGSRCSELECT_A::QSPI_TX)
    }
}
#[doc = "Field `TRIGACT` reader - Trigger Action"]
pub type TRIGACT_R = crate::FieldReader<u8, TRIGACTSELECT_A>;
#[doc = "Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGACTSELECT_A {
    #[doc = "0: One trigger required for each block transfer"]
    BLOCK = 0,
    #[doc = "2: One trigger required for each burst transfer"]
    BURST = 2,
    #[doc = "3: One trigger required for each transaction"]
    TRANSACTION = 3,
}
impl From<TRIGACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGACTSELECT_A) -> Self {
        variant as _
    }
}
impl TRIGACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGACTSELECT_A> {
        match self.bits {
            0 => Some(TRIGACTSELECT_A::BLOCK),
            2 => Some(TRIGACTSELECT_A::BURST),
            3 => Some(TRIGACTSELECT_A::TRANSACTION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == TRIGACTSELECT_A::BLOCK
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == TRIGACTSELECT_A::BURST
    }
    #[doc = "Checks if the value of the field is `TRANSACTION`"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        *self == TRIGACTSELECT_A::TRANSACTION
    }
}
#[doc = "Field `TRIGACT` writer - Trigger Action"]
pub type TRIGACT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHCTRLA_SPEC, u8, TRIGACTSELECT_A, 2, O>;
impl<'a, const O: u8> TRIGACT_W<'a, O> {
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn block(self) -> &'a mut W {
        self.variant(TRIGACTSELECT_A::BLOCK)
    }
    #[doc = "One trigger required for each burst transfer"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(TRIGACTSELECT_A::BURST)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut W {
        self.variant(TRIGACTSELECT_A::TRANSACTION)
    }
}
#[doc = "Field `BURSTLEN` reader - Burst Length"]
pub type BURSTLEN_R = crate::FieldReader<u8, BURSTLENSELECT_A>;
#[doc = "Burst Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BURSTLENSELECT_A {
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
impl From<BURSTLENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BURSTLENSELECT_A) -> Self {
        variant as _
    }
}
impl BURSTLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTLENSELECT_A {
        match self.bits {
            0 => BURSTLENSELECT_A::SINGLE,
            1 => BURSTLENSELECT_A::_2BEAT,
            2 => BURSTLENSELECT_A::_3BEAT,
            3 => BURSTLENSELECT_A::_4BEAT,
            4 => BURSTLENSELECT_A::_5BEAT,
            5 => BURSTLENSELECT_A::_6BEAT,
            6 => BURSTLENSELECT_A::_7BEAT,
            7 => BURSTLENSELECT_A::_8BEAT,
            8 => BURSTLENSELECT_A::_9BEAT,
            9 => BURSTLENSELECT_A::_10BEAT,
            10 => BURSTLENSELECT_A::_11BEAT,
            11 => BURSTLENSELECT_A::_12BEAT,
            12 => BURSTLENSELECT_A::_13BEAT,
            13 => BURSTLENSELECT_A::_14BEAT,
            14 => BURSTLENSELECT_A::_15BEAT,
            15 => BURSTLENSELECT_A::_16BEAT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == BURSTLENSELECT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `_2BEAT`"]
    #[inline(always)]
    pub fn is_2beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_2BEAT
    }
    #[doc = "Checks if the value of the field is `_3BEAT`"]
    #[inline(always)]
    pub fn is_3beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_3BEAT
    }
    #[doc = "Checks if the value of the field is `_4BEAT`"]
    #[inline(always)]
    pub fn is_4beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_4BEAT
    }
    #[doc = "Checks if the value of the field is `_5BEAT`"]
    #[inline(always)]
    pub fn is_5beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_5BEAT
    }
    #[doc = "Checks if the value of the field is `_6BEAT`"]
    #[inline(always)]
    pub fn is_6beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_6BEAT
    }
    #[doc = "Checks if the value of the field is `_7BEAT`"]
    #[inline(always)]
    pub fn is_7beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_7BEAT
    }
    #[doc = "Checks if the value of the field is `_8BEAT`"]
    #[inline(always)]
    pub fn is_8beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_8BEAT
    }
    #[doc = "Checks if the value of the field is `_9BEAT`"]
    #[inline(always)]
    pub fn is_9beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_9BEAT
    }
    #[doc = "Checks if the value of the field is `_10BEAT`"]
    #[inline(always)]
    pub fn is_10beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_10BEAT
    }
    #[doc = "Checks if the value of the field is `_11BEAT`"]
    #[inline(always)]
    pub fn is_11beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_11BEAT
    }
    #[doc = "Checks if the value of the field is `_12BEAT`"]
    #[inline(always)]
    pub fn is_12beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_12BEAT
    }
    #[doc = "Checks if the value of the field is `_13BEAT`"]
    #[inline(always)]
    pub fn is_13beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_13BEAT
    }
    #[doc = "Checks if the value of the field is `_14BEAT`"]
    #[inline(always)]
    pub fn is_14beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_14BEAT
    }
    #[doc = "Checks if the value of the field is `_15BEAT`"]
    #[inline(always)]
    pub fn is_15beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_15BEAT
    }
    #[doc = "Checks if the value of the field is `_16BEAT`"]
    #[inline(always)]
    pub fn is_16beat(&self) -> bool {
        *self == BURSTLENSELECT_A::_16BEAT
    }
}
#[doc = "Field `BURSTLEN` writer - Burst Length"]
pub type BURSTLEN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CHCTRLA_SPEC, u8, BURSTLENSELECT_A, 4, O>;
impl<'a, const O: u8> BURSTLEN_W<'a, O> {
    #[doc = "Single-beat burst length"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::SINGLE)
    }
    #[doc = "2-beats burst length"]
    #[inline(always)]
    pub fn _2beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_2BEAT)
    }
    #[doc = "3-beats burst length"]
    #[inline(always)]
    pub fn _3beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_3BEAT)
    }
    #[doc = "4-beats burst length"]
    #[inline(always)]
    pub fn _4beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_4BEAT)
    }
    #[doc = "5-beats burst length"]
    #[inline(always)]
    pub fn _5beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_5BEAT)
    }
    #[doc = "6-beats burst length"]
    #[inline(always)]
    pub fn _6beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_6BEAT)
    }
    #[doc = "7-beats burst length"]
    #[inline(always)]
    pub fn _7beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_7BEAT)
    }
    #[doc = "8-beats burst length"]
    #[inline(always)]
    pub fn _8beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_8BEAT)
    }
    #[doc = "9-beats burst length"]
    #[inline(always)]
    pub fn _9beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_9BEAT)
    }
    #[doc = "10-beats burst length"]
    #[inline(always)]
    pub fn _10beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_10BEAT)
    }
    #[doc = "11-beats burst length"]
    #[inline(always)]
    pub fn _11beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_11BEAT)
    }
    #[doc = "12-beats burst length"]
    #[inline(always)]
    pub fn _12beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_12BEAT)
    }
    #[doc = "13-beats burst length"]
    #[inline(always)]
    pub fn _13beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_13BEAT)
    }
    #[doc = "14-beats burst length"]
    #[inline(always)]
    pub fn _14beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_14BEAT)
    }
    #[doc = "15-beats burst length"]
    #[inline(always)]
    pub fn _15beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_15BEAT)
    }
    #[doc = "16-beats burst length"]
    #[inline(always)]
    pub fn _16beat(self) -> &'a mut W {
        self.variant(BURSTLENSELECT_A::_16BEAT)
    }
}
#[doc = "Field `THRESHOLD` reader - FIFO Threshold"]
pub type THRESHOLD_R = crate::FieldReader<u8, THRESHOLDSELECT_A>;
#[doc = "FIFO Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THRESHOLDSELECT_A {
    #[doc = "0: Destination write starts after each beat source address read"]
    _1BEAT = 0,
    #[doc = "1: Destination write starts after 2-beats source address read"]
    _2BEATS = 1,
    #[doc = "2: Destination write starts after 4-beats source address read"]
    _4BEATS = 2,
    #[doc = "3: Destination write starts after 8-beats source address read"]
    _8BEATS = 3,
}
impl From<THRESHOLDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: THRESHOLDSELECT_A) -> Self {
        variant as _
    }
}
impl THRESHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THRESHOLDSELECT_A {
        match self.bits {
            0 => THRESHOLDSELECT_A::_1BEAT,
            1 => THRESHOLDSELECT_A::_2BEATS,
            2 => THRESHOLDSELECT_A::_4BEATS,
            3 => THRESHOLDSELECT_A::_8BEATS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1BEAT`"]
    #[inline(always)]
    pub fn is_1beat(&self) -> bool {
        *self == THRESHOLDSELECT_A::_1BEAT
    }
    #[doc = "Checks if the value of the field is `_2BEATS`"]
    #[inline(always)]
    pub fn is_2beats(&self) -> bool {
        *self == THRESHOLDSELECT_A::_2BEATS
    }
    #[doc = "Checks if the value of the field is `_4BEATS`"]
    #[inline(always)]
    pub fn is_4beats(&self) -> bool {
        *self == THRESHOLDSELECT_A::_4BEATS
    }
    #[doc = "Checks if the value of the field is `_8BEATS`"]
    #[inline(always)]
    pub fn is_8beats(&self) -> bool {
        *self == THRESHOLDSELECT_A::_8BEATS
    }
}
#[doc = "Field `THRESHOLD` writer - FIFO Threshold"]
pub type THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CHCTRLA_SPEC, u8, THRESHOLDSELECT_A, 2, O>;
impl<'a, const O: u8> THRESHOLD_W<'a, O> {
    #[doc = "Destination write starts after each beat source address read"]
    #[inline(always)]
    pub fn _1beat(self) -> &'a mut W {
        self.variant(THRESHOLDSELECT_A::_1BEAT)
    }
    #[doc = "Destination write starts after 2-beats source address read"]
    #[inline(always)]
    pub fn _2beats(self) -> &'a mut W {
        self.variant(THRESHOLDSELECT_A::_2BEATS)
    }
    #[doc = "Destination write starts after 4-beats source address read"]
    #[inline(always)]
    pub fn _4beats(self) -> &'a mut W {
        self.variant(THRESHOLDSELECT_A::_4BEATS)
    }
    #[doc = "Destination write starts after 8-beats source address read"]
    #[inline(always)]
    pub fn _8beats(self) -> &'a mut W {
        self.variant(THRESHOLDSELECT_A::_8BEATS)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TRIGSRC_R {
        TRIGSRC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&self) -> TRIGACT_R {
        TRIGACT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline(always)]
    pub fn burstlen(&self) -> BURSTLEN_R {
        BURSTLEN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline(always)]
    #[must_use]
    pub fn trigsrc(&mut self) -> TRIGSRC_W<8> {
        TRIGSRC_W::new(self)
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline(always)]
    #[must_use]
    pub fn trigact(&mut self) -> TRIGACT_W<20> {
        TRIGACT_W::new(self)
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn burstlen(&mut self) -> BURSTLEN_W<24> {
        BURSTLEN_W::new(self)
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> THRESHOLD_W<28> {
        THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrla](index.html) module"]
pub struct CHCTRLA_SPEC;
impl crate::RegisterSpec for CHCTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctrla::R](R) reader structure"]
impl crate::Readable for CHCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctrla::W](W) writer structure"]
impl crate::Writable for CHCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTRLA to value 0"]
impl crate::Resettable for CHCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
