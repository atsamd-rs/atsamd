#[doc = "Register `ACTIVE` reader"]
pub type R = crate::R<ActiveSpec>;
#[doc = "Field `LVLEX0` reader - Level 0 Channel Trigger Request Executing"]
pub type Lvlex0R = crate::BitReader;
#[doc = "Field `LVLEX1` reader - Level 1 Channel Trigger Request Executing"]
pub type Lvlex1R = crate::BitReader;
#[doc = "Field `LVLEX2` reader - Level 2 Channel Trigger Request Executing"]
pub type Lvlex2R = crate::BitReader;
#[doc = "Field `LVLEX3` reader - Level 3 Channel Trigger Request Executing"]
pub type Lvlex3R = crate::BitReader;
#[doc = "Field `ID` reader - Active Channel ID"]
pub type IdR = crate::FieldReader;
#[doc = "Field `ABUSY` reader - Active Channel Busy"]
pub type AbusyR = crate::BitReader;
#[doc = "Field `BTCNT` reader - Active Channel Block Transfer Count"]
pub type BtcntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Level 0 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex0(&self) -> Lvlex0R {
        Lvlex0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level 1 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex1(&self) -> Lvlex1R {
        Lvlex1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level 2 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex2(&self) -> Lvlex2R {
        Lvlex2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level 3 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex3(&self) -> Lvlex3R {
        Lvlex3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Active Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Active Channel Busy"]
    #[inline(always)]
    pub fn abusy(&self) -> AbusyR {
        AbusyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Active Channel Block Transfer Count"]
    #[inline(always)]
    pub fn btcnt(&self) -> BtcntR {
        BtcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Active Channel and Levels\n\nYou can [`read`](crate::Reg::read) this register and get [`active::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActiveSpec;
impl crate::RegisterSpec for ActiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active::R`](R) reader structure"]
impl crate::Readable for ActiveSpec {}
#[doc = "`reset()` method sets ACTIVE to value 0"]
impl crate::Resettable for ActiveSpec {
    const RESET_VALUE: u32 = 0;
}
