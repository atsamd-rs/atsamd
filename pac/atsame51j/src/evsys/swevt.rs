#[doc = "Register `SWEVT` writer"]
pub type W = crate::W<SwevtSpec>;
#[doc = "Field `CHANNEL0` writer - Channel 0 Software Selection"]
pub type Channel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL1` writer - Channel 1 Software Selection"]
pub type Channel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL2` writer - Channel 2 Software Selection"]
pub type Channel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL3` writer - Channel 3 Software Selection"]
pub type Channel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL4` writer - Channel 4 Software Selection"]
pub type Channel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL5` writer - Channel 5 Software Selection"]
pub type Channel5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL6` writer - Channel 6 Software Selection"]
pub type Channel6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL7` writer - Channel 7 Software Selection"]
pub type Channel7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL8` writer - Channel 8 Software Selection"]
pub type Channel8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL9` writer - Channel 9 Software Selection"]
pub type Channel9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL10` writer - Channel 10 Software Selection"]
pub type Channel10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL11` writer - Channel 11 Software Selection"]
pub type Channel11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL12` writer - Channel 12 Software Selection"]
pub type Channel12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL13` writer - Channel 13 Software Selection"]
pub type Channel13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL14` writer - Channel 14 Software Selection"]
pub type Channel14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL15` writer - Channel 15 Software Selection"]
pub type Channel15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL16` writer - Channel 16 Software Selection"]
pub type Channel16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL17` writer - Channel 17 Software Selection"]
pub type Channel17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL18` writer - Channel 18 Software Selection"]
pub type Channel18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL19` writer - Channel 19 Software Selection"]
pub type Channel19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL20` writer - Channel 20 Software Selection"]
pub type Channel20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL21` writer - Channel 21 Software Selection"]
pub type Channel21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL22` writer - Channel 22 Software Selection"]
pub type Channel22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL23` writer - Channel 23 Software Selection"]
pub type Channel23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL24` writer - Channel 24 Software Selection"]
pub type Channel24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL25` writer - Channel 25 Software Selection"]
pub type Channel25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL26` writer - Channel 26 Software Selection"]
pub type Channel26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL27` writer - Channel 27 Software Selection"]
pub type Channel27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL28` writer - Channel 28 Software Selection"]
pub type Channel28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL29` writer - Channel 29 Software Selection"]
pub type Channel29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL30` writer - Channel 30 Software Selection"]
pub type Channel30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL31` writer - Channel 31 Software Selection"]
pub type Channel31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Software Selection"]
    #[inline(always)]
    pub fn channel0(&mut self) -> Channel0W<SwevtSpec> {
        Channel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Selection"]
    #[inline(always)]
    pub fn channel1(&mut self) -> Channel1W<SwevtSpec> {
        Channel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Software Selection"]
    #[inline(always)]
    pub fn channel2(&mut self) -> Channel2W<SwevtSpec> {
        Channel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Software Selection"]
    #[inline(always)]
    pub fn channel3(&mut self) -> Channel3W<SwevtSpec> {
        Channel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Software Selection"]
    #[inline(always)]
    pub fn channel4(&mut self) -> Channel4W<SwevtSpec> {
        Channel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Software Selection"]
    #[inline(always)]
    pub fn channel5(&mut self) -> Channel5W<SwevtSpec> {
        Channel5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Software Selection"]
    #[inline(always)]
    pub fn channel6(&mut self) -> Channel6W<SwevtSpec> {
        Channel6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Software Selection"]
    #[inline(always)]
    pub fn channel7(&mut self) -> Channel7W<SwevtSpec> {
        Channel7W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 8 Software Selection"]
    #[inline(always)]
    pub fn channel8(&mut self) -> Channel8W<SwevtSpec> {
        Channel8W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 9 Software Selection"]
    #[inline(always)]
    pub fn channel9(&mut self) -> Channel9W<SwevtSpec> {
        Channel9W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 10 Software Selection"]
    #[inline(always)]
    pub fn channel10(&mut self) -> Channel10W<SwevtSpec> {
        Channel10W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 11 Software Selection"]
    #[inline(always)]
    pub fn channel11(&mut self) -> Channel11W<SwevtSpec> {
        Channel11W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 12 Software Selection"]
    #[inline(always)]
    pub fn channel12(&mut self) -> Channel12W<SwevtSpec> {
        Channel12W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 13 Software Selection"]
    #[inline(always)]
    pub fn channel13(&mut self) -> Channel13W<SwevtSpec> {
        Channel13W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel 14 Software Selection"]
    #[inline(always)]
    pub fn channel14(&mut self) -> Channel14W<SwevtSpec> {
        Channel14W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel 15 Software Selection"]
    #[inline(always)]
    pub fn channel15(&mut self) -> Channel15W<SwevtSpec> {
        Channel15W::new(self, 15)
    }
    #[doc = "Bit 16 - Channel 16 Software Selection"]
    #[inline(always)]
    pub fn channel16(&mut self) -> Channel16W<SwevtSpec> {
        Channel16W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel 17 Software Selection"]
    #[inline(always)]
    pub fn channel17(&mut self) -> Channel17W<SwevtSpec> {
        Channel17W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel 18 Software Selection"]
    #[inline(always)]
    pub fn channel18(&mut self) -> Channel18W<SwevtSpec> {
        Channel18W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel 19 Software Selection"]
    #[inline(always)]
    pub fn channel19(&mut self) -> Channel19W<SwevtSpec> {
        Channel19W::new(self, 19)
    }
    #[doc = "Bit 20 - Channel 20 Software Selection"]
    #[inline(always)]
    pub fn channel20(&mut self) -> Channel20W<SwevtSpec> {
        Channel20W::new(self, 20)
    }
    #[doc = "Bit 21 - Channel 21 Software Selection"]
    #[inline(always)]
    pub fn channel21(&mut self) -> Channel21W<SwevtSpec> {
        Channel21W::new(self, 21)
    }
    #[doc = "Bit 22 - Channel 22 Software Selection"]
    #[inline(always)]
    pub fn channel22(&mut self) -> Channel22W<SwevtSpec> {
        Channel22W::new(self, 22)
    }
    #[doc = "Bit 23 - Channel 23 Software Selection"]
    #[inline(always)]
    pub fn channel23(&mut self) -> Channel23W<SwevtSpec> {
        Channel23W::new(self, 23)
    }
    #[doc = "Bit 24 - Channel 24 Software Selection"]
    #[inline(always)]
    pub fn channel24(&mut self) -> Channel24W<SwevtSpec> {
        Channel24W::new(self, 24)
    }
    #[doc = "Bit 25 - Channel 25 Software Selection"]
    #[inline(always)]
    pub fn channel25(&mut self) -> Channel25W<SwevtSpec> {
        Channel25W::new(self, 25)
    }
    #[doc = "Bit 26 - Channel 26 Software Selection"]
    #[inline(always)]
    pub fn channel26(&mut self) -> Channel26W<SwevtSpec> {
        Channel26W::new(self, 26)
    }
    #[doc = "Bit 27 - Channel 27 Software Selection"]
    #[inline(always)]
    pub fn channel27(&mut self) -> Channel27W<SwevtSpec> {
        Channel27W::new(self, 27)
    }
    #[doc = "Bit 28 - Channel 28 Software Selection"]
    #[inline(always)]
    pub fn channel28(&mut self) -> Channel28W<SwevtSpec> {
        Channel28W::new(self, 28)
    }
    #[doc = "Bit 29 - Channel 29 Software Selection"]
    #[inline(always)]
    pub fn channel29(&mut self) -> Channel29W<SwevtSpec> {
        Channel29W::new(self, 29)
    }
    #[doc = "Bit 30 - Channel 30 Software Selection"]
    #[inline(always)]
    pub fn channel30(&mut self) -> Channel30W<SwevtSpec> {
        Channel30W::new(self, 30)
    }
    #[doc = "Bit 31 - Channel 31 Software Selection"]
    #[inline(always)]
    pub fn channel31(&mut self) -> Channel31W<SwevtSpec> {
        Channel31W::new(self, 31)
    }
}
#[doc = "Software Event\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwevtSpec;
impl crate::RegisterSpec for SwevtSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swevt::W`](W) writer structure"]
impl crate::Writable for SwevtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWEVT to value 0"]
impl crate::Resettable for SwevtSpec {}
