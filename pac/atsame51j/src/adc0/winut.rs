#[doc = "Register `WINUT` reader"]
pub type R = crate::R<WINUT_SPEC>;
#[doc = "Register `WINUT` writer"]
pub type W = crate::W<WINUT_SPEC>;
#[doc = "Field `WINUT` reader - Window Upper Threshold"]
pub type WINUT_R = crate::FieldReader<u16>;
#[doc = "Field `WINUT` writer - Window Upper Threshold"]
pub type WINUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Window Upper Threshold"]
    #[inline(always)]
    pub fn winut(&self) -> WINUT_R {
        WINUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Window Upper Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn winut(&mut self) -> WINUT_W<WINUT_SPEC, 0> {
        WINUT_W::new(self)
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
#[doc = "Window Monitor Upper Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winut::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`winut::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WINUT_SPEC;
impl crate::RegisterSpec for WINUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`winut::R`](R) reader structure"]
impl crate::Readable for WINUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`winut::W`](W) writer structure"]
impl crate::Writable for WINUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WINUT to value 0"]
impl crate::Resettable for WINUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
