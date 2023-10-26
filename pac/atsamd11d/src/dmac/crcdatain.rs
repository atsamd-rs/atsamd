#[doc = "Register `CRCDATAIN` reader"]
pub type R = crate::R<CRCDATAIN_SPEC>;
#[doc = "Register `CRCDATAIN` writer"]
pub type W = crate::W<CRCDATAIN_SPEC>;
#[doc = "Field `CRCDATAIN` reader - CRC Data Input"]
pub type CRCDATAIN_R = crate::FieldReader<u32>;
#[doc = "Field `CRCDATAIN` writer - CRC Data Input"]
pub type CRCDATAIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Data Input"]
    #[inline(always)]
    pub fn crcdatain(&self) -> CRCDATAIN_R {
        CRCDATAIN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Data Input"]
    #[inline(always)]
    #[must_use]
    pub fn crcdatain(&mut self) -> CRCDATAIN_W<CRCDATAIN_SPEC, 0> {
        CRCDATAIN_W::new(self)
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
#[doc = "CRC Data Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdatain::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdatain::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCDATAIN_SPEC;
impl crate::RegisterSpec for CRCDATAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcdatain::R`](R) reader structure"]
impl crate::Readable for CRCDATAIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcdatain::W`](W) writer structure"]
impl crate::Writable for CRCDATAIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDATAIN to value 0"]
impl crate::Resettable for CRCDATAIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
