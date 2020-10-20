#[doc = "Reader of register CID2"]
pub type R = crate::R<u32, super::CID2>;
#[doc = "Reader of field `PREAMBLEB2`"]
pub type PREAMBLEB2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Preamble Byte 2"]
    #[inline(always)]
    pub fn preambleb2(&self) -> PREAMBLEB2_R {
        PREAMBLEB2_R::new((self.bits & 0xff) as u8)
    }
}
