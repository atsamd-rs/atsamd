#[doc = "Reader of register ITATBCTR0"]
pub type R = crate::R<u32, super::ITATBCTR0>;
#[doc = "Reader of field `ATREADY`"]
pub type ATREADY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn atready(&self) -> ATREADY_R {
        ATREADY_R::new((self.bits & 0x01) != 0)
    }
}
