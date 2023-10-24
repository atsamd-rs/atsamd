#[doc = "Register `SEESTAT` reader"]
pub type R = crate::R<SEESTAT_SPEC>;
#[doc = "Field `ASEES` reader - Active SmartEEPROM Sector"]
pub type ASEES_R = crate::BitReader;
#[doc = "Field `LOAD` reader - Page Buffer Loaded"]
pub type LOAD_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `LOCK` reader - SmartEEPROM Write Access Is Locked"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `RLOCK` reader - SmartEEPROM Write Access To Register Address Space Is Locked"]
pub type RLOCK_R = crate::BitReader;
#[doc = "Field `SBLK` reader - Blocks Number In a Sector"]
pub type SBLK_R = crate::FieldReader;
#[doc = "Field `PSZ` reader - SmartEEPROM Page Size"]
pub type PSZ_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Active SmartEEPROM Sector"]
    #[inline(always)]
    pub fn asees(&self) -> ASEES_R {
        ASEES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page Buffer Loaded"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SmartEEPROM Write Access Is Locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SmartEEPROM Write Access To Register Address Space Is Locked"]
    #[inline(always)]
    pub fn rlock(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Blocks Number In a Sector"]
    #[inline(always)]
    pub fn sblk(&self) -> SBLK_R {
        SBLK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - SmartEEPROM Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PSZ_R {
        PSZ_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "SmartEEPROM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seestat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEESTAT_SPEC;
impl crate::RegisterSpec for SEESTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seestat::R`](R) reader structure"]
impl crate::Readable for SEESTAT_SPEC {}
#[doc = "`reset()` method sets SEESTAT to value 0"]
impl crate::Resettable for SEESTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
