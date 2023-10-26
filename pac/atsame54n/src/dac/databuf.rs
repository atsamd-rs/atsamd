#[doc = "Register `DATABUF[%s]` writer"]
pub type W = crate::W<DATABUF_SPEC>;
#[doc = "Field `DATABUF` writer - DAC0 Data Buffer"]
pub type DATABUF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - DAC0 Data Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn databuf(&mut self) -> DATABUF_W<DATABUF_SPEC, 0> {
        DATABUF_W::new(self)
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
#[doc = "DAC n Data Buffer\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`databuf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATABUF_SPEC;
impl crate::RegisterSpec for DATABUF_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`databuf::W`](W) writer structure"]
impl crate::Writable for DATABUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATABUF[%s]
to value 0"]
impl crate::Resettable for DATABUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
