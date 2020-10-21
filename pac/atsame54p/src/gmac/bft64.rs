#[doc = "Reader of register BFT64"]
pub type R = crate::R<u32, super::BFT64>;
#[doc = "Reader of field `NFTX`"]
pub type NFTX_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 64 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
