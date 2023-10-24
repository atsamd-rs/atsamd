#[doc = "Register `SPPR` reader"]
pub type R = crate::R<SPPR_SPEC>;
#[doc = "Register `SPPR` writer"]
pub type W = crate::W<SPPR_SPEC>;
#[doc = "Field `TXMODE` reader - "]
pub type TXMODE_R = crate::FieldReader;
#[doc = "Field `TXMODE` writer - "]
pub type TXMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn txmode(&mut self) -> TXMODE_W<SPPR_SPEC, 0> {
        TXMODE_W::new(self)
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
#[doc = "Selected Pin Protocol Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sppr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sppr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPPR_SPEC;
impl crate::RegisterSpec for SPPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sppr::R`](R) reader structure"]
impl crate::Readable for SPPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sppr::W`](W) writer structure"]
impl crate::Writable for SPPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPPR to value 0"]
impl crate::Resettable for SPPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
