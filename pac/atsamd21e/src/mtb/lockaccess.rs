#[doc = "Register `LOCKACCESS` reader"]
pub type R = crate::R<LockaccessSpec>;
#[doc = "Register `LOCKACCESS` writer"]
pub type W = crate::W<LockaccessSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "MTB Lock Access\n\nYou can [`read`](crate::Reg::read) this register and get [`lockaccess::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockaccess::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockaccessSpec;
impl crate::RegisterSpec for LockaccessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockaccess::R`](R) reader structure"]
impl crate::Readable for LockaccessSpec {}
#[doc = "`write(|w| ..)` method takes [`lockaccess::W`](W) writer structure"]
impl crate::Writable for LockaccessSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCKACCESS to value 0"]
impl crate::Resettable for LockaccessSpec {
    const RESET_VALUE: u32 = 0;
}
