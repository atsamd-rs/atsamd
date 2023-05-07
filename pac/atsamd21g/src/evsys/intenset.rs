#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVR0` reader - Channel 0 Overrun Interrupt Enable"]
pub type OVR0_R = crate::BitReader<bool>;
#[doc = "Field `OVR0` writer - Channel 0 Overrun Interrupt Enable"]
pub type OVR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OVR1` reader - Channel 1 Overrun Interrupt Enable"]
pub type OVR1_R = crate::BitReader<bool>;
#[doc = "Field `OVR1` writer - Channel 1 Overrun Interrupt Enable"]
pub type OVR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OVR2` reader - Channel 2 Overrun Interrupt Enable"]
pub type OVR2_R = crate::BitReader<bool>;
#[doc = "Field `OVR2` writer - Channel 2 Overrun Interrupt Enable"]
pub type OVR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OVR3` reader - Channel 3 Overrun Interrupt Enable"]
pub type OVR3_R = crate::BitReader<bool>;
#[doc = "Field `OVR3` writer - Channel 3 Overrun Interrupt Enable"]
pub type OVR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OVR4` reader - Channel 4 Overrun Interrupt Enable"]
pub type OVR4_R = crate::BitReader<bool>;
#[doc = "Field `OVR4` writer - Channel 4 Overrun Interrupt Enable"]
pub type OVR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OVR5` reader - Channel 5 Overrun Interrupt Enable"]
pub type OVR5_R = crate::BitReader<bool>;
#[doc = "Field `OVR5` writer - Channel 5 Overrun Interrupt Enable"]
pub type OVR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OVR6` reader - Channel 6 Overrun Interrupt Enable"]
pub type OVR6_R = crate::BitReader<bool>;
#[doc = "Field `OVR6` writer - Channel 6 Overrun Interrupt Enable"]
pub type OVR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OVR7` reader - Channel 7 Overrun Interrupt Enable"]
pub type OVR7_R = crate::BitReader<bool>;
#[doc = "Field `OVR7` writer - Channel 7 Overrun Interrupt Enable"]
pub type OVR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `EVD0` reader - Channel 0 Event Detection Interrupt Enable"]
pub type EVD0_R = crate::BitReader<bool>;
#[doc = "Field `EVD0` writer - Channel 0 Event Detection Interrupt Enable"]
pub type EVD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `EVD1` reader - Channel 1 Event Detection Interrupt Enable"]
pub type EVD1_R = crate::BitReader<bool>;
#[doc = "Field `EVD1` writer - Channel 1 Event Detection Interrupt Enable"]
pub type EVD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `EVD2` reader - Channel 2 Event Detection Interrupt Enable"]
pub type EVD2_R = crate::BitReader<bool>;
#[doc = "Field `EVD2` writer - Channel 2 Event Detection Interrupt Enable"]
pub type EVD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `EVD3` reader - Channel 3 Event Detection Interrupt Enable"]
pub type EVD3_R = crate::BitReader<bool>;
#[doc = "Field `EVD3` writer - Channel 3 Event Detection Interrupt Enable"]
pub type EVD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `EVD4` reader - Channel 4 Event Detection Interrupt Enable"]
pub type EVD4_R = crate::BitReader<bool>;
#[doc = "Field `EVD4` writer - Channel 4 Event Detection Interrupt Enable"]
pub type EVD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `EVD5` reader - Channel 5 Event Detection Interrupt Enable"]
pub type EVD5_R = crate::BitReader<bool>;
#[doc = "Field `EVD5` writer - Channel 5 Event Detection Interrupt Enable"]
pub type EVD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `EVD6` reader - Channel 6 Event Detection Interrupt Enable"]
pub type EVD6_R = crate::BitReader<bool>;
#[doc = "Field `EVD6` writer - Channel 6 Event Detection Interrupt Enable"]
pub type EVD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `EVD7` reader - Channel 7 Event Detection Interrupt Enable"]
pub type EVD7_R = crate::BitReader<bool>;
#[doc = "Field `EVD7` writer - Channel 7 Event Detection Interrupt Enable"]
pub type EVD7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OVR8` reader - Channel 8 Overrun Interrupt Enable"]
pub type OVR8_R = crate::BitReader<bool>;
#[doc = "Field `OVR8` writer - Channel 8 Overrun Interrupt Enable"]
pub type OVR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OVR9` reader - Channel 9 Overrun Interrupt Enable"]
pub type OVR9_R = crate::BitReader<bool>;
#[doc = "Field `OVR9` writer - Channel 9 Overrun Interrupt Enable"]
pub type OVR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OVR10` reader - Channel 10 Overrun Interrupt Enable"]
pub type OVR10_R = crate::BitReader<bool>;
#[doc = "Field `OVR10` writer - Channel 10 Overrun Interrupt Enable"]
pub type OVR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OVR11` reader - Channel 11 Overrun Interrupt Enable"]
pub type OVR11_R = crate::BitReader<bool>;
#[doc = "Field `OVR11` writer - Channel 11 Overrun Interrupt Enable"]
pub type OVR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `EVD8` reader - Channel 8 Event Detection Interrupt Enable"]
pub type EVD8_R = crate::BitReader<bool>;
#[doc = "Field `EVD8` writer - Channel 8 Event Detection Interrupt Enable"]
pub type EVD8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `EVD9` reader - Channel 9 Event Detection Interrupt Enable"]
pub type EVD9_R = crate::BitReader<bool>;
#[doc = "Field `EVD9` writer - Channel 9 Event Detection Interrupt Enable"]
pub type EVD9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `EVD10` reader - Channel 10 Event Detection Interrupt Enable"]
pub type EVD10_R = crate::BitReader<bool>;
#[doc = "Field `EVD10` writer - Channel 10 Event Detection Interrupt Enable"]
pub type EVD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `EVD11` reader - Channel 11 Event Detection Interrupt Enable"]
pub type EVD11_R = crate::BitReader<bool>;
#[doc = "Field `EVD11` writer - Channel 11 Event Detection Interrupt Enable"]
pub type EVD11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel 0 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr0(&self) -> OVR0_R {
        OVR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr4(&self) -> OVR4_R {
        OVR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr5(&self) -> OVR5_R {
        OVR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr6(&self) -> OVR6_R {
        OVR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr7(&self) -> OVR7_R {
        OVR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd0(&self) -> EVD0_R {
        EVD0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd1(&self) -> EVD1_R {
        EVD1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd2(&self) -> EVD2_R {
        EVD2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd3(&self) -> EVD3_R {
        EVD3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd4(&self) -> EVD4_R {
        EVD4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd5(&self) -> EVD5_R {
        EVD5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 6 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd6(&self) -> EVD6_R {
        EVD6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 7 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd7(&self) -> EVD7_R {
        EVD7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 8 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr8(&self) -> OVR8_R {
        OVR8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 9 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr9(&self) -> OVR9_R {
        OVR9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 10 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr10(&self) -> OVR10_R {
        OVR10_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 11 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr11(&self) -> OVR11_R {
        OVR11_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 8 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd8(&self) -> EVD8_R {
        EVD8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 9 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd9(&self) -> EVD9_R {
        EVD9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 10 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd10(&self) -> EVD10_R {
        EVD10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 11 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd11(&self) -> EVD11_R {
        EVD11_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr0(&mut self) -> OVR0_W<0> {
        OVR0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr1(&mut self) -> OVR1_W<1> {
        OVR1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr2(&mut self) -> OVR2_W<2> {
        OVR2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr3(&mut self) -> OVR3_W<3> {
        OVR3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr4(&mut self) -> OVR4_W<4> {
        OVR4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr5(&mut self) -> OVR5_W<5> {
        OVR5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr6(&mut self) -> OVR6_W<6> {
        OVR6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr7(&mut self) -> OVR7_W<7> {
        OVR7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 0 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd0(&mut self) -> EVD0_W<8> {
        EVD0_W::new(self)
    }
    #[doc = "Bit 9 - Channel 1 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd1(&mut self) -> EVD1_W<9> {
        EVD1_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd2(&mut self) -> EVD2_W<10> {
        EVD2_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd3(&mut self) -> EVD3_W<11> {
        EVD3_W::new(self)
    }
    #[doc = "Bit 12 - Channel 4 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd4(&mut self) -> EVD4_W<12> {
        EVD4_W::new(self)
    }
    #[doc = "Bit 13 - Channel 5 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd5(&mut self) -> EVD5_W<13> {
        EVD5_W::new(self)
    }
    #[doc = "Bit 14 - Channel 6 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd6(&mut self) -> EVD6_W<14> {
        EVD6_W::new(self)
    }
    #[doc = "Bit 15 - Channel 7 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd7(&mut self) -> EVD7_W<15> {
        EVD7_W::new(self)
    }
    #[doc = "Bit 16 - Channel 8 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr8(&mut self) -> OVR8_W<16> {
        OVR8_W::new(self)
    }
    #[doc = "Bit 17 - Channel 9 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr9(&mut self) -> OVR9_W<17> {
        OVR9_W::new(self)
    }
    #[doc = "Bit 18 - Channel 10 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr10(&mut self) -> OVR10_W<18> {
        OVR10_W::new(self)
    }
    #[doc = "Bit 19 - Channel 11 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr11(&mut self) -> OVR11_W<19> {
        OVR11_W::new(self)
    }
    #[doc = "Bit 24 - Channel 8 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd8(&mut self) -> EVD8_W<24> {
        EVD8_W::new(self)
    }
    #[doc = "Bit 25 - Channel 9 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd9(&mut self) -> EVD9_W<25> {
        EVD9_W::new(self)
    }
    #[doc = "Bit 26 - Channel 10 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd10(&mut self) -> EVD10_W<26> {
        EVD10_W::new(self)
    }
    #[doc = "Bit 27 - Channel 11 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd11(&mut self) -> EVD11_W<27> {
        EVD11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
