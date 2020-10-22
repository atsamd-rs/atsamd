#[doc = "Reader of register RLPITR"]
pub type R = crate::R<u32, super::RLPITR>;
#[doc = "Reader of field `RLPITR`"]
pub type RLPITR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count number of times transition from rx normal idle to low power idle"]
    #[inline(always)]
    pub fn rlpitr(&self) -> RLPITR_R {
        RLPITR_R::new((self.bits & 0xffff) as u16)
    }
}
