#[doc = "Reader of register LC"]
pub type R = crate::R<u32, super::LC>;
#[doc = "Reader of field `LCOL`"]
pub type LCOL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Late Collisions"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new((self.bits & 0x03ff) as u16)
    }
}
