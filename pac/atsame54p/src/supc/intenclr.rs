#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<INTENCLR_SPEC>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<INTENCLR_SPEC>;
#[doc = "Field `BOD33RDY` reader - BOD33 Ready"]
pub type BOD33RDY_R = crate::BitReader;
#[doc = "Field `BOD33RDY` writer - BOD33 Ready"]
pub type BOD33RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOD33DET` reader - BOD33 Detection"]
pub type BOD33DET_R = crate::BitReader;
#[doc = "Field `BOD33DET` writer - BOD33 Detection"]
pub type BOD33DET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready"]
pub type B33SRDY_R = crate::BitReader;
#[doc = "Field `B33SRDY` writer - BOD33 Synchronization Ready"]
pub type B33SRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VREGRDY` reader - Voltage Regulator Ready"]
pub type VREGRDY_R = crate::BitReader;
#[doc = "Field `VREGRDY` writer - Voltage Regulator Ready"]
pub type VREGRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VCORERDY` reader - VDDCORE Ready"]
pub type VCORERDY_R = crate::BitReader;
#[doc = "Field `VCORERDY` writer - VDDCORE Ready"]
pub type VCORERDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    pub fn vregrdy(&self) -> VREGRDY_R {
        VREGRDY_R::new(((self.bits >> 8) & 1) != 0)
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
    pub fn bod33rdy(&mut self) -> BOD33RDY_W<INTENCLR_SPEC, 0> {
        BOD33RDY_W::new(self)
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline(always)]
    #[must_use]
    pub fn bod33det(&mut self) -> BOD33DET_W<INTENCLR_SPEC, 1> {
        BOD33DET_W::new(self)
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn b33srdy(&mut self) -> B33SRDY_W<INTENCLR_SPEC, 2> {
        B33SRDY_W::new(self)
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    #[must_use]
    pub fn vregrdy(&mut self) -> VREGRDY_W<INTENCLR_SPEC, 8> {
        VREGRDY_W::new(self)
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline(always)]
    #[must_use]
    pub fn vcorerdy(&mut self) -> VCORERDY_W<INTENCLR_SPEC, 10> {
        VCORERDY_W::new(self)
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
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
