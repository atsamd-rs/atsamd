#[doc = "Reader of register TCE"]
pub type R = crate::R<u32, super::TCE>;
#[doc = "Reader of field `TCKER`"]
pub type TCKER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - TCP Checksum Errors"]
    #[inline(always)]
    pub fn tcker(&self) -> TCKER_R {
        TCKER_R::new((self.bits & 0xff) as u8)
    }
}
