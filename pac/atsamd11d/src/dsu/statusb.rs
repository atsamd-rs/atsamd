#[doc = "Reader of register STATUSB"]
pub type R = crate::R<u8, super::STATUSB>;
#[doc = "Reader of field `PROT`"]
pub type PROT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DBGPRES`"]
pub type DBGPRES_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCCD0`"]
pub type DCCD0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCCD1`"]
pub type DCCD1_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPE`"]
pub type HPE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Protected"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Debugger Present"]
    #[inline(always)]
    pub fn dbgpres(&self) -> DBGPRES_R {
        DBGPRES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Debug Communication Channel 0 Dirty"]
    #[inline(always)]
    pub fn dccd0(&self) -> DCCD0_R {
        DCCD0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Debug Communication Channel 1 Dirty"]
    #[inline(always)]
    pub fn dccd1(&self) -> DCCD1_R {
        DCCD1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hot-Plugging Enable"]
    #[inline(always)]
    pub fn hpe(&self) -> HPE_R {
        HPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
