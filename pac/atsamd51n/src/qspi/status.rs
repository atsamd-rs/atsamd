#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSSTATUS`"]
pub type CSSTATUS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Chip Select"]
    #[inline(always)]
    pub fn csstatus(&self) -> CSSTATUS_R {
        CSSTATUS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
