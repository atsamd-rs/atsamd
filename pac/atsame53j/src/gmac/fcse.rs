#[doc = "Reader of register FCSE"]
pub type R = crate::R<u32, super::FCSE>;
#[doc = "Reader of field `FCKR`"]
pub type FCKR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Frame Check Sequence Errors"]
    #[inline(always)]
    pub fn fckr(&self) -> FCKR_R {
        FCKR_R::new((self.bits & 0x03ff) as u16)
    }
}
