#[doc = "Reader of register RSE"]
pub type R = crate::R<u32, super::RSE>;
#[doc = "Reader of field `RXSE`"]
pub type RXSE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Receive Symbol Errors"]
    #[inline(always)]
    pub fn rxse(&self) -> RXSE_R {
        RXSE_R::new((self.bits & 0x03ff) as u16)
    }
}
