#[doc = "Reader of register GTBFT1518"]
pub type R = crate::R<u32, super::GTBFT1518>;
#[doc = "Reader of field `NFTX`"]
pub type NFTX_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Greater than 1518 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
