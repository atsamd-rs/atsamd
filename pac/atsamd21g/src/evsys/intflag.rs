#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `OVR0` reader - Channel 0 Overrun"]
pub type Ovr0R = crate::BitReader;
#[doc = "Field `OVR0` writer - Channel 0 Overrun"]
pub type Ovr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR1` reader - Channel 1 Overrun"]
pub type Ovr1R = crate::BitReader;
#[doc = "Field `OVR1` writer - Channel 1 Overrun"]
pub type Ovr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR2` reader - Channel 2 Overrun"]
pub type Ovr2R = crate::BitReader;
#[doc = "Field `OVR2` writer - Channel 2 Overrun"]
pub type Ovr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR3` reader - Channel 3 Overrun"]
pub type Ovr3R = crate::BitReader;
#[doc = "Field `OVR3` writer - Channel 3 Overrun"]
pub type Ovr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR4` reader - Channel 4 Overrun"]
pub type Ovr4R = crate::BitReader;
#[doc = "Field `OVR4` writer - Channel 4 Overrun"]
pub type Ovr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR5` reader - Channel 5 Overrun"]
pub type Ovr5R = crate::BitReader;
#[doc = "Field `OVR5` writer - Channel 5 Overrun"]
pub type Ovr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR6` reader - Channel 6 Overrun"]
pub type Ovr6R = crate::BitReader;
#[doc = "Field `OVR6` writer - Channel 6 Overrun"]
pub type Ovr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR7` reader - Channel 7 Overrun"]
pub type Ovr7R = crate::BitReader;
#[doc = "Field `OVR7` writer - Channel 7 Overrun"]
pub type Ovr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD0` reader - Channel 0 Event Detection"]
pub type Evd0R = crate::BitReader;
#[doc = "Field `EVD0` writer - Channel 0 Event Detection"]
pub type Evd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD1` reader - Channel 1 Event Detection"]
pub type Evd1R = crate::BitReader;
#[doc = "Field `EVD1` writer - Channel 1 Event Detection"]
pub type Evd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD2` reader - Channel 2 Event Detection"]
pub type Evd2R = crate::BitReader;
#[doc = "Field `EVD2` writer - Channel 2 Event Detection"]
pub type Evd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD3` reader - Channel 3 Event Detection"]
pub type Evd3R = crate::BitReader;
#[doc = "Field `EVD3` writer - Channel 3 Event Detection"]
pub type Evd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD4` reader - Channel 4 Event Detection"]
pub type Evd4R = crate::BitReader;
#[doc = "Field `EVD4` writer - Channel 4 Event Detection"]
pub type Evd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD5` reader - Channel 5 Event Detection"]
pub type Evd5R = crate::BitReader;
#[doc = "Field `EVD5` writer - Channel 5 Event Detection"]
pub type Evd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD6` reader - Channel 6 Event Detection"]
pub type Evd6R = crate::BitReader;
#[doc = "Field `EVD6` writer - Channel 6 Event Detection"]
pub type Evd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD7` reader - Channel 7 Event Detection"]
pub type Evd7R = crate::BitReader;
#[doc = "Field `EVD7` writer - Channel 7 Event Detection"]
pub type Evd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR8` reader - Channel 8 Overrun"]
pub type Ovr8R = crate::BitReader;
#[doc = "Field `OVR8` writer - Channel 8 Overrun"]
pub type Ovr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR9` reader - Channel 9 Overrun"]
pub type Ovr9R = crate::BitReader;
#[doc = "Field `OVR9` writer - Channel 9 Overrun"]
pub type Ovr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR10` reader - Channel 10 Overrun"]
pub type Ovr10R = crate::BitReader;
#[doc = "Field `OVR10` writer - Channel 10 Overrun"]
pub type Ovr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR11` reader - Channel 11 Overrun"]
pub type Ovr11R = crate::BitReader;
#[doc = "Field `OVR11` writer - Channel 11 Overrun"]
pub type Ovr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD8` reader - Channel 8 Event Detection"]
pub type Evd8R = crate::BitReader;
#[doc = "Field `EVD8` writer - Channel 8 Event Detection"]
pub type Evd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD9` reader - Channel 9 Event Detection"]
pub type Evd9R = crate::BitReader;
#[doc = "Field `EVD9` writer - Channel 9 Event Detection"]
pub type Evd9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD10` reader - Channel 10 Event Detection"]
pub type Evd10R = crate::BitReader;
#[doc = "Field `EVD10` writer - Channel 10 Event Detection"]
pub type Evd10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD11` reader - Channel 11 Event Detection"]
pub type Evd11R = crate::BitReader;
#[doc = "Field `EVD11` writer - Channel 11 Event Detection"]
pub type Evd11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 Overrun"]
    #[inline(always)]
    pub fn ovr0(&self) -> Ovr0R {
        Ovr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Overrun"]
    #[inline(always)]
    pub fn ovr1(&self) -> Ovr1R {
        Ovr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Overrun"]
    #[inline(always)]
    pub fn ovr2(&self) -> Ovr2R {
        Ovr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Overrun"]
    #[inline(always)]
    pub fn ovr3(&self) -> Ovr3R {
        Ovr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Overrun"]
    #[inline(always)]
    pub fn ovr4(&self) -> Ovr4R {
        Ovr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Overrun"]
    #[inline(always)]
    pub fn ovr5(&self) -> Ovr5R {
        Ovr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Overrun"]
    #[inline(always)]
    pub fn ovr6(&self) -> Ovr6R {
        Ovr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Overrun"]
    #[inline(always)]
    pub fn ovr7(&self) -> Ovr7R {
        Ovr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Event Detection"]
    #[inline(always)]
    pub fn evd0(&self) -> Evd0R {
        Evd0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Event Detection"]
    #[inline(always)]
    pub fn evd1(&self) -> Evd1R {
        Evd1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Event Detection"]
    #[inline(always)]
    pub fn evd2(&self) -> Evd2R {
        Evd2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Event Detection"]
    #[inline(always)]
    pub fn evd3(&self) -> Evd3R {
        Evd3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Event Detection"]
    #[inline(always)]
    pub fn evd4(&self) -> Evd4R {
        Evd4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Event Detection"]
    #[inline(always)]
    pub fn evd5(&self) -> Evd5R {
        Evd5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 6 Event Detection"]
    #[inline(always)]
    pub fn evd6(&self) -> Evd6R {
        Evd6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 7 Event Detection"]
    #[inline(always)]
    pub fn evd7(&self) -> Evd7R {
        Evd7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 8 Overrun"]
    #[inline(always)]
    pub fn ovr8(&self) -> Ovr8R {
        Ovr8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 9 Overrun"]
    #[inline(always)]
    pub fn ovr9(&self) -> Ovr9R {
        Ovr9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 10 Overrun"]
    #[inline(always)]
    pub fn ovr10(&self) -> Ovr10R {
        Ovr10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 11 Overrun"]
    #[inline(always)]
    pub fn ovr11(&self) -> Ovr11R {
        Ovr11R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 8 Event Detection"]
    #[inline(always)]
    pub fn evd8(&self) -> Evd8R {
        Evd8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 9 Event Detection"]
    #[inline(always)]
    pub fn evd9(&self) -> Evd9R {
        Evd9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 10 Event Detection"]
    #[inline(always)]
    pub fn evd10(&self) -> Evd10R {
        Evd10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 11 Event Detection"]
    #[inline(always)]
    pub fn evd11(&self) -> Evd11R {
        Evd11R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr0(&mut self) -> Ovr0W<IntflagSpec> {
        Ovr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr1(&mut self) -> Ovr1W<IntflagSpec> {
        Ovr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr2(&mut self) -> Ovr2W<IntflagSpec> {
        Ovr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr3(&mut self) -> Ovr3W<IntflagSpec> {
        Ovr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr4(&mut self) -> Ovr4W<IntflagSpec> {
        Ovr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr5(&mut self) -> Ovr5W<IntflagSpec> {
        Ovr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr6(&mut self) -> Ovr6W<IntflagSpec> {
        Ovr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr7(&mut self) -> Ovr7W<IntflagSpec> {
        Ovr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 0 Event Detection"]
    #[inline(always)]
    #[must_use]
    pub fn evd0(&mut self) -> Evd0W<IntflagSpec> {
        Evd0W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 Event Detection"]
    #[inline(always)]
    #[must_use]
    pub fn evd1(&mut self) -> Evd1W<IntflagSpec> {
        Evd1W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 Event Detection"]
    #[inline(always)]
    #[must_use]
    pub fn evd2(&mut self) -> Evd2W<IntflagSpec> {
        Evd2W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 Event Detection"]
    #[inline(always)]
    #[must_use]
    pub fn evd3(&mut self) -> Evd3W<IntflagSpec> {
        Evd3W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 4 Event Detection"]
    #[inline(always)]
    #[must_use]
    pub fn evd4(&mut self) -> Evd4W<IntflagSpec> {
        Evd4W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 5 Event Detection"]
    #[inline(always)]
    #[must_use]
    pub fn evd5(&mut self) -> Evd5W<IntflagSpec> {
        Evd5W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel 6 Event Detection"]
    #[inline(always)]
    #[must_use]
    pub fn evd6(&mut self) -> Evd6W<IntflagSpec> {
        Evd6W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel 7 Event Detection"]
    #[inline(always)]
    #[must_use]
    pub fn evd7(&mut self) -> Evd7W<IntflagSpec> {
        Evd7W::new(self, 15)
    }
    #[doc = "Bit 16 - Channel 8 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr8(&mut self) -> Ovr8W<IntflagSpec> {
        Ovr8W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel 9 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr9(&mut self) -> Ovr9W<IntflagSpec> {
        Ovr9W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel 10 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr10(&mut self) -> Ovr10W<IntflagSpec> {
        Ovr10W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel 11 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr11(&mut self) -> Ovr11W<IntflagSpec> {
        Ovr11W::new(self, 19)
    }
    #[doc = "Bit 24 - Channel 8 Event Detection"]
    #[inline(always)]
    #[must_use]
    pub fn evd8(&mut self) -> Evd8W<IntflagSpec> {
        Evd8W::new(self, 24)
    }
    #[doc = "Bit 25 - Channel 9 Event Detection"]
    #[inline(always)]
    #[must_use]
    pub fn evd9(&mut self) -> Evd9W<IntflagSpec> {
        Evd9W::new(self, 25)
    }
    #[doc = "Bit 26 - Channel 10 Event Detection"]
    #[inline(always)]
    #[must_use]
    pub fn evd10(&mut self) -> Evd10W<IntflagSpec> {
        Evd10W::new(self, 26)
    }
    #[doc = "Bit 27 - Channel 11 Event Detection"]
    #[inline(always)]
    #[must_use]
    pub fn evd11(&mut self) -> Evd11W<IntflagSpec> {
        Evd11W::new(self, 27)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u32 = 0;
}
