#[doc = "Reader of register TXBTO"]
pub type R = crate::R<u32, super::TXBTO>;
#[doc = "Reader of field `TO0`"]
pub type TO0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO1`"]
pub type TO1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO2`"]
pub type TO2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO3`"]
pub type TO3_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO4`"]
pub type TO4_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO5`"]
pub type TO5_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO6`"]
pub type TO6_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO7`"]
pub type TO7_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO8`"]
pub type TO8_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO9`"]
pub type TO9_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO10`"]
pub type TO10_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO11`"]
pub type TO11_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO12`"]
pub type TO12_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO13`"]
pub type TO13_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO14`"]
pub type TO14_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO15`"]
pub type TO15_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO16`"]
pub type TO16_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO17`"]
pub type TO17_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO18`"]
pub type TO18_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO19`"]
pub type TO19_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO20`"]
pub type TO20_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO21`"]
pub type TO21_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO22`"]
pub type TO22_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO23`"]
pub type TO23_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO24`"]
pub type TO24_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO25`"]
pub type TO25_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO26`"]
pub type TO26_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO27`"]
pub type TO27_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO28`"]
pub type TO28_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO29`"]
pub type TO29_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO30`"]
pub type TO30_R = crate::R<bool, bool>;
#[doc = "Reader of field `TO31`"]
pub type TO31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transmission Occurred 0"]
    #[inline(always)]
    pub fn to0(&self) -> TO0_R {
        TO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmission Occurred 1"]
    #[inline(always)]
    pub fn to1(&self) -> TO1_R {
        TO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmission Occurred 2"]
    #[inline(always)]
    pub fn to2(&self) -> TO2_R {
        TO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmission Occurred 3"]
    #[inline(always)]
    pub fn to3(&self) -> TO3_R {
        TO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmission Occurred 4"]
    #[inline(always)]
    pub fn to4(&self) -> TO4_R {
        TO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmission Occurred 5"]
    #[inline(always)]
    pub fn to5(&self) -> TO5_R {
        TO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmission Occurred 6"]
    #[inline(always)]
    pub fn to6(&self) -> TO6_R {
        TO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmission Occurred 7"]
    #[inline(always)]
    pub fn to7(&self) -> TO7_R {
        TO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmission Occurred 8"]
    #[inline(always)]
    pub fn to8(&self) -> TO8_R {
        TO8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Occurred 9"]
    #[inline(always)]
    pub fn to9(&self) -> TO9_R {
        TO9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission Occurred 10"]
    #[inline(always)]
    pub fn to10(&self) -> TO10_R {
        TO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmission Occurred 11"]
    #[inline(always)]
    pub fn to11(&self) -> TO11_R {
        TO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmission Occurred 12"]
    #[inline(always)]
    pub fn to12(&self) -> TO12_R {
        TO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmission Occurred 13"]
    #[inline(always)]
    pub fn to13(&self) -> TO13_R {
        TO13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmission Occurred 14"]
    #[inline(always)]
    pub fn to14(&self) -> TO14_R {
        TO14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmission Occurred 15"]
    #[inline(always)]
    pub fn to15(&self) -> TO15_R {
        TO15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmission Occurred 16"]
    #[inline(always)]
    pub fn to16(&self) -> TO16_R {
        TO16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmission Occurred 17"]
    #[inline(always)]
    pub fn to17(&self) -> TO17_R {
        TO17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmission Occurred 18"]
    #[inline(always)]
    pub fn to18(&self) -> TO18_R {
        TO18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmission Occurred 19"]
    #[inline(always)]
    pub fn to19(&self) -> TO19_R {
        TO19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Transmission Occurred 20"]
    #[inline(always)]
    pub fn to20(&self) -> TO20_R {
        TO20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmission Occurred 21"]
    #[inline(always)]
    pub fn to21(&self) -> TO21_R {
        TO21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Transmission Occurred 22"]
    #[inline(always)]
    pub fn to22(&self) -> TO22_R {
        TO22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmission Occurred 23"]
    #[inline(always)]
    pub fn to23(&self) -> TO23_R {
        TO23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Transmission Occurred 24"]
    #[inline(always)]
    pub fn to24(&self) -> TO24_R {
        TO24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Transmission Occurred 25"]
    #[inline(always)]
    pub fn to25(&self) -> TO25_R {
        TO25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Transmission Occurred 26"]
    #[inline(always)]
    pub fn to26(&self) -> TO26_R {
        TO26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transmission Occurred 27"]
    #[inline(always)]
    pub fn to27(&self) -> TO27_R {
        TO27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Transmission Occurred 28"]
    #[inline(always)]
    pub fn to28(&self) -> TO28_R {
        TO28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Transmission Occurred 29"]
    #[inline(always)]
    pub fn to29(&self) -> TO29_R {
        TO29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Transmission Occurred 30"]
    #[inline(always)]
    pub fn to30(&self) -> TO30_R {
        TO30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Transmission Occurred 31"]
    #[inline(always)]
    pub fn to31(&self) -> TO31_R {
        TO31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
