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
}
