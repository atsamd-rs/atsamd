#[doc = "Reader of register STATUS"]
pub type R = crate::R<u16, super::STATUS>;
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRM`"]
pub type PRM_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOAD`"]
pub type LOAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSP`"]
pub type SUSP_R = crate::R<bool, bool>;
#[doc = "Reader of field `AFIRST`"]
pub type AFIRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `BPDIS`"]
pub type BPDIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOOTPROT`"]
pub type BOOTPROT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Ready to accept a command"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power Reduction Mode"]
    #[inline(always)]
    pub fn prm(&self) -> PRM_R {
        PRM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NVM Page Buffer Active Loading"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NVM Write Or Erase Operation Is Suspended"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BANKA First"]
    #[inline(always)]
    pub fn afirst(&self) -> AFIRST_R {
        AFIRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Boot Loader Protection Disable"]
    #[inline(always)]
    pub fn bpdis(&self) -> BPDIS_R {
        BPDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Boot Loader Protection Size"]
    #[inline(always)]
    pub fn bootprot(&self) -> BOOTPROT_R {
        BOOTPROT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
