#[doc = "Register `PENDCH` reader"]
pub type R = crate::R<PendchSpec>;
#[doc = "Field `PENDCH0` reader - Pending Channel 0"]
pub type Pendch0R = crate::BitReader;
#[doc = "Field `PENDCH1` reader - Pending Channel 1"]
pub type Pendch1R = crate::BitReader;
#[doc = "Field `PENDCH2` reader - Pending Channel 2"]
pub type Pendch2R = crate::BitReader;
#[doc = "Field `PENDCH3` reader - Pending Channel 3"]
pub type Pendch3R = crate::BitReader;
#[doc = "Field `PENDCH4` reader - Pending Channel 4"]
pub type Pendch4R = crate::BitReader;
#[doc = "Field `PENDCH5` reader - Pending Channel 5"]
pub type Pendch5R = crate::BitReader;
#[doc = "Field `PENDCH6` reader - Pending Channel 6"]
pub type Pendch6R = crate::BitReader;
#[doc = "Field `PENDCH7` reader - Pending Channel 7"]
pub type Pendch7R = crate::BitReader;
#[doc = "Field `PENDCH8` reader - Pending Channel 8"]
pub type Pendch8R = crate::BitReader;
#[doc = "Field `PENDCH9` reader - Pending Channel 9"]
pub type Pendch9R = crate::BitReader;
#[doc = "Field `PENDCH10` reader - Pending Channel 10"]
pub type Pendch10R = crate::BitReader;
#[doc = "Field `PENDCH11` reader - Pending Channel 11"]
pub type Pendch11R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Pending Channel 0"]
    #[inline(always)]
    pub fn pendch0(&self) -> Pendch0R {
        Pendch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending Channel 1"]
    #[inline(always)]
    pub fn pendch1(&self) -> Pendch1R {
        Pendch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending Channel 2"]
    #[inline(always)]
    pub fn pendch2(&self) -> Pendch2R {
        Pendch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending Channel 3"]
    #[inline(always)]
    pub fn pendch3(&self) -> Pendch3R {
        Pendch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending Channel 4"]
    #[inline(always)]
    pub fn pendch4(&self) -> Pendch4R {
        Pendch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending Channel 5"]
    #[inline(always)]
    pub fn pendch5(&self) -> Pendch5R {
        Pendch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending Channel 6"]
    #[inline(always)]
    pub fn pendch6(&self) -> Pendch6R {
        Pendch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending Channel 7"]
    #[inline(always)]
    pub fn pendch7(&self) -> Pendch7R {
        Pendch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending Channel 8"]
    #[inline(always)]
    pub fn pendch8(&self) -> Pendch8R {
        Pendch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending Channel 9"]
    #[inline(always)]
    pub fn pendch9(&self) -> Pendch9R {
        Pendch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pending Channel 10"]
    #[inline(always)]
    pub fn pendch10(&self) -> Pendch10R {
        Pendch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending Channel 11"]
    #[inline(always)]
    pub fn pendch11(&self) -> Pendch11R {
        Pendch11R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Pending Channels\n\nYou can [`read`](crate::Reg::read) this register and get [`pendch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PendchSpec;
impl crate::RegisterSpec for PendchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pendch::R`](R) reader structure"]
impl crate::Readable for PendchSpec {}
#[doc = "`reset()` method sets PENDCH to value 0"]
impl crate::Resettable for PendchSpec {
    const RESET_VALUE: u32 = 0;
}
