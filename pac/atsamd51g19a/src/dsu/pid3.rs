#[doc = "Reader of register PID3"]
pub type R = crate::R<u32, super::PID3>;
#[doc = "Reader of field `CUSMOD`"]
pub type CUSMOD_R = crate::R<u8, u8>;
#[doc = "Reader of field `REVAND`"]
pub type REVAND_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - ARM CUSMOD"]
    #[inline(always)]
    pub fn cusmod(&self) -> CUSMOD_R {
        CUSMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Revision Number"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
