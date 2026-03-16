#[doc = "Register `DEBOUNCEN` reader"]
pub type R = crate::R<DebouncenSpec>;
#[doc = "Register `DEBOUNCEN` writer"]
pub type W = crate::W<DebouncenSpec>;
#[doc = "Field `DEBOUNCEN` reader - Debouncer Enable"]
pub type DebouncenR = crate::FieldReader<u16>;
#[doc = "Field `DEBOUNCEN` writer - Debouncer Enable"]
pub type DebouncenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Debouncer Enable"]
    #[inline(always)]
    pub fn debouncen(&self) -> DebouncenR {
        DebouncenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Debouncer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debouncen(&mut self) -> DebouncenW<DebouncenSpec> {
        DebouncenW::new(self, 0)
    }
}
#[doc = "Debouncer Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`debouncen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debouncen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebouncenSpec;
impl crate::RegisterSpec for DebouncenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debouncen::R`](R) reader structure"]
impl crate::Readable for DebouncenSpec {}
#[doc = "`write(|w| ..)` method takes [`debouncen::W`](W) writer structure"]
impl crate::Writable for DebouncenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBOUNCEN to value 0"]
impl crate::Resettable for DebouncenSpec {
    const RESET_VALUE: u32 = 0;
}
