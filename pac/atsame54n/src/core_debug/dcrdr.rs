#[doc = "Register `DCRDR` reader"]
pub type R = crate::R<DcrdrSpec>;
#[doc = "Register `DCRDR` writer"]
pub type W = crate::W<DcrdrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Debug Core Register Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrdrSpec;
impl crate::RegisterSpec for DcrdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcrdr::R`](R) reader structure"]
impl crate::Readable for DcrdrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcrdr::W`](W) writer structure"]
impl crate::Writable for DcrdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCRDR to value 0"]
impl crate::Resettable for DcrdrSpec {}
