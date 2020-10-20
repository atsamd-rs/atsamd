#[doc = "Reader of register CHSTATUS"]
pub type R = crate::R<u8, super::CHSTATUS>;
#[doc = "Reader of field `RDYUSR`"]
pub type RDYUSR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH`"]
pub type BUSYCH_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Ready User"]
    #[inline(always)]
    pub fn rdyusr(&self) -> RDYUSR_R {
        RDYUSR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Busy Channel"]
    #[inline(always)]
    pub fn busych(&self) -> BUSYCH_R {
        BUSYCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
