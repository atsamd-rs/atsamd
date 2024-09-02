#[doc = "Register `SEESTAT` reader"]
pub type R = crate::R<SeestatSpec>;
#[doc = "Field `ASEES` reader - Active SmartEEPROM Sector"]
pub type AseesR = crate::BitReader;
#[doc = "Field `LOAD` reader - Page Buffer Loaded"]
pub type LoadR = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `LOCK` reader - SmartEEPROM Write Access Is Locked"]
pub type LockR = crate::BitReader;
#[doc = "Field `RLOCK` reader - SmartEEPROM Write Access To Register Address Space Is Locked"]
pub type RlockR = crate::BitReader;
#[doc = "Field `SBLK` reader - Blocks Number In a Sector"]
pub type SblkR = crate::FieldReader;
#[doc = "Field `PSZ` reader - SmartEEPROM Page Size"]
pub type PszR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Active SmartEEPROM Sector"]
    #[inline(always)]
    pub fn asees(&self) -> AseesR {
        AseesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page Buffer Loaded"]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SmartEEPROM Write Access Is Locked"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SmartEEPROM Write Access To Register Address Space Is Locked"]
    #[inline(always)]
    pub fn rlock(&self) -> RlockR {
        RlockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Blocks Number In a Sector"]
    #[inline(always)]
    pub fn sblk(&self) -> SblkR {
        SblkR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - SmartEEPROM Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PszR {
        PszR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "SmartEEPROM Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`seestat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeestatSpec;
impl crate::RegisterSpec for SeestatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seestat::R`](R) reader structure"]
impl crate::Readable for SeestatSpec {}
#[doc = "`reset()` method sets SEESTAT to value 0"]
impl crate::Resettable for SeestatSpec {
    const RESET_VALUE: u32 = 0;
}
