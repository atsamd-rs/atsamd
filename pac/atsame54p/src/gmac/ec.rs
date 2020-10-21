#[doc = "Reader of register EC"]
pub type R = crate::R<u32, super::EC>;
#[doc = "Reader of field `XCOL`"]
pub type XCOL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Excessive Collisions"]
    #[inline(always)]
    pub fn xcol(&self) -> XCOL_R {
        XCOL_R::new((self.bits & 0x03ff) as u16)
    }
}
