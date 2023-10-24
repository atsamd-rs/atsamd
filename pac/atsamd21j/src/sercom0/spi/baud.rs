#[doc = "Register `BAUD` reader"]
pub type R = crate::R<BAUD_SPEC>;
#[doc = "Register `BAUD` writer"]
pub type W = crate::W<BAUD_SPEC>;
#[doc = "Field `BAUD` reader - Baud Rate Value"]
pub type BAUD_R = crate::FieldReader;
#[doc = "Field `BAUD` writer - Baud Rate Value"]
pub type BAUD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud Rate Value"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BAUD_W<BAUD_SPEC, 0> {
        BAUD_W::new(self)
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
#[doc = "SPI Baud Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BAUD_SPEC;
impl crate::RegisterSpec for BAUD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`baud::R`](R) reader structure"]
impl crate::Readable for BAUD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baud::W`](W) writer structure"]
impl crate::Writable for BAUD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUD to value 0"]
impl crate::Resettable for BAUD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
