#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `CSTS`"]
pub type CSTS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Cache Controller Status"]
    #[inline(always)]
    pub fn csts(&self) -> CSTS_R {
        CSTS_R::new((self.bits & 0x01) != 0)
    }
}
