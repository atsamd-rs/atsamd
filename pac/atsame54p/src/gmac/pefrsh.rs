#[doc = "Reader of register PEFRSH"]
pub type R = crate::R<u32, super::PEFRSH>;
#[doc = "Reader of field `RUD`"]
pub type RUD_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new((self.bits & 0xffff) as u16)
    }
}
