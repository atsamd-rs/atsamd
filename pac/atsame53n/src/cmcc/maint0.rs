#[doc = "Register `MAINT0` writer"]
pub type W = crate::W<Maint0Spec>;
#[doc = "Field `INVALL` writer - Cache Controller invalidate All"]
pub type InvallW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Cache Controller invalidate All"]
    #[inline(always)]
    pub fn invall(&mut self) -> InvallW<Maint0Spec> {
        InvallW::new(self, 0)
    }
}
#[doc = "Cache Maintenance Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maint0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maint0Spec;
impl crate::RegisterSpec for Maint0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`maint0::W`](W) writer structure"]
impl crate::Writable for Maint0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAINT0 to value 0"]
impl crate::Resettable for Maint0Spec {}
