#[doc = "Reader of register TXEFS"]
pub type R = crate::R<u32, super::TXEFS>;
#[doc = "Reader of field `EFFL`"]
pub type EFFL_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFGI`"]
pub type EFGI_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFPI`"]
pub type EFPI_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFF`"]
pub type EFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEFL`"]
pub type TEFL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:5 - Event FIFO Fill Level"]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index"]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Event FIFO Put Index"]
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Event FIFO Full"]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
