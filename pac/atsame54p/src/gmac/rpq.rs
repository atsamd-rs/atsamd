#[doc = "Reader of register RPQ"]
pub type R = crate::R<u32, super::RPQ>;
#[doc = "Reader of field `RPQ`"]
pub type RPQ_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received Pause Quantum"]
    #[inline(always)]
    pub fn rpq(&self) -> RPQ_R {
        RPQ_R::new((self.bits & 0xffff) as u16)
    }
}
