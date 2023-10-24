#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<INTENCLR_SPEC>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<INTENCLR_SPEC>;
#[doc = "Field `RESRDY` reader - Result Ready Interrupt Disable"]
pub type RESRDY_R = crate::BitReader;
#[doc = "Field `RESRDY` writer - Result Ready Interrupt Disable"]
pub type RESRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERRUN` reader - Overrun Interrupt Disable"]
pub type OVERRUN_R = crate::BitReader;
#[doc = "Field `OVERRUN` writer - Overrun Interrupt Disable"]
pub type OVERRUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WINMON` reader - Window Monitor Interrupt Disable"]
pub type WINMON_R = crate::BitReader;
#[doc = "Field `WINMON` writer - Window Monitor Interrupt Disable"]
pub type WINMON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Result Ready Interrupt Disable"]
    #[inline(always)]
    pub fn resrdy(&self) -> RESRDY_R {
        RESRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Interrupt Disable"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Monitor Interrupt Disable"]
    #[inline(always)]
    pub fn winmon(&self) -> WINMON_R {
        WINMON_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn resrdy(&mut self) -> RESRDY_W<INTENCLR_SPEC, 0> {
        RESRDY_W::new(self)
    }
    #[doc = "Bit 1 - Overrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OVERRUN_W<INTENCLR_SPEC, 1> {
        OVERRUN_W::new(self)
    }
    #[doc = "Bit 2 - Window Monitor Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn winmon(&mut self) -> WINMON_W<INTENCLR_SPEC, 2> {
        WINMON_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u8;
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
