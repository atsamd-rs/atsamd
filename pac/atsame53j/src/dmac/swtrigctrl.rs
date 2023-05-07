#[doc = "Register `SWTRIGCTRL` reader"]
pub struct R(crate::R<SWTRIGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWTRIGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWTRIGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWTRIGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWTRIGCTRL` writer"]
pub struct W(crate::W<SWTRIGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRIGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SWTRIGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTRIGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWTRIG0` reader - Channel 0 Software Trigger"]
pub type SWTRIG0_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG0` writer - Channel 0 Software Trigger"]
pub type SWTRIG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG1` reader - Channel 1 Software Trigger"]
pub type SWTRIG1_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG1` writer - Channel 1 Software Trigger"]
pub type SWTRIG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG2` reader - Channel 2 Software Trigger"]
pub type SWTRIG2_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG2` writer - Channel 2 Software Trigger"]
pub type SWTRIG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG3` reader - Channel 3 Software Trigger"]
pub type SWTRIG3_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG3` writer - Channel 3 Software Trigger"]
pub type SWTRIG3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG4` reader - Channel 4 Software Trigger"]
pub type SWTRIG4_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG4` writer - Channel 4 Software Trigger"]
pub type SWTRIG4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG5` reader - Channel 5 Software Trigger"]
pub type SWTRIG5_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG5` writer - Channel 5 Software Trigger"]
pub type SWTRIG5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG6` reader - Channel 6 Software Trigger"]
pub type SWTRIG6_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG6` writer - Channel 6 Software Trigger"]
pub type SWTRIG6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG7` reader - Channel 7 Software Trigger"]
pub type SWTRIG7_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG7` writer - Channel 7 Software Trigger"]
pub type SWTRIG7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG8` reader - Channel 8 Software Trigger"]
pub type SWTRIG8_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG8` writer - Channel 8 Software Trigger"]
pub type SWTRIG8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG9` reader - Channel 9 Software Trigger"]
pub type SWTRIG9_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG9` writer - Channel 9 Software Trigger"]
pub type SWTRIG9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG10` reader - Channel 10 Software Trigger"]
pub type SWTRIG10_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG10` writer - Channel 10 Software Trigger"]
pub type SWTRIG10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG11` reader - Channel 11 Software Trigger"]
pub type SWTRIG11_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG11` writer - Channel 11 Software Trigger"]
pub type SWTRIG11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG12` reader - Channel 12 Software Trigger"]
pub type SWTRIG12_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG12` writer - Channel 12 Software Trigger"]
pub type SWTRIG12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG13` reader - Channel 13 Software Trigger"]
pub type SWTRIG13_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG13` writer - Channel 13 Software Trigger"]
pub type SWTRIG13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG14` reader - Channel 14 Software Trigger"]
pub type SWTRIG14_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG14` writer - Channel 14 Software Trigger"]
pub type SWTRIG14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG15` reader - Channel 15 Software Trigger"]
pub type SWTRIG15_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG15` writer - Channel 15 Software Trigger"]
pub type SWTRIG15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG16` reader - Channel 16 Software Trigger"]
pub type SWTRIG16_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG16` writer - Channel 16 Software Trigger"]
pub type SWTRIG16_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG17` reader - Channel 17 Software Trigger"]
pub type SWTRIG17_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG17` writer - Channel 17 Software Trigger"]
pub type SWTRIG17_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG18` reader - Channel 18 Software Trigger"]
pub type SWTRIG18_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG18` writer - Channel 18 Software Trigger"]
pub type SWTRIG18_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG19` reader - Channel 19 Software Trigger"]
pub type SWTRIG19_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG19` writer - Channel 19 Software Trigger"]
pub type SWTRIG19_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG20` reader - Channel 20 Software Trigger"]
pub type SWTRIG20_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG20` writer - Channel 20 Software Trigger"]
pub type SWTRIG20_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG21` reader - Channel 21 Software Trigger"]
pub type SWTRIG21_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG21` writer - Channel 21 Software Trigger"]
pub type SWTRIG21_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG22` reader - Channel 22 Software Trigger"]
pub type SWTRIG22_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG22` writer - Channel 22 Software Trigger"]
pub type SWTRIG22_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG23` reader - Channel 23 Software Trigger"]
pub type SWTRIG23_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG23` writer - Channel 23 Software Trigger"]
pub type SWTRIG23_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG24` reader - Channel 24 Software Trigger"]
pub type SWTRIG24_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG24` writer - Channel 24 Software Trigger"]
pub type SWTRIG24_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG25` reader - Channel 25 Software Trigger"]
pub type SWTRIG25_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG25` writer - Channel 25 Software Trigger"]
pub type SWTRIG25_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG26` reader - Channel 26 Software Trigger"]
pub type SWTRIG26_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG26` writer - Channel 26 Software Trigger"]
pub type SWTRIG26_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG27` reader - Channel 27 Software Trigger"]
pub type SWTRIG27_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG27` writer - Channel 27 Software Trigger"]
pub type SWTRIG27_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG28` reader - Channel 28 Software Trigger"]
pub type SWTRIG28_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG28` writer - Channel 28 Software Trigger"]
pub type SWTRIG28_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG29` reader - Channel 29 Software Trigger"]
pub type SWTRIG29_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG29` writer - Channel 29 Software Trigger"]
pub type SWTRIG29_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG30` reader - Channel 30 Software Trigger"]
pub type SWTRIG30_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG30` writer - Channel 30 Software Trigger"]
pub type SWTRIG30_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
#[doc = "Field `SWTRIG31` reader - Channel 31 Software Trigger"]
pub type SWTRIG31_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG31` writer - Channel 31 Software Trigger"]
pub type SWTRIG31_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    pub fn swtrig0(&self) -> SWTRIG0_R {
        SWTRIG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    pub fn swtrig1(&self) -> SWTRIG1_R {
        SWTRIG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline(always)]
    pub fn swtrig2(&self) -> SWTRIG2_R {
        SWTRIG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline(always)]
    pub fn swtrig3(&self) -> SWTRIG3_R {
        SWTRIG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline(always)]
    pub fn swtrig4(&self) -> SWTRIG4_R {
        SWTRIG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline(always)]
    pub fn swtrig5(&self) -> SWTRIG5_R {
        SWTRIG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline(always)]
    pub fn swtrig6(&self) -> SWTRIG6_R {
        SWTRIG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline(always)]
    pub fn swtrig7(&self) -> SWTRIG7_R {
        SWTRIG7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline(always)]
    pub fn swtrig8(&self) -> SWTRIG8_R {
        SWTRIG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline(always)]
    pub fn swtrig9(&self) -> SWTRIG9_R {
        SWTRIG9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline(always)]
    pub fn swtrig10(&self) -> SWTRIG10_R {
        SWTRIG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline(always)]
    pub fn swtrig11(&self) -> SWTRIG11_R {
        SWTRIG11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 12 Software Trigger"]
    #[inline(always)]
    pub fn swtrig12(&self) -> SWTRIG12_R {
        SWTRIG12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 13 Software Trigger"]
    #[inline(always)]
    pub fn swtrig13(&self) -> SWTRIG13_R {
        SWTRIG13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 14 Software Trigger"]
    #[inline(always)]
    pub fn swtrig14(&self) -> SWTRIG14_R {
        SWTRIG14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 15 Software Trigger"]
    #[inline(always)]
    pub fn swtrig15(&self) -> SWTRIG15_R {
        SWTRIG15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 16 Software Trigger"]
    #[inline(always)]
    pub fn swtrig16(&self) -> SWTRIG16_R {
        SWTRIG16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 17 Software Trigger"]
    #[inline(always)]
    pub fn swtrig17(&self) -> SWTRIG17_R {
        SWTRIG17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 18 Software Trigger"]
    #[inline(always)]
    pub fn swtrig18(&self) -> SWTRIG18_R {
        SWTRIG18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 19 Software Trigger"]
    #[inline(always)]
    pub fn swtrig19(&self) -> SWTRIG19_R {
        SWTRIG19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 20 Software Trigger"]
    #[inline(always)]
    pub fn swtrig20(&self) -> SWTRIG20_R {
        SWTRIG20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 21 Software Trigger"]
    #[inline(always)]
    pub fn swtrig21(&self) -> SWTRIG21_R {
        SWTRIG21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 22 Software Trigger"]
    #[inline(always)]
    pub fn swtrig22(&self) -> SWTRIG22_R {
        SWTRIG22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 23 Software Trigger"]
    #[inline(always)]
    pub fn swtrig23(&self) -> SWTRIG23_R {
        SWTRIG23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 24 Software Trigger"]
    #[inline(always)]
    pub fn swtrig24(&self) -> SWTRIG24_R {
        SWTRIG24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 25 Software Trigger"]
    #[inline(always)]
    pub fn swtrig25(&self) -> SWTRIG25_R {
        SWTRIG25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 26 Software Trigger"]
    #[inline(always)]
    pub fn swtrig26(&self) -> SWTRIG26_R {
        SWTRIG26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 27 Software Trigger"]
    #[inline(always)]
    pub fn swtrig27(&self) -> SWTRIG27_R {
        SWTRIG27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Channel 28 Software Trigger"]
    #[inline(always)]
    pub fn swtrig28(&self) -> SWTRIG28_R {
        SWTRIG28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Channel 29 Software Trigger"]
    #[inline(always)]
    pub fn swtrig29(&self) -> SWTRIG29_R {
        SWTRIG29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel 30 Software Trigger"]
    #[inline(always)]
    pub fn swtrig30(&self) -> SWTRIG30_R {
        SWTRIG30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel 31 Software Trigger"]
    #[inline(always)]
    pub fn swtrig31(&self) -> SWTRIG31_R {
        SWTRIG31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig0(&mut self) -> SWTRIG0_W<0> {
        SWTRIG0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<1> {
        SWTRIG1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<2> {
        SWTRIG2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig3(&mut self) -> SWTRIG3_W<3> {
        SWTRIG3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig4(&mut self) -> SWTRIG4_W<4> {
        SWTRIG4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig5(&mut self) -> SWTRIG5_W<5> {
        SWTRIG5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig6(&mut self) -> SWTRIG6_W<6> {
        SWTRIG6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig7(&mut self) -> SWTRIG7_W<7> {
        SWTRIG7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig8(&mut self) -> SWTRIG8_W<8> {
        SWTRIG8_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig9(&mut self) -> SWTRIG9_W<9> {
        SWTRIG9_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig10(&mut self) -> SWTRIG10_W<10> {
        SWTRIG10_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig11(&mut self) -> SWTRIG11_W<11> {
        SWTRIG11_W::new(self)
    }
    #[doc = "Bit 12 - Channel 12 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig12(&mut self) -> SWTRIG12_W<12> {
        SWTRIG12_W::new(self)
    }
    #[doc = "Bit 13 - Channel 13 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig13(&mut self) -> SWTRIG13_W<13> {
        SWTRIG13_W::new(self)
    }
    #[doc = "Bit 14 - Channel 14 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig14(&mut self) -> SWTRIG14_W<14> {
        SWTRIG14_W::new(self)
    }
    #[doc = "Bit 15 - Channel 15 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig15(&mut self) -> SWTRIG15_W<15> {
        SWTRIG15_W::new(self)
    }
    #[doc = "Bit 16 - Channel 16 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig16(&mut self) -> SWTRIG16_W<16> {
        SWTRIG16_W::new(self)
    }
    #[doc = "Bit 17 - Channel 17 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig17(&mut self) -> SWTRIG17_W<17> {
        SWTRIG17_W::new(self)
    }
    #[doc = "Bit 18 - Channel 18 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig18(&mut self) -> SWTRIG18_W<18> {
        SWTRIG18_W::new(self)
    }
    #[doc = "Bit 19 - Channel 19 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig19(&mut self) -> SWTRIG19_W<19> {
        SWTRIG19_W::new(self)
    }
    #[doc = "Bit 20 - Channel 20 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig20(&mut self) -> SWTRIG20_W<20> {
        SWTRIG20_W::new(self)
    }
    #[doc = "Bit 21 - Channel 21 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig21(&mut self) -> SWTRIG21_W<21> {
        SWTRIG21_W::new(self)
    }
    #[doc = "Bit 22 - Channel 22 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig22(&mut self) -> SWTRIG22_W<22> {
        SWTRIG22_W::new(self)
    }
    #[doc = "Bit 23 - Channel 23 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig23(&mut self) -> SWTRIG23_W<23> {
        SWTRIG23_W::new(self)
    }
    #[doc = "Bit 24 - Channel 24 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig24(&mut self) -> SWTRIG24_W<24> {
        SWTRIG24_W::new(self)
    }
    #[doc = "Bit 25 - Channel 25 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig25(&mut self) -> SWTRIG25_W<25> {
        SWTRIG25_W::new(self)
    }
    #[doc = "Bit 26 - Channel 26 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig26(&mut self) -> SWTRIG26_W<26> {
        SWTRIG26_W::new(self)
    }
    #[doc = "Bit 27 - Channel 27 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig27(&mut self) -> SWTRIG27_W<27> {
        SWTRIG27_W::new(self)
    }
    #[doc = "Bit 28 - Channel 28 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig28(&mut self) -> SWTRIG28_W<28> {
        SWTRIG28_W::new(self)
    }
    #[doc = "Bit 29 - Channel 29 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig29(&mut self) -> SWTRIG29_W<29> {
        SWTRIG29_W::new(self)
    }
    #[doc = "Bit 30 - Channel 30 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig30(&mut self) -> SWTRIG30_W<30> {
        SWTRIG30_W::new(self)
    }
    #[doc = "Bit 31 - Channel 31 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig31(&mut self) -> SWTRIG31_W<31> {
        SWTRIG31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Trigger Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrigctrl](index.html) module"]
pub struct SWTRIGCTRL_SPEC;
impl crate::RegisterSpec for SWTRIGCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swtrigctrl::R](R) reader structure"]
impl crate::Readable for SWTRIGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swtrigctrl::W](W) writer structure"]
impl crate::Writable for SWTRIGCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWTRIGCTRL to value 0"]
impl crate::Resettable for SWTRIGCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
