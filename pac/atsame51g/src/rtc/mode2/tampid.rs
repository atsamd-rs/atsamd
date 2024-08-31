#[doc = "Register `TAMPID` reader"]
pub type R = crate::R<TampidSpec>;
#[doc = "Register `TAMPID` writer"]
pub type W = crate::W<TampidSpec>;
#[doc = "Field `TAMPID0` reader - Tamper Input 0 Detected"]
pub type Tampid0R = crate::BitReader;
#[doc = "Field `TAMPID0` writer - Tamper Input 0 Detected"]
pub type Tampid0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPID1` reader - Tamper Input 1 Detected"]
pub type Tampid1R = crate::BitReader;
#[doc = "Field `TAMPID1` writer - Tamper Input 1 Detected"]
pub type Tampid1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPID2` reader - Tamper Input 2 Detected"]
pub type Tampid2R = crate::BitReader;
#[doc = "Field `TAMPID2` writer - Tamper Input 2 Detected"]
pub type Tampid2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPID3` reader - Tamper Input 3 Detected"]
pub type Tampid3R = crate::BitReader;
#[doc = "Field `TAMPID3` writer - Tamper Input 3 Detected"]
pub type Tampid3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPID4` reader - Tamper Input 4 Detected"]
pub type Tampid4R = crate::BitReader;
#[doc = "Field `TAMPID4` writer - Tamper Input 4 Detected"]
pub type Tampid4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPEVT` reader - Tamper Event Detected"]
pub type TampevtR = crate::BitReader;
#[doc = "Field `TAMPEVT` writer - Tamper Event Detected"]
pub type TampevtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper Input 0 Detected"]
    #[inline(always)]
    pub fn tampid0(&self) -> Tampid0R {
        Tampid0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper Input 1 Detected"]
    #[inline(always)]
    pub fn tampid1(&self) -> Tampid1R {
        Tampid1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper Input 2 Detected"]
    #[inline(always)]
    pub fn tampid2(&self) -> Tampid2R {
        Tampid2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper Input 3 Detected"]
    #[inline(always)]
    pub fn tampid3(&self) -> Tampid3R {
        Tampid3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper Input 4 Detected"]
    #[inline(always)]
    pub fn tampid4(&self) -> Tampid4R {
        Tampid4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Tamper Event Detected"]
    #[inline(always)]
    pub fn tampevt(&self) -> TampevtR {
        TampevtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper Input 0 Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tampid0(&mut self) -> Tampid0W<TampidSpec> {
        Tampid0W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper Input 1 Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tampid1(&mut self) -> Tampid1W<TampidSpec> {
        Tampid1W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper Input 2 Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tampid2(&mut self) -> Tampid2W<TampidSpec> {
        Tampid2W::new(self, 2)
    }
    #[doc = "Bit 3 - Tamper Input 3 Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tampid3(&mut self) -> Tampid3W<TampidSpec> {
        Tampid3W::new(self, 3)
    }
    #[doc = "Bit 4 - Tamper Input 4 Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tampid4(&mut self) -> Tampid4W<TampidSpec> {
        Tampid4W::new(self, 4)
    }
    #[doc = "Bit 31 - Tamper Event Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tampevt(&mut self) -> TampevtW<TampidSpec> {
        TampevtW::new(self, 31)
    }
}
#[doc = "Tamper ID\n\nYou can [`read`](crate::Reg::read) this register and get [`tampid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampidSpec;
impl crate::RegisterSpec for TampidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tampid::R`](R) reader structure"]
impl crate::Readable for TampidSpec {}
#[doc = "`write(|w| ..)` method takes [`tampid::W`](W) writer structure"]
impl crate::Writable for TampidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMPID to value 0"]
impl crate::Resettable for TampidSpec {
    const RESET_VALUE: u32 = 0;
}
