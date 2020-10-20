#[doc = "Reader of register UFR"]
pub type R = crate::R<u32, super::UFR>;
#[doc = "Reader of field `UFRX`"]
pub type UFRX_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Undersize Frames Received"]
    #[inline(always)]
    pub fn ufrx(&self) -> UFRX_R {
        UFRX_R::new((self.bits & 0x03ff) as u16)
    }
}
