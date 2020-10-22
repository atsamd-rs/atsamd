#[doc = "Reader of register EPINTSMRY"]
pub type R = crate::R<u16, super::EPINTSMRY>;
#[doc = "Reader of field `EPINT0`"]
pub type EPINT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPINT1`"]
pub type EPINT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPINT2`"]
pub type EPINT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPINT3`"]
pub type EPINT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPINT4`"]
pub type EPINT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPINT5`"]
pub type EPINT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPINT6`"]
pub type EPINT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPINT7`"]
pub type EPINT7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - End Point 0 Interrupt"]
    #[inline(always)]
    pub fn epint0(&self) -> EPINT0_R {
        EPINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End Point 1 Interrupt"]
    #[inline(always)]
    pub fn epint1(&self) -> EPINT1_R {
        EPINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End Point 2 Interrupt"]
    #[inline(always)]
    pub fn epint2(&self) -> EPINT2_R {
        EPINT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End Point 3 Interrupt"]
    #[inline(always)]
    pub fn epint3(&self) -> EPINT3_R {
        EPINT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End Point 4 Interrupt"]
    #[inline(always)]
    pub fn epint4(&self) -> EPINT4_R {
        EPINT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End Point 5 Interrupt"]
    #[inline(always)]
    pub fn epint5(&self) -> EPINT5_R {
        EPINT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - End Point 6 Interrupt"]
    #[inline(always)]
    pub fn epint6(&self) -> EPINT6_R {
        EPINT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - End Point 7 Interrupt"]
    #[inline(always)]
    pub fn epint7(&self) -> EPINT7_R {
        EPINT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
