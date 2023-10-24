#[doc = "Register `RXPL` reader"]
pub type R = crate::R<RXPL_SPEC>;
#[doc = "Register `RXPL` writer"]
pub type W = crate::W<RXPL_SPEC>;
#[doc = "Field `RXPL` reader - Receive Pulse Length"]
pub type RXPL_R = crate::FieldReader;
#[doc = "Field `RXPL` writer - Receive Pulse Length"]
pub type RXPL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive Pulse Length"]
    #[inline(always)]
    pub fn rxpl(&self) -> RXPL_R {
        RXPL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Pulse Length"]
    #[inline(always)]
    #[must_use]
    pub fn rxpl(&mut self) -> RXPL_W<RXPL_SPEC, 0> {
        RXPL_W::new(self)
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
#[doc = "USART_EXT Receive Pulse Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxpl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxpl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXPL_SPEC;
impl crate::RegisterSpec for RXPL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxpl::R`](R) reader structure"]
impl crate::Readable for RXPL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxpl::W`](W) writer structure"]
impl crate::Writable for RXPL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXPL to value 0"]
impl crate::Resettable for RXPL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
