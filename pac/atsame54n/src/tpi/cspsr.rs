#[doc = "Register `CSPSR` reader"]
pub type R = crate::R<CspsrSpec>;
#[doc = "Register `CSPSR` writer"]
pub type W = crate::W<CspsrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Current Parallel Port Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cspsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cspsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CspsrSpec;
impl crate::RegisterSpec for CspsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspsr::R`](R) reader structure"]
impl crate::Readable for CspsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cspsr::W`](W) writer structure"]
impl crate::Writable for CspsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSPSR to value 0"]
impl crate::Resettable for CspsrSpec {
    const RESET_VALUE: u32 = 0;
}
