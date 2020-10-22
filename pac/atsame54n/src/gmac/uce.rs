#[doc = "Reader of register UCE"]
pub type R = crate::R<u32, super::UCE>;
#[doc = "Reader of field `UCKER`"]
pub type UCKER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - UDP Checksum Errors"]
    #[inline(always)]
    pub fn ucker(&self) -> UCKER_R {
        UCKER_R::new((self.bits & 0xff) as u8)
    }
}
