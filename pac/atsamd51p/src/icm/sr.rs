#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWRMDIS`"]
pub type RAWRMDIS_R = crate::R<u8, u8>;
#[doc = "Reader of field `RMDIS`"]
pub type RMDIS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - ICM Controller Enable Register"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - RAW Region Monitoring Disabled Status"]
    #[inline(always)]
    pub fn rawrmdis(&self) -> RAWRMDIS_R {
        RAWRMDIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Monitoring Disabled Status"]
    #[inline(always)]
    pub fn rmdis(&self) -> RMDIS_R {
        RMDIS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
