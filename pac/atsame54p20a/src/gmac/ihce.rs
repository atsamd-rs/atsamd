#[doc = "Reader of register IHCE"]
pub type R = crate::R<u32, super::IHCE>;
#[doc = "Reader of field `HCKER`"]
pub type HCKER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - IP Header Checksum Errors"]
    #[inline(always)]
    pub fn hcker(&self) -> HCKER_R {
        HCKER_R::new((self.bits & 0xff) as u8)
    }
}
