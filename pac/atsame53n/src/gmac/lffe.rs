#[doc = "Reader of register LFFE"]
pub type R = crate::R<u32, super::LFFE>;
#[doc = "Reader of field `LFER`"]
pub type LFER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Length Field Frame Errors"]
    #[inline(always)]
    pub fn lfer(&self) -> LFER_R {
        LFER_R::new((self.bits & 0x03ff) as u16)
    }
}
