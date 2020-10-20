#[doc = "Reader of register TXFQS"]
pub type R = crate::R<u32, super::TXFQS>;
#[doc = "Reader of field `TFFL`"]
pub type TFFL_R = crate::R<u8, u8>;
#[doc = "Reader of field `TFGI`"]
pub type TFGI_R = crate::R<u8, u8>;
#[doc = "Reader of field `TFQPI`"]
pub type TFQPI_R = crate::R<u8, u8>;
#[doc = "Reader of field `TFQF`"]
pub type TFQF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:5 - Tx FIFO Free Level"]
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Tx FIFO Get Index"]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/Queue Put Index"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/Queue Full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
