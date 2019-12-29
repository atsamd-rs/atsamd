#[doc = "Reader of register CID3"]
pub type R = crate::R<u32, super::CID3>;
#[doc = "Reader of field `PREAMBLEB3`"]
pub type PREAMBLEB3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Preamble Byte 3"]
    #[inline(always)]
    pub fn preambleb3(&self) -> PREAMBLEB3_R {
        PREAMBLEB3_R::new((self.bits & 0xff) as u8)
    }
}
