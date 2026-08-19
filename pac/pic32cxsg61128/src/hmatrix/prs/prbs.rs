#[doc = "Register `PRBS` reader"]
pub type R = crate::R<PrbsSpec>;
#[doc = "Register `PRBS` writer"]
pub type W = crate::W<PrbsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority B for Slave\n\nYou can [`read`](crate::Reg::read) this register and get [`prbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrbsSpec;
impl crate::RegisterSpec for PrbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prbs::R`](R) reader structure"]
impl crate::Readable for PrbsSpec {}
#[doc = "`write(|w| ..)` method takes [`prbs::W`](W) writer structure"]
impl crate::Writable for PrbsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRBS to value 0"]
impl crate::Resettable for PrbsSpec {
    const RESET_VALUE: u32 = 0;
}
