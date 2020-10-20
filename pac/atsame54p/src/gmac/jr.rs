#[doc = "Reader of register JR"]
pub type R = crate::R<u32, super::JR>;
#[doc = "Reader of field `JRX`"]
pub type JRX_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Jabbers Received"]
    #[inline(always)]
    pub fn jrx(&self) -> JRX_R {
        JRX_R::new((self.bits & 0x03ff) as u16)
    }
}
