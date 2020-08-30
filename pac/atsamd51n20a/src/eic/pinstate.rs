#[doc = "Reader of register PINSTATE"]
pub type R = crate::R<u32, super::PINSTATE>;
#[doc = "Reader of field `PINSTATE`"]
pub type PINSTATE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Pin State"]
    #[inline(always)]
    pub fn pinstate(&self) -> PINSTATE_R {
        PINSTATE_R::new((self.bits & 0xffff) as u16)
    }
}
