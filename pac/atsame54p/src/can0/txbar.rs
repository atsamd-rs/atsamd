#[doc = "Register `TXBAR` reader"]
pub struct R(crate::R<TXBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBAR` writer"]
pub struct W(crate::W<TXBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBAR_SPEC>;
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
impl From<crate::W<TXBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AR0` reader - Add Request 0"]
pub type AR0_R = crate::BitReader<bool>;
#[doc = "Field `AR0` writer - Add Request 0"]
pub type AR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR1` reader - Add Request 1"]
pub type AR1_R = crate::BitReader<bool>;
#[doc = "Field `AR1` writer - Add Request 1"]
pub type AR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR2` reader - Add Request 2"]
pub type AR2_R = crate::BitReader<bool>;
#[doc = "Field `AR2` writer - Add Request 2"]
pub type AR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR3` reader - Add Request 3"]
pub type AR3_R = crate::BitReader<bool>;
#[doc = "Field `AR3` writer - Add Request 3"]
pub type AR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR4` reader - Add Request 4"]
pub type AR4_R = crate::BitReader<bool>;
#[doc = "Field `AR4` writer - Add Request 4"]
pub type AR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR5` reader - Add Request 5"]
pub type AR5_R = crate::BitReader<bool>;
#[doc = "Field `AR5` writer - Add Request 5"]
pub type AR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR6` reader - Add Request 6"]
pub type AR6_R = crate::BitReader<bool>;
#[doc = "Field `AR6` writer - Add Request 6"]
pub type AR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR7` reader - Add Request 7"]
pub type AR7_R = crate::BitReader<bool>;
#[doc = "Field `AR7` writer - Add Request 7"]
pub type AR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR8` reader - Add Request 8"]
pub type AR8_R = crate::BitReader<bool>;
#[doc = "Field `AR8` writer - Add Request 8"]
pub type AR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR9` reader - Add Request 9"]
pub type AR9_R = crate::BitReader<bool>;
#[doc = "Field `AR9` writer - Add Request 9"]
pub type AR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR10` reader - Add Request 10"]
pub type AR10_R = crate::BitReader<bool>;
#[doc = "Field `AR10` writer - Add Request 10"]
pub type AR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR11` reader - Add Request 11"]
pub type AR11_R = crate::BitReader<bool>;
#[doc = "Field `AR11` writer - Add Request 11"]
pub type AR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR12` reader - Add Request 12"]
pub type AR12_R = crate::BitReader<bool>;
#[doc = "Field `AR12` writer - Add Request 12"]
pub type AR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR13` reader - Add Request 13"]
pub type AR13_R = crate::BitReader<bool>;
#[doc = "Field `AR13` writer - Add Request 13"]
pub type AR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR14` reader - Add Request 14"]
pub type AR14_R = crate::BitReader<bool>;
#[doc = "Field `AR14` writer - Add Request 14"]
pub type AR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR15` reader - Add Request 15"]
pub type AR15_R = crate::BitReader<bool>;
#[doc = "Field `AR15` writer - Add Request 15"]
pub type AR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR16` reader - Add Request 16"]
pub type AR16_R = crate::BitReader<bool>;
#[doc = "Field `AR16` writer - Add Request 16"]
pub type AR16_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR17` reader - Add Request 17"]
pub type AR17_R = crate::BitReader<bool>;
#[doc = "Field `AR17` writer - Add Request 17"]
pub type AR17_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR18` reader - Add Request 18"]
pub type AR18_R = crate::BitReader<bool>;
#[doc = "Field `AR18` writer - Add Request 18"]
pub type AR18_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR19` reader - Add Request 19"]
pub type AR19_R = crate::BitReader<bool>;
#[doc = "Field `AR19` writer - Add Request 19"]
pub type AR19_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR20` reader - Add Request 20"]
pub type AR20_R = crate::BitReader<bool>;
#[doc = "Field `AR20` writer - Add Request 20"]
pub type AR20_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR21` reader - Add Request 21"]
pub type AR21_R = crate::BitReader<bool>;
#[doc = "Field `AR21` writer - Add Request 21"]
pub type AR21_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR22` reader - Add Request 22"]
pub type AR22_R = crate::BitReader<bool>;
#[doc = "Field `AR22` writer - Add Request 22"]
pub type AR22_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR23` reader - Add Request 23"]
pub type AR23_R = crate::BitReader<bool>;
#[doc = "Field `AR23` writer - Add Request 23"]
pub type AR23_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR24` reader - Add Request 24"]
pub type AR24_R = crate::BitReader<bool>;
#[doc = "Field `AR24` writer - Add Request 24"]
pub type AR24_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR25` reader - Add Request 25"]
pub type AR25_R = crate::BitReader<bool>;
#[doc = "Field `AR25` writer - Add Request 25"]
pub type AR25_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR26` reader - Add Request 26"]
pub type AR26_R = crate::BitReader<bool>;
#[doc = "Field `AR26` writer - Add Request 26"]
pub type AR26_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR27` reader - Add Request 27"]
pub type AR27_R = crate::BitReader<bool>;
#[doc = "Field `AR27` writer - Add Request 27"]
pub type AR27_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR28` reader - Add Request 28"]
pub type AR28_R = crate::BitReader<bool>;
#[doc = "Field `AR28` writer - Add Request 28"]
pub type AR28_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR29` reader - Add Request 29"]
pub type AR29_R = crate::BitReader<bool>;
#[doc = "Field `AR29` writer - Add Request 29"]
pub type AR29_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR30` reader - Add Request 30"]
pub type AR30_R = crate::BitReader<bool>;
#[doc = "Field `AR30` writer - Add Request 30"]
pub type AR30_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
#[doc = "Field `AR31` reader - Add Request 31"]
pub type AR31_R = crate::BitReader<bool>;
#[doc = "Field `AR31` writer - Add Request 31"]
pub type AR31_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Add Request 0"]
    #[inline(always)]
    pub fn ar0(&self) -> AR0_R {
        AR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Add Request 1"]
    #[inline(always)]
    pub fn ar1(&self) -> AR1_R {
        AR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Add Request 2"]
    #[inline(always)]
    pub fn ar2(&self) -> AR2_R {
        AR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Add Request 3"]
    #[inline(always)]
    pub fn ar3(&self) -> AR3_R {
        AR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Add Request 4"]
    #[inline(always)]
    pub fn ar4(&self) -> AR4_R {
        AR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Add Request 5"]
    #[inline(always)]
    pub fn ar5(&self) -> AR5_R {
        AR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Add Request 6"]
    #[inline(always)]
    pub fn ar6(&self) -> AR6_R {
        AR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Add Request 7"]
    #[inline(always)]
    pub fn ar7(&self) -> AR7_R {
        AR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Add Request 8"]
    #[inline(always)]
    pub fn ar8(&self) -> AR8_R {
        AR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Add Request 9"]
    #[inline(always)]
    pub fn ar9(&self) -> AR9_R {
        AR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Add Request 10"]
    #[inline(always)]
    pub fn ar10(&self) -> AR10_R {
        AR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Add Request 11"]
    #[inline(always)]
    pub fn ar11(&self) -> AR11_R {
        AR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Add Request 12"]
    #[inline(always)]
    pub fn ar12(&self) -> AR12_R {
        AR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Add Request 13"]
    #[inline(always)]
    pub fn ar13(&self) -> AR13_R {
        AR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Add Request 14"]
    #[inline(always)]
    pub fn ar14(&self) -> AR14_R {
        AR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Add Request 15"]
    #[inline(always)]
    pub fn ar15(&self) -> AR15_R {
        AR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Add Request 16"]
    #[inline(always)]
    pub fn ar16(&self) -> AR16_R {
        AR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Add Request 17"]
    #[inline(always)]
    pub fn ar17(&self) -> AR17_R {
        AR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Add Request 18"]
    #[inline(always)]
    pub fn ar18(&self) -> AR18_R {
        AR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Add Request 19"]
    #[inline(always)]
    pub fn ar19(&self) -> AR19_R {
        AR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Add Request 20"]
    #[inline(always)]
    pub fn ar20(&self) -> AR20_R {
        AR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Add Request 21"]
    #[inline(always)]
    pub fn ar21(&self) -> AR21_R {
        AR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Add Request 22"]
    #[inline(always)]
    pub fn ar22(&self) -> AR22_R {
        AR22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Add Request 23"]
    #[inline(always)]
    pub fn ar23(&self) -> AR23_R {
        AR23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Add Request 24"]
    #[inline(always)]
    pub fn ar24(&self) -> AR24_R {
        AR24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Add Request 25"]
    #[inline(always)]
    pub fn ar25(&self) -> AR25_R {
        AR25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Add Request 26"]
    #[inline(always)]
    pub fn ar26(&self) -> AR26_R {
        AR26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Add Request 27"]
    #[inline(always)]
    pub fn ar27(&self) -> AR27_R {
        AR27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Add Request 28"]
    #[inline(always)]
    pub fn ar28(&self) -> AR28_R {
        AR28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Add Request 29"]
    #[inline(always)]
    pub fn ar29(&self) -> AR29_R {
        AR29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Add Request 30"]
    #[inline(always)]
    pub fn ar30(&self) -> AR30_R {
        AR30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Add Request 31"]
    #[inline(always)]
    pub fn ar31(&self) -> AR31_R {
        AR31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Add Request 0"]
    #[inline(always)]
    #[must_use]
    pub fn ar0(&mut self) -> AR0_W<0> {
        AR0_W::new(self)
    }
    #[doc = "Bit 1 - Add Request 1"]
    #[inline(always)]
    #[must_use]
    pub fn ar1(&mut self) -> AR1_W<1> {
        AR1_W::new(self)
    }
    #[doc = "Bit 2 - Add Request 2"]
    #[inline(always)]
    #[must_use]
    pub fn ar2(&mut self) -> AR2_W<2> {
        AR2_W::new(self)
    }
    #[doc = "Bit 3 - Add Request 3"]
    #[inline(always)]
    #[must_use]
    pub fn ar3(&mut self) -> AR3_W<3> {
        AR3_W::new(self)
    }
    #[doc = "Bit 4 - Add Request 4"]
    #[inline(always)]
    #[must_use]
    pub fn ar4(&mut self) -> AR4_W<4> {
        AR4_W::new(self)
    }
    #[doc = "Bit 5 - Add Request 5"]
    #[inline(always)]
    #[must_use]
    pub fn ar5(&mut self) -> AR5_W<5> {
        AR5_W::new(self)
    }
    #[doc = "Bit 6 - Add Request 6"]
    #[inline(always)]
    #[must_use]
    pub fn ar6(&mut self) -> AR6_W<6> {
        AR6_W::new(self)
    }
    #[doc = "Bit 7 - Add Request 7"]
    #[inline(always)]
    #[must_use]
    pub fn ar7(&mut self) -> AR7_W<7> {
        AR7_W::new(self)
    }
    #[doc = "Bit 8 - Add Request 8"]
    #[inline(always)]
    #[must_use]
    pub fn ar8(&mut self) -> AR8_W<8> {
        AR8_W::new(self)
    }
    #[doc = "Bit 9 - Add Request 9"]
    #[inline(always)]
    #[must_use]
    pub fn ar9(&mut self) -> AR9_W<9> {
        AR9_W::new(self)
    }
    #[doc = "Bit 10 - Add Request 10"]
    #[inline(always)]
    #[must_use]
    pub fn ar10(&mut self) -> AR10_W<10> {
        AR10_W::new(self)
    }
    #[doc = "Bit 11 - Add Request 11"]
    #[inline(always)]
    #[must_use]
    pub fn ar11(&mut self) -> AR11_W<11> {
        AR11_W::new(self)
    }
    #[doc = "Bit 12 - Add Request 12"]
    #[inline(always)]
    #[must_use]
    pub fn ar12(&mut self) -> AR12_W<12> {
        AR12_W::new(self)
    }
    #[doc = "Bit 13 - Add Request 13"]
    #[inline(always)]
    #[must_use]
    pub fn ar13(&mut self) -> AR13_W<13> {
        AR13_W::new(self)
    }
    #[doc = "Bit 14 - Add Request 14"]
    #[inline(always)]
    #[must_use]
    pub fn ar14(&mut self) -> AR14_W<14> {
        AR14_W::new(self)
    }
    #[doc = "Bit 15 - Add Request 15"]
    #[inline(always)]
    #[must_use]
    pub fn ar15(&mut self) -> AR15_W<15> {
        AR15_W::new(self)
    }
    #[doc = "Bit 16 - Add Request 16"]
    #[inline(always)]
    #[must_use]
    pub fn ar16(&mut self) -> AR16_W<16> {
        AR16_W::new(self)
    }
    #[doc = "Bit 17 - Add Request 17"]
    #[inline(always)]
    #[must_use]
    pub fn ar17(&mut self) -> AR17_W<17> {
        AR17_W::new(self)
    }
    #[doc = "Bit 18 - Add Request 18"]
    #[inline(always)]
    #[must_use]
    pub fn ar18(&mut self) -> AR18_W<18> {
        AR18_W::new(self)
    }
    #[doc = "Bit 19 - Add Request 19"]
    #[inline(always)]
    #[must_use]
    pub fn ar19(&mut self) -> AR19_W<19> {
        AR19_W::new(self)
    }
    #[doc = "Bit 20 - Add Request 20"]
    #[inline(always)]
    #[must_use]
    pub fn ar20(&mut self) -> AR20_W<20> {
        AR20_W::new(self)
    }
    #[doc = "Bit 21 - Add Request 21"]
    #[inline(always)]
    #[must_use]
    pub fn ar21(&mut self) -> AR21_W<21> {
        AR21_W::new(self)
    }
    #[doc = "Bit 22 - Add Request 22"]
    #[inline(always)]
    #[must_use]
    pub fn ar22(&mut self) -> AR22_W<22> {
        AR22_W::new(self)
    }
    #[doc = "Bit 23 - Add Request 23"]
    #[inline(always)]
    #[must_use]
    pub fn ar23(&mut self) -> AR23_W<23> {
        AR23_W::new(self)
    }
    #[doc = "Bit 24 - Add Request 24"]
    #[inline(always)]
    #[must_use]
    pub fn ar24(&mut self) -> AR24_W<24> {
        AR24_W::new(self)
    }
    #[doc = "Bit 25 - Add Request 25"]
    #[inline(always)]
    #[must_use]
    pub fn ar25(&mut self) -> AR25_W<25> {
        AR25_W::new(self)
    }
    #[doc = "Bit 26 - Add Request 26"]
    #[inline(always)]
    #[must_use]
    pub fn ar26(&mut self) -> AR26_W<26> {
        AR26_W::new(self)
    }
    #[doc = "Bit 27 - Add Request 27"]
    #[inline(always)]
    #[must_use]
    pub fn ar27(&mut self) -> AR27_W<27> {
        AR27_W::new(self)
    }
    #[doc = "Bit 28 - Add Request 28"]
    #[inline(always)]
    #[must_use]
    pub fn ar28(&mut self) -> AR28_W<28> {
        AR28_W::new(self)
    }
    #[doc = "Bit 29 - Add Request 29"]
    #[inline(always)]
    #[must_use]
    pub fn ar29(&mut self) -> AR29_W<29> {
        AR29_W::new(self)
    }
    #[doc = "Bit 30 - Add Request 30"]
    #[inline(always)]
    #[must_use]
    pub fn ar30(&mut self) -> AR30_W<30> {
        AR30_W::new(self)
    }
    #[doc = "Bit 31 - Add Request 31"]
    #[inline(always)]
    #[must_use]
    pub fn ar31(&mut self) -> AR31_W<31> {
        AR31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Buffer Add Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbar](index.html) module"]
pub struct TXBAR_SPEC;
impl crate::RegisterSpec for TXBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbar::R](R) reader structure"]
impl crate::Readable for TXBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbar::W](W) writer structure"]
impl crate::Writable for TXBAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXBAR to value 0"]
impl crate::Resettable for TXBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
