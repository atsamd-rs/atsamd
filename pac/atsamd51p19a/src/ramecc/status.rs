#[doc = "Reader of register STATUS"]
pub type R = crate::R<u8, super::STATUS>;
#[doc = "Reader of field `ECCDIS`"]
pub type ECCDIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ECC Disable"]
    #[inline(always)]
    pub fn eccdis(&self) -> ECCDIS_R {
        ECCDIS_R::new((self.bits & 0x01) != 0)
    }
}
