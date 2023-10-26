#[doc = "Register `PERBUF` reader"]
pub type R = crate::R<PERBUF_SPEC>;
#[doc = "Register `PERBUF` writer"]
pub type W = crate::W<PERBUF_SPEC>;
#[doc = "Field `PERBUF` reader - Period Buffer Value"]
pub type PERBUF_R = crate::FieldReader;
#[doc = "Field `PERBUF` writer - Period Buffer Value"]
pub type PERBUF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&self) -> PERBUF_R {
        PERBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Period Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn perbuf(&mut self) -> PERBUF_W<PERBUF_SPEC, 0> {
        PERBUF_W::new(self)
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
#[doc = "COUNT8 Period Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERBUF_SPEC;
impl crate::RegisterSpec for PERBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`perbuf::R`](R) reader structure"]
impl crate::Readable for PERBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perbuf::W`](W) writer structure"]
impl crate::Writable for PERBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERBUF to value 0xff"]
impl crate::Resettable for PERBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
