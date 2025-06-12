#[doc = "Register `CHCTRLA` reader"]
pub type R = crate::R<ChctrlaSpec>;
#[doc = "Register `CHCTRLA` writer"]
pub type W = crate::W<ChctrlaSpec>;
#[doc = "Field `SWRST` reader - Channel Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Channel Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Channel Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Channel Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Channel Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Channel Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsrcselect {
    #[doc = "0: Only software/event triggers"]
    Disable = 0,
    #[doc = "1: DMA RTC timestamp trigger"]
    RtcTimestamp = 1,
    #[doc = "2: DMAC ID for DCC0 register"]
    DsuDcc0 = 2,
    #[doc = "3: DMAC ID for DCC1 register"]
    DsuDcc1 = 3,
    #[doc = "4: Index of DMA RX trigger"]
    Sercom0Rx = 4,
    #[doc = "5: Index of DMA TX trigger"]
    Sercom0Tx = 5,
    #[doc = "6: Index of DMA RX trigger"]
    Sercom1Rx = 6,
    #[doc = "7: Index of DMA TX trigger"]
    Sercom1Tx = 7,
    #[doc = "8: Index of DMA RX trigger"]
    Sercom2Rx = 8,
    #[doc = "9: Index of DMA TX trigger"]
    Sercom2Tx = 9,
    #[doc = "10: Index of DMA RX trigger"]
    Sercom3Rx = 10,
    #[doc = "11: Index of DMA TX trigger"]
    Sercom3Tx = 11,
    #[doc = "12: Index of DMA RX trigger"]
    Sercom4Rx = 12,
    #[doc = "13: Index of DMA TX trigger"]
    Sercom4Tx = 13,
    #[doc = "14: Index of DMA RX trigger"]
    Sercom5Rx = 14,
    #[doc = "15: Index of DMA TX trigger"]
    Sercom5Tx = 15,
    #[doc = "16: Index of DMA RX trigger"]
    Sercom6Rx = 16,
    #[doc = "17: Index of DMA TX trigger"]
    Sercom6Tx = 17,
    #[doc = "18: Index of DMA RX trigger"]
    Sercom7Rx = 18,
    #[doc = "19: Index of DMA TX trigger"]
    Sercom7Tx = 19,
    #[doc = "20: DMA CAN Debug Req"]
    Can0Debug = 20,
    #[doc = "21: DMA CAN Debug Req"]
    Can1Debug = 21,
    #[doc = "22: DMA overflow/underflow/retrigger trigger"]
    Tcc0Ovf = 22,
    #[doc = "23: Indexes of DMA Match/Compare triggers"]
    Tcc0Mc0 = 23,
    #[doc = "24: Indexes of DMA Match/Compare triggers"]
    Tcc0Mc1 = 24,
    #[doc = "25: Indexes of DMA Match/Compare triggers"]
    Tcc0Mc2 = 25,
    #[doc = "26: Indexes of DMA Match/Compare triggers"]
    Tcc0Mc3 = 26,
    #[doc = "27: Indexes of DMA Match/Compare triggers"]
    Tcc0Mc4 = 27,
    #[doc = "28: Indexes of DMA Match/Compare triggers"]
    Tcc0Mc5 = 28,
    #[doc = "29: DMA overflow/underflow/retrigger trigger"]
    Tcc1Ovf = 29,
    #[doc = "30: Indexes of DMA Match/Compare triggers"]
    Tcc1Mc0 = 30,
    #[doc = "31: Indexes of DMA Match/Compare triggers"]
    Tcc1Mc1 = 31,
    #[doc = "32: Indexes of DMA Match/Compare triggers"]
    Tcc1Mc2 = 32,
    #[doc = "33: Indexes of DMA Match/Compare triggers"]
    Tcc1Mc3 = 33,
    #[doc = "34: DMA overflow/underflow/retrigger trigger"]
    Tcc2Ovf = 34,
    #[doc = "35: Indexes of DMA Match/Compare triggers"]
    Tcc2Mc0 = 35,
    #[doc = "36: Indexes of DMA Match/Compare triggers"]
    Tcc2Mc1 = 36,
    #[doc = "37: Indexes of DMA Match/Compare triggers"]
    Tcc2Mc2 = 37,
    #[doc = "38: DMA overflow/underflow/retrigger trigger"]
    Tcc3Ovf = 38,
    #[doc = "39: Indexes of DMA Match/Compare triggers"]
    Tcc3Mc0 = 39,
    #[doc = "40: Indexes of DMA Match/Compare triggers"]
    Tcc3Mc1 = 40,
    #[doc = "41: DMA overflow/underflow/retrigger trigger"]
    Tcc4Ovf = 41,
    #[doc = "42: Indexes of DMA Match/Compare triggers"]
    Tcc4Mc0 = 42,
    #[doc = "43: Indexes of DMA Match/Compare triggers"]
    Tcc4Mc1 = 43,
    #[doc = "44: Indexes of DMA Overflow trigger"]
    Tc0Ovf = 44,
    #[doc = "45: Indexes of DMA Match/Compare triggers"]
    Tc0Mc0 = 45,
    #[doc = "46: Indexes of DMA Match/Compare triggers"]
    Tc0Mc1 = 46,
    #[doc = "47: Indexes of DMA Overflow trigger"]
    Tc1Ovf = 47,
    #[doc = "48: Indexes of DMA Match/Compare triggers"]
    Tc1Mc0 = 48,
    #[doc = "49: Indexes of DMA Match/Compare triggers"]
    Tc1Mc1 = 49,
    #[doc = "50: Indexes of DMA Overflow trigger"]
    Tc2Ovf = 50,
    #[doc = "51: Indexes of DMA Match/Compare triggers"]
    Tc2Mc0 = 51,
    #[doc = "52: Indexes of DMA Match/Compare triggers"]
    Tc2Mc1 = 52,
    #[doc = "53: Indexes of DMA Overflow trigger"]
    Tc3Ovf = 53,
    #[doc = "54: Indexes of DMA Match/Compare triggers"]
    Tc3Mc0 = 54,
    #[doc = "55: Indexes of DMA Match/Compare triggers"]
    Tc3Mc1 = 55,
    #[doc = "56: Indexes of DMA Overflow trigger"]
    Tc4Ovf = 56,
    #[doc = "57: Indexes of DMA Match/Compare triggers"]
    Tc4Mc0 = 57,
    #[doc = "58: Indexes of DMA Match/Compare triggers"]
    Tc4Mc1 = 58,
    #[doc = "59: Indexes of DMA Overflow trigger"]
    Tc5Ovf = 59,
    #[doc = "60: Indexes of DMA Match/Compare triggers"]
    Tc5Mc0 = 60,
    #[doc = "61: Indexes of DMA Match/Compare triggers"]
    Tc5Mc1 = 61,
    #[doc = "62: Indexes of DMA Overflow trigger"]
    Tc6Ovf = 62,
    #[doc = "63: Indexes of DMA Match/Compare triggers"]
    Tc6Mc0 = 63,
    #[doc = "64: Indexes of DMA Match/Compare triggers"]
    Tc6Mc1 = 64,
    #[doc = "65: Indexes of DMA Overflow trigger"]
    Tc7Ovf = 65,
    #[doc = "66: Indexes of DMA Match/Compare triggers"]
    Tc7Mc0 = 66,
    #[doc = "67: Indexes of DMA Match/Compare triggers"]
    Tc7Mc1 = 67,
    #[doc = "68: index of DMA RESRDY trigger"]
    Adc0Resrdy = 68,
    #[doc = "69: Index of DMA SEQ trigger"]
    Adc0Seq = 69,
    #[doc = "70: Index of DMA RESRDY trigger"]
    Adc1Resrdy = 70,
    #[doc = "71: Index of DMA SEQ trigger"]
    Adc1Seq = 71,
    #[doc = "72: DMA DAC Empty Req"]
    DacEmpty0 = 72,
    #[doc = "73: DMA DAC Empty Req"]
    DacEmpty1 = 73,
    #[doc = "74: DMA DAC Result Ready Req"]
    DacResrdy0 = 74,
    #[doc = "75: DMA DAC Result Ready Req"]
    DacResrdy1 = 75,
    #[doc = "76: Indexes of DMA RX triggers"]
    I2sRx0 = 76,
    #[doc = "77: Indexes of DMA RX triggers"]
    I2sRx1 = 77,
    #[doc = "78: Indexes of DMA TX triggers"]
    I2sTx0 = 78,
    #[doc = "79: Indexes of DMA TX triggers"]
    I2sTx1 = 79,
    #[doc = "80: Indexes of PCC RX trigger"]
    PccRx = 80,
    #[doc = "81: DMA DATA Write trigger"]
    AesWr = 81,
    #[doc = "82: DMA DATA Read trigger"]
    AesRd = 82,
    #[doc = "83: Indexes of QSPI RX trigger"]
    QspiRx = 83,
    #[doc = "84: Indexes of QSPI TX trigger"]
    QspiTx = 84,
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
            1 => Some(Trigsrcselect::RtcTimestamp),
            2 => Some(Trigsrcselect::DsuDcc0),
            3 => Some(Trigsrcselect::DsuDcc1),
            4 => Some(Trigsrcselect::Sercom0Rx),
            5 => Some(Trigsrcselect::Sercom0Tx),
            6 => Some(Trigsrcselect::Sercom1Rx),
            7 => Some(Trigsrcselect::Sercom1Tx),
            8 => Some(Trigsrcselect::Sercom2Rx),
            9 => Some(Trigsrcselect::Sercom2Tx),
            10 => Some(Trigsrcselect::Sercom3Rx),
            11 => Some(Trigsrcselect::Sercom3Tx),
            12 => Some(Trigsrcselect::Sercom4Rx),
            13 => Some(Trigsrcselect::Sercom4Tx),
            14 => Some(Trigsrcselect::Sercom5Rx),
            15 => Some(Trigsrcselect::Sercom5Tx),
            16 => Some(Trigsrcselect::Sercom6Rx),
            17 => Some(Trigsrcselect::Sercom6Tx),
            18 => Some(Trigsrcselect::Sercom7Rx),
            19 => Some(Trigsrcselect::Sercom7Tx),
            20 => Some(Trigsrcselect::Can0Debug),
            21 => Some(Trigsrcselect::Can1Debug),
            22 => Some(Trigsrcselect::Tcc0Ovf),
            23 => Some(Trigsrcselect::Tcc0Mc0),
            24 => Some(Trigsrcselect::Tcc0Mc1),
            25 => Some(Trigsrcselect::Tcc0Mc2),
            26 => Some(Trigsrcselect::Tcc0Mc3),
            27 => Some(Trigsrcselect::Tcc0Mc4),
            28 => Some(Trigsrcselect::Tcc0Mc5),
            29 => Some(Trigsrcselect::Tcc1Ovf),
            30 => Some(Trigsrcselect::Tcc1Mc0),
            31 => Some(Trigsrcselect::Tcc1Mc1),
            32 => Some(Trigsrcselect::Tcc1Mc2),
            33 => Some(Trigsrcselect::Tcc1Mc3),
            34 => Some(Trigsrcselect::Tcc2Ovf),
            35 => Some(Trigsrcselect::Tcc2Mc0),
            36 => Some(Trigsrcselect::Tcc2Mc1),
            37 => Some(Trigsrcselect::Tcc2Mc2),
            38 => Some(Trigsrcselect::Tcc3Ovf),
            39 => Some(Trigsrcselect::Tcc3Mc0),
            40 => Some(Trigsrcselect::Tcc3Mc1),
            41 => Some(Trigsrcselect::Tcc4Ovf),
            42 => Some(Trigsrcselect::Tcc4Mc0),
            43 => Some(Trigsrcselect::Tcc4Mc1),
            44 => Some(Trigsrcselect::Tc0Ovf),
            45 => Some(Trigsrcselect::Tc0Mc0),
            46 => Some(Trigsrcselect::Tc0Mc1),
            47 => Some(Trigsrcselect::Tc1Ovf),
            48 => Some(Trigsrcselect::Tc1Mc0),
            49 => Some(Trigsrcselect::Tc1Mc1),
            50 => Some(Trigsrcselect::Tc2Ovf),
            51 => Some(Trigsrcselect::Tc2Mc0),
            52 => Some(Trigsrcselect::Tc2Mc1),
            53 => Some(Trigsrcselect::Tc3Ovf),
            54 => Some(Trigsrcselect::Tc3Mc0),
            55 => Some(Trigsrcselect::Tc3Mc1),
            56 => Some(Trigsrcselect::Tc4Ovf),
            57 => Some(Trigsrcselect::Tc4Mc0),
            58 => Some(Trigsrcselect::Tc4Mc1),
            59 => Some(Trigsrcselect::Tc5Ovf),
            60 => Some(Trigsrcselect::Tc5Mc0),
            61 => Some(Trigsrcselect::Tc5Mc1),
            62 => Some(Trigsrcselect::Tc6Ovf),
            63 => Some(Trigsrcselect::Tc6Mc0),
            64 => Some(Trigsrcselect::Tc6Mc1),
            65 => Some(Trigsrcselect::Tc7Ovf),
            66 => Some(Trigsrcselect::Tc7Mc0),
            67 => Some(Trigsrcselect::Tc7Mc1),
            68 => Some(Trigsrcselect::Adc0Resrdy),
            69 => Some(Trigsrcselect::Adc0Seq),
            70 => Some(Trigsrcselect::Adc1Resrdy),
            71 => Some(Trigsrcselect::Adc1Seq),
            72 => Some(Trigsrcselect::DacEmpty0),
            73 => Some(Trigsrcselect::DacEmpty1),
            74 => Some(Trigsrcselect::DacResrdy0),
            75 => Some(Trigsrcselect::DacResrdy1),
            76 => Some(Trigsrcselect::I2sRx0),
            77 => Some(Trigsrcselect::I2sRx1),
            78 => Some(Trigsrcselect::I2sTx0),
            79 => Some(Trigsrcselect::I2sTx1),
            80 => Some(Trigsrcselect::PccRx),
            81 => Some(Trigsrcselect::AesWr),
            82 => Some(Trigsrcselect::AesRd),
            83 => Some(Trigsrcselect::QspiRx),
            84 => Some(Trigsrcselect::QspiTx),
            _ => None,
        }
    }
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Trigsrcselect::Disable
    }
    #[doc = "DMA RTC timestamp trigger"]
    #[inline(always)]
    pub fn is_rtc_timestamp(&self) -> bool {
        *self == Trigsrcselect::RtcTimestamp
    }
    #[doc = "DMAC ID for DCC0 register"]
    #[inline(always)]
    pub fn is_dsu_dcc0(&self) -> bool {
        *self == Trigsrcselect::DsuDcc0
    }
    #[doc = "DMAC ID for DCC1 register"]
    #[inline(always)]
    pub fn is_dsu_dcc1(&self) -> bool {
        *self == Trigsrcselect::DsuDcc1
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn is_sercom0_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom0Rx
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn is_sercom0_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom0Tx
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn is_sercom1_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom1Rx
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn is_sercom1_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom1Tx
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn is_sercom2_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom2Rx
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn is_sercom2_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom2Tx
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn is_sercom3_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom3Rx
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn is_sercom3_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom3Tx
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn is_sercom4_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom4Rx
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn is_sercom4_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom4Tx
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn is_sercom5_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom5Rx
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn is_sercom5_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom5Tx
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn is_sercom6_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom6Rx
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn is_sercom6_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom6Tx
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn is_sercom7_rx(&self) -> bool {
        *self == Trigsrcselect::Sercom7Rx
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn is_sercom7_tx(&self) -> bool {
        *self == Trigsrcselect::Sercom7Tx
    }
    #[doc = "DMA CAN Debug Req"]
    #[inline(always)]
    pub fn is_can0_debug(&self) -> bool {
        *self == Trigsrcselect::Can0Debug
    }
    #[doc = "DMA CAN Debug Req"]
    #[inline(always)]
    pub fn is_can1_debug(&self) -> bool {
        *self == Trigsrcselect::Can1Debug
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn is_tcc0_ovf(&self) -> bool {
        *self == Trigsrcselect::Tcc0Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc0_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc0_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc1
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc0_mc_2(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc2
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc0_mc_3(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc3
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc0_mc_4(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc4
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc0_mc_5(&self) -> bool {
        *self == Trigsrcselect::Tcc0Mc5
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn is_tcc1_ovf(&self) -> bool {
        *self == Trigsrcselect::Tcc1Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc1_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tcc1Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc1_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tcc1Mc1
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc1_mc_2(&self) -> bool {
        *self == Trigsrcselect::Tcc1Mc2
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc1_mc_3(&self) -> bool {
        *self == Trigsrcselect::Tcc1Mc3
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn is_tcc2_ovf(&self) -> bool {
        *self == Trigsrcselect::Tcc2Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc2_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tcc2Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc2_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tcc2Mc1
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc2_mc_2(&self) -> bool {
        *self == Trigsrcselect::Tcc2Mc2
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn is_tcc3_ovf(&self) -> bool {
        *self == Trigsrcselect::Tcc3Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc3_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tcc3Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc3_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tcc3Mc1
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn is_tcc4_ovf(&self) -> bool {
        *self == Trigsrcselect::Tcc4Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc4_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tcc4Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tcc4_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tcc4Mc1
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn is_tc0_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc0Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc0_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tc0Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc0_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tc0Mc1
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn is_tc1_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc1Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc1_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tc1Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc1_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tc1Mc1
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn is_tc2_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc2Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc2_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tc2Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc2_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tc2Mc1
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn is_tc3_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc3Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc3_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tc3Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc3_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tc3Mc1
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn is_tc4_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc4Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc4_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tc4Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc4_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tc4Mc1
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn is_tc5_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc5Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc5_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tc5Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc5_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tc5Mc1
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn is_tc6_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc6Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc6_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tc6Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc6_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tc6Mc1
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn is_tc7_ovf(&self) -> bool {
        *self == Trigsrcselect::Tc7Ovf
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc7_mc_0(&self) -> bool {
        *self == Trigsrcselect::Tc7Mc0
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn is_tc7_mc_1(&self) -> bool {
        *self == Trigsrcselect::Tc7Mc1
    }
    #[doc = "index of DMA RESRDY trigger"]
    #[inline(always)]
    pub fn is_adc0_resrdy(&self) -> bool {
        *self == Trigsrcselect::Adc0Resrdy
    }
    #[doc = "Index of DMA SEQ trigger"]
    #[inline(always)]
    pub fn is_adc0_seq(&self) -> bool {
        *self == Trigsrcselect::Adc0Seq
    }
    #[doc = "Index of DMA RESRDY trigger"]
    #[inline(always)]
    pub fn is_adc1_resrdy(&self) -> bool {
        *self == Trigsrcselect::Adc1Resrdy
    }
    #[doc = "Index of DMA SEQ trigger"]
    #[inline(always)]
    pub fn is_adc1_seq(&self) -> bool {
        *self == Trigsrcselect::Adc1Seq
    }
    #[doc = "DMA DAC Empty Req"]
    #[inline(always)]
    pub fn is_dac_empty_0(&self) -> bool {
        *self == Trigsrcselect::DacEmpty0
    }
    #[doc = "DMA DAC Empty Req"]
    #[inline(always)]
    pub fn is_dac_empty_1(&self) -> bool {
        *self == Trigsrcselect::DacEmpty1
    }
    #[doc = "DMA DAC Result Ready Req"]
    #[inline(always)]
    pub fn is_dac_resrdy_0(&self) -> bool {
        *self == Trigsrcselect::DacResrdy0
    }
    #[doc = "DMA DAC Result Ready Req"]
    #[inline(always)]
    pub fn is_dac_resrdy_1(&self) -> bool {
        *self == Trigsrcselect::DacResrdy1
    }
    #[doc = "Indexes of DMA RX triggers"]
    #[inline(always)]
    pub fn is_i2s_rx_0(&self) -> bool {
        *self == Trigsrcselect::I2sRx0
    }
    #[doc = "Indexes of DMA RX triggers"]
    #[inline(always)]
    pub fn is_i2s_rx_1(&self) -> bool {
        *self == Trigsrcselect::I2sRx1
    }
    #[doc = "Indexes of DMA TX triggers"]
    #[inline(always)]
    pub fn is_i2s_tx_0(&self) -> bool {
        *self == Trigsrcselect::I2sTx0
    }
    #[doc = "Indexes of DMA TX triggers"]
    #[inline(always)]
    pub fn is_i2s_tx_1(&self) -> bool {
        *self == Trigsrcselect::I2sTx1
    }
    #[doc = "Indexes of PCC RX trigger"]
    #[inline(always)]
    pub fn is_pcc_rx(&self) -> bool {
        *self == Trigsrcselect::PccRx
    }
    #[doc = "DMA DATA Write trigger"]
    #[inline(always)]
    pub fn is_aes_wr(&self) -> bool {
        *self == Trigsrcselect::AesWr
    }
    #[doc = "DMA DATA Read trigger"]
    #[inline(always)]
    pub fn is_aes_rd(&self) -> bool {
        *self == Trigsrcselect::AesRd
    }
    #[doc = "Indexes of QSPI RX trigger"]
    #[inline(always)]
    pub fn is_qspi_rx(&self) -> bool {
        *self == Trigsrcselect::QspiRx
    }
    #[doc = "Indexes of QSPI TX trigger"]
    #[inline(always)]
    pub fn is_qspi_tx(&self) -> bool {
        *self == Trigsrcselect::QspiTx
    }
}
#[doc = "Field `TRIGSRC` writer - Trigger Source"]
pub type TrigsrcW<'a, REG> = crate::FieldWriter<'a, REG, 7, Trigsrcselect>;
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
    #[doc = "DMA RTC timestamp trigger"]
    #[inline(always)]
    pub fn rtc_timestamp(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::RtcTimestamp)
    }
    #[doc = "DMAC ID for DCC0 register"]
    #[inline(always)]
    pub fn dsu_dcc0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::DsuDcc0)
    }
    #[doc = "DMAC ID for DCC1 register"]
    #[inline(always)]
    pub fn dsu_dcc1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::DsuDcc1)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom0Rx)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom0Tx)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom1Rx)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom1Tx)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom2Rx)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom2Tx)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom3_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom3Rx)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom3_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom3Tx)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom4_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom4Rx)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom4_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom4Tx)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom5_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom5Rx)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom5_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom5Tx)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom6_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom6Rx)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom6_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom6Tx)
    }
    #[doc = "Index of DMA RX trigger"]
    #[inline(always)]
    pub fn sercom7_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom7Rx)
    }
    #[doc = "Index of DMA TX trigger"]
    #[inline(always)]
    pub fn sercom7_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Sercom7Tx)
    }
    #[doc = "DMA CAN Debug Req"]
    #[inline(always)]
    pub fn can0_debug(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Can0Debug)
    }
    #[doc = "DMA CAN Debug Req"]
    #[inline(always)]
    pub fn can1_debug(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Can1Debug)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc0_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc1)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc2)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc3)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_4(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc4)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc0_mc_5(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc0Mc5)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc1_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc1Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc1_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc1Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc1_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc1Mc1)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc1_mc_2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc1Mc2)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc1_mc_3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc1Mc3)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc2_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc2Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc2_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc2Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc2_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc2Mc1)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc2_mc_2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc2Mc2)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc3_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc3Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc3_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc3Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc3_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc3Mc1)
    }
    #[doc = "DMA overflow/underflow/retrigger trigger"]
    #[inline(always)]
    pub fn tcc4_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc4Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc4_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc4Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tcc4_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tcc4Mc1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc0_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc0Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc0_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc0Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc0_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc0Mc1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc1_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc1Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc1_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc1Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc1_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc1Mc1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc2_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc2Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc2_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc2Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc2_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc2Mc1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc3_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc3Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc3_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc3Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc3_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc3Mc1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc4_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc4Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc4_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc4Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc4_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc4Mc1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc5_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc5Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc5_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc5Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc5_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc5Mc1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc6_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc6Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc6_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc6Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc6_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc6Mc1)
    }
    #[doc = "Indexes of DMA Overflow trigger"]
    #[inline(always)]
    pub fn tc7_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc7Ovf)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc7_mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc7Mc0)
    }
    #[doc = "Indexes of DMA Match/Compare triggers"]
    #[inline(always)]
    pub fn tc7_mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Tc7Mc1)
    }
    #[doc = "index of DMA RESRDY trigger"]
    #[inline(always)]
    pub fn adc0_resrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Adc0Resrdy)
    }
    #[doc = "Index of DMA SEQ trigger"]
    #[inline(always)]
    pub fn adc0_seq(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Adc0Seq)
    }
    #[doc = "Index of DMA RESRDY trigger"]
    #[inline(always)]
    pub fn adc1_resrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Adc1Resrdy)
    }
    #[doc = "Index of DMA SEQ trigger"]
    #[inline(always)]
    pub fn adc1_seq(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Adc1Seq)
    }
    #[doc = "DMA DAC Empty Req"]
    #[inline(always)]
    pub fn dac_empty_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::DacEmpty0)
    }
    #[doc = "DMA DAC Empty Req"]
    #[inline(always)]
    pub fn dac_empty_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::DacEmpty1)
    }
    #[doc = "DMA DAC Result Ready Req"]
    #[inline(always)]
    pub fn dac_resrdy_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::DacResrdy0)
    }
    #[doc = "DMA DAC Result Ready Req"]
    #[inline(always)]
    pub fn dac_resrdy_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::DacResrdy1)
    }
    #[doc = "Indexes of DMA RX triggers"]
    #[inline(always)]
    pub fn i2s_rx_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::I2sRx0)
    }
    #[doc = "Indexes of DMA RX triggers"]
    #[inline(always)]
    pub fn i2s_rx_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::I2sRx1)
    }
    #[doc = "Indexes of DMA TX triggers"]
    #[inline(always)]
    pub fn i2s_tx_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::I2sTx0)
    }
    #[doc = "Indexes of DMA TX triggers"]
    #[inline(always)]
    pub fn i2s_tx_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::I2sTx1)
    }
    #[doc = "Indexes of PCC RX trigger"]
    #[inline(always)]
    pub fn pcc_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::PccRx)
    }
    #[doc = "DMA DATA Write trigger"]
    #[inline(always)]
    pub fn aes_wr(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::AesWr)
    }
    #[doc = "DMA DATA Read trigger"]
    #[inline(always)]
    pub fn aes_rd(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::AesRd)
    }
    #[doc = "Indexes of QSPI RX trigger"]
    #[inline(always)]
    pub fn qspi_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::QspiRx)
    }
    #[doc = "Indexes of QSPI TX trigger"]
    #[inline(always)]
    pub fn qspi_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::QspiTx)
    }
}
#[doc = "Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigactselect {
    #[doc = "0: One trigger required for each block transfer"]
    Block = 0,
    #[doc = "2: One trigger required for each burst transfer"]
    Burst = 2,
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
            2 => Some(Trigactselect::Burst),
            3 => Some(Trigactselect::Transaction),
            _ => None,
        }
    }
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == Trigactselect::Block
    }
    #[doc = "One trigger required for each burst transfer"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == Trigactselect::Burst
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
    #[doc = "One trigger required for each burst transfer"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Burst)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Transaction)
    }
}
#[doc = "Burst Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Burstlenselect {
    #[doc = "0: Single-beat burst length"]
    Single = 0,
    #[doc = "1: 2-beats burst length"]
    _2beat = 1,
    #[doc = "2: 3-beats burst length"]
    _3beat = 2,
    #[doc = "3: 4-beats burst length"]
    _4beat = 3,
    #[doc = "4: 5-beats burst length"]
    _5beat = 4,
    #[doc = "5: 6-beats burst length"]
    _6beat = 5,
    #[doc = "6: 7-beats burst length"]
    _7beat = 6,
    #[doc = "7: 8-beats burst length"]
    _8beat = 7,
    #[doc = "8: 9-beats burst length"]
    _9beat = 8,
    #[doc = "9: 10-beats burst length"]
    _10beat = 9,
    #[doc = "10: 11-beats burst length"]
    _11beat = 10,
    #[doc = "11: 12-beats burst length"]
    _12beat = 11,
    #[doc = "12: 13-beats burst length"]
    _13beat = 12,
    #[doc = "13: 14-beats burst length"]
    _14beat = 13,
    #[doc = "14: 15-beats burst length"]
    _15beat = 14,
    #[doc = "15: 16-beats burst length"]
    _16beat = 15,
}
impl From<Burstlenselect> for u8 {
    #[inline(always)]
    fn from(variant: Burstlenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Burstlenselect {
    type Ux = u8;
}
impl crate::IsEnum for Burstlenselect {}
#[doc = "Field `BURSTLEN` reader - Burst Length"]
pub type BurstlenR = crate::FieldReader<Burstlenselect>;
impl BurstlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burstlenselect {
        match self.bits {
            0 => Burstlenselect::Single,
            1 => Burstlenselect::_2beat,
            2 => Burstlenselect::_3beat,
            3 => Burstlenselect::_4beat,
            4 => Burstlenselect::_5beat,
            5 => Burstlenselect::_6beat,
            6 => Burstlenselect::_7beat,
            7 => Burstlenselect::_8beat,
            8 => Burstlenselect::_9beat,
            9 => Burstlenselect::_10beat,
            10 => Burstlenselect::_11beat,
            11 => Burstlenselect::_12beat,
            12 => Burstlenselect::_13beat,
            13 => Burstlenselect::_14beat,
            14 => Burstlenselect::_15beat,
            15 => Burstlenselect::_16beat,
            _ => unreachable!(),
        }
    }
    #[doc = "Single-beat burst length"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Burstlenselect::Single
    }
    #[doc = "2-beats burst length"]
    #[inline(always)]
    pub fn is_2beat(&self) -> bool {
        *self == Burstlenselect::_2beat
    }
    #[doc = "3-beats burst length"]
    #[inline(always)]
    pub fn is_3beat(&self) -> bool {
        *self == Burstlenselect::_3beat
    }
    #[doc = "4-beats burst length"]
    #[inline(always)]
    pub fn is_4beat(&self) -> bool {
        *self == Burstlenselect::_4beat
    }
    #[doc = "5-beats burst length"]
    #[inline(always)]
    pub fn is_5beat(&self) -> bool {
        *self == Burstlenselect::_5beat
    }
    #[doc = "6-beats burst length"]
    #[inline(always)]
    pub fn is_6beat(&self) -> bool {
        *self == Burstlenselect::_6beat
    }
    #[doc = "7-beats burst length"]
    #[inline(always)]
    pub fn is_7beat(&self) -> bool {
        *self == Burstlenselect::_7beat
    }
    #[doc = "8-beats burst length"]
    #[inline(always)]
    pub fn is_8beat(&self) -> bool {
        *self == Burstlenselect::_8beat
    }
    #[doc = "9-beats burst length"]
    #[inline(always)]
    pub fn is_9beat(&self) -> bool {
        *self == Burstlenselect::_9beat
    }
    #[doc = "10-beats burst length"]
    #[inline(always)]
    pub fn is_10beat(&self) -> bool {
        *self == Burstlenselect::_10beat
    }
    #[doc = "11-beats burst length"]
    #[inline(always)]
    pub fn is_11beat(&self) -> bool {
        *self == Burstlenselect::_11beat
    }
    #[doc = "12-beats burst length"]
    #[inline(always)]
    pub fn is_12beat(&self) -> bool {
        *self == Burstlenselect::_12beat
    }
    #[doc = "13-beats burst length"]
    #[inline(always)]
    pub fn is_13beat(&self) -> bool {
        *self == Burstlenselect::_13beat
    }
    #[doc = "14-beats burst length"]
    #[inline(always)]
    pub fn is_14beat(&self) -> bool {
        *self == Burstlenselect::_14beat
    }
    #[doc = "15-beats burst length"]
    #[inline(always)]
    pub fn is_15beat(&self) -> bool {
        *self == Burstlenselect::_15beat
    }
    #[doc = "16-beats burst length"]
    #[inline(always)]
    pub fn is_16beat(&self) -> bool {
        *self == Burstlenselect::_16beat
    }
}
#[doc = "Field `BURSTLEN` writer - Burst Length"]
pub type BurstlenW<'a, REG> = crate::FieldWriter<'a, REG, 4, Burstlenselect, crate::Safe>;
impl<'a, REG> BurstlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-beat burst length"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::Single)
    }
    #[doc = "2-beats burst length"]
    #[inline(always)]
    pub fn _2beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_2beat)
    }
    #[doc = "3-beats burst length"]
    #[inline(always)]
    pub fn _3beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_3beat)
    }
    #[doc = "4-beats burst length"]
    #[inline(always)]
    pub fn _4beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_4beat)
    }
    #[doc = "5-beats burst length"]
    #[inline(always)]
    pub fn _5beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_5beat)
    }
    #[doc = "6-beats burst length"]
    #[inline(always)]
    pub fn _6beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_6beat)
    }
    #[doc = "7-beats burst length"]
    #[inline(always)]
    pub fn _7beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_7beat)
    }
    #[doc = "8-beats burst length"]
    #[inline(always)]
    pub fn _8beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_8beat)
    }
    #[doc = "9-beats burst length"]
    #[inline(always)]
    pub fn _9beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_9beat)
    }
    #[doc = "10-beats burst length"]
    #[inline(always)]
    pub fn _10beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_10beat)
    }
    #[doc = "11-beats burst length"]
    #[inline(always)]
    pub fn _11beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_11beat)
    }
    #[doc = "12-beats burst length"]
    #[inline(always)]
    pub fn _12beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_12beat)
    }
    #[doc = "13-beats burst length"]
    #[inline(always)]
    pub fn _13beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_13beat)
    }
    #[doc = "14-beats burst length"]
    #[inline(always)]
    pub fn _14beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_14beat)
    }
    #[doc = "15-beats burst length"]
    #[inline(always)]
    pub fn _15beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_15beat)
    }
    #[doc = "16-beats burst length"]
    #[inline(always)]
    pub fn _16beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_16beat)
    }
}
#[doc = "FIFO Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Thresholdselect {
    #[doc = "0: Destination write starts after each beat source address read"]
    _1beat = 0,
    #[doc = "1: Destination write starts after 2-beats source address read"]
    _2beats = 1,
    #[doc = "2: Destination write starts after 4-beats source address read"]
    _4beats = 2,
    #[doc = "3: Destination write starts after 8-beats source address read"]
    _8beats = 3,
}
impl From<Thresholdselect> for u8 {
    #[inline(always)]
    fn from(variant: Thresholdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Thresholdselect {
    type Ux = u8;
}
impl crate::IsEnum for Thresholdselect {}
#[doc = "Field `THRESHOLD` reader - FIFO Threshold"]
pub type ThresholdR = crate::FieldReader<Thresholdselect>;
impl ThresholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Thresholdselect {
        match self.bits {
            0 => Thresholdselect::_1beat,
            1 => Thresholdselect::_2beats,
            2 => Thresholdselect::_4beats,
            3 => Thresholdselect::_8beats,
            _ => unreachable!(),
        }
    }
    #[doc = "Destination write starts after each beat source address read"]
    #[inline(always)]
    pub fn is_1beat(&self) -> bool {
        *self == Thresholdselect::_1beat
    }
    #[doc = "Destination write starts after 2-beats source address read"]
    #[inline(always)]
    pub fn is_2beats(&self) -> bool {
        *self == Thresholdselect::_2beats
    }
    #[doc = "Destination write starts after 4-beats source address read"]
    #[inline(always)]
    pub fn is_4beats(&self) -> bool {
        *self == Thresholdselect::_4beats
    }
    #[doc = "Destination write starts after 8-beats source address read"]
    #[inline(always)]
    pub fn is_8beats(&self) -> bool {
        *self == Thresholdselect::_8beats
    }
}
#[doc = "Field `THRESHOLD` writer - FIFO Threshold"]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Thresholdselect, crate::Safe>;
impl<'a, REG> ThresholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Destination write starts after each beat source address read"]
    #[inline(always)]
    pub fn _1beat(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdselect::_1beat)
    }
    #[doc = "Destination write starts after 2-beats source address read"]
    #[inline(always)]
    pub fn _2beats(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdselect::_2beats)
    }
    #[doc = "Destination write starts after 4-beats source address read"]
    #[inline(always)]
    pub fn _4beats(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdselect::_4beats)
    }
    #[doc = "Destination write starts after 8-beats source address read"]
    #[inline(always)]
    pub fn _8beats(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdselect::_8beats)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TrigsrcR {
        TrigsrcR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&self) -> TrigactR {
        TrigactR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline(always)]
    pub fn burstlen(&self) -> BurstlenR {
        BurstlenR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<ChctrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<ChctrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<ChctrlaSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&mut self) -> TrigsrcW<ChctrlaSpec> {
        TrigsrcW::new(self, 8)
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&mut self) -> TrigactW<ChctrlaSpec> {
        TrigactW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline(always)]
    pub fn burstlen(&mut self) -> BurstlenW<ChctrlaSpec> {
        BurstlenW::new(self, 24)
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline(always)]
    pub fn threshold(&mut self) -> ThresholdW<ChctrlaSpec> {
        ThresholdW::new(self, 28)
    }
}
#[doc = "Channel n Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctrlaSpec;
impl crate::RegisterSpec for ChctrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctrla::R`](R) reader structure"]
impl crate::Readable for ChctrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`chctrla::W`](W) writer structure"]
impl crate::Writable for ChctrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTRLA to value 0"]
impl crate::Resettable for ChctrlaSpec {}
