#[doc = "Register `WRBADDR` reader"]
pub type R = crate::R<WRBADDR_SPEC>;
#[doc = "Register `WRBADDR` writer"]
pub type W = crate::W<WRBADDR_SPEC>;
#[doc = "Field `WRBADDR` reader - Write-Back Memory Base Address"]
pub type WRBADDR_R = crate::FieldReader<u32>;
#[doc = "Field `WRBADDR` writer - Write-Back Memory Base Address"]
pub type WRBADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Write-Back Memory Base Address"]
    #[inline(always)]
    pub fn wrbaddr(&self) -> WRBADDR_R {
        WRBADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write-Back Memory Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn wrbaddr(&mut self) -> WRBADDR_W<WRBADDR_SPEC, 0> {
        WRBADDR_W::new(self)
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
#[doc = "Write-Back Memory Section Base Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrbaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrbaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRBADDR_SPEC;
impl crate::RegisterSpec for WRBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrbaddr::R`](R) reader structure"]
impl crate::Readable for WRBADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wrbaddr::W`](W) writer structure"]
impl crate::Writable for WRBADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRBADDR to value 0"]
impl crate::Resettable for WRBADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
