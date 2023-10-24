#[doc = "Register `BAUD_FRAC_MODE` reader"]
pub type R = crate::R<BAUD_FRAC_MODE_SPEC>;
#[doc = "Register `BAUD_FRAC_MODE` writer"]
pub type W = crate::W<BAUD_FRAC_MODE_SPEC>;
#[doc = "Field `BAUD` reader - Baud Rate Value"]
pub type BAUD_R = crate::FieldReader<u16>;
#[doc = "Field `BAUD` writer - Baud Rate Value"]
pub type BAUD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
#[doc = "Field `FP` reader - Fractional Part"]
pub type FP_R = crate::FieldReader;
#[doc = "Field `FP` writer - Fractional Part"]
pub type FP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new(self.bits & 0x1fff)
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FP_R {
        FP_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BAUD_W<BAUD_FRAC_MODE_SPEC, 0> {
        BAUD_W::new(self)
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline(always)]
    #[must_use]
    pub fn fp(&mut self) -> FP_W<BAUD_FRAC_MODE_SPEC, 13> {
        FP_W::new(self)
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
#[doc = "USART_EXT Baud Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud_frac_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud_frac_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BAUD_FRAC_MODE_SPEC;
impl crate::RegisterSpec for BAUD_FRAC_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`baud_frac_mode::R`](R) reader structure"]
impl crate::Readable for BAUD_FRAC_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baud_frac_mode::W`](W) writer structure"]
impl crate::Writable for BAUD_FRAC_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUD_FRAC_MODE to value 0"]
impl crate::Resettable for BAUD_FRAC_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
