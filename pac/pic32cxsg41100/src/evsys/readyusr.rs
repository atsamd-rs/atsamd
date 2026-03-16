#[doc = "Register `READYUSR` reader"]
pub type R = crate::R<ReadyusrSpec>;
#[doc = "Field `READYUSR0` reader - Ready User for Channel 0"]
pub type Readyusr0R = crate::BitReader;
#[doc = "Field `READYUSR1` reader - Ready User for Channel 1"]
pub type Readyusr1R = crate::BitReader;
#[doc = "Field `READYUSR2` reader - Ready User for Channel 2"]
pub type Readyusr2R = crate::BitReader;
#[doc = "Field `READYUSR3` reader - Ready User for Channel 3"]
pub type Readyusr3R = crate::BitReader;
#[doc = "Field `READYUSR4` reader - Ready User for Channel 4"]
pub type Readyusr4R = crate::BitReader;
#[doc = "Field `READYUSR5` reader - Ready User for Channel 5"]
pub type Readyusr5R = crate::BitReader;
#[doc = "Field `READYUSR6` reader - Ready User for Channel 6"]
pub type Readyusr6R = crate::BitReader;
#[doc = "Field `READYUSR7` reader - Ready User for Channel 7"]
pub type Readyusr7R = crate::BitReader;
#[doc = "Field `READYUSR8` reader - Ready User for Channel 8"]
pub type Readyusr8R = crate::BitReader;
#[doc = "Field `READYUSR9` reader - Ready User for Channel 9"]
pub type Readyusr9R = crate::BitReader;
#[doc = "Field `READYUSR10` reader - Ready User for Channel 10"]
pub type Readyusr10R = crate::BitReader;
#[doc = "Field `READYUSR11` reader - Ready User for Channel 11"]
pub type Readyusr11R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Ready User for Channel 0"]
    #[inline(always)]
    pub fn readyusr0(&self) -> Readyusr0R {
        Readyusr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ready User for Channel 1"]
    #[inline(always)]
    pub fn readyusr1(&self) -> Readyusr1R {
        Readyusr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ready User for Channel 2"]
    #[inline(always)]
    pub fn readyusr2(&self) -> Readyusr2R {
        Readyusr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ready User for Channel 3"]
    #[inline(always)]
    pub fn readyusr3(&self) -> Readyusr3R {
        Readyusr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ready User for Channel 4"]
    #[inline(always)]
    pub fn readyusr4(&self) -> Readyusr4R {
        Readyusr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ready User for Channel 5"]
    #[inline(always)]
    pub fn readyusr5(&self) -> Readyusr5R {
        Readyusr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ready User for Channel 6"]
    #[inline(always)]
    pub fn readyusr6(&self) -> Readyusr6R {
        Readyusr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ready User for Channel 7"]
    #[inline(always)]
    pub fn readyusr7(&self) -> Readyusr7R {
        Readyusr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Ready User for Channel 8"]
    #[inline(always)]
    pub fn readyusr8(&self) -> Readyusr8R {
        Readyusr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ready User for Channel 9"]
    #[inline(always)]
    pub fn readyusr9(&self) -> Readyusr9R {
        Readyusr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Ready User for Channel 10"]
    #[inline(always)]
    pub fn readyusr10(&self) -> Readyusr10R {
        Readyusr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Ready User for Channel 11"]
    #[inline(always)]
    pub fn readyusr11(&self) -> Readyusr11R {
        Readyusr11R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Ready Users\n\nYou can [`read`](crate::Reg::read) this register and get [`readyusr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadyusrSpec;
impl crate::RegisterSpec for ReadyusrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readyusr::R`](R) reader structure"]
impl crate::Readable for ReadyusrSpec {}
#[doc = "`reset()` method sets READYUSR to value 0xffff_ffff"]
impl crate::Resettable for ReadyusrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
