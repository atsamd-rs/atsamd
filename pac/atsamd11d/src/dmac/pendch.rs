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
