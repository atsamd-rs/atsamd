#[doc = "Register `PATT` reader"]
pub type R = crate::R<PattSpec>;
#[doc = "Register `PATT` writer"]
pub type W = crate::W<PattSpec>;
#[doc = "Field `PGE0` reader - Pattern Generator 0 Output Enable"]
pub type Pge0R = crate::BitReader;
#[doc = "Field `PGE0` writer - Pattern Generator 0 Output Enable"]
pub type Pge0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGE1` reader - Pattern Generator 1 Output Enable"]
pub type Pge1R = crate::BitReader;
#[doc = "Field `PGE1` writer - Pattern Generator 1 Output Enable"]
pub type Pge1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGE2` reader - Pattern Generator 2 Output Enable"]
pub type Pge2R = crate::BitReader;
#[doc = "Field `PGE2` writer - Pattern Generator 2 Output Enable"]
pub type Pge2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGE3` reader - Pattern Generator 3 Output Enable"]
pub type Pge3R = crate::BitReader;
#[doc = "Field `PGE3` writer - Pattern Generator 3 Output Enable"]
pub type Pge3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGE4` reader - Pattern Generator 4 Output Enable"]
pub type Pge4R = crate::BitReader;
#[doc = "Field `PGE4` writer - Pattern Generator 4 Output Enable"]
pub type Pge4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGE5` reader - Pattern Generator 5 Output Enable"]
pub type Pge5R = crate::BitReader;
#[doc = "Field `PGE5` writer - Pattern Generator 5 Output Enable"]
pub type Pge5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGE6` reader - Pattern Generator 6 Output Enable"]
pub type Pge6R = crate::BitReader;
#[doc = "Field `PGE6` writer - Pattern Generator 6 Output Enable"]
pub type Pge6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGE7` reader - Pattern Generator 7 Output Enable"]
pub type Pge7R = crate::BitReader;
#[doc = "Field `PGE7` writer - Pattern Generator 7 Output Enable"]
pub type Pge7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGV0` reader - Pattern Generator 0 Output Value"]
pub type Pgv0R = crate::BitReader;
#[doc = "Field `PGV0` writer - Pattern Generator 0 Output Value"]
pub type Pgv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGV1` reader - Pattern Generator 1 Output Value"]
pub type Pgv1R = crate::BitReader;
#[doc = "Field `PGV1` writer - Pattern Generator 1 Output Value"]
pub type Pgv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGV2` reader - Pattern Generator 2 Output Value"]
pub type Pgv2R = crate::BitReader;
#[doc = "Field `PGV2` writer - Pattern Generator 2 Output Value"]
pub type Pgv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGV3` reader - Pattern Generator 3 Output Value"]
pub type Pgv3R = crate::BitReader;
#[doc = "Field `PGV3` writer - Pattern Generator 3 Output Value"]
pub type Pgv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGV4` reader - Pattern Generator 4 Output Value"]
pub type Pgv4R = crate::BitReader;
#[doc = "Field `PGV4` writer - Pattern Generator 4 Output Value"]
pub type Pgv4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGV5` reader - Pattern Generator 5 Output Value"]
pub type Pgv5R = crate::BitReader;
#[doc = "Field `PGV5` writer - Pattern Generator 5 Output Value"]
pub type Pgv5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGV6` reader - Pattern Generator 6 Output Value"]
pub type Pgv6R = crate::BitReader;
#[doc = "Field `PGV6` writer - Pattern Generator 6 Output Value"]
pub type Pgv6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGV7` reader - Pattern Generator 7 Output Value"]
pub type Pgv7R = crate::BitReader;
#[doc = "Field `PGV7` writer - Pattern Generator 7 Output Value"]
pub type Pgv7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    pub fn pge0(&self) -> Pge0R {
        Pge0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    pub fn pge1(&self) -> Pge1R {
        Pge1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    pub fn pge2(&self) -> Pge2R {
        Pge2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    pub fn pge3(&self) -> Pge3R {
        Pge3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    pub fn pge4(&self) -> Pge4R {
        Pge4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    pub fn pge5(&self) -> Pge5R {
        Pge5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    pub fn pge6(&self) -> Pge6R {
        Pge6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    pub fn pge7(&self) -> Pge7R {
        Pge7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Value"]
    #[inline(always)]
    pub fn pgv0(&self) -> Pgv0R {
        Pgv0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Value"]
    #[inline(always)]
    pub fn pgv1(&self) -> Pgv1R {
        Pgv1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Value"]
    #[inline(always)]
    pub fn pgv2(&self) -> Pgv2R {
        Pgv2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Value"]
    #[inline(always)]
    pub fn pgv3(&self) -> Pgv3R {
        Pgv3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Value"]
    #[inline(always)]
    pub fn pgv4(&self) -> Pgv4R {
        Pgv4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Value"]
    #[inline(always)]
    pub fn pgv5(&self) -> Pgv5R {
        Pgv5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Value"]
    #[inline(always)]
    pub fn pgv6(&self) -> Pgv6R {
        Pgv6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Value"]
    #[inline(always)]
    pub fn pgv7(&self) -> Pgv7R {
        Pgv7R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge0(&mut self) -> Pge0W<PattSpec> {
        Pge0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge1(&mut self) -> Pge1W<PattSpec> {
        Pge1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge2(&mut self) -> Pge2W<PattSpec> {
        Pge2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge3(&mut self) -> Pge3W<PattSpec> {
        Pge3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge4(&mut self) -> Pge4W<PattSpec> {
        Pge4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge5(&mut self) -> Pge5W<PattSpec> {
        Pge5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge6(&mut self) -> Pge6W<PattSpec> {
        Pge6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge7(&mut self) -> Pge7W<PattSpec> {
        Pge7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv0(&mut self) -> Pgv0W<PattSpec> {
        Pgv0W::new(self, 8)
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv1(&mut self) -> Pgv1W<PattSpec> {
        Pgv1W::new(self, 9)
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv2(&mut self) -> Pgv2W<PattSpec> {
        Pgv2W::new(self, 10)
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv3(&mut self) -> Pgv3W<PattSpec> {
        Pgv3W::new(self, 11)
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv4(&mut self) -> Pgv4W<PattSpec> {
        Pgv4W::new(self, 12)
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv5(&mut self) -> Pgv5W<PattSpec> {
        Pgv5W::new(self, 13)
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv6(&mut self) -> Pgv6W<PattSpec> {
        Pgv6W::new(self, 14)
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv7(&mut self) -> Pgv7W<PattSpec> {
        Pgv7W::new(self, 15)
    }
}
#[doc = "Pattern\n\nYou can [`read`](crate::Reg::read) this register and get [`patt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PattSpec;
impl crate::RegisterSpec for PattSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`patt::R`](R) reader structure"]
impl crate::Readable for PattSpec {}
#[doc = "`write(|w| ..)` method takes [`patt::W`](W) writer structure"]
impl crate::Writable for PattSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PATT to value 0"]
impl crate::Resettable for PattSpec {
    const RESET_VALUE: u16 = 0;
}
