#[doc = "Register `NMIFLAG` reader"]
pub type R = crate::R<NMIFLAG_SPEC>;
#[doc = "Register `NMIFLAG` writer"]
pub type W = crate::W<NMIFLAG_SPEC>;
#[doc = "Field `NMI` reader - Non-Maskable Interrupt"]
pub type NMI_R = crate::BitReader;
#[doc = "Field `NMI` writer - Non-Maskable Interrupt"]
pub type NMI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Non-Maskable Interrupt"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Maskable Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nmi(&mut self) -> NMI_W<NMIFLAG_SPEC, 0> {
        NMI_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Non-Maskable Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmiflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmiflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NMIFLAG_SPEC;
impl crate::RegisterSpec for NMIFLAG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nmiflag::R`](R) reader structure"]
impl crate::Readable for NMIFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nmiflag::W`](W) writer structure"]
impl crate::Writable for NMIFLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NMIFLAG to value 0"]
impl crate::Resettable for NMIFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
