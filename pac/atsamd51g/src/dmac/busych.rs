#[doc = "Reader of register BUSYCH"]
pub type R = crate::R<u32, super::BUSYCH>;
#[doc = "Reader of field `BUSYCH0`"]
pub type BUSYCH0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH1`"]
pub type BUSYCH1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH2`"]
pub type BUSYCH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH3`"]
pub type BUSYCH3_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH4`"]
pub type BUSYCH4_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH5`"]
pub type BUSYCH5_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH6`"]
pub type BUSYCH6_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH7`"]
pub type BUSYCH7_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH8`"]
pub type BUSYCH8_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH9`"]
pub type BUSYCH9_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH10`"]
pub type BUSYCH10_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH11`"]
pub type BUSYCH11_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH12`"]
pub type BUSYCH12_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH13`"]
pub type BUSYCH13_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH14`"]
pub type BUSYCH14_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH15`"]
pub type BUSYCH15_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH16`"]
pub type BUSYCH16_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH17`"]
pub type BUSYCH17_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH18`"]
pub type BUSYCH18_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH19`"]
pub type BUSYCH19_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH20`"]
pub type BUSYCH20_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH21`"]
pub type BUSYCH21_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH22`"]
pub type BUSYCH22_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH23`"]
pub type BUSYCH23_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH24`"]
pub type BUSYCH24_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH25`"]
pub type BUSYCH25_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH26`"]
pub type BUSYCH26_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH27`"]
pub type BUSYCH27_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH28`"]
pub type BUSYCH28_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH29`"]
pub type BUSYCH29_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH30`"]
pub type BUSYCH30_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH31`"]
pub type BUSYCH31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Busy Channel 0"]
    #[inline(always)]
    pub fn busych0(&self) -> BUSYCH0_R {
        BUSYCH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Busy Channel 1"]
    #[inline(always)]
    pub fn busych1(&self) -> BUSYCH1_R {
        BUSYCH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Busy Channel 2"]
    #[inline(always)]
    pub fn busych2(&self) -> BUSYCH2_R {
        BUSYCH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Busy Channel 3"]
    #[inline(always)]
    pub fn busych3(&self) -> BUSYCH3_R {
        BUSYCH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Busy Channel 4"]
    #[inline(always)]
    pub fn busych4(&self) -> BUSYCH4_R {
        BUSYCH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Busy Channel 5"]
    #[inline(always)]
    pub fn busych5(&self) -> BUSYCH5_R {
        BUSYCH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Busy Channel 6"]
    #[inline(always)]
    pub fn busych6(&self) -> BUSYCH6_R {
        BUSYCH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Busy Channel 7"]
    #[inline(always)]
    pub fn busych7(&self) -> BUSYCH7_R {
        BUSYCH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Busy Channel 8"]
    #[inline(always)]
    pub fn busych8(&self) -> BUSYCH8_R {
        BUSYCH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Busy Channel 9"]
    #[inline(always)]
    pub fn busych9(&self) -> BUSYCH9_R {
        BUSYCH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Busy Channel 10"]
    #[inline(always)]
    pub fn busych10(&self) -> BUSYCH10_R {
        BUSYCH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Busy Channel 11"]
    #[inline(always)]
    pub fn busych11(&self) -> BUSYCH11_R {
        BUSYCH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Busy Channel 12"]
    #[inline(always)]
    pub fn busych12(&self) -> BUSYCH12_R {
        BUSYCH12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Busy Channel 13"]
    #[inline(always)]
    pub fn busych13(&self) -> BUSYCH13_R {
        BUSYCH13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Busy Channel 14"]
    #[inline(always)]
    pub fn busych14(&self) -> BUSYCH14_R {
        BUSYCH14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Busy Channel 15"]
    #[inline(always)]
    pub fn busych15(&self) -> BUSYCH15_R {
        BUSYCH15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Busy Channel 16"]
    #[inline(always)]
    pub fn busych16(&self) -> BUSYCH16_R {
        BUSYCH16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Busy Channel 17"]
    #[inline(always)]
    pub fn busych17(&self) -> BUSYCH17_R {
        BUSYCH17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Busy Channel 18"]
    #[inline(always)]
    pub fn busych18(&self) -> BUSYCH18_R {
        BUSYCH18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Busy Channel 19"]
    #[inline(always)]
    pub fn busych19(&self) -> BUSYCH19_R {
        BUSYCH19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Busy Channel 20"]
    #[inline(always)]
    pub fn busych20(&self) -> BUSYCH20_R {
        BUSYCH20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Busy Channel 21"]
    #[inline(always)]
    pub fn busych21(&self) -> BUSYCH21_R {
        BUSYCH21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Busy Channel 22"]
    #[inline(always)]
    pub fn busych22(&self) -> BUSYCH22_R {
        BUSYCH22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Busy Channel 23"]
    #[inline(always)]
    pub fn busych23(&self) -> BUSYCH23_R {
        BUSYCH23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Busy Channel 24"]
    #[inline(always)]
    pub fn busych24(&self) -> BUSYCH24_R {
        BUSYCH24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Busy Channel 25"]
    #[inline(always)]
    pub fn busych25(&self) -> BUSYCH25_R {
        BUSYCH25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Busy Channel 26"]
    #[inline(always)]
    pub fn busych26(&self) -> BUSYCH26_R {
        BUSYCH26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Busy Channel 27"]
    #[inline(always)]
    pub fn busych27(&self) -> BUSYCH27_R {
        BUSYCH27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Busy Channel 28"]
    #[inline(always)]
    pub fn busych28(&self) -> BUSYCH28_R {
        BUSYCH28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Busy Channel 29"]
    #[inline(always)]
    pub fn busych29(&self) -> BUSYCH29_R {
        BUSYCH29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Busy Channel 30"]
    #[inline(always)]
    pub fn busych30(&self) -> BUSYCH30_R {
        BUSYCH30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Busy Channel 31"]
    #[inline(always)]
    pub fn busych31(&self) -> BUSYCH31_R {
        BUSYCH31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
