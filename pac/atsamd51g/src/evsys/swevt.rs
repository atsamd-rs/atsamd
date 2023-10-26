#[doc = "Register `SWEVT` writer"]
pub type W = crate::W<SWEVT_SPEC>;
#[doc = "Field `CHANNEL0` writer - Channel 0 Software Selection"]
pub type CHANNEL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL1` writer - Channel 1 Software Selection"]
pub type CHANNEL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL2` writer - Channel 2 Software Selection"]
pub type CHANNEL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL3` writer - Channel 3 Software Selection"]
pub type CHANNEL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL4` writer - Channel 4 Software Selection"]
pub type CHANNEL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL5` writer - Channel 5 Software Selection"]
pub type CHANNEL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL6` writer - Channel 6 Software Selection"]
pub type CHANNEL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL7` writer - Channel 7 Software Selection"]
pub type CHANNEL7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL8` writer - Channel 8 Software Selection"]
pub type CHANNEL8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL9` writer - Channel 9 Software Selection"]
pub type CHANNEL9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL10` writer - Channel 10 Software Selection"]
pub type CHANNEL10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL11` writer - Channel 11 Software Selection"]
pub type CHANNEL11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL12` writer - Channel 12 Software Selection"]
pub type CHANNEL12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL13` writer - Channel 13 Software Selection"]
pub type CHANNEL13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL14` writer - Channel 14 Software Selection"]
pub type CHANNEL14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL15` writer - Channel 15 Software Selection"]
pub type CHANNEL15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL16` writer - Channel 16 Software Selection"]
pub type CHANNEL16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL17` writer - Channel 17 Software Selection"]
pub type CHANNEL17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL18` writer - Channel 18 Software Selection"]
pub type CHANNEL18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL19` writer - Channel 19 Software Selection"]
pub type CHANNEL19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL20` writer - Channel 20 Software Selection"]
pub type CHANNEL20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL21` writer - Channel 21 Software Selection"]
pub type CHANNEL21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL22` writer - Channel 22 Software Selection"]
pub type CHANNEL22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL23` writer - Channel 23 Software Selection"]
pub type CHANNEL23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL24` writer - Channel 24 Software Selection"]
pub type CHANNEL24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL25` writer - Channel 25 Software Selection"]
pub type CHANNEL25_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL26` writer - Channel 26 Software Selection"]
pub type CHANNEL26_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL27` writer - Channel 27 Software Selection"]
pub type CHANNEL27_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL28` writer - Channel 28 Software Selection"]
pub type CHANNEL28_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL29` writer - Channel 29 Software Selection"]
pub type CHANNEL29_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL30` writer - Channel 30 Software Selection"]
pub type CHANNEL30_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHANNEL31` writer - Channel 31 Software Selection"]
pub type CHANNEL31_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel0(&mut self) -> CHANNEL0_W<SWEVT_SPEC, 0> {
        CHANNEL0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel1(&mut self) -> CHANNEL1_W<SWEVT_SPEC, 1> {
        CHANNEL1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel2(&mut self) -> CHANNEL2_W<SWEVT_SPEC, 2> {
        CHANNEL2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel3(&mut self) -> CHANNEL3_W<SWEVT_SPEC, 3> {
        CHANNEL3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel4(&mut self) -> CHANNEL4_W<SWEVT_SPEC, 4> {
        CHANNEL4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel5(&mut self) -> CHANNEL5_W<SWEVT_SPEC, 5> {
        CHANNEL5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel6(&mut self) -> CHANNEL6_W<SWEVT_SPEC, 6> {
        CHANNEL6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel7(&mut self) -> CHANNEL7_W<SWEVT_SPEC, 7> {
        CHANNEL7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel8(&mut self) -> CHANNEL8_W<SWEVT_SPEC, 8> {
        CHANNEL8_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel9(&mut self) -> CHANNEL9_W<SWEVT_SPEC, 9> {
        CHANNEL9_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel10(&mut self) -> CHANNEL10_W<SWEVT_SPEC, 10> {
        CHANNEL10_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel11(&mut self) -> CHANNEL11_W<SWEVT_SPEC, 11> {
        CHANNEL11_W::new(self)
    }
    #[doc = "Bit 12 - Channel 12 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel12(&mut self) -> CHANNEL12_W<SWEVT_SPEC, 12> {
        CHANNEL12_W::new(self)
    }
    #[doc = "Bit 13 - Channel 13 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel13(&mut self) -> CHANNEL13_W<SWEVT_SPEC, 13> {
        CHANNEL13_W::new(self)
    }
    #[doc = "Bit 14 - Channel 14 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel14(&mut self) -> CHANNEL14_W<SWEVT_SPEC, 14> {
        CHANNEL14_W::new(self)
    }
    #[doc = "Bit 15 - Channel 15 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel15(&mut self) -> CHANNEL15_W<SWEVT_SPEC, 15> {
        CHANNEL15_W::new(self)
    }
    #[doc = "Bit 16 - Channel 16 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel16(&mut self) -> CHANNEL16_W<SWEVT_SPEC, 16> {
        CHANNEL16_W::new(self)
    }
    #[doc = "Bit 17 - Channel 17 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel17(&mut self) -> CHANNEL17_W<SWEVT_SPEC, 17> {
        CHANNEL17_W::new(self)
    }
    #[doc = "Bit 18 - Channel 18 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel18(&mut self) -> CHANNEL18_W<SWEVT_SPEC, 18> {
        CHANNEL18_W::new(self)
    }
    #[doc = "Bit 19 - Channel 19 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel19(&mut self) -> CHANNEL19_W<SWEVT_SPEC, 19> {
        CHANNEL19_W::new(self)
    }
    #[doc = "Bit 20 - Channel 20 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel20(&mut self) -> CHANNEL20_W<SWEVT_SPEC, 20> {
        CHANNEL20_W::new(self)
    }
    #[doc = "Bit 21 - Channel 21 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel21(&mut self) -> CHANNEL21_W<SWEVT_SPEC, 21> {
        CHANNEL21_W::new(self)
    }
    #[doc = "Bit 22 - Channel 22 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel22(&mut self) -> CHANNEL22_W<SWEVT_SPEC, 22> {
        CHANNEL22_W::new(self)
    }
    #[doc = "Bit 23 - Channel 23 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel23(&mut self) -> CHANNEL23_W<SWEVT_SPEC, 23> {
        CHANNEL23_W::new(self)
    }
    #[doc = "Bit 24 - Channel 24 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel24(&mut self) -> CHANNEL24_W<SWEVT_SPEC, 24> {
        CHANNEL24_W::new(self)
    }
    #[doc = "Bit 25 - Channel 25 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel25(&mut self) -> CHANNEL25_W<SWEVT_SPEC, 25> {
        CHANNEL25_W::new(self)
    }
    #[doc = "Bit 26 - Channel 26 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel26(&mut self) -> CHANNEL26_W<SWEVT_SPEC, 26> {
        CHANNEL26_W::new(self)
    }
    #[doc = "Bit 27 - Channel 27 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel27(&mut self) -> CHANNEL27_W<SWEVT_SPEC, 27> {
        CHANNEL27_W::new(self)
    }
    #[doc = "Bit 28 - Channel 28 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel28(&mut self) -> CHANNEL28_W<SWEVT_SPEC, 28> {
        CHANNEL28_W::new(self)
    }
    #[doc = "Bit 29 - Channel 29 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel29(&mut self) -> CHANNEL29_W<SWEVT_SPEC, 29> {
        CHANNEL29_W::new(self)
    }
    #[doc = "Bit 30 - Channel 30 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel30(&mut self) -> CHANNEL30_W<SWEVT_SPEC, 30> {
        CHANNEL30_W::new(self)
    }
    #[doc = "Bit 31 - Channel 31 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel31(&mut self) -> CHANNEL31_W<SWEVT_SPEC, 31> {
        CHANNEL31_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software Event\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVT_SPEC;
impl crate::RegisterSpec for SWEVT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swevt::W`](W) writer structure"]
impl crate::Writable for SWEVT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWEVT to value 0"]
impl crate::Resettable for SWEVT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
