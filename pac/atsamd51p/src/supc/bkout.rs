#[doc = "Register `BKOUT` reader"]
pub type R = crate::R<BkoutSpec>;
#[doc = "Register `BKOUT` writer"]
pub type W = crate::W<BkoutSpec>;
#[doc = "Field `ENOUT0` reader - Enable OUT0"]
pub type Enout0R = crate::BitReader;
#[doc = "Field `ENOUT0` writer - Enable OUT0"]
pub type Enout0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENOUT1` reader - Enable OUT1"]
pub type Enout1R = crate::BitReader;
#[doc = "Field `ENOUT1` writer - Enable OUT1"]
pub type Enout1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLROUT0` reader - Clear OUT0"]
pub type Clrout0R = crate::BitReader;
#[doc = "Field `CLROUT0` writer - Clear OUT0"]
pub type Clrout0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLROUT1` reader - Clear OUT1"]
pub type Clrout1R = crate::BitReader;
#[doc = "Field `CLROUT1` writer - Clear OUT1"]
pub type Clrout1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETOUT0` reader - Set OUT0"]
pub type Setout0R = crate::BitReader;
#[doc = "Field `SETOUT0` writer - Set OUT0"]
pub type Setout0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETOUT1` reader - Set OUT1"]
pub type Setout1R = crate::BitReader;
#[doc = "Field `SETOUT1` writer - Set OUT1"]
pub type Setout1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCTGLOUT0` reader - RTC Toggle OUT0"]
pub type Rtctglout0R = crate::BitReader;
#[doc = "Field `RTCTGLOUT0` writer - RTC Toggle OUT0"]
pub type Rtctglout0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCTGLOUT1` reader - RTC Toggle OUT1"]
pub type Rtctglout1R = crate::BitReader;
#[doc = "Field `RTCTGLOUT1` writer - RTC Toggle OUT1"]
pub type Rtctglout1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable OUT0"]
    #[inline(always)]
    pub fn enout0(&self) -> Enout0R {
        Enout0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable OUT1"]
    #[inline(always)]
    pub fn enout1(&self) -> Enout1R {
        Enout1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear OUT0"]
    #[inline(always)]
    pub fn clrout0(&self) -> Clrout0R {
        Clrout0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear OUT1"]
    #[inline(always)]
    pub fn clrout1(&self) -> Clrout1R {
        Clrout1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Set OUT0"]
    #[inline(always)]
    pub fn setout0(&self) -> Setout0R {
        Setout0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set OUT1"]
    #[inline(always)]
    pub fn setout1(&self) -> Setout1R {
        Setout1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - RTC Toggle OUT0"]
    #[inline(always)]
    pub fn rtctglout0(&self) -> Rtctglout0R {
        Rtctglout0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RTC Toggle OUT1"]
    #[inline(always)]
    pub fn rtctglout1(&self) -> Rtctglout1R {
        Rtctglout1R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable OUT0"]
    #[inline(always)]
    pub fn enout0(&mut self) -> Enout0W<BkoutSpec> {
        Enout0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable OUT1"]
    #[inline(always)]
    pub fn enout1(&mut self) -> Enout1W<BkoutSpec> {
        Enout1W::new(self, 1)
    }
    #[doc = "Bit 8 - Clear OUT0"]
    #[inline(always)]
    pub fn clrout0(&mut self) -> Clrout0W<BkoutSpec> {
        Clrout0W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear OUT1"]
    #[inline(always)]
    pub fn clrout1(&mut self) -> Clrout1W<BkoutSpec> {
        Clrout1W::new(self, 9)
    }
    #[doc = "Bit 16 - Set OUT0"]
    #[inline(always)]
    pub fn setout0(&mut self) -> Setout0W<BkoutSpec> {
        Setout0W::new(self, 16)
    }
    #[doc = "Bit 17 - Set OUT1"]
    #[inline(always)]
    pub fn setout1(&mut self) -> Setout1W<BkoutSpec> {
        Setout1W::new(self, 17)
    }
    #[doc = "Bit 24 - RTC Toggle OUT0"]
    #[inline(always)]
    pub fn rtctglout0(&mut self) -> Rtctglout0W<BkoutSpec> {
        Rtctglout0W::new(self, 24)
    }
    #[doc = "Bit 25 - RTC Toggle OUT1"]
    #[inline(always)]
    pub fn rtctglout1(&mut self) -> Rtctglout1W<BkoutSpec> {
        Rtctglout1W::new(self, 25)
    }
}
#[doc = "Backup Output Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bkout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BkoutSpec;
impl crate::RegisterSpec for BkoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkout::R`](R) reader structure"]
impl crate::Readable for BkoutSpec {}
#[doc = "`write(|w| ..)` method takes [`bkout::W`](W) writer structure"]
impl crate::Writable for BkoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKOUT to value 0"]
impl crate::Resettable for BkoutSpec {}
