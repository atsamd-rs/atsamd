#[doc = "Reader of register MFT"]
pub type R = crate::R<u32, super::MFT>;
#[doc = "Reader of field `MFTX`"]
pub type MFTX_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast Frames Transmitted without Error"]
    #[inline(always)]
    pub fn mftx(&self) -> MFTX_R {
        MFTX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
