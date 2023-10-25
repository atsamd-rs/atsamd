#[doc = "Register `BASEADDR` reader"]
pub type R = crate::R<BASEADDR_SPEC>;
#[doc = "Register `BASEADDR` writer"]
pub type W = crate::W<BASEADDR_SPEC>;
#[doc = "Field `BASEADDR` reader - Descriptor Memory Base Address"]
pub type BASEADDR_R = crate::FieldReader<u32>;
#[doc = "Field `BASEADDR` writer - Descriptor Memory Base Address"]
pub type BASEADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Descriptor Memory Base Address"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor Memory Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn baseaddr(&mut self) -> BASEADDR_W<BASEADDR_SPEC, 0> {
        BASEADDR_W::new(self)
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
#[doc = "Descriptor Memory Section Base Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baseaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baseaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BASEADDR_SPEC;
impl crate::RegisterSpec for BASEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baseaddr::R`](R) reader structure"]
impl crate::Readable for BASEADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baseaddr::W`](W) writer structure"]
impl crate::Writable for BASEADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BASEADDR to value 0"]
impl crate::Resettable for BASEADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
