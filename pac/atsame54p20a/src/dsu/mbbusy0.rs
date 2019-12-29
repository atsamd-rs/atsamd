#[doc = "Reader of register MBBUSY0"]
pub type R = crate::R<u32, super::MBBUSY0>;
#[doc = "Reader of field `BUSY0`"]
pub type BUSY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY1`"]
pub type BUSY1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY2`"]
pub type BUSY2_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY3`"]
pub type BUSY3_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY4`"]
pub type BUSY4_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY5`"]
pub type BUSY5_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY6`"]
pub type BUSY6_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY7`"]
pub type BUSY7_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY8`"]
pub type BUSY8_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY9`"]
pub type BUSY9_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY10`"]
pub type BUSY10_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY11`"]
pub type BUSY11_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY12`"]
pub type BUSY12_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY13`"]
pub type BUSY13_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY14`"]
pub type BUSY14_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY15`"]
pub type BUSY15_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY16`"]
pub type BUSY16_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY17`"]
pub type BUSY17_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY18`"]
pub type BUSY18_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY19`"]
pub type BUSY19_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY20`"]
pub type BUSY20_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY21`"]
pub type BUSY21_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY22`"]
pub type BUSY22_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY23`"]
pub type BUSY23_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY24`"]
pub type BUSY24_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY25`"]
pub type BUSY25_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY26`"]
pub type BUSY26_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY27`"]
pub type BUSY27_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY28`"]
pub type BUSY28_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Memory 0 BIST Busy"]
    #[inline(always)]
    pub fn busy0(&self) -> BUSY0_R {
        BUSY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Memory 1 BIST Busy"]
    #[inline(always)]
    pub fn busy1(&self) -> BUSY1_R {
        BUSY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Memory 2 BIST Busy"]
    #[inline(always)]
    pub fn busy2(&self) -> BUSY2_R {
        BUSY2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Memory 3 BIST Busy"]
    #[inline(always)]
    pub fn busy3(&self) -> BUSY3_R {
        BUSY3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Memory 4 BIST Busy"]
    #[inline(always)]
    pub fn busy4(&self) -> BUSY4_R {
        BUSY4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Memory 5 BIST Busy"]
    #[inline(always)]
    pub fn busy5(&self) -> BUSY5_R {
        BUSY5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Memory 6 BIST Busy"]
    #[inline(always)]
    pub fn busy6(&self) -> BUSY6_R {
        BUSY6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Memory 7 BIST Busy"]
    #[inline(always)]
    pub fn busy7(&self) -> BUSY7_R {
        BUSY7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Memory 8 BIST Busy"]
    #[inline(always)]
    pub fn busy8(&self) -> BUSY8_R {
        BUSY8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Memory 9 BIST Busy"]
    #[inline(always)]
    pub fn busy9(&self) -> BUSY9_R {
        BUSY9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Memory 10 BIST Busy"]
    #[inline(always)]
    pub fn busy10(&self) -> BUSY10_R {
        BUSY10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Memory 11 BIST Busy"]
    #[inline(always)]
    pub fn busy11(&self) -> BUSY11_R {
        BUSY11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Memory 12 BIST Busy"]
    #[inline(always)]
    pub fn busy12(&self) -> BUSY12_R {
        BUSY12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Memory 13 BIST Busy"]
    #[inline(always)]
    pub fn busy13(&self) -> BUSY13_R {
        BUSY13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Memory 14 BIST Busy"]
    #[inline(always)]
    pub fn busy14(&self) -> BUSY14_R {
        BUSY14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Memory 15 BIST Busy"]
    #[inline(always)]
    pub fn busy15(&self) -> BUSY15_R {
        BUSY15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Memory 16 BIST Busy"]
    #[inline(always)]
    pub fn busy16(&self) -> BUSY16_R {
        BUSY16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Memory 17 BIST Busy"]
    #[inline(always)]
    pub fn busy17(&self) -> BUSY17_R {
        BUSY17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Memory 18 BIST Busy"]
    #[inline(always)]
    pub fn busy18(&self) -> BUSY18_R {
        BUSY18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Memory 19 BIST Busy"]
    #[inline(always)]
    pub fn busy19(&self) -> BUSY19_R {
        BUSY19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Memory 20 BIST Busy"]
    #[inline(always)]
    pub fn busy20(&self) -> BUSY20_R {
        BUSY20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Memory 21 BIST Busy"]
    #[inline(always)]
    pub fn busy21(&self) -> BUSY21_R {
        BUSY21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Memory 22 BIST Busy"]
    #[inline(always)]
    pub fn busy22(&self) -> BUSY22_R {
        BUSY22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Memory 23 BIST Busy"]
    #[inline(always)]
    pub fn busy23(&self) -> BUSY23_R {
        BUSY23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Memory 24 BIST Busy"]
    #[inline(always)]
    pub fn busy24(&self) -> BUSY24_R {
        BUSY24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Memory 25 BIST Busy"]
    #[inline(always)]
    pub fn busy25(&self) -> BUSY25_R {
        BUSY25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Memory 26 BIST Busy"]
    #[inline(always)]
    pub fn busy26(&self) -> BUSY26_R {
        BUSY26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Memory 27 BIST Busy"]
    #[inline(always)]
    pub fn busy27(&self) -> BUSY27_R {
        BUSY27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Memory 28 BIST Busy"]
    #[inline(always)]
    pub fn busy28(&self) -> BUSY28_R {
        BUSY28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
