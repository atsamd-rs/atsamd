#[doc = "Reader of register RRE"]
pub type R = crate::R<u32, super::RRE>;
#[doc = "Reader of field `RXRER`"]
pub type RXRER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Receive Resource Errors"]
    #[inline(always)]
    pub fn rxrer(&self) -> RXRER_R {
        RXRER_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
