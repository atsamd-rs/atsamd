#[doc = "Register `BDPR` reader"]
pub type R = crate::R<BDPR_SPEC>;
#[doc = "Register `BDPR` writer"]
pub type W = crate::W<BDPR_SPEC>;
#[doc = "Field `BUFDATA` reader - Buffer Data"]
pub type BUFDATA_R = crate::FieldReader<u32>;
#[doc = "Field `BUFDATA` writer - Buffer Data"]
pub type BUFDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdata(&self) -> BUFDATA_R {
        BUFDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    #[must_use]
    pub fn bufdata(&mut self) -> BUFDATA_W<BDPR_SPEC, 0> {
        BUFDATA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Buffer Data Port\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDPR_SPEC;
impl crate::RegisterSpec for BDPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdpr::R`](R) reader structure"]
impl crate::Readable for BDPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bdpr::W`](W) writer structure"]
impl crate::Writable for BDPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDPR to value 0"]
impl crate::Resettable for BDPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
