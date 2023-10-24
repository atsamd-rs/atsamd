#[doc = "Register `BFAR` reader"]
pub type R = crate::R<BFAR_SPEC>;
#[doc = "Register `BFAR` writer"]
pub type W = crate::W<BFAR_SPEC>;
#[doc = "Field `ADDRESS` reader - Address that generated the BusFault"]
pub type ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - Address that generated the BusFault"]
pub type ADDRESS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Address that generated the BusFault"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address that generated the BusFault"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<BFAR_SPEC, 0> {
        ADDRESS_W::new(self)
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
#[doc = "BusFault Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BFAR_SPEC;
impl crate::RegisterSpec for BFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bfar::R`](R) reader structure"]
impl crate::Readable for BFAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bfar::W`](W) writer structure"]
impl crate::Writable for BFAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFAR to value 0"]
impl crate::Resettable for BFAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
