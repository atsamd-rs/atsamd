#[doc = "Reader of register BKIN"]
pub type R = crate::R<u32, super::BKIN>;
#[doc = "Reader of field `BKIN`"]
pub type BKIN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Backup Input Value"]
    #[inline(always)]
    pub fn bkin(&self) -> BKIN_R {
        BKIN_R::new((self.bits & 0xff) as u8)
    }
}
