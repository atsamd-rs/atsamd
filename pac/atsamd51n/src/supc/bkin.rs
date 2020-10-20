#[doc = "Reader of register BKIN"]
pub type R = crate::R<u32, super::BKIN>;
#[doc = "Reader of field `BKIN0`"]
pub type BKIN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BKIN1`"]
pub type BKIN1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Backup Input 0"]
    #[inline(always)]
    pub fn bkin0(&self) -> BKIN0_R {
        BKIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Backup Input 1"]
    #[inline(always)]
    pub fn bkin1(&self) -> BKIN1_R {
        BKIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
