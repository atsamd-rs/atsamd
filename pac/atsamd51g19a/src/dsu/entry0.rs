#[doc = "Reader of register ENTRY0"]
pub type R = crate::R<u32, super::ENTRY0>;
#[doc = "Reader of field `EPRES`"]
pub type EPRES_R = crate::R<bool, bool>;
#[doc = "Reader of field `FMT`"]
pub type FMT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADDOFF`"]
pub type ADDOFF_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bit 0 - Entry Present"]
    #[inline(always)]
    pub fn epres(&self) -> EPRES_R {
        EPRES_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Format"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 12:31 - Address Offset"]
    #[inline(always)]
    pub fn addoff(&self) -> ADDOFF_R {
        ADDOFF_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
