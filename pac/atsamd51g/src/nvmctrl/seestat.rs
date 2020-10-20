#[doc = "Reader of register SEESTAT"]
pub type R = crate::R<u32, super::SEESTAT>;
#[doc = "Reader of field `ASEES`"]
pub type ASEES_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOAD`"]
pub type LOAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `RLOCK`"]
pub type RLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `SBLK`"]
pub type SBLK_R = crate::R<u8, u8>;
#[doc = "Reader of field `PSZ`"]
pub type PSZ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Active SmartEEPROM Sector"]
    #[inline(always)]
    pub fn asees(&self) -> ASEES_R {
        ASEES_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Page Buffer Loaded"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SmartEEPROM Write Access Is Locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SmartEEPROM Write Access To Register Address Space Is Locked"]
    #[inline(always)]
    pub fn rlock(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Blocks Number In a Sector"]
    #[inline(always)]
    pub fn sblk(&self) -> SBLK_R {
        SBLK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - SmartEEPROM Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PSZ_R {
        PSZ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
