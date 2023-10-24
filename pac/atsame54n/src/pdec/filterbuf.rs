#[doc = "Register `FILTERBUF` reader"]
pub type R = crate::R<FILTERBUF_SPEC>;
#[doc = "Register `FILTERBUF` writer"]
pub type W = crate::W<FILTERBUF_SPEC>;
#[doc = "Field `FILTERBUF` reader - Filter Buffer Value"]
pub type FILTERBUF_R = crate::FieldReader;
#[doc = "Field `FILTERBUF` writer - Filter Buffer Value"]
pub type FILTERBUF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Filter Buffer Value"]
    #[inline(always)]
    pub fn filterbuf(&self) -> FILTERBUF_R {
        FILTERBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn filterbuf(&mut self) -> FILTERBUF_W<FILTERBUF_SPEC, 0> {
        FILTERBUF_W::new(self)
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
#[doc = "Filter Buffer Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filterbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filterbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTERBUF_SPEC;
impl crate::RegisterSpec for FILTERBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`filterbuf::R`](R) reader structure"]
impl crate::Readable for FILTERBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filterbuf::W`](W) writer structure"]
impl crate::Writable for FILTERBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTERBUF to value 0"]
impl crate::Resettable for FILTERBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
