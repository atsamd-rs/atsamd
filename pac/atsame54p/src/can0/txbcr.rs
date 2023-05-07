#[doc = "Register `TXBCR` reader"]
pub struct R(crate::R<TXBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBCR` writer"]
pub struct W(crate::W<TXBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBCR_SPEC>;
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
impl From<crate::W<TXBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR0` reader - Cancellation Request 0"]
pub type CR0_R = crate::BitReader<bool>;
#[doc = "Field `CR0` writer - Cancellation Request 0"]
pub type CR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR1` reader - Cancellation Request 1"]
pub type CR1_R = crate::BitReader<bool>;
#[doc = "Field `CR1` writer - Cancellation Request 1"]
pub type CR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR2` reader - Cancellation Request 2"]
pub type CR2_R = crate::BitReader<bool>;
#[doc = "Field `CR2` writer - Cancellation Request 2"]
pub type CR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR3` reader - Cancellation Request 3"]
pub type CR3_R = crate::BitReader<bool>;
#[doc = "Field `CR3` writer - Cancellation Request 3"]
pub type CR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR4` reader - Cancellation Request 4"]
pub type CR4_R = crate::BitReader<bool>;
#[doc = "Field `CR4` writer - Cancellation Request 4"]
pub type CR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR5` reader - Cancellation Request 5"]
pub type CR5_R = crate::BitReader<bool>;
#[doc = "Field `CR5` writer - Cancellation Request 5"]
pub type CR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR6` reader - Cancellation Request 6"]
pub type CR6_R = crate::BitReader<bool>;
#[doc = "Field `CR6` writer - Cancellation Request 6"]
pub type CR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR7` reader - Cancellation Request 7"]
pub type CR7_R = crate::BitReader<bool>;
#[doc = "Field `CR7` writer - Cancellation Request 7"]
pub type CR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR8` reader - Cancellation Request 8"]
pub type CR8_R = crate::BitReader<bool>;
#[doc = "Field `CR8` writer - Cancellation Request 8"]
pub type CR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR9` reader - Cancellation Request 9"]
pub type CR9_R = crate::BitReader<bool>;
#[doc = "Field `CR9` writer - Cancellation Request 9"]
pub type CR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR10` reader - Cancellation Request 10"]
pub type CR10_R = crate::BitReader<bool>;
#[doc = "Field `CR10` writer - Cancellation Request 10"]
pub type CR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR11` reader - Cancellation Request 11"]
pub type CR11_R = crate::BitReader<bool>;
#[doc = "Field `CR11` writer - Cancellation Request 11"]
pub type CR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR12` reader - Cancellation Request 12"]
pub type CR12_R = crate::BitReader<bool>;
#[doc = "Field `CR12` writer - Cancellation Request 12"]
pub type CR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR13` reader - Cancellation Request 13"]
pub type CR13_R = crate::BitReader<bool>;
#[doc = "Field `CR13` writer - Cancellation Request 13"]
pub type CR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR14` reader - Cancellation Request 14"]
pub type CR14_R = crate::BitReader<bool>;
#[doc = "Field `CR14` writer - Cancellation Request 14"]
pub type CR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR15` reader - Cancellation Request 15"]
pub type CR15_R = crate::BitReader<bool>;
#[doc = "Field `CR15` writer - Cancellation Request 15"]
pub type CR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR16` reader - Cancellation Request 16"]
pub type CR16_R = crate::BitReader<bool>;
#[doc = "Field `CR16` writer - Cancellation Request 16"]
pub type CR16_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR17` reader - Cancellation Request 17"]
pub type CR17_R = crate::BitReader<bool>;
#[doc = "Field `CR17` writer - Cancellation Request 17"]
pub type CR17_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR18` reader - Cancellation Request 18"]
pub type CR18_R = crate::BitReader<bool>;
#[doc = "Field `CR18` writer - Cancellation Request 18"]
pub type CR18_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR19` reader - Cancellation Request 19"]
pub type CR19_R = crate::BitReader<bool>;
#[doc = "Field `CR19` writer - Cancellation Request 19"]
pub type CR19_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR20` reader - Cancellation Request 20"]
pub type CR20_R = crate::BitReader<bool>;
#[doc = "Field `CR20` writer - Cancellation Request 20"]
pub type CR20_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR21` reader - Cancellation Request 21"]
pub type CR21_R = crate::BitReader<bool>;
#[doc = "Field `CR21` writer - Cancellation Request 21"]
pub type CR21_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR22` reader - Cancellation Request 22"]
pub type CR22_R = crate::BitReader<bool>;
#[doc = "Field `CR22` writer - Cancellation Request 22"]
pub type CR22_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR23` reader - Cancellation Request 23"]
pub type CR23_R = crate::BitReader<bool>;
#[doc = "Field `CR23` writer - Cancellation Request 23"]
pub type CR23_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR24` reader - Cancellation Request 24"]
pub type CR24_R = crate::BitReader<bool>;
#[doc = "Field `CR24` writer - Cancellation Request 24"]
pub type CR24_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR25` reader - Cancellation Request 25"]
pub type CR25_R = crate::BitReader<bool>;
#[doc = "Field `CR25` writer - Cancellation Request 25"]
pub type CR25_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR26` reader - Cancellation Request 26"]
pub type CR26_R = crate::BitReader<bool>;
#[doc = "Field `CR26` writer - Cancellation Request 26"]
pub type CR26_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR27` reader - Cancellation Request 27"]
pub type CR27_R = crate::BitReader<bool>;
#[doc = "Field `CR27` writer - Cancellation Request 27"]
pub type CR27_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR28` reader - Cancellation Request 28"]
pub type CR28_R = crate::BitReader<bool>;
#[doc = "Field `CR28` writer - Cancellation Request 28"]
pub type CR28_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR29` reader - Cancellation Request 29"]
pub type CR29_R = crate::BitReader<bool>;
#[doc = "Field `CR29` writer - Cancellation Request 29"]
pub type CR29_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR30` reader - Cancellation Request 30"]
pub type CR30_R = crate::BitReader<bool>;
#[doc = "Field `CR30` writer - Cancellation Request 30"]
pub type CR30_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
#[doc = "Field `CR31` reader - Cancellation Request 31"]
pub type CR31_R = crate::BitReader<bool>;
#[doc = "Field `CR31` writer - Cancellation Request 31"]
pub type CR31_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Cancellation Request 0"]
    #[inline(always)]
    pub fn cr0(&self) -> CR0_R {
        CR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cancellation Request 1"]
    #[inline(always)]
    pub fn cr1(&self) -> CR1_R {
        CR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cancellation Request 2"]
    #[inline(always)]
    pub fn cr2(&self) -> CR2_R {
        CR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cancellation Request 3"]
    #[inline(always)]
    pub fn cr3(&self) -> CR3_R {
        CR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cancellation Request 4"]
    #[inline(always)]
    pub fn cr4(&self) -> CR4_R {
        CR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Cancellation Request 5"]
    #[inline(always)]
    pub fn cr5(&self) -> CR5_R {
        CR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Cancellation Request 6"]
    #[inline(always)]
    pub fn cr6(&self) -> CR6_R {
        CR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Cancellation Request 7"]
    #[inline(always)]
    pub fn cr7(&self) -> CR7_R {
        CR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Cancellation Request 8"]
    #[inline(always)]
    pub fn cr8(&self) -> CR8_R {
        CR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Cancellation Request 9"]
    #[inline(always)]
    pub fn cr9(&self) -> CR9_R {
        CR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Cancellation Request 10"]
    #[inline(always)]
    pub fn cr10(&self) -> CR10_R {
        CR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Cancellation Request 11"]
    #[inline(always)]
    pub fn cr11(&self) -> CR11_R {
        CR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Cancellation Request 12"]
    #[inline(always)]
    pub fn cr12(&self) -> CR12_R {
        CR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Cancellation Request 13"]
    #[inline(always)]
    pub fn cr13(&self) -> CR13_R {
        CR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Cancellation Request 14"]
    #[inline(always)]
    pub fn cr14(&self) -> CR14_R {
        CR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Cancellation Request 15"]
    #[inline(always)]
    pub fn cr15(&self) -> CR15_R {
        CR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Cancellation Request 16"]
    #[inline(always)]
    pub fn cr16(&self) -> CR16_R {
        CR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Cancellation Request 17"]
    #[inline(always)]
    pub fn cr17(&self) -> CR17_R {
        CR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Cancellation Request 18"]
    #[inline(always)]
    pub fn cr18(&self) -> CR18_R {
        CR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Cancellation Request 19"]
    #[inline(always)]
    pub fn cr19(&self) -> CR19_R {
        CR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Cancellation Request 20"]
    #[inline(always)]
    pub fn cr20(&self) -> CR20_R {
        CR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Cancellation Request 21"]
    #[inline(always)]
    pub fn cr21(&self) -> CR21_R {
        CR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Cancellation Request 22"]
    #[inline(always)]
    pub fn cr22(&self) -> CR22_R {
        CR22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Cancellation Request 23"]
    #[inline(always)]
    pub fn cr23(&self) -> CR23_R {
        CR23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Cancellation Request 24"]
    #[inline(always)]
    pub fn cr24(&self) -> CR24_R {
        CR24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Cancellation Request 25"]
    #[inline(always)]
    pub fn cr25(&self) -> CR25_R {
        CR25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Cancellation Request 26"]
    #[inline(always)]
    pub fn cr26(&self) -> CR26_R {
        CR26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Cancellation Request 27"]
    #[inline(always)]
    pub fn cr27(&self) -> CR27_R {
        CR27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Cancellation Request 28"]
    #[inline(always)]
    pub fn cr28(&self) -> CR28_R {
        CR28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Cancellation Request 29"]
    #[inline(always)]
    pub fn cr29(&self) -> CR29_R {
        CR29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Cancellation Request 30"]
    #[inline(always)]
    pub fn cr30(&self) -> CR30_R {
        CR30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Cancellation Request 31"]
    #[inline(always)]
    pub fn cr31(&self) -> CR31_R {
        CR31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cancellation Request 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr0(&mut self) -> CR0_W<0> {
        CR0_W::new(self)
    }
    #[doc = "Bit 1 - Cancellation Request 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr1(&mut self) -> CR1_W<1> {
        CR1_W::new(self)
    }
    #[doc = "Bit 2 - Cancellation Request 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr2(&mut self) -> CR2_W<2> {
        CR2_W::new(self)
    }
    #[doc = "Bit 3 - Cancellation Request 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr3(&mut self) -> CR3_W<3> {
        CR3_W::new(self)
    }
    #[doc = "Bit 4 - Cancellation Request 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr4(&mut self) -> CR4_W<4> {
        CR4_W::new(self)
    }
    #[doc = "Bit 5 - Cancellation Request 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr5(&mut self) -> CR5_W<5> {
        CR5_W::new(self)
    }
    #[doc = "Bit 6 - Cancellation Request 6"]
    #[inline(always)]
    #[must_use]
    pub fn cr6(&mut self) -> CR6_W<6> {
        CR6_W::new(self)
    }
    #[doc = "Bit 7 - Cancellation Request 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr7(&mut self) -> CR7_W<7> {
        CR7_W::new(self)
    }
    #[doc = "Bit 8 - Cancellation Request 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr8(&mut self) -> CR8_W<8> {
        CR8_W::new(self)
    }
    #[doc = "Bit 9 - Cancellation Request 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr9(&mut self) -> CR9_W<9> {
        CR9_W::new(self)
    }
    #[doc = "Bit 10 - Cancellation Request 10"]
    #[inline(always)]
    #[must_use]
    pub fn cr10(&mut self) -> CR10_W<10> {
        CR10_W::new(self)
    }
    #[doc = "Bit 11 - Cancellation Request 11"]
    #[inline(always)]
    #[must_use]
    pub fn cr11(&mut self) -> CR11_W<11> {
        CR11_W::new(self)
    }
    #[doc = "Bit 12 - Cancellation Request 12"]
    #[inline(always)]
    #[must_use]
    pub fn cr12(&mut self) -> CR12_W<12> {
        CR12_W::new(self)
    }
    #[doc = "Bit 13 - Cancellation Request 13"]
    #[inline(always)]
    #[must_use]
    pub fn cr13(&mut self) -> CR13_W<13> {
        CR13_W::new(self)
    }
    #[doc = "Bit 14 - Cancellation Request 14"]
    #[inline(always)]
    #[must_use]
    pub fn cr14(&mut self) -> CR14_W<14> {
        CR14_W::new(self)
    }
    #[doc = "Bit 15 - Cancellation Request 15"]
    #[inline(always)]
    #[must_use]
    pub fn cr15(&mut self) -> CR15_W<15> {
        CR15_W::new(self)
    }
    #[doc = "Bit 16 - Cancellation Request 16"]
    #[inline(always)]
    #[must_use]
    pub fn cr16(&mut self) -> CR16_W<16> {
        CR16_W::new(self)
    }
    #[doc = "Bit 17 - Cancellation Request 17"]
    #[inline(always)]
    #[must_use]
    pub fn cr17(&mut self) -> CR17_W<17> {
        CR17_W::new(self)
    }
    #[doc = "Bit 18 - Cancellation Request 18"]
    #[inline(always)]
    #[must_use]
    pub fn cr18(&mut self) -> CR18_W<18> {
        CR18_W::new(self)
    }
    #[doc = "Bit 19 - Cancellation Request 19"]
    #[inline(always)]
    #[must_use]
    pub fn cr19(&mut self) -> CR19_W<19> {
        CR19_W::new(self)
    }
    #[doc = "Bit 20 - Cancellation Request 20"]
    #[inline(always)]
    #[must_use]
    pub fn cr20(&mut self) -> CR20_W<20> {
        CR20_W::new(self)
    }
    #[doc = "Bit 21 - Cancellation Request 21"]
    #[inline(always)]
    #[must_use]
    pub fn cr21(&mut self) -> CR21_W<21> {
        CR21_W::new(self)
    }
    #[doc = "Bit 22 - Cancellation Request 22"]
    #[inline(always)]
    #[must_use]
    pub fn cr22(&mut self) -> CR22_W<22> {
        CR22_W::new(self)
    }
    #[doc = "Bit 23 - Cancellation Request 23"]
    #[inline(always)]
    #[must_use]
    pub fn cr23(&mut self) -> CR23_W<23> {
        CR23_W::new(self)
    }
    #[doc = "Bit 24 - Cancellation Request 24"]
    #[inline(always)]
    #[must_use]
    pub fn cr24(&mut self) -> CR24_W<24> {
        CR24_W::new(self)
    }
    #[doc = "Bit 25 - Cancellation Request 25"]
    #[inline(always)]
    #[must_use]
    pub fn cr25(&mut self) -> CR25_W<25> {
        CR25_W::new(self)
    }
    #[doc = "Bit 26 - Cancellation Request 26"]
    #[inline(always)]
    #[must_use]
    pub fn cr26(&mut self) -> CR26_W<26> {
        CR26_W::new(self)
    }
    #[doc = "Bit 27 - Cancellation Request 27"]
    #[inline(always)]
    #[must_use]
    pub fn cr27(&mut self) -> CR27_W<27> {
        CR27_W::new(self)
    }
    #[doc = "Bit 28 - Cancellation Request 28"]
    #[inline(always)]
    #[must_use]
    pub fn cr28(&mut self) -> CR28_W<28> {
        CR28_W::new(self)
    }
    #[doc = "Bit 29 - Cancellation Request 29"]
    #[inline(always)]
    #[must_use]
    pub fn cr29(&mut self) -> CR29_W<29> {
        CR29_W::new(self)
    }
    #[doc = "Bit 30 - Cancellation Request 30"]
    #[inline(always)]
    #[must_use]
    pub fn cr30(&mut self) -> CR30_W<30> {
        CR30_W::new(self)
    }
    #[doc = "Bit 31 - Cancellation Request 31"]
    #[inline(always)]
    #[must_use]
    pub fn cr31(&mut self) -> CR31_W<31> {
        CR31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Buffer Cancellation Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcr](index.html) module"]
pub struct TXBCR_SPEC;
impl crate::RegisterSpec for TXBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbcr::R](R) reader structure"]
impl crate::Readable for TXBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbcr::W](W) writer structure"]
impl crate::Writable for TXBCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXBCR to value 0"]
impl crate::Resettable for TXBCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
