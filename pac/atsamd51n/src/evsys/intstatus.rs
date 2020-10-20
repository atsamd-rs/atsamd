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
}
