#[doc = "Register `FFLR` reader"]
pub type R = crate::R<FflrSpec>;
#[doc = "Register `FFLR` writer"]
pub type W = crate::W<FflrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ETM FIFO Full Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fflr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fflr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FflrSpec;
impl crate::RegisterSpec for FflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fflr::R`](R) reader structure"]
impl crate::Readable for FflrSpec {}
#[doc = "`write(|w| ..)` method takes [`fflr::W`](W) writer structure"]
impl crate::Writable for FflrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFLR to value 0"]
impl crate::Resettable for FflrSpec {
    const RESET_VALUE: u32 = 0;
}
