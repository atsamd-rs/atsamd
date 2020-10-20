#[doc = "Reader of register STATUS"]
pub type R = crate::R<u8, super::STATUS>;
#[doc = "Reader of field `SYNCBUSY`"]
pub type SYNCBUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 7 - Synchronization Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SYNCBUSY_R {
        SYNCBUSY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
