#[doc = "Reader of register TXBRP"]
pub type R = crate::R<u32, super::TXBRP>;
#[doc = "Reader of field `TRP0`"]
pub type TRP0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP1`"]
pub type TRP1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP2`"]
pub type TRP2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP3`"]
pub type TRP3_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP4`"]
pub type TRP4_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP5`"]
pub type TRP5_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP6`"]
pub type TRP6_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP7`"]
pub type TRP7_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP8`"]
pub type TRP8_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP9`"]
pub type TRP9_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP10`"]
pub type TRP10_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP11`"]
pub type TRP11_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP12`"]
pub type TRP12_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP13`"]
pub type TRP13_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP14`"]
pub type TRP14_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP15`"]
pub type TRP15_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP16`"]
pub type TRP16_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP17`"]
pub type TRP17_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP18`"]
pub type TRP18_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP19`"]
pub type TRP19_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP20`"]
pub type TRP20_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP21`"]
pub type TRP21_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP22`"]
pub type TRP22_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP23`"]
pub type TRP23_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP24`"]
pub type TRP24_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP25`"]
pub type TRP25_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP26`"]
pub type TRP26_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP27`"]
pub type TRP27_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP28`"]
pub type TRP28_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP29`"]
pub type TRP29_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP30`"]
pub type TRP30_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRP31`"]
pub type TRP31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transmission Request Pending 0"]
    #[inline(always)]
    pub fn trp0(&self) -> TRP0_R {
        TRP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmission Request Pending 1"]
    #[inline(always)]
    pub fn trp1(&self) -> TRP1_R {
        TRP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmission Request Pending 2"]
    #[inline(always)]
    pub fn trp2(&self) -> TRP2_R {
        TRP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmission Request Pending 3"]
    #[inline(always)]
    pub fn trp3(&self) -> TRP3_R {
        TRP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmission Request Pending 4"]
    #[inline(always)]
    pub fn trp4(&self) -> TRP4_R {
        TRP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmission Request Pending 5"]
    #[inline(always)]
    pub fn trp5(&self) -> TRP5_R {
        TRP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmission Request Pending 6"]
    #[inline(always)]
    pub fn trp6(&self) -> TRP6_R {
        TRP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmission Request Pending 7"]
    #[inline(always)]
    pub fn trp7(&self) -> TRP7_R {
        TRP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmission Request Pending 8"]
    #[inline(always)]
    pub fn trp8(&self) -> TRP8_R {
        TRP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Request Pending 9"]
    #[inline(always)]
    pub fn trp9(&self) -> TRP9_R {
        TRP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission Request Pending 10"]
    #[inline(always)]
    pub fn trp10(&self) -> TRP10_R {
        TRP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmission Request Pending 11"]
    #[inline(always)]
    pub fn trp11(&self) -> TRP11_R {
        TRP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmission Request Pending 12"]
    #[inline(always)]
    pub fn trp12(&self) -> TRP12_R {
        TRP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmission Request Pending 13"]
    #[inline(always)]
    pub fn trp13(&self) -> TRP13_R {
        TRP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmission Request Pending 14"]
    #[inline(always)]
    pub fn trp14(&self) -> TRP14_R {
        TRP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmission Request Pending 15"]
    #[inline(always)]
    pub fn trp15(&self) -> TRP15_R {
        TRP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmission Request Pending 16"]
    #[inline(always)]
    pub fn trp16(&self) -> TRP16_R {
        TRP16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmission Request Pending 17"]
    #[inline(always)]
    pub fn trp17(&self) -> TRP17_R {
        TRP17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmission Request Pending 18"]
    #[inline(always)]
    pub fn trp18(&self) -> TRP18_R {
        TRP18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmission Request Pending 19"]
    #[inline(always)]
    pub fn trp19(&self) -> TRP19_R {
        TRP19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Transmission Request Pending 20"]
    #[inline(always)]
    pub fn trp20(&self) -> TRP20_R {
        TRP20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmission Request Pending 21"]
    #[inline(always)]
    pub fn trp21(&self) -> TRP21_R {
        TRP21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Transmission Request Pending 22"]
    #[inline(always)]
    pub fn trp22(&self) -> TRP22_R {
        TRP22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmission Request Pending 23"]
    #[inline(always)]
    pub fn trp23(&self) -> TRP23_R {
        TRP23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Transmission Request Pending 24"]
    #[inline(always)]
    pub fn trp24(&self) -> TRP24_R {
        TRP24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Transmission Request Pending 25"]
    #[inline(always)]
    pub fn trp25(&self) -> TRP25_R {
        TRP25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Transmission Request Pending 26"]
    #[inline(always)]
    pub fn trp26(&self) -> TRP26_R {
        TRP26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transmission Request Pending 27"]
    #[inline(always)]
    pub fn trp27(&self) -> TRP27_R {
        TRP27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Transmission Request Pending 28"]
    #[inline(always)]
    pub fn trp28(&self) -> TRP28_R {
        TRP28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Transmission Request Pending 29"]
    #[inline(always)]
    pub fn trp29(&self) -> TRP29_R {
        TRP29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Transmission Request Pending 30"]
    #[inline(always)]
    pub fn trp30(&self) -> TRP30_R {
        TRP30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Transmission Request Pending 31"]
    #[inline(always)]
    pub fn trp31(&self) -> TRP31_R {
        TRP31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
