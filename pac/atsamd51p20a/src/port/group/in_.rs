#[doc = "Reader of register IN"]
pub type R = crate::R<u32, super::IN>;
#[doc = "Reader of field `IN`"]
pub type IN_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PORT Data Input Value"]
    #[inline(always)]
    pub fn in_(&self) -> IN_R {
        IN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
