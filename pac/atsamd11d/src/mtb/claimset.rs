#[doc = "Register `CLAIMSET` reader"]
pub type R = crate::R<ClaimsetSpec>;
#[doc = "Register `CLAIMSET` writer"]
pub type W = crate::W<ClaimsetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "MTB Claim Set\n\nYou can [`read`](crate::Reg::read) this register and get [`claimset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClaimsetSpec;
impl crate::RegisterSpec for ClaimsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claimset::R`](R) reader structure"]
impl crate::Readable for ClaimsetSpec {}
#[doc = "`write(|w| ..)` method takes [`claimset::W`](W) writer structure"]
impl crate::Writable for ClaimsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLAIMSET to value 0"]
impl crate::Resettable for ClaimsetSpec {
    const RESET_VALUE: u32 = 0;
}
