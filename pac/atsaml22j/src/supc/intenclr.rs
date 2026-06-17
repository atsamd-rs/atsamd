#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOD33RDY` reader - BOD33 Ready"]
pub type BOD33RDY_R = crate::BitReader<bool>;
#[doc = "Field `BOD33RDY` writer - BOD33 Ready"]
pub type BOD33RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `BOD33DET` reader - BOD33 Detection"]
pub type BOD33DET_R = crate::BitReader<bool>;
#[doc = "Field `BOD33DET` writer - BOD33 Detection"]
pub type BOD33DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready"]
pub type B33SRDY_R = crate::BitReader<bool>;
#[doc = "Field `B33SRDY` writer - BOD33 Synchronization Ready"]
pub type B33SRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `BOD12RDY` reader - BOD12 Ready"]
pub type BOD12RDY_R = crate::BitReader<bool>;
#[doc = "Field `BOD12RDY` writer - BOD12 Ready"]
pub type BOD12RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `BOD12DET` reader - BOD12 Detection"]
pub type BOD12DET_R = crate::BitReader<bool>;
#[doc = "Field `BOD12DET` writer - BOD12 Detection"]
pub type BOD12DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `B12SRDY` reader - BOD12 Synchronization Ready"]
pub type B12SRDY_R = crate::BitReader<bool>;
#[doc = "Field `B12SRDY` writer - BOD12 Synchronization Ready"]
pub type B12SRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `VREGRDY` reader - Voltage Regulator Ready"]
pub type VREGRDY_R = crate::BitReader<bool>;
#[doc = "Field `VREGRDY` writer - Voltage Regulator Ready"]
pub type VREGRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `APWSRDY` reader - Automatic Power Switch Ready"]
pub type APWSRDY_R = crate::BitReader<bool>;
#[doc = "Field `APWSRDY` writer - Automatic Power Switch Ready"]
pub type APWSRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `VCORERDY` reader - VDDCORE Ready"]
pub type VCORERDY_R = crate::BitReader<bool>;
#[doc = "Field `VCORERDY` writer - VDDCORE Ready"]
pub type VCORERDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> BOD33RDY_R {
        BOD33RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33SRDY_R {
        B33SRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BOD12 Ready"]
    #[inline(always)]
    pub fn bod12rdy(&self) -> BOD12RDY_R {
        BOD12RDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BOD12 Detection"]
    #[inline(always)]
    pub fn bod12det(&self) -> BOD12DET_R {
        BOD12DET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BOD12 Synchronization Ready"]
    #[inline(always)]
    pub fn b12srdy(&self) -> B12SRDY_R {
        B12SRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    pub fn vregrdy(&self) -> VREGRDY_R {
        VREGRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Automatic Power Switch Ready"]
    #[inline(always)]
    pub fn apwsrdy(&self) -> APWSRDY_R {
        APWSRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline(always)]
    pub fn vcorerdy(&self) -> VCORERDY_R {
        VCORERDY_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOD33 Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bod33rdy(&mut self) -> BOD33RDY_W<0> {
        BOD33RDY_W::new(self)
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline(always)]
    #[must_use]
    pub fn bod33det(&mut self) -> BOD33DET_W<1> {
        BOD33DET_W::new(self)
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn b33srdy(&mut self) -> B33SRDY_W<2> {
        B33SRDY_W::new(self)
    }
    #[doc = "Bit 3 - BOD12 Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bod12rdy(&mut self) -> BOD12RDY_W<3> {
        BOD12RDY_W::new(self)
    }
    #[doc = "Bit 4 - BOD12 Detection"]
    #[inline(always)]
    #[must_use]
    pub fn bod12det(&mut self) -> BOD12DET_W<4> {
        BOD12DET_W::new(self)
    }
    #[doc = "Bit 5 - BOD12 Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn b12srdy(&mut self) -> B12SRDY_W<5> {
        B12SRDY_W::new(self)
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    #[must_use]
    pub fn vregrdy(&mut self) -> VREGRDY_W<8> {
        VREGRDY_W::new(self)
    }
    #[doc = "Bit 9 - Automatic Power Switch Ready"]
    #[inline(always)]
    #[must_use]
    pub fn apwsrdy(&mut self) -> APWSRDY_W<9> {
        APWSRDY_W::new(self)
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline(always)]
    #[must_use]
    pub fn vcorerdy(&mut self) -> VCORERDY_W<10> {
        VCORERDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
