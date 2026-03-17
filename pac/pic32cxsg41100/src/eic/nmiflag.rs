#[doc = "Register `NMIFLAG` reader"]
pub type R = crate::R<NmiflagSpec>;
#[doc = "Register `NMIFLAG` writer"]
pub type W = crate::W<NmiflagSpec>;
#[doc = "Field `NMI` reader - Non-Maskable Interrupt"]
pub type NmiR = crate::BitReader;
#[doc = "Field `NMI` writer - Non-Maskable Interrupt"]
pub type NmiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Non-Maskable Interrupt"]
    #[inline(always)]
    pub fn nmi(&self) -> NmiR {
        NmiR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Maskable Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nmi(&mut self) -> NmiW<NmiflagSpec> {
        NmiW::new(self, 0)
    }
}
#[doc = "Non-Maskable Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiflagSpec;
impl crate::RegisterSpec for NmiflagSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nmiflag::R`](R) reader structure"]
impl crate::Readable for NmiflagSpec {}
#[doc = "`write(|w| ..)` method takes [`nmiflag::W`](W) writer structure"]
impl crate::Writable for NmiflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets NMIFLAG to value 0"]
impl crate::Resettable for NmiflagSpec {
    const RESET_VALUE: u16 = 0;
}
