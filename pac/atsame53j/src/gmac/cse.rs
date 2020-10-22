#[doc = "Reader of register CSE"]
pub type R = crate::R<u32, super::CSE>;
#[doc = "Reader of field `CSR`"]
pub type CSR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Carrier Sense Error"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new((self.bits & 0x03ff) as u16)
    }
}
