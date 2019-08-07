#[doc = "Reader of register CID0"]
pub type R = crate::R<u32, super::CID0>;
#[doc = "Reader of field `PREAMBLEB0`"]
pub type PREAMBLEB0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Preamble Byte 0"]
    #[inline(always)]
    pub fn preambleb0(&self) -> PREAMBLEB0_R {
        PREAMBLEB0_R::new((self.bits & 0xff) as u8)
    }
}
