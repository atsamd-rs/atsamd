#[doc = "Register `ASAR[%s]` reader"]
pub type R = crate::R<ASAR_SPEC>;
#[doc = "Register `ASAR[%s]` writer"]
pub type W = crate::W<ASAR_SPEC>;
#[doc = "Field `ADMASA` reader - ADMA System Address"]
pub type ADMASA_R = crate::FieldReader<u32>;
#[doc = "Field `ADMASA` writer - ADMA System Address"]
pub type ADMASA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - ADMA System Address"]
    #[inline(always)]
    pub fn admasa(&self) -> ADMASA_R {
        ADMASA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA System Address"]
    #[inline(always)]
    #[must_use]
    pub fn admasa(&mut self) -> ADMASA_W<ASAR_SPEC, 0> {
        ADMASA_W::new(self)
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
#[doc = "ADMA System Address n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASAR_SPEC;
impl crate::RegisterSpec for ASAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asar::R`](R) reader structure"]
impl crate::Readable for ASAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asar::W`](W) writer structure"]
impl crate::Writable for ASAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASAR[%s]
to value 0"]
impl crate::Resettable for ASAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
