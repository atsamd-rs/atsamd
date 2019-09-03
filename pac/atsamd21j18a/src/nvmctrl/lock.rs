#[doc = "Reader of register LOCK"]
pub type R = crate::R<u16, super::LOCK>;
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Region Lock Bits"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0xffff) as u16)
    }
}
