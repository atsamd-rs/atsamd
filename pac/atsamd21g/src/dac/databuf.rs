#[doc = "Register `DATABUF` reader"]
pub type R = crate::R<DATABUF_SPEC>;
#[doc = "Register `DATABUF` writer"]
pub type W = crate::W<DATABUF_SPEC>;
#[doc = "Field `DATABUF` reader - Data Buffer"]
pub type DATABUF_R = crate::FieldReader<u16>;
#[doc = "Field `DATABUF` writer - Data Buffer"]
pub type DATABUF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Buffer"]
    #[inline(always)]
    pub fn databuf(&self) -> DATABUF_R {
        DATABUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Buffer"]
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
#[doc = "Data Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`databuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`databuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATABUF_SPEC;
impl crate::RegisterSpec for DATABUF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`databuf::R`](R) reader structure"]
impl crate::Readable for DATABUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`databuf::W`](W) writer structure"]
impl crate::Writable for DATABUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATABUF to value 0"]
impl crate::Resettable for DATABUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
