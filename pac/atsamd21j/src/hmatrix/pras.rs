#[doc = "Register `PRAS%s` reader"]
pub type R = crate::R<PrasSpec>;
#[doc = "Register `PRAS%s` writer"]
pub type W = crate::W<PrasSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority A for Slave\n\nYou can [`read`](crate::Reg::read) this register and get [`pras::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrasSpec;
impl crate::RegisterSpec for PrasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pras::R`](R) reader structure"]
impl crate::Readable for PrasSpec {}
#[doc = "`write(|w| ..)` method takes [`pras::W`](W) writer structure"]
impl crate::Writable for PrasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRAS%s to value 0"]
impl crate::Resettable for PrasSpec {}
