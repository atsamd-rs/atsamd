#[doc = "Reader of register PENDCH"]
pub type R = crate::R<u32, super::PENDCH>;
#[doc = "Reader of field `PENDCH0`"]
pub type PENDCH0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PENDCH1`"]
pub type PENDCH1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PENDCH2`"]
pub type PENDCH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `PENDCH3`"]
pub type PENDCH3_R = crate::R<bool, bool>;
#[doc = "Reader of field `PENDCH4`"]
pub type PENDCH4_R = crate::R<bool, bool>;
#[doc = "Reader of field `PENDCH5`"]
pub type PENDCH5_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Pending Channel 0"]
    #[inline(always)]
    pub fn pendch0(&self) -> PENDCH0_R {
        PENDCH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pending Channel 1"]
    #[inline(always)]
    pub fn pendch1(&self) -> PENDCH1_R {
        PENDCH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pending Channel 2"]
    #[inline(always)]
    pub fn pendch2(&self) -> PENDCH2_R {
        PENDCH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pending Channel 3"]
    #[inline(always)]
    pub fn pendch3(&self) -> PENDCH3_R {
        PENDCH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pending Channel 4"]
    #[inline(always)]
    pub fn pendch4(&self) -> PENDCH4_R {
        PENDCH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pending Channel 5"]
    #[inline(always)]
    pub fn pendch5(&self) -> PENDCH5_R {
        PENDCH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
