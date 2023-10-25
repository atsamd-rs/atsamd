#[doc = "Register `LCKWAY` reader"]
pub type R = crate::R<LCKWAY_SPEC>;
#[doc = "Register `LCKWAY` writer"]
pub type W = crate::W<LCKWAY_SPEC>;
#[doc = "Field `LCKWAY` reader - Lockdown way Register"]
pub type LCKWAY_R = crate::FieldReader;
#[doc = "Field `LCKWAY` writer - Lockdown way Register"]
pub type LCKWAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Lockdown way Register"]
    #[inline(always)]
    pub fn lckway(&self) -> LCKWAY_R {
        LCKWAY_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Lockdown way Register"]
    #[inline(always)]
    #[must_use]
    pub fn lckway(&mut self) -> LCKWAY_W<LCKWAY_SPEC, 0> {
        LCKWAY_W::new(self)
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
#[doc = "Cache Lock per Way Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckway::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckway::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCKWAY_SPEC;
impl crate::RegisterSpec for LCKWAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lckway::R`](R) reader structure"]
impl crate::Readable for LCKWAY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lckway::W`](W) writer structure"]
impl crate::Writable for LCKWAY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCKWAY to value 0"]
impl crate::Resettable for LCKWAY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
