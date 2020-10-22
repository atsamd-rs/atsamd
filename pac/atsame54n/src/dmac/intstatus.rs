#[doc = "Reader of register INTSTATUS"]
pub type R = crate::R<u32, super::INTSTATUS>;
#[doc = "Reader of field `CHINT0`"]
pub type CHINT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT1`"]
pub type CHINT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT2`"]
pub type CHINT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT3`"]
pub type CHINT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT4`"]
pub type CHINT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT5`"]
pub type CHINT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT6`"]
pub type CHINT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT7`"]
pub type CHINT7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT8`"]
pub type CHINT8_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT9`"]
pub type CHINT9_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT10`"]
pub type CHINT10_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT11`"]
pub type CHINT11_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT12`"]
pub type CHINT12_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT13`"]
pub type CHINT13_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT14`"]
pub type CHINT14_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT15`"]
pub type CHINT15_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT16`"]
pub type CHINT16_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT17`"]
pub type CHINT17_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT18`"]
pub type CHINT18_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT19`"]
pub type CHINT19_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT20`"]
pub type CHINT20_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT21`"]
pub type CHINT21_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT22`"]
pub type CHINT22_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT23`"]
pub type CHINT23_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT24`"]
pub type CHINT24_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT25`"]
pub type CHINT25_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT26`"]
pub type CHINT26_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT27`"]
pub type CHINT27_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT28`"]
pub type CHINT28_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT29`"]
pub type CHINT29_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT30`"]
pub type CHINT30_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHINT31`"]
pub type CHINT31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Pending Interrupt"]
    #[inline(always)]
    pub fn chint0(&self) -> CHINT0_R {
        CHINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Pending Interrupt"]
    #[inline(always)]
    pub fn chint1(&self) -> CHINT1_R {
        CHINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Pending Interrupt"]
    #[inline(always)]
    pub fn chint2(&self) -> CHINT2_R {
        CHINT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Pending Interrupt"]
    #[inline(always)]
    pub fn chint3(&self) -> CHINT3_R {
        CHINT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Pending Interrupt"]
    #[inline(always)]
    pub fn chint4(&self) -> CHINT4_R {
        CHINT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Pending Interrupt"]
    #[inline(always)]
    pub fn chint5(&self) -> CHINT5_R {
        CHINT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Pending Interrupt"]
    #[inline(always)]
    pub fn chint6(&self) -> CHINT6_R {
        CHINT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Pending Interrupt"]
    #[inline(always)]
    pub fn chint7(&self) -> CHINT7_R {
        CHINT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Pending Interrupt"]
    #[inline(always)]
    pub fn chint8(&self) -> CHINT8_R {
        CHINT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Pending Interrupt"]
    #[inline(always)]
    pub fn chint9(&self) -> CHINT9_R {
        CHINT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Pending Interrupt"]
    #[inline(always)]
    pub fn chint10(&self) -> CHINT10_R {
        CHINT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Pending Interrupt"]
    #[inline(always)]
    pub fn chint11(&self) -> CHINT11_R {
        CHINT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 12 Pending Interrupt"]
    #[inline(always)]
    pub fn chint12(&self) -> CHINT12_R {
        CHINT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 13 Pending Interrupt"]
    #[inline(always)]
    pub fn chint13(&self) -> CHINT13_R {
        CHINT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 14 Pending Interrupt"]
    #[inline(always)]
    pub fn chint14(&self) -> CHINT14_R {
        CHINT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 15 Pending Interrupt"]
    #[inline(always)]
    pub fn chint15(&self) -> CHINT15_R {
        CHINT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 16 Pending Interrupt"]
    #[inline(always)]
    pub fn chint16(&self) -> CHINT16_R {
        CHINT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 17 Pending Interrupt"]
    #[inline(always)]
    pub fn chint17(&self) -> CHINT17_R {
        CHINT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 18 Pending Interrupt"]
    #[inline(always)]
    pub fn chint18(&self) -> CHINT18_R {
        CHINT18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 19 Pending Interrupt"]
    #[inline(always)]
    pub fn chint19(&self) -> CHINT19_R {
        CHINT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 20 Pending Interrupt"]
    #[inline(always)]
    pub fn chint20(&self) -> CHINT20_R {
        CHINT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 21 Pending Interrupt"]
    #[inline(always)]
    pub fn chint21(&self) -> CHINT21_R {
        CHINT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 22 Pending Interrupt"]
    #[inline(always)]
    pub fn chint22(&self) -> CHINT22_R {
        CHINT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 23 Pending Interrupt"]
    #[inline(always)]
    pub fn chint23(&self) -> CHINT23_R {
        CHINT23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel 24 Pending Interrupt"]
    #[inline(always)]
    pub fn chint24(&self) -> CHINT24_R {
        CHINT24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel 25 Pending Interrupt"]
    #[inline(always)]
    pub fn chint25(&self) -> CHINT25_R {
        CHINT25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel 26 Pending Interrupt"]
    #[inline(always)]
    pub fn chint26(&self) -> CHINT26_R {
        CHINT26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Channel 27 Pending Interrupt"]
    #[inline(always)]
    pub fn chint27(&self) -> CHINT27_R {
        CHINT27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Channel 28 Pending Interrupt"]
    #[inline(always)]
    pub fn chint28(&self) -> CHINT28_R {
        CHINT28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Channel 29 Pending Interrupt"]
    #[inline(always)]
    pub fn chint29(&self) -> CHINT29_R {
        CHINT29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel 30 Pending Interrupt"]
    #[inline(always)]
    pub fn chint30(&self) -> CHINT30_R {
        CHINT30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Channel 31 Pending Interrupt"]
    #[inline(always)]
    pub fn chint31(&self) -> CHINT31_R {
        CHINT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
