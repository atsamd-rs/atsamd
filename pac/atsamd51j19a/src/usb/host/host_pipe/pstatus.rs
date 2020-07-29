#[doc = "Reader of register PSTATUS"]
pub type R = crate::R<u8, super::PSTATUS>;
#[doc = "Reader of field `DTGL`"]
pub type DTGL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CURBK`"]
pub type CURBK_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFREEZE`"]
pub type PFREEZE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BK0RDY`"]
pub type BK0RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BK1RDY`"]
pub type BK1RDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Data Toggle"]
    #[inline(always)]
    pub fn dtgl(&self) -> DTGL_R {
        DTGL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Current Bank"]
    #[inline(always)]
    pub fn curbk(&self) -> CURBK_R {
        CURBK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pipe Freeze"]
    #[inline(always)]
    pub fn pfreeze(&self) -> PFREEZE_R {
        PFREEZE_R::new(((self.bits >> 4) & 0x01) != 0)
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
