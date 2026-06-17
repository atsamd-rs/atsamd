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
#[doc = "Field `FC0O` reader - Frame Counter 0 Overflow Interrupt Enable"]
pub type FC0O_R = crate::BitReader<bool>;
#[doc = "Field `FC0O` writer - Frame Counter 0 Overflow Interrupt Enable"]
pub type FC0O_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, O>;
#[doc = "Field `FC1O` reader - Frame Counter 1 Overflow Interrupt Enable"]
pub type FC1O_R = crate::BitReader<bool>;
#[doc = "Field `FC1O` writer - Frame Counter 1 Overflow Interrupt Enable"]
pub type FC1O_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, O>;
#[doc = "Field `FC2O` reader - Frame Counter 2 Overflow Interrupt Enable"]
pub type FC2O_R = crate::BitReader<bool>;
#[doc = "Field `FC2O` writer - Frame Counter 2 Overflow Interrupt Enable"]
pub type FC2O_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, O>;
#[doc = "Field `VLCDRT` reader - VLCD Ready Toggle Interrupt Enable"]
pub type VLCDRT_R = crate::BitReader<bool>;
#[doc = "Field `VLCDRT` writer - VLCD Ready Toggle Interrupt Enable"]
pub type VLCDRT_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, O>;
#[doc = "Field `VLCDST` reader - VLCD Status Toggle Interrupt Enable"]
pub type VLCDST_R = crate::BitReader<bool>;
#[doc = "Field `VLCDST` writer - VLCD Status Toggle Interrupt Enable"]
pub type VLCDST_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, O>;
#[doc = "Field `PRST` reader - Pump Run Status Toggle Interrupt Enable"]
pub type PRST_R = crate::BitReader<bool>;
#[doc = "Field `PRST` writer - Pump Run Status Toggle Interrupt Enable"]
pub type PRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Frame Counter 0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fc0o(&self) -> FC0O_R {
        FC0O_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Counter 1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fc1o(&self) -> FC1O_R {
        FC1O_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame Counter 2 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fc2o(&self) -> FC2O_R {
        FC2O_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VLCD Ready Toggle Interrupt Enable"]
    #[inline(always)]
    pub fn vlcdrt(&self) -> VLCDRT_R {
        VLCDRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VLCD Status Toggle Interrupt Enable"]
    #[inline(always)]
    pub fn vlcdst(&self) -> VLCDST_R {
        VLCDST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pump Run Status Toggle Interrupt Enable"]
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Counter 0 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc0o(&mut self) -> FC0O_W<0> {
        FC0O_W::new(self)
    }
    #[doc = "Bit 1 - Frame Counter 1 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc1o(&mut self) -> FC1O_W<1> {
        FC1O_W::new(self)
    }
    #[doc = "Bit 2 - Frame Counter 2 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc2o(&mut self) -> FC2O_W<2> {
        FC2O_W::new(self)
    }
    #[doc = "Bit 3 - VLCD Ready Toggle Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vlcdrt(&mut self) -> VLCDRT_W<3> {
        VLCDRT_W::new(self)
    }
    #[doc = "Bit 4 - VLCD Status Toggle Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vlcdst(&mut self) -> VLCDST_W<4> {
        VLCDST_W::new(self)
    }
    #[doc = "Bit 5 - Pump Run Status Toggle Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prst(&mut self) -> PRST_W<5> {
        PRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u8;
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
