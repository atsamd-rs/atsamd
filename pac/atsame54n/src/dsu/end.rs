#[doc = "Reader of register END"]
pub type R = crate::R<u32, super::END>;
#[doc = "Reader of field `END`"]
pub type END_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - End Marker"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
