#[doc = "Reader of register ROE"]
pub type R = crate::R<u32, super::ROE>;
#[doc = "Reader of field `RXOVR`"]
pub type RXOVR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Receive Overruns"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new((self.bits & 0x03ff) as u16)
    }
}
