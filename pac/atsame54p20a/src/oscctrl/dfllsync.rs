#[doc = "Reader of register DFLLSYNC"]
pub type R = crate::R<u8, super::DFLLSYNC>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLCTRLB`"]
pub type DFLLCTRLB_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLVAL`"]
pub type DFLLVAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLMUL`"]
pub type DFLLMUL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DFLLCTRLB Synchronization Busy"]
    #[inline(always)]
    pub fn dfllctrlb(&self) -> DFLLCTRLB_R {
        DFLLCTRLB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DFLLVAL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllval(&self) -> DFLLVAL_R {
        DFLLVAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DFLLMUL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllmul(&self) -> DFLLMUL_R {
        DFLLMUL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
