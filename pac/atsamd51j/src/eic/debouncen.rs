#[doc = "Register `DEBOUNCEN` reader"]
pub type R = crate::R<DEBOUNCEN_SPEC>;
#[doc = "Register `DEBOUNCEN` writer"]
pub type W = crate::W<DEBOUNCEN_SPEC>;
#[doc = "Field `DEBOUNCEN` reader - Debouncer Enable"]
pub type DEBOUNCEN_R = crate::FieldReader<u16>;
#[doc = "Field `DEBOUNCEN` writer - Debouncer Enable"]
pub type DEBOUNCEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Debouncer Enable"]
    #[inline(always)]
    pub fn debouncen(&self) -> DEBOUNCEN_R {
        DEBOUNCEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Debouncer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debouncen(&mut self) -> DEBOUNCEN_W<DEBOUNCEN_SPEC, 0> {
        DEBOUNCEN_W::new(self)
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
#[doc = "Debouncer Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debouncen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debouncen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBOUNCEN_SPEC;
impl crate::RegisterSpec for DEBOUNCEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debouncen::R`](R) reader structure"]
impl crate::Readable for DEBOUNCEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debouncen::W`](W) writer structure"]
impl crate::Writable for DEBOUNCEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBOUNCEN to value 0"]
impl crate::Resettable for DEBOUNCEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
