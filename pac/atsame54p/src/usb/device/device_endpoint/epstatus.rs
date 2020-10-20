#[doc = "Reader of register EPSTATUS"]
pub type R = crate::R<u8, super::EPSTATUS>;
#[doc = "Reader of field `DTGLOUT`"]
pub type DTGLOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTGLIN`"]
pub type DTGLIN_R = crate::R<bool, bool>;
#[doc = "Reader of field `CURBK`"]
pub type CURBK_R = crate::R<bool, bool>;
#[doc = "Reader of field `STALLRQ0`"]
pub type STALLRQ0_R = crate::R<bool, bool>;
#[doc = "Reader of field `STALLRQ1`"]
pub type STALLRQ1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BK0RDY`"]
pub type BK0RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BK1RDY`"]
pub type BK1RDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Data Toggle Out"]
    #[inline(always)]
    pub fn dtglout(&self) -> DTGLOUT_R {
        DTGLOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Toggle In"]
    #[inline(always)]
    pub fn dtglin(&self) -> DTGLIN_R {
        DTGLIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Current Bank"]
    #[inline(always)]
    pub fn curbk(&self) -> CURBK_R {
        CURBK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stall 0 Request"]
    #[inline(always)]
    pub fn stallrq0(&self) -> STALLRQ0_R {
        STALLRQ0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stall 1 Request"]
    #[inline(always)]
    pub fn stallrq1(&self) -> STALLRQ1_R {
        STALLRQ1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bank 0 ready"]
    #[inline(always)]
    pub fn bk0rdy(&self) -> BK0RDY_R {
        BK0RDY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bank 1 ready"]
    #[inline(always)]
    pub fn bk1rdy(&self) -> BK1RDY_R {
        BK1RDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
